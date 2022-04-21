use crate::*;
use fj_math::{Aabb, Point};
use vger::vger3d::vertices::{Vertex, Vertices};

/// Struct for `mesh3d`
pub struct Mesh3d<FnAabb, FnMesh> {
    func_aabb: FnAabb,
    func_mesh: FnMesh,
}

impl<FnAabb, FnMesh> View for Mesh3d<FnAabb, FnMesh>
where
    FnAabb: Fn() -> Aabb<3> + 'static,
    FnMesh: Fn() -> Vertices + 'static,
{
    fn print(&self, _id: ViewId, _cx: &mut Context) {
        println!("canvas");
    }

    fn draw(&self, _id: ViewId, _cx: &mut Context, vger: &mut VGER) {
        // let rect = cx.layout.entry(id).or_default().rect;

        vger.aabb = (self.func_aabb)();
        vger.mesh = (self.func_mesh)();
        // vger.transforms =  (self.update_transforms)();
        // vger.save();
        // (self.func)(cx, rect, vger);
        // vger.restore();
    }

    fn layout(&self, id: ViewId, sz: LocalSize, cx: &mut Context, _vger: &mut VGER) -> LocalSize {
        println!("Layout LocalSize {:?}", sz);
        // ???? vger.setup3d(rect); ????
        cx.layout.insert(
            id,
            LayoutBox {
                rect: LocalRect::new(LocalPoint::zero(), sz),
                offset: LocalOffset::zero(),
            },
        );
        sz
    }

    fn hittest(
        &self,
        id: ViewId,
        pt: LocalPoint,
        cx: &mut Context,
        _vger: &mut VGER,
    ) -> Option<ViewId> {
        let rect = cx.layout.entry(id).or_default().rect;

        if rect.contains(pt) {
            Some(id)
        } else {
            None
        }
    }
}

/// Canvas for GPU drawing with VGER. See https://github.com/audulus/vger-rs.
pub fn mesh3d<
    // F: Fn(&mut Context, LocalRect, &mut VGER) + 'static,
    FnAabb: Fn() -> Aabb<3> + 'static,
    FnMesh: Fn() -> Vertices + 'static,
