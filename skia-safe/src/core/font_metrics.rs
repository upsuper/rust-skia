use crate::prelude::*;
use crate::scalar;
use skia_bindings::SkFontMetrics;

bitflags! {
    pub struct Flags: u32 {
        const UNDERLINE_THICKNESS_IS_VALID = skia_bindings::SkFontMetrics_FontMetricsFlags_kUnderlineThicknessIsValid_Flag as _;
        const UNDERLINE_POSITION_IS_VALID = skia_bindings::SkFontMetrics_FontMetricsFlags_kUnderlinePositionIsValid_Flag as _;
        const STRIKEOUT_THICKNESS_IS_VALID = skia_bindings::SkFontMetrics_FontMetricsFlags_kStrikeoutThicknessIsValid_Flag as _;
        const STRIKEOUT_POSITION_IS_VALID = skia_bindings::SkFontMetrics_FontMetricsFlags_kStrikeoutPositionIsValid_Flag as _;
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct FontMetrics {
    flags: self::Flags,
    pub top: scalar,
    pub ascent: scalar,
    pub descent: scalar,
    pub bottom: scalar,
    pub leading: scalar,
    pub avg_char_width: scalar,
    pub max_char_width: scalar,
    pub x_min: scalar,
    pub x_max: scalar,
    pub x_height: scalar,
    pub cap_height: scalar,
    underline_thickness: scalar,
    underline_position: scalar,
    strikeout_thickness: scalar,
    strikeout_position: scalar,
}

impl NativeTransmutable<SkFontMetrics> for FontMetrics {}

#[test]
fn test_font_metrics_layout() {
    FontMetrics::test_layout();
}

impl FontMetrics {
    // the following 4 functions do not link.

    pub fn underline_thickness(&self) -> Option<scalar> {
        self.if_valid(
            Flags::UNDERLINE_THICKNESS_IS_VALID,
            self.underline_thickness,
        )
    }

    pub fn underline_position(&self) -> Option<scalar> {
        self.if_valid(Flags::UNDERLINE_POSITION_IS_VALID, self.underline_position)
    }

    pub fn strikeout_thickness(&self) -> Option<scalar> {
        self.if_valid(
            Flags::STRIKEOUT_THICKNESS_IS_VALID,
            self.strikeout_thickness,
        )
    }

    pub fn strikeout_position(&self) -> Option<scalar> {
        self.if_valid(Flags::STRIKEOUT_POSITION_IS_VALID, self.strikeout_position)
    }

    fn if_valid(&self, flag: Flags, value: scalar) -> Option<scalar> {
        self.flags.contains(flag).if_true_some(value)
    }
}
