mod codec;
mod core;
mod docs;
mod effects;
pub mod gpu;
mod interop;
mod pathops;
mod prelude;
#[cfg(feature = "svg")]
pub mod svg;
// TODO: We don't export utils/* into the crate's root yet. Should we?
pub mod utils;

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;

// Prelude re-exports
pub use crate::prelude::Borrows;

/// All Sk* types are accessible via skia_safe::
pub use crate::codec::*;
pub use crate::core::*;
pub use crate::docs::*;
pub use crate::effects::*;
pub use crate::pathops::*;

#[cfg(test)]
mod transmutation_tests {

    use crate::prelude::NativeTransmutableSliceAccess;
    use crate::Point;
    use skia_bindings::SkPoint;

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_transmutation_of_fixed_size_arrays_to_slice() {
        let mut points = [Point::default(); 4];

        let points_native = points.native_mut();
        let native_point = SkPoint { fX: 10.0, fY: 11.0 };
        points_native[1] = native_point;

        assert_eq!(points[1].x, native_point.fX);
        assert_eq!(points[1].y, native_point.fY);
    }
}
