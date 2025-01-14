use crate::prelude::*;
use crate::{
    scalar, FontHinting, FontMetrics, GlyphId, Paint, Path, Point, Rect, TextEncoding, Typeface,
    Unichar,
};
use skia_bindings::{
    C_SkFont_ConstructFromTypeface, C_SkFont_ConstructFromTypefaceWithSize,
    C_SkFont_ConstructFromTypefaceWithSizeScaleAndSkew, C_SkFont_Equals, C_SkFont_destruct,
    C_SkFont_makeWithSize, C_SkFont_setTypeface, SkFont, SkFont_Edging,
};
use std::{mem, ptr};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum Edging {
    Alias = SkFont_Edging::kAlias as _,
    AntiAlias = SkFont_Edging::kAntiAlias as _,
    SubpixelAntiAlias = SkFont_Edging::kSubpixelAntiAlias as _,
}

impl NativeTransmutable<SkFont_Edging> for Edging {}
#[test]
fn test_font_edging_layout() {
    Edging::test_layout()
}

pub type Font = Handle<SkFont>;

impl NativeDrop for SkFont {
    fn drop(&mut self) {
        unsafe { C_SkFont_destruct(self) }
    }
}

impl NativePartialEq for SkFont {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { C_SkFont_Equals(self, rhs) }
    }
}

impl Default for Font {
    fn default() -> Self {
        Self::from_native(unsafe { SkFont::new() })
    }
}

impl Handle<SkFont> {
    pub fn from_typeface(typeface: &Typeface, size: impl Into<Option<scalar>>) -> Self {
        match size.into() {
            None => Self::construct(|font| unsafe {
                C_SkFont_ConstructFromTypeface(font, typeface.shared_native())
            }),
            Some(size) => Self::construct(|font| unsafe {
                C_SkFont_ConstructFromTypefaceWithSize(font, typeface.shared_native(), size)
            }),
        }
    }

    #[deprecated(since = "0.12.0", note = "use from_typeface()")]
    pub fn from_typeface_with_size(typeface: &Typeface, size: scalar) -> Self {
        Self::construct(|font| unsafe {
            C_SkFont_ConstructFromTypefaceWithSize(font, typeface.shared_native(), size)
        })
    }

    #[deprecated(since = "0.12.0", note = "use from_typeface_with_params()")]
    pub fn from_typeface_with_size_scale_and_skew(
        typeface: &Typeface,
        size: scalar,
        scale: scalar,
        skew: scalar,
    ) -> Self {
        Self::construct(|font| unsafe {
            C_SkFont_ConstructFromTypefaceWithSizeScaleAndSkew(
                font,
                typeface.shared_native(),
                size,
                scale,
                skew,
            )
        })
    }

    pub fn from_typeface_with_params(
        typeface: &Typeface,
        size: scalar,
        scale: scalar,
        skew: scalar,
    ) -> Self {
        Self::construct(|font| unsafe {
            C_SkFont_ConstructFromTypefaceWithSizeScaleAndSkew(
                font,
                typeface.shared_native(),
                size,
                scale,
                skew,
            )
        })
    }

    pub fn is_force_auto_hinting(&self) -> bool {
        unsafe { self.native().isForceAutoHinting() }
    }

    pub fn is_embedded_bitmaps(&self) -> bool {
        unsafe { self.native().isEmbeddedBitmaps() }
    }

    pub fn is_subpixel(&self) -> bool {
        unsafe { self.native().isSubpixel() }
    }

    pub fn is_linear_metrics(&self) -> bool {
        unsafe { self.native().isLinearMetrics() }
    }

    pub fn is_embolden(&self) -> bool {
        unsafe { self.native().isEmbolden() }
    }

    pub fn set_force_autohinting(&mut self, force_auto_hinting: bool) -> &mut Self {
        unsafe { self.native_mut().setForceAutoHinting(force_auto_hinting) }
        self
    }

    pub fn set_embedded_bitmaps(&mut self, embedded_bitmaps: bool) -> &mut Self {
        unsafe { self.native_mut().setEmbeddedBitmaps(embedded_bitmaps) }
        self
    }

    pub fn set_subpixel(&mut self, subpixel: bool) -> &mut Self {
        unsafe { self.native_mut().setSubpixel(subpixel) }
        self
    }

    pub fn set_linear_metrics(&mut self, linear_metrics: bool) -> &mut Self {
        unsafe { self.native_mut().setLinearMetrics(linear_metrics) }
        self
    }

    pub fn set_embolden(&mut self, embolden: bool) -> &mut Self {
        unsafe { self.native_mut().setEmbolden(embolden) }
        self
    }

    pub fn edging(&self) -> Edging {
        Edging::from_native(unsafe { self.native().getEdging() })
    }

    pub fn set_edging(&mut self, edging: Edging) -> &mut Self {
        unsafe { self.native_mut().setEdging(edging.into_native()) }
        self
    }

