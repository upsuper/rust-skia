use crate::prelude::*;
use crate::scalar;
use skia_bindings::{C_SkSize_toFloor, SkISize, SkSize};

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub struct ISize {
    pub width: i32,
    pub height: i32,
}

impl NativeTransmutable<SkISize> for ISize {}

#[test]
fn test_isize_layout() {
    ISize::test_layout()
}

impl ISize {
    pub fn new(w: i32, h: i32) -> ISize {
        ISize {
            width: w,
            height: h,
        }
    }

    pub fn new_empty() -> ISize {
        Self::new(0, 0)
    }

    pub fn set(&mut self, w: i32, h: i32) {
        *self = Self::new(w, h);
    }

    pub fn is_zero(self) -> bool {
        self.width == 0 && self.height == 0
    }

    pub fn is_empty(self) -> bool {
        self.width <= 0 || self.height <= 0
    }

    pub fn set_empty(&mut self) {
        *self = Self::new_empty();
    }

    // TODO: should the functions with() and height() be supported?

    pub fn equals(self, w: i32, h: i32) -> bool {
        self == Self::new(w, h)
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Size {
    pub width: scalar,
    pub height: scalar,
}

impl NativeTransmutable<SkSize> for Size {}

#[test]
fn test_size_layout() {
    Size::test_layout()
}

impl Size {
    pub fn new(w: scalar, h: scalar) -> Size {
        Size {
            width: w,
            height: h,
        }
    }

    pub fn from_isize(src: ISize) -> Size {
        Self::new(src.width as _, src.height as _)
    }

    pub fn new_empty() -> Self {
        Self::new(0.0, 0.0)
    }

    pub fn set(&mut self, w: scalar, h: scalar) {
        *self = Self::new(w, h);
    }

    pub fn is_zero(self) -> bool {
        self.width == 0.0 && self.height == 0.0
    }

    pub fn is_empty(self) -> bool {
        self.width <= 0.0 || self.height <= 0.0
    }

    pub fn set_empty(&mut self) {
        *self = Self::new_empty()
    }

    // TODO: should width() and height() be supported?

    pub fn equals(self, w: scalar, h: scalar) -> bool {
        self == Self::new(w, h)
    }

    pub fn to_round(self) -> ISize {
        ISize::from_native(unsafe { self.native().toRound() })
    }

    pub fn to_ceil(self) -> ISize {
        ISize::from_native(unsafe { self.native().toCeil() })
    }

    pub fn to_floor(self) -> ISize {
        // does not link:
        // ISize::from_native(unsafe { self.native().toFloor() })
        ISize::from_native(unsafe { C_SkSize_toFloor(self.native()) })
    }
}

//
// From
//

impl From<(i32, i32)> for ISize {
    fn from(source: (i32, i32)) -> Self {
        Self::new(source.0, source.1)
    }
}

impl From<(scalar, scalar)> for Size {
    fn from(source: (scalar, scalar)) -> Self {
        Self::new(source.0, source.1)
    }
}

impl From<ISize> for Size {
    fn from(size: ISize) -> Self {
        Self::new(size.width as _, size.height as _)
    }
}

// TODO: this is experimental.
impl From<(i32, i32)> for Size {
    fn from(source: (i32, i32)) -> Self {
        (source.0 as scalar, source.1 as scalar).into()
    }
}
