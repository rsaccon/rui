use {
    crate::*,
    std::{collections::VecDeque, sync::Mutex},
    tao::event_loop::EventLoopProxy,
};

pub(crate) type WorkQueue = VecDeque<Box<dyn FnOnce(&mut Context) + Send>>;

lazy_static! {
    /// Allows us to wake the event loop whenever we want.
    pub(crate) static ref GLOBAL_EVENT_LOOP_PROXY: Mutex<Option<EventLoopProxy<()>>> = Mutex::new(None);

    pub(crate) static ref GLOBAL_WORK_QUEUE: Mutex<WorkQueue> = Mutex::new(WorkQueue::new());
}

fn wake_event_loop() {
    // Wake up the event loop.
    let opt_proxy = GLOBAL_EVENT_LOOP_PROXY.lock().unwrap();
    if let Some(proxy) = &*opt_proxy {
        if let Err(err) = proxy.send_event(()) {
            println!("error waking up event loop: {:?}", err);
        }
    }
}

pub fn on_main(f: impl FnOnce(&mut Context) + Send + 'static) {
    GLOBAL_WORK_QUEUE.lock().unwrap().push_back(Box::new(f));
    wake_event_loop();
}

/// Weak reference to app state.
pub struct State<S> {
    pub(crate) id: ViewId,
    phantom: std::marker::PhantomData<S>,
}

impl<S> Copy for State<S> {}

impl<S> Clone for State<S> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S: 'static> State<S> {
    pub fn new(id: ViewId) -> Self {
        Self {
            id,
            phantom: Default::default(),
        }
    }

    /// Makes it convenient to get a function to set the value.
    pub fn setter(self) -> impl Fn(S, &mut Context) {
        move |s, cx| cx[self] = s
    }
}

impl<S: 'static> Binding<S> for State<S> {
    fn get<'a>(&self, cx: &'a mut Context) -> &'a S {
        cx.get(*self)
    }
    fn get_mut<'a>(&self, cx: &'a mut Context) -> &'a mut S {
        cx.get_mut(*self)
    }
}

struct StateView<D, F> {
    default: D,
    func: F,
}

impl<S, V, D, F> View for StateView<D, F>
where
    V: View,
    S: 'static,
    D: Fn() -> S + 'static,
    F: Fn(State<S>, &mut Context) -> V + 'static,
{
    fn print(&self, id: ViewId, cx: &mut Context) {
        cx.init_state(id, &self.default);
        (self.func)(State::new(id), cx).print(id.child(&0), cx);
    }

    fn process(&self, event: &Event, id: ViewId, cx: &mut Context, vger: &mut Vger) {
        cx.init_state(id, &self.default);
        (self.func)(State::new(id), cx).process(event, id.child(&0), cx, vger);
    }

    fn draw(&self, id: ViewId, cx: &mut Context, vger: &mut Vger) {
        cx.init_state(id, &self.default);
        (self.func)(State::new(id), cx).draw(id.child(&0), cx, vger);
    }

    fn layout(&self, id: ViewId, sz: LocalSize, cx: &mut Context, vger: &mut Vger) -> LocalSize {
        cx.init_state(id, &self.default);

        let child_size = (self.func)(State::new(id), cx).layout(id.child(&0), sz, cx, vger);

        cx.layout.insert(
            id,
            LayoutBox {
                rect: LocalRect::new(LocalPoint::zero(), child_size),
                offset: LocalOffset::zero(),
            },
        );

        child_size
    }

    fn dirty(
        &self,
        id: ViewId,
        xform: LocalToWorld,
        cx: &mut Context,
        region: &mut Region<WorldSpace>,
    ) {
        let default = &self.default;
        let holder = cx.state_map.entry(id).or_insert_with(|| StateHolder {
            state: Box::new((default)()),
            dirty: false,
        });

        if holder.dirty {
            // Add a region.
            let rect = cx.layout[&id].rect;
            let pts: [LocalPoint; 4] = [
                rect.min(),
                [rect.max_x(), rect.min_y()].into(),
                [rect.min_x(), rect.max_y()].into(),
                rect.max(),
            ];
            let world_pts = pts.map(|p| xform.transform_point(p));
            region.add_rect(WorldRect::from_points(world_pts));
        } else {
            (self.func)(State::new(id), cx).dirty(id.child(&0), xform, cx, region);
        }
    }

    fn hittest(
        &self,
        id: ViewId,
        pt: LocalPoint,
        cx: &mut Context,
        vger: &mut Vger,
    ) -> Option<ViewId> {
        cx.init_state(id, &self.default);
        (self.func)(State::new(id), cx).hittest(id.child(&0), pt, cx, vger)
    }

    fn commands(&self, id: ViewId, cx: &mut Context, cmds: &mut Vec<CommandInfo>) {
        cx.init_state(id, &self.default);
        (self.func)(State::new(id), cx).commands(id.child(&0), cx, cmds);
    }

    fn gc(&self, id: ViewId, cx: &mut Context, map: &mut Vec<ViewId>) {
        cx.init_state(id, &self.default);
        map.push(id);
        (self.func)(State::new(id), cx).gc(id.child(&0), cx, map);
    }

    fn access(
        &self,
        id: ViewId,
        cx: &mut Context,
        nodes: &mut Vec<accesskit::Node>,
    ) -> Option<accesskit::NodeId> {
        cx.init_state(id, &self.default);
        (self.func)(State::new(id), cx).access(id.child(&0), cx, nodes)
    }
}

impl<S, F> private::Sealed for StateView<S, F> {}

/// State allows you to associate some state with a view.
/// This is what you'll use for a data model, as well as per-view state.
/// Your state should be efficiently clonable. Use Rc as necessary.
///
/// `initial` is the initial value for your state.
///
/// `f` callback which is passed a `State<S>`
pub fn state<
    S: 'static,
    V: View,
    D: Fn() -> S + 'static,
    F: Fn(State<S>, &mut Context) -> V + 'static,
>(
    initial: D,
    f: F,
) -> impl View {
    StateView {
        default: initial,
        func: f,
    }
}