    pub fn set_hinting(&mut self, hinting: FontHinting) -> &mut Self {
        unsafe { self.native_mut().setHinting(hinting.into_native()) }
        self
    }

    pub fn hinting(&self) -> FontHinting {
        FontHinting::from_native(unsafe { self.native().getHinting() })
    }

    #[must_use]
    pub fn with_size(&self, size: scalar) -> Option<Self> {
        if size >= 0.0 && !size.is_infinite() && !size.is_nan() {
            let mut font = unsafe { SkFont::new() };
            unsafe { C_SkFont_makeWithSize(self.native(), size, &mut font) }
            Some(Self::from_native(font))
        } else {
            None
        }
    }

    pub fn typeface(&self) -> Option<Typeface> {
        Typeface::from_unshared_ptr(unsafe { self.native().getTypeface() })
    }

    pub fn typeface_or_default(&self) -> Typeface {
        Typeface::from_unshared_ptr(unsafe { self.native().getTypefaceOrDefault() }).unwrap()
    }

    pub fn size(&self) -> scalar {
        unsafe { self.native().getSize() }
    }

    pub fn scale_x(&self) -> scalar {
        unsafe { self.native().getScaleX() }
    }

    pub fn skew_x(&self) -> scalar {
        unsafe { self.native().getSkewX() }
    }

    pub fn set_typeface(&mut self, tf: &Typeface) -> &mut Self {
        unsafe { C_SkFont_setTypeface(self.native_mut(), tf.shared_native()) }
        self
    }

    pub fn set_size(&mut self, size: scalar) -> &mut Self {
        unsafe { self.native_mut().setSize(size) }
        self
    }

    pub fn set_scale_x(&mut self, scale_x: scalar) -> &mut Self {
        unsafe { self.native_mut().setScaleX(scale_x) }
        self
    }

    pub fn set_skew_x(&mut self, skew_x: scalar) -> &mut Self {
        unsafe { self.native_mut().setSkewX(skew_x) }
        self
    }

    pub fn str_to_glyphs(&self, str: impl AsRef<str>, glyphs: &mut [GlyphId]) -> usize {
        self.text_to_glyphs(str.as_ref().as_bytes(), TextEncoding::UTF8, glyphs)
    }

    pub fn text_to_glyphs(
        &self,
        text: &[u8],
        encoding: TextEncoding,
        glyphs: &mut [GlyphId],
    ) -> usize {
        unsafe {
            self.native()
                .textToGlyphs(
                    text.as_ptr() as _,
                    text.len(),
                    encoding.into_native(),
                    glyphs.as_mut_ptr(),
                    // don't fail if glyphs.len() is too large to fit into an i32.
                    glyphs
                        .len()
                        .min(i32::max_value().try_into().unwrap())
                        .try_into()
                        .unwrap(),
                )
                .try_into()
                .unwrap()
        }
    }

    pub fn count_str(&self, str: impl AsRef<str>) -> usize {
        self.count_text(str.as_ref().as_bytes(), TextEncoding::UTF8)
    }

    pub fn count_text(&self, text: &[u8], encoding: TextEncoding) -> usize {
        unsafe {
            self.native()
                .textToGlyphs(
                    text.as_ptr() as _,
                    text.len(),
                    encoding.into_native(),
                    ptr::null_mut(),
                    i32::max_value(),
                )
                .try_into()
                .unwrap()
        }
    }

    // convenience function
    pub fn str_to_glyphs_vec(&self, str: impl AsRef<str>) -> Vec<GlyphId> {
        let str = str.as_ref().as_bytes();
        self.text_to_glyphs_vec(str, TextEncoding::UTF8)
    }

    // convenience function
    pub fn text_to_glyphs_vec(&self, text: &[u8], encoding: TextEncoding) -> Vec<GlyphId> {
        let count = self.count_text(text, encoding);
        let mut glyphs: Vec<GlyphId> = vec![Default::default(); count];
        let resulting_count = self.text_to_glyphs(text, encoding, glyphs.as_mut_slice());
        assert_eq!(count, resulting_count);
        glyphs
    }

    pub fn measure_str(&self, str: impl AsRef<str>, paint: Option<&Paint>) -> (scalar, Rect) {
        let bytes = str.as_ref().as_bytes();
        self.measure_text(bytes, TextEncoding::UTF8, paint)
    }

    pub fn measure_text(
        &self,
        text: &[u8],
        encoding: TextEncoding,
        paint: Option<&Paint>,
    ) -> (scalar, Rect) {
        let mut bounds = Rect::default();
        let width = unsafe {
            self.native().measureText1(
                text.as_ptr() as _,
                text.len(),
                encoding.into_native(),
                bounds.native_mut(),
                paint.native_ptr_or_null(),
            )
        };

        (width, bounds)
    }

    pub fn unichar_to_glyph(&self, uni: Unichar) -> GlyphId {
        unsafe { self.native().unicharToGlyph(uni) }
    }

