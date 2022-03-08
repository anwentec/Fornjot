use crate::{
    kernel::{shape::Shape, topology::faces::Face},
    math::{Scalar, Transform, Vector},
};

use super::{approximation::Approximation, transform::transform_face};

/// Create a new shape by sweeping an existing one
pub fn sweep_shape(
    mut original: Shape,
    path: Vector<3>,
    tolerance: Scalar,
) -> Shape {
    // TASK: This could be called with 3-dimensional shapes, but it only works
    //       for 2-dimensional ones.

    let mut shape = Shape::new();

    let translation = Transform::translation(path);

    let mut bottom_faces = Vec::new();
    let mut top_faces = Vec::new();
    let mut side_faces = Vec::new();

    for face in original.faces().all() {
        bottom_faces.push(face.clone());

        // TASK: This can only work, if all the original faces don't share any
        //       vertices. If they do, this will create duplicate vertices, as
        //       `transform_face` creates new vertices per-face.
        top_faces.push(transform_face(&face, &translation, &mut shape));
    }

    for cycle in original.cycles().all() {
        let approx = Approximation::for_cycle(&cycle, tolerance);

        // This will only work correctly, if the cycle consists of one edge. If
        // there are more, this will create some kind of weird face chimera, a
        // single face to represent all the side faces.

        let mut quads = Vec::new();
        for segment in approx.segments {
            let [v0, v1] = segment.points();
            let [v3, v2] = {
                let segment =
                    Transform::translation(path).transform_segment(&segment);
                segment.points()
            };

            quads.push([v0, v1, v2, v3]);
        }

        let mut side_face = Vec::new();
        for [v0, v1, v2, v3] in quads {
            side_face.push([v0, v1, v2].into());
            side_face.push([v0, v2, v3].into());
        }

        side_faces.push(Face::Triangles(side_face));
    }

    for face in bottom_faces {
        shape.faces().add((*face).clone());
    }
    for face in top_faces {
        shape.faces().add(face);
    }
    for face in side_faces {
        shape.faces().add(face);
    }

    shape
}

#[cfg(test)]
mod tests {
    use crate::{
        kernel::{
            geometry::{surfaces::Swept, Surface},
            shape::{handle::Handle, Shape},
            topology::{edges::Cycle, faces::Face},
        },
        math::{Point, Scalar, Vector},
    };

    use super::sweep_shape;

    #[test]
    fn sweep() {
        let sketch = Triangle::new([[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]]);

        let mut swept = sweep_shape(
            sketch.shape,
            Vector::from([0., 0., 1.]),
            Scalar::from_f64(0.),
        );

        let bottom_face = sketch.face;
        let top_face =
            Triangle::new([[0., 0., 1.], [1., 0., 1.], [0., 1., 1.]]).face;

        assert!(swept.faces().contains(&bottom_face));
        assert!(swept.faces().contains(&top_face));

        // Side faces are not tested, as those use triangle representation. The
        // plan is to start testing them, as they are transitioned to b-rep.
    }

    pub struct Triangle {
        shape: Shape,
        face: Handle<Face>,
    }

    impl Triangle {
        fn new([a, b, c]: [impl Into<Point<3>>; 3]) -> Self {
            let mut shape = Shape::new();

            let a = shape.vertices().add(a.into());
            let b = shape.vertices().add(b.into());
            let c = shape.vertices().add(c.into());

            let ab = shape.edges().add_line_segment([a.clone(), b.clone()]);
            let bc = shape.edges().add_line_segment([b.clone(), c.clone()]);
            let ca = shape.edges().add_line_segment([c.clone(), a.clone()]);

            let cycles = shape.cycles().add(Cycle {
                edges: vec![ab, bc, ca],
            });

            let surface =
                shape
                    .surfaces()
                    .add(Surface::Swept(Swept::plane_from_points(
                        [a, b, c].map(|vertex| vertex.point()),
                    )));
            let abc = Face::Face {
                surface,
                cycles: vec![cycles],
            };

            let face = shape.faces().add(abc);

            Self { shape, face }
        }
    }
}
