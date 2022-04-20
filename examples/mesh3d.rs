use fj_math::{Aabb, Point};
use rui::*;
use tao::keyboard::ModifiersState;
use vger::vger3d::vertices::{Index, Vertex, Vertices};

struct Translate {
    x: f64,
    y: f64,
    z: f64,
}

struct Rotate {
    axis_x_angle: f64,
    axis_y_angle: f64,
}
struct AppState {
    // mesh: Vertices,
    translate: Translate,
    rotate: Rotate,
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
            translate: Translate {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            rotate: Rotate {
                axis_x_angle: 0.0,
                axis_y_angle: 0.0,
            },
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
                        mesh3d(
                            || {
                                // TODO: generate it form mesh
                                Aabb {
                                    min: Point::from([-1.5, -1.0, 0.0]),
                                    max: Point::from([1.5, 1.0, 1.0]),
                                }
                            },
                            || {
                                // TODO: read form external file
                                Vertices::new(
                                    vec![
                                        Vertex {
                                            position: [1.5, 1.0, 0.0],
                                            normal: [0.0, 0.0, 1.0],
                                            color: [0.39215687, 1.0, 0.0, 0.78431374],
                                        },
                                        Vertex {
                                            position: [-1.5, -1.0, 0.0],
                                            normal: [0.0, 0.0, 1.0],
                                            color: [0.39215687, 1.0, 0.0, 0.78431374],
                                        },
                                        Vertex {
                                            position: [1.5, -1.0, 0.0],
                                            normal: [0.0, 0.0, 1.0],
                                            color: [0.39215687, 1.0, 0.0, 0.78431374],
                                        },
                                        Vertex {
                                            position: [-1.5, 1.0, 0.0],
                                            normal: [-0.0, 0.0, 1.0],
                                            color: [0.39215687, 1.0, 0.0, 0.78431374],
                                        },
                                        Vertex {
                                            position: [1.5, 1.0, 1.0],
                                            normal: [0.0, 0.0, 1.0],
                                            color: [0.39215687, 1.0, 0.0, 0.78431374],
                                        },
                                        Vertex {
                                            position: [-1.5, -1.0, 1.0],
                                            normal: [0.0, 0.0, 1.0],
                                            color: [0.39215687, 1.0, 0.0, 0.78431374],
                                        },
                                        Vertex {
                                            position: [1.5, -1.0, 1.0],
                                            normal: [0.0, 0.0, 1.0],
                                            color: [0.39215687, 1.0, 0.0, 0.78431374],
                                        },
                                        Vertex {
                                            position: [-1.5, 1.0, 1.0],
                                            normal: [-0.0, 0.0, 1.0],
                                            color: [0.39215687, 1.0, 0.0, 0.78431374],
                                        },
                                        Vertex {
                                            position: [1.5, -1.0, 1.0],
                                            normal: [-0.0, -1.0, 0.0],
                                            color: [0.39215687, 1.0, 0.0, 0.78431374],
                                        },
                                        Vertex {
                                            position: [-1.5, -1.0, 1.0],
                                            normal: [-0.0, -1.0, 0.0],
                                            color: [0.39215687, 1.0, 0.0, 0.78431374],
                                        },
                                        Vertex {
                                            position: [-1.5, -1.0, 0.0],
                                            normal: [-0.0, -1.0, 0.0],
                                            color: [0.39215687, 1.0, 0.0, 0.78431374],
                                        },
                                        Vertex {
                                            position: [1.5, -1.0, 0.0],
                                            normal: [0.0, -1.0, 0.0],
                                            color: [0.39215687, 1.0, 0.0, 0.78431374],
                                        },
                                        Vertex {
                                            position: [1.5, 1.0, 0.0],
                                            normal: [1.0, 0.0, -0.0],
                                            color: [0.39215687, 1.0, 0.0, 0.78431374],
                                        },
                                        Vertex {
                                            position: [1.5, 1.0, 1.0],
                                            normal: [1.0, 0.0, -0.0],
                                            color: [0.39215687, 1.0, 0.0, 0.78431374],
                                        },
                                        Vertex {
                                            position: [1.5, -1.0, 0.0],
                                            normal: [1.0, 0.0, -0.0],
                                            color: [0.39215687, 1.0, 0.0, 0.78431374],
                                        },
                                        Vertex {
                                            position: [1.5, -1.0, 1.0],
                                            normal: [1.0, -0.0, 0.0],
                                            color: [0.39215687, 1.0, 0.0, 0.78431374],
                                        },
                                        Vertex {
                                            position: [-1.5, 1.0, 0.0],
                                            normal: [0.0, 1.0, 0.0],
                                            color: [0.39215687, 1.0, 0.0, 0.78431374],
                                        },
                                        Vertex {
                                            position: [-1.5, 1.0, 1.0],
                                            normal: [0.0, 1.0, 0.0],
                                            color: [0.39215687, 1.0, 0.0, 0.78431374],
                                        },
                                        Vertex {
                                            position: [1.5, 1.0, 0.0],
                                            normal: [0.0, 1.0, 0.0],
                                            color: [0.39215687, 1.0, 0.0, 0.78431374],
                                        },
                                        Vertex {
                                            position: [1.5, 1.0, 1.0],
                                            normal: [0.0, 1.0, 0.0],
                                            color: [0.39215687, 1.0, 0.0, 0.78431374],
                                        },
                                        Vertex {
                                            position: [-1.5, -1.0, 1.0],
                                            normal: [-1.0, 0.0, 0.0],
                                            color: [0.39215687, 1.0, 0.0, 0.78431374],
                                        },
                                        Vertex {
                                            position: [-1.5, 1.0, 0.0],
                                            normal: [-1.0, 0.0, 0.0],
                                            color: [0.39215687, 1.0, 0.0, 0.78431374],
                                        },
                                        Vertex {
                                            position: [-1.5, -1.0, 0.0],
                                            normal: [-1.0, 0.0, 0.0],
                                            color: [0.39215687, 1.0, 0.0, 0.78431374],
                                        },
                                        Vertex {
                                            position: [-1.5, 1.0, 1.0],
                                            normal: [-1.0, -0.0, -0.0],
                                            color: [0.39215687, 1.0, 0.0, 0.78431374],
                                        },
                                    ],
                                    vec![
                                        0, 1, 2, 3, 1, 0, 4, 5, 6, 7, 5, 4, 8, 9, 10, 11, 8, 10,
                                        12, 13, 14, 15, 14, 13, 16, 17, 18, 19, 18, 17, 20, 21, 22,
                                        23, 21, 20,
                                    ],
                                )
                            },
                        )
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
