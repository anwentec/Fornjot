use crate::{kernel::topology::vertices::Vertex, math::Scalar};

use super::{
    handle::{Handle, Storage},
    VerticesInner,
};

/// The vertices of a shape
pub struct Vertices<'r> {
    pub(super) min_distance: Scalar,
    pub(super) vertices: &'r mut VerticesInner,
}

impl Vertices<'_> {
    /// Add a vertex to the shape
    ///
    /// # Panics
    ///
    /// Panics, if the vertex is not unique, meaning if another vertex defined
    /// by the same point already exists.
    ///
    /// In the context of of vertex uniqueness, points that are close to each
    /// other are considered identical. The minimum distance between distinct
    /// vertices can be configured using [`Shape::with_minimum_distance`].
    ///
    /// # Implementation note
    ///
    /// In the future, this method is likely to validate more than just vertex
    /// uniqueness. See documentation of [`crate::kernel`] for some context on
    /// that.
    pub fn add(&mut self, vertex: impl Into<Vertex>) -> Handle<Vertex> {
        let vertex = vertex.into();

        // Make sure the new vertex is a minimum distance away from all existing
        // vertices. This minimum distance is defined to be half a µm, which
        // should provide more than enough precision for common use cases, while
        // being large enough to catch all invalid cases.
        for existing in &*self.vertices {
            let distance = (existing.point() - vertex.point()).magnitude();

            if distance < self.min_distance {
                panic!(
                    "Invalid vertex: {vertex:?}; \
                    identical vertex at {existing:?}",
                );
            }
        }

        let storage = Storage::new(vertex);
        let handle = storage.handle();
        self.vertices.push(storage);

        handle
    }
}

#[cfg(test)]
mod tests {
    use crate::{kernel::shape::Shape, math::Point};

    const MIN_DISTANCE: f64 = 5e-7;

    #[test]
    fn create_valid() {
        let mut shape = Shape::new().with_min_distance(MIN_DISTANCE);

        shape.vertices().add(Point::from([0., 0., 0.]));
        shape.vertices().add(Point::from([5e-6, 0., 0.]));
    }

    #[test]
    #[should_panic]
    fn create_invalid() {
        let mut shape = Shape::new().with_min_distance(MIN_DISTANCE);

        shape.vertices().add(Point::from([0., 0., 0.]));
        shape.vertices().add(Point::from([5e-8, 0., 0.]));
    }
}
