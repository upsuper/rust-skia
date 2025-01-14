use crate::prelude::*;
use crate::{image_filter::CropRect, scalar, Color, ImageFilter, Vector};
use skia_bindings::{
    C_SkDropShadowImageFilter_Make, SkDropShadowImageFilter_ShadowMode, SkImageFilter,
};

impl RCHandle<SkImageFilter> {
    pub fn drop_shadow<'a>(
        &self,
        crop_rect: impl Into<Option<&'a CropRect>>,
        delta: impl Into<Vector>,
        sigma: (scalar, scalar),
        color: impl Into<Color>,
        shadow_mode: ShadowMode,
    ) -> Option<Self> {
        new(delta, sigma, color, shadow_mode, self, crop_rect)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum ShadowMode {
    DrawShadowAndForeground =
        SkDropShadowImageFilter_ShadowMode::kDrawShadowAndForeground_ShadowMode as _,
    DrawShadowOnly = SkDropShadowImageFilter_ShadowMode::kDrawShadowOnly_ShadowMode as _,
}

impl NativeTransmutable<SkDropShadowImageFilter_ShadowMode> for ShadowMode {}
#[test]
fn test_shadow_mode_layout() {
    ShadowMode::test_layout();
}

pub fn new<'a>(
    delta: impl Into<Vector>,
    (sigma_x, sigma_y): (scalar, scalar),
    color: impl Into<Color>,
    shadow_mode: ShadowMode,
    input: &ImageFilter,
    crop_rect: impl Into<Option<&'a CropRect>>,
) -> Option<ImageFilter> {
    let delta = delta.into();
    let color = color.into();
    ImageFilter::from_ptr(unsafe {
        C_SkDropShadowImageFilter_Make(
            delta.x,
            delta.y,
            sigma_x,
            sigma_y,
            color.into_native(),
            shadow_mode.into_native(),
            input.shared_native(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}
