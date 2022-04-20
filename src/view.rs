use crate::*;
use std::any::TypeId;

/// Trait for the unit of UI composition.
pub trait View: private::Sealed + 'static {
    /// Returns the type ID of the underlying view.
    fn tid(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    /// Prints a description of the view for debugging.
    fn print(&self, id: ViewId, cx: &mut Context);

    /// Processes an event.
    fn process(&self, _event: &Event, _id: ViewId, _cx: &mut Context, _vger: &mut VGER) {}

    /// Draws the view using vger.
    fn draw(&self, id: ViewId, cx: &mut Context, vger: &mut VGER);

    /// Lays out subviews and return the size of the view.
    fn layout(&self, id: ViewId, sz: LocalSize, cx: &mut Context, vger: &mut VGER) -> LocalSize;

    /// Determines dirty regions which need repainting.
    fn dirty(
        &self,
        _id: ViewId,
        _xform: LocalToWorld,
        _cx: &mut Context,
        _region: &mut Region<WorldSpace>,
    ) {
    }

    /// Returns the topmost view which the point intersects.
    fn hittest(
        &self,
        _id: ViewId,
        _pt: LocalPoint,
        _cx: &mut Context,
        _vger: &mut VGER,
    ) -> Option<ViewId> {
        None
    }

    /// Accumulates information about menu bar commands.
    fn commands(&self, _id: ViewId, _cx: &mut Context, _cmds: &mut Vec<CommandInfo>) {}

    // /// Copies state currently in use to a new StateMap (the rest are dropped).
    fn gc(&self, _id: ViewId, _cx: &mut Context, _map: &mut Vec<ViewId>) {}

    /// Builds an AccessKit tree. The node ID for the subtree is returned. All generated nodes are accumulated.
    fn access(
        &self,
        _id: ViewId,
        _cx: &mut Context,
        _nodes: &mut Vec<accesskit::Node>,
    ) -> Option<accesskit::NodeId> {
        None
    }

    /// For detecting spacers.
    fn is_spacer(&self) -> bool {
        false
    }
}
