use crate::*;
use euclid::{
    default::{Box3D, Rotation3D},
    Angle,
};
// use nalgebra as na;
use vger::vger3d::{
    camera::Camera, transform::Transform, uniforms::Transforms, vertices::Vertices, Rotate,
    Translate,
};

/// Struct for `mesh3d`
pub struct Mesh3d<FnMesh, FnAabb, FnRotate, FnTranslate> {
    func_mesh: FnMesh,
    func_aabb: FnAabb,
    func_rotate: FnRotate,
    func_translate: FnTranslate,
}

impl<FnMesh, FnAabb, FnRotate, FnTranslate> View for Mesh3d<FnMesh, FnAabb, FnRotate, FnTranslate>
where
    FnMesh: Fn() -> Vertices + 'static,
    FnAabb: Fn() -> Box3D<f64> + 'static,
    FnRotate: Fn(&mut Context) -> Rotate + 'static,
    FnTranslate: Fn(&mut Context) -> Translate + 'static,
{
    fn print(&self, _id: ViewId, _cx: &mut Context) {
        println!("mesh3d");
    }

    fn draw(&self, id: ViewId, cx: &mut Context, vger: &mut Vger) {
        let rect = cx.layout.entry(id).or_default().rect;

        let rotate = (self.func_rotate)(cx);
        let translate = (self.func_translate)(cx);
        if let Some(camera) = cx.cameras.get_mut(&id) {
            println!("draw mesh3d");

            // let rot_x = na::Rotation3::from_axis_angle(&na::Vector::x_axis(), rotate.axis_x_angle);
            // let rot_y = na::Rotation3::from_axis_angle(&na::Vector::y_axis(), rotate.axis_y_angle);
            // camera.rotation = rot_x * rot_y * camera.rotation;

            let rot_x = Rotation3D::around_x(Angle::radians(rotate.axis_x_angle));
            let rot_y = Rotation3D::around_y(Angle::radians(rotate.axis_y_angle));
            camera.rotation = rot_x.then(&rot_y).to_transform().then(&camera.rotation);

            camera.translation.x += translate.x;
            camera.translation.y += translate.y;
            camera.translation.z += translate.z;

            let aspect_ratio = rect.width() / rect.height();
            vger.transforms3d = Transforms {
                transform: Transform::for_vertices(&camera, aspect_ratio.into()),
                transform_normals: Transform::for_normals(&camera),
            };
        } else {
            let aabb = (self.func_aabb)();
            let camera = Camera::new(&aabb);

            println!("draw mesh3d FIRST");
            vger.mesh = (self.func_mesh)();

            let aspect_ratio = rect.width() / rect.height();
            vger.transforms3d = Transforms {
                transform: Transform::for_vertices(&camera, aspect_ratio.into()),
                transform_normals: Transform::for_normals(&camera),
            };

            cx.cameras.insert(id, camera);
        };
    }

    fn layout(&self, id: ViewId, sz: LocalSize, cx: &mut Context, vger: &mut Vger) -> LocalSize {
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
        _vger: &mut Vger,
    ) -> Option<ViewId> {
        let rect = cx.layout.entry(id).or_default().rect;

        if rect.contains(pt) {
            Some(id)
        } else {
            None
        }
    }
}

/// Rendering 3d mesh
pub fn mesh3d<FnMesh, FnAabb, FnRotate, FnTranslate>(
    fn_mesh: FnMesh,
    fn_aabb: FnAabb,
    fn_rotate: FnRotate,
    fn_translate: FnTranslate,
) -> impl View
where
    FnMesh: Fn() -> Vertices + 'static,
    FnAabb: Fn() -> Box3D<f64> + 'static,
    FnRotate: Fn(&mut Context) -> Rotate + 'static,
    FnTranslate: Fn(&mut Context) -> Translate + 'static,
{
    Mesh3d {
        func_mesh: fn_mesh,
        func_aabb: fn_aabb,
        func_rotate: fn_rotate,
        func_translate: fn_translate,
    }
}

impl<FnMesh, FnAabb, FnRotate, FnTranslate> private::Sealed
    for Mesh3d<FnMesh, FnAabb, FnRotate, FnTranslate>
{
}
