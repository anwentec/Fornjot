//! Sketch approximation

use std::collections::BTreeSet;

use crate::objects::Sketch;

use super::{curve::CurveCache, face::FaceApprox, Approx, Tolerance};

impl Approx for &Sketch {
    type Approximation = BTreeSet<FaceApprox>;
    type Cache = CurveCache;

    fn approx_with_cache(
        self,
        tolerance: impl Into<Tolerance>,
        cache: &mut Self::Cache,
    ) -> Self::Approximation {
        self.faces().approx_with_cache(tolerance, cache)
    }
}
