use crate::prelude::*;
use crate::{image_filter::CropRect, scalar, ImageFilter};
use skia_bindings::{C_SkBlurImageFilter_Make, SkBlurImageFilter_TileMode, SkImageFilter};

impl RCHandle<SkImageFilter> {
    pub fn blur<'a>(
        &self,
        crop_rect: impl Into<Option<&'a CropRect>>,
        sigma: (scalar, scalar),
        tile_mode: impl Into<Option<TileMode>>,
    ) -> Option<Self> {
        new(sigma, self, crop_rect, tile_mode)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum TileMode {
    Clamp = SkBlurImageFilter_TileMode::kClamp_TileMode as _,
    Repeat = SkBlurImageFilter_TileMode::kRepeat_TileMode as _,
    ClampToBlack = SkBlurImageFilter_TileMode::kClampToBlack_TileMode as _,
}

impl NativeTransmutable<SkBlurImageFilter_TileMode> for TileMode {}
#[test]
fn test_tile_mode_layout() {
    TileMode::test_layout();
}

pub fn new<'a>(
    (sigma_x, sigma_y): (scalar, scalar),
    input: &ImageFilter,
    crop_rect: impl Into<Option<&'a CropRect>>,
    tile_mode: impl Into<Option<TileMode>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        C_SkBlurImageFilter_Make(
            sigma_x,
            sigma_y,
            input.shared_native(),
            crop_rect.into().native_ptr_or_null(),
            tile_mode
                .into()
                .unwrap_or(TileMode::ClampToBlack)
                .into_native(),
        )
    })
}
