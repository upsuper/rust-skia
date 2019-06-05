use crate::prelude::*;
use crate::{scalar, Canvas, Color, Path, Point3};
use skia_bindings::SkShadowUtils;

bitflags! {
    pub struct ShadowFlags: u32 {
        const TRANSPARENT_OCCLUDER = skia_bindings::SkShadowFlags_kTransparentOccluder_ShadowFlag as u32;
        const GEOMETRIC_ONLY = skia_bindings::SkShadowFlags_kGeometricOnly_ShadowFlag as u32;
        const ALL = Self::TRANSPARENT_OCCLUDER.bits | Self::GEOMETRIC_ONLY.bits;
    }
}

#[allow(clippy::too_many_arguments)]
pub fn draw_shadow(
    mut canvas: impl AsMut<Canvas>,
    path: &Path,
    z_plane_params: impl Into<Point3>,
    light_pos: impl Into<Point3>,
    light_radius: scalar,
    ambient_color: impl Into<Color>,
    spot_color: impl Into<Color>,
    flags: impl Into<Option<ShadowFlags>>,
) {
    unsafe {
        SkShadowUtils::DrawShadow(
            canvas.as_mut().native_mut(),
            path.native(),
            z_plane_params.into().native(),
            light_pos.into().native(),
            light_radius,
            ambient_color.into().into_native(),
            spot_color.into().into_native(),
            flags.into().unwrap_or(ShadowFlags::empty()).bits(),
        )
    }
}

impl Canvas {
    #[allow(clippy::too_many_arguments)]
    pub fn draw_shadow(
        &mut self,
        path: &Path,
        z_plane_params: impl Into<Point3>,
        light_pos: impl Into<Point3>,
        light_radius: scalar,
        ambient_color: impl Into<Color>,
        spot_color: impl Into<Color>,
        flags: impl Into<Option<ShadowFlags>>,
    ) -> &mut Self {
        draw_shadow(
            self.as_mut(),
            path,
            z_plane_params,
            light_pos,
            light_radius,
            ambient_color,
            spot_color,
            flags,
        );
        self
    }
}

pub fn compute_tonal_colors(
    ambient_color: impl Into<Color>,
    spot_color: impl Into<Color>,
) -> (Color, Color) {
    let mut out_ambient_color = Color::default();
    let mut out_spot_color = Color::default();
    unsafe {
        SkShadowUtils::ComputeTonalColors(
            ambient_color.into().into_native(),
            spot_color.into().into_native(),
            out_ambient_color.native_mut(),
            out_spot_color.native_mut(),
        )
    }
    (out_ambient_color, out_spot_color)
}
