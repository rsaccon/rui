use rui::{
    vertices::{Vertex, Vertices},
    *,
};
use tao::keyboard::ModifiersState;

struct AppState {
    // mesh: Vertices,
}

impl AppState {
    fn left(&mut self) {
        // self.translate.x = 0.05;
        // self.rotate.axis_y_angle = 0.3;
    }
    fn right(&mut self) {
        // self.translate.x = -0.05;
        // self.rotate.axis_y_angle = -0.3;
    }
    fn up(&mut self) {
        // self.translate.y = -0.05;
        // self.rotate.axis_x_angle = 0.3;
    }
    fn down(&mut self) {
        // self.translate.y = 0.05;
        // self.rotate.axis_x_angle = -0.3;
    }
    fn key(&mut self, key: &KeyPress, _mods: &ModifiersState) {
        match key {
            KeyPress::ArrowLeft => self.left(),
            KeyPress::ArrowRight => self.right(),
            KeyPress::ArrowUp => self.up(),
            KeyPress::ArrowDown => self.down(),
            _ => (),
        }
    }
}

fn main() {
    rui(state(
        || AppState {
            // mesh: TODO
        },
        move |state, cx| {
            hstack((
                rectangle()
                    .corner_radius(5.0)
                    .color(AZURE_HIGHLIGHT)
                    .padding(PaddingParam::Px(20.0))
                    .background(rectangle().color(RED_HIGHLIGHT)),
                focus(move |has_focus| {
                    zstack((
                        canvas(|_, rect, vger| {
                            vger.translate(rect.center() - LocalPoint::zero());

                            let paint = vger.linear_gradient(
                                [-100.0, -100.0],
                                [100.0, 100.0],
                                AZURE_HIGHLIGHT,
                                RED_HIGHLIGHT,
                                0.0,
                            );

                            let radius = 100.0;
                            vger.fill_circle(LocalPoint::zero(), radius, paint);
                        })
                        .key(move |cx, key, mods| {
                            if has_focus {
                                println!("key canvas 2d");
                                cx[state].key(&key, &mods);
                            }
                        }),
                        text("Canvas 2d"),
                    ))
                }),
                focus(move |has_focus| {
                    zstack((
                        canvas3d(|_, rect, vger| {
                            vger.translate(rect.center() - LocalPoint::zero());

                            let paint = vger.linear_gradient(
                                [-100.0, -100.0],
                                [100.0, 100.0],
                                RED_HIGHLIGHT,
                                AZURE_HIGHLIGHT,
                                0.0,
                            );

                            let radius = 100.0;
                            vger.fill_circle(LocalPoint::zero(), radius, paint);
                        })
                        .key(move |cx, key, mods| {
                            if has_focus {
                                println!("key canvas 3d");
                                cx[state].key(&key, &mods);
                            }
                        }),
                        text("Canvas 3d"),
                    ))
                }),
            ))
        },
    ));
}