    pub fn unichar_to_glyphs(&self, uni: &[Unichar], glyphs: &mut [GlyphId]) {
        assert_eq!(uni.len(), glyphs.len());
        unsafe {
            self.native().unicharsToGlyphs(
                uni.as_ptr(),
                uni.len().try_into().unwrap(),
                glyphs.as_mut_ptr(),
            )
        }
    }

    #[deprecated(since = "0.12.0", note = "use get_widths")]
    pub fn widths(&self, glyphs: &[GlyphId], widths: &mut [scalar]) {
        self.get_widths(glyphs, widths)
    }

    pub fn get_widths(&self, glyphs: &[GlyphId], widths: &mut [scalar]) {
        self.get_widths_bounds(glyphs, Some(widths), None, None)
    }

    #[deprecated(since = "0.12.0", note = "use get_widths_bounds()")]
    pub fn widths_bounds(
        &self,
        glyphs: &[GlyphId],
        widths: Option<&mut [scalar]>,
        bounds: Option<&mut [Rect]>,
        paint: Option<&Paint>,
    ) {
        self.get_widths_bounds(glyphs, widths, bounds, paint)
    }

    pub fn get_widths_bounds(
        &self,
        glyphs: &[GlyphId],
        mut widths: Option<&mut [scalar]>,
        mut bounds: Option<&mut [Rect]>,
        paint: Option<&Paint>,
    ) {
        let count = glyphs.len();

        {
            if let Some(slice) = &widths {
                assert_eq!(count, slice.len())
            };
            if let Some(slice) = &bounds {
                assert_eq!(count, slice.len())
            };
        }

        let bounds_ptr = bounds.native_mut().as_ptr_or_null_mut();
        let widths_ptr = widths.as_ptr_or_null_mut();
        let paint_ptr = paint.native_ptr_or_null();

        unsafe {
            self.native().getWidthsBounds(
                glyphs.as_ptr(),
                count.try_into().unwrap(),
                widths_ptr,
                bounds_ptr,
                paint_ptr,
            )
        }
    }

    #[deprecated(since = "0.12.0", note = "use get_bounds()")]
    pub fn bounds(&self, glyphs: &[GlyphId], bounds: &mut [Rect], paint: Option<&Paint>) {
        self.get_bounds(glyphs, bounds, paint)
    }

    pub fn get_bounds(&self, glyphs: &[GlyphId], bounds: &mut [Rect], paint: Option<&Paint>) {
        self.get_widths_bounds(glyphs, None, Some(bounds), paint)
    }

    #[deprecated(since = "0.12.0", note = "use get_pos()")]
    pub fn pos(&self, glyphs: &[GlyphId], pos: &mut [Point], origin: Option<Point>) {
        self.get_pos(glyphs, pos, origin)
    }

    pub fn get_pos(&self, glyphs: &[GlyphId], pos: &mut [Point], origin: Option<Point>) {
        let count = glyphs.len();
        assert_eq!(count, pos.len());

        let origin = origin.unwrap_or_default();

        unsafe {
            self.native().getPos(
                glyphs.as_ptr(),
                count.try_into().unwrap(),
                pos.native_mut().as_mut_ptr(),
                origin.native().clone(),
            )
        }
    }

    #[deprecated(since = "0.12.0", note = "use get_x_pos()")]
    pub fn x_pos(&self, glyphs: &[GlyphId], xpos: &mut [scalar], origin: Option<scalar>) {
        self.get_x_pos(glyphs, xpos, origin)
    }

    pub fn get_x_pos(&self, glyphs: &[GlyphId], xpos: &mut [scalar], origin: Option<scalar>) {
        let count = glyphs.len();
        assert_eq!(count, xpos.len());
        let origin = origin.unwrap_or_default();

        unsafe {
            self.native().getXPos(
                glyphs.as_ptr(),
                count.try_into().unwrap(),
                xpos.as_mut_ptr(),
                origin,
            )
        }
    }

    #[deprecated(since = "0.12.0", note = "use get_path()")]
    pub fn path(&self, glyph_id: GlyphId) -> Option<Path> {
        self.get_path(glyph_id)
    }

    pub fn get_path(&self, glyph_id: GlyphId) -> Option<Path> {
        let mut path = Path::default();
        unsafe { self.native().getPath(glyph_id, path.native_mut()) }.if_true_some(path)
    }

    // TODO: getPaths() (needs a function to be passed, but supports a context).

    pub fn metrics(&self) -> (scalar, FontMetrics) {
        let mut fm = unsafe { mem::zeroed() };
        let line_spacing = unsafe { self.native().getMetrics(&mut fm) };
        (line_spacing, FontMetrics::from_native(fm))
    }

    pub fn spacing(&self) -> scalar {
        unsafe { self.native().getMetrics(ptr::null_mut()) }
    }
}
