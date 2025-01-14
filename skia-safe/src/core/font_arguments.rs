use crate::prelude::*;
use skia_bindings::{
    C_SkFontArguments_construct, C_SkFontArguments_destruct,
    C_SkFontArguments_getVariationDesignPosition, C_SkFontArguments_setVariationDesignPosition,
    SkFontArguments, SkFontArguments_VariationPosition,
};
use std::marker::PhantomData;
use std::{mem, slice};

#[derive(Debug)]
pub struct VariationPosition<'a> {
    pub coordinates: &'a [variation_position::Coordinate],
}

pub mod variation_position {
    use crate::prelude::*;
    use crate::FourByteTag;
    use skia_bindings::SkFontArguments_VariationPosition_Coordinate;

    #[derive(Copy, Clone, PartialEq, Default, Debug)]
    #[repr(C)]
    pub struct Coordinate {
        pub axis: FourByteTag,
        pub value: f32,
    }

    impl NativeTransmutable<SkFontArguments_VariationPosition_Coordinate> for Coordinate {}
    #[test]
    fn test_coordinate_layout() {
        Coordinate::test_layout()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct FontArguments<'a> {
    args: SkFontArguments,
    pd: PhantomData<&'a [variation_position::Coordinate]>,
}

impl<'a> NativeTransmutable<SkFontArguments> for FontArguments<'a> {}
#[test]
fn test_font_arguments_layout() {
    FontArguments::test_layout()
}

impl<'a> Drop for FontArguments<'a> {
    fn drop(&mut self) {
        unsafe { C_SkFontArguments_destruct(self.native_mut()) }
    }
}

impl<'a> Default for FontArguments<'a> {
    fn default() -> Self {
        FontArguments::new()
    }
}

impl<'a> FontArguments<'a> {
    pub fn new() -> Self {
        Self::from_native(unsafe {
            // does not link under Linux / macOS
            // SkFontArguments::new()
            let mut font_arguments = mem::zeroed();
            C_SkFontArguments_construct(&mut font_arguments);
            font_arguments
        })
    }

    pub fn set_collection_index(&mut self, collection_index: usize) -> &mut Self {
        unsafe {
            self.native_mut()
                .setCollectionIndex(collection_index.try_into().unwrap());
        }
        self
    }

    // This function consumes self for it to be able to change its lifetime,
    // because it borrows the coordinates referenced by FontArgumentsVariationPosition.
    pub fn set_variation_design_position(mut self, position: VariationPosition) -> FontArguments /* not Self!! */
    {
        let position = SkFontArguments_VariationPosition {
            coordinates: position.coordinates.native().as_ptr(),
            coordinateCount: position.coordinates.len().try_into().unwrap(),
        };
        unsafe {
            // does not link on Linux / macOS:
            C_SkFontArguments_setVariationDesignPosition(self.native_mut(), position);
            // note: we are _not_ returning Self here, but VariationPosition with a
            // changed lifetime.
            mem::transmute(self)
        }
    }

    pub fn collection_index(&self) -> usize {
        unsafe { self.native().getCollectionIndex() }
            .try_into()
            .unwrap()
    }

    pub fn variation_design_position(&self) -> VariationPosition {
        unsafe {
            let position = C_SkFontArguments_getVariationDesignPosition(self.native());
            VariationPosition {
                coordinates: slice::from_raw_parts(
                    position.coordinates as *const _,
                    position.coordinateCount.try_into().unwrap(),
                ),
            }
        }
    }
}

#[test]
fn test_font_arguments_with_no_coordinates() {
    let fa = FontArguments::new();
    dbg!(&fa);
    let coordinates = fa.variation_design_position();
    assert_eq!(coordinates.coordinates, []);
}

#[test]
#[allow(clippy::float_cmp)]
fn access_coordinates() {
    let coordinates = Box::new([variation_position::Coordinate {
        axis: 0.into(),
        value: 1.0,
    }]);
    let args = FontArguments::new();
    let args = args.set_variation_design_position(VariationPosition {
        coordinates: coordinates.as_ref(),
    });
    assert_eq!(args.variation_design_position().coordinates[0].value, 1.0);
    drop(args);
}
