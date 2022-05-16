use fj_interop::debug::DebugInfo;
use fj_kernel::{algorithms::Tolerance, shape::Shape};
use fj_math::{Aabb, Transform, Vector};

use super::ToShape;

impl ToShape for fj::Transform {
    fn to_shape(
        &self,
        tolerance: Tolerance,
        debug_info: &mut DebugInfo,
    ) -> Shape {
        let mut shape = self.shape.to_shape(tolerance, debug_info);
        let transform = transform(self);

        shape.transform(&transform);

        shape
    }

    fn bounding_volume(&self) -> Aabb<3> {
        transform(self).transform_aabb(&self.shape.bounding_volume())
    }
}

fn transform(transform: &fj::Transform) -> Transform {
    let axis = Vector::from(transform.axis).normalize();
    Transform::translation(transform.offset)
        * Transform::rotation(axis * transform.angle)
}