use crate::graphics::Mesh;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Triangle {
    pub a: [f32; 3],
    pub b: [f32; 3],
    pub c: [f32; 3],
}

impl Triangle {
    pub fn new(a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> Self {
        Self { a, b, c }
    }

    pub fn to_mesh(&self) -> Mesh {
        let mut mesh = Mesh::new();

        let i0 = mesh.vertex(self.a);
        let i1 = mesh.vertex(self.b);
        let i2 = mesh.vertex(self.c);

        mesh.triangle(i0, i1, i2);

        mesh
    }
}

impl From<Array> for Triangle {
    fn from([a, b, c]: Array) -> Self {
        Self::new(a, b, c)
    }
}

impl From<Triangle> for Array {
    fn from(triangle: Triangle) -> Self {
        Self::from(&triangle)
    }
}

impl From<&Triangle> for Array {
    fn from(triangle: &Triangle) -> Self {
        [triangle.a, triangle.b, triangle.c]
    }
}

type Array = [[f32; 3]; 3];

#[derive(Debug, PartialEq)]
pub struct Triangles(pub Vec<Triangle>);

#[cfg(test)]
mod tests {
    use super::{Array, Triangle};

    #[test]
    fn triangle_should_support_conversions_to_and_from_arrays() {
        let original =
            Triangle::new([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);

        let array: Array = original.into();
        let converted: Triangle = array.into();
        assert_eq!(original, converted);

        let array: Array = (&original).into();
        let converted: Triangle = array.into();
        assert_eq!(original, converted);
    }

    #[test]
    fn test() {
        let triangle =
            Triangle::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);

        let mesh = triangle.to_mesh();
        let triangles = mesh.triangles();

        assert_eq!(triangles.0, vec![triangle]);
    }
}