>(
    // f: F,
    fn_aabb: FnAabb,
    fn_mesh: FnMesh,
) -> impl View {
    Mesh3d {
        // mesh: Vertices::new(
        //     vec![
        //         Vertex {
        //             position: [1.5, 1.0, 0.0],
        //             normal: [0.0, 0.0, 1.0],
        //             color: [0.39215687, 1.0, 0.0, 0.78431374],
        //         },
        //         Vertex {
        //             position: [-1.5, -1.0, 0.0],
        //             normal: [0.0, 0.0, 1.0],
        //             color: [0.39215687, 1.0, 0.0, 0.78431374],
        //         },
        //         Vertex {
        //             position: [1.5, -1.0, 0.0],
        //             normal: [0.0, 0.0, 1.0],
        //             color: [0.39215687, 1.0, 0.0, 0.78431374],
        //         },
        //         Vertex {
        //             position: [-1.5, 1.0, 0.0],
        //             normal: [-0.0, 0.0, 1.0],
        //             color: [0.39215687, 1.0, 0.0, 0.78431374],
        //         },
        //         Vertex {
        //             position: [1.5, 1.0, 1.0],
        //             normal: [0.0, 0.0, 1.0],
        //             color: [0.39215687, 1.0, 0.0, 0.78431374],
        //         },
        //         Vertex {
        //             position: [-1.5, -1.0, 1.0],
        //             normal: [0.0, 0.0, 1.0],
        //             color: [0.39215687, 1.0, 0.0, 0.78431374],
        //         },
        //         Vertex {
        //             position: [1.5, -1.0, 1.0],
        //             normal: [0.0, 0.0, 1.0],
        //             color: [0.39215687, 1.0, 0.0, 0.78431374],
        //         },
        //         Vertex {
        //             position: [-1.5, 1.0, 1.0],
        //             normal: [-0.0, 0.0, 1.0],
        //             color: [0.39215687, 1.0, 0.0, 0.78431374],
        //         },
        //         Vertex {
        //             position: [1.5, -1.0, 1.0],
        //             normal: [-0.0, -1.0, 0.0],
        //             color: [0.39215687, 1.0, 0.0, 0.78431374],
        //         },
        //         Vertex {
        //             position: [-1.5, -1.0, 1.0],
        //             normal: [-0.0, -1.0, 0.0],
        //             color: [0.39215687, 1.0, 0.0, 0.78431374],
        //         },
        //         Vertex {
        //             position: [-1.5, -1.0, 0.0],
        //             normal: [-0.0, -1.0, 0.0],
        //             color: [0.39215687, 1.0, 0.0, 0.78431374],
        //         },
        //         Vertex {
        //             position: [1.5, -1.0, 0.0],
        //             normal: [0.0, -1.0, 0.0],
        //             color: [0.39215687, 1.0, 0.0, 0.78431374],
        //         },
        //         Vertex {
        //             position: [1.5, 1.0, 0.0],
        //             normal: [1.0, 0.0, -0.0],
        //             color: [0.39215687, 1.0, 0.0, 0.78431374],
        //         },
        //         Vertex {
        //             position: [1.5, 1.0, 1.0],
        //             normal: [1.0, 0.0, -0.0],
        //             color: [0.39215687, 1.0, 0.0, 0.78431374],
        //         },
        //         Vertex {
        //             position: [1.5, -1.0, 0.0],
        //             normal: [1.0, 0.0, -0.0],
        //             color: [0.39215687, 1.0, 0.0, 0.78431374],
        //         },
        //         Vertex {
        //             position: [1.5, -1.0, 1.0],
        //             normal: [1.0, -0.0, 0.0],
        //             color: [0.39215687, 1.0, 0.0, 0.78431374],
        //         },
        //         Vertex {
        //             position: [-1.5, 1.0, 0.0],
        //             normal: [0.0, 1.0, 0.0],
        //             color: [0.39215687, 1.0, 0.0, 0.78431374],
        //         },
        //         Vertex {
        //             position: [-1.5, 1.0, 1.0],
        //             normal: [0.0, 1.0, 0.0],
        //             color: [0.39215687, 1.0, 0.0, 0.78431374],
        //         },
        //         Vertex {
        //             position: [1.5, 1.0, 0.0],
        //             normal: [0.0, 1.0, 0.0],
        //             color: [0.39215687, 1.0, 0.0, 0.78431374],
        //         },
        //         Vertex {
        //             position: [1.5, 1.0, 1.0],
        //             normal: [0.0, 1.0, 0.0],
        //             color: [0.39215687, 1.0, 0.0, 0.78431374],
        //         },
        //         Vertex {
        //             position: [-1.5, -1.0, 1.0],
        //             normal: [-1.0, 0.0, 0.0],
        //             color: [0.39215687, 1.0, 0.0, 0.78431374],
        //         },
        //         Vertex {
        //             position: [-1.5, 1.0, 0.0],
        //             normal: [-1.0, 0.0, 0.0],
        //             color: [0.39215687, 1.0, 0.0, 0.78431374],
        //         },
        //         Vertex {
        //             position: [-1.5, -1.0, 0.0],
        //             normal: [-1.0, 0.0, 0.0],
        //             color: [0.39215687, 1.0, 0.0, 0.78431374],
        //         },
        //         Vertex {
        //             position: [-1.5, 1.0, 1.0],
        //             normal: [-1.0, -0.0, -0.0],
        //             color: [0.39215687, 1.0, 0.0, 0.78431374],
        //         },
        //     ],
        //     vec![
        //         0, 1, 2, 3, 1, 0, 4, 5, 6, 7, 5, 4, 8, 9, 10, 11, 8, 10, 12, 13, 14, 15, 14, 13,
        //         16, 17, 18, 19, 18, 17, 20, 21, 22, 23, 21, 20,
        //     ],
        // ),
        // aabb: Aabb {
        //     min: Point::from([-1.5, -1.0, 0.0]),
        //     max: Point::from([1.5, 1.0, 1.0]),
        // },
        // func: f,
        func_aabb: fn_aabb,
        func_mesh: fn_mesh,
    }
}

impl<FnAabb, FnMesh> private::Sealed for Mesh3d<FnAabb, FnMesh> {}
