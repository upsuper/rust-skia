use crate::prelude::*;
use crate::{scalar, ISize, Size};
use skia_bindings::{SkIPoint, SkPoint};
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub type IVector = IPoint;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub struct IPoint {
    pub x: i32,
    pub y: i32,
}

impl NativeTransmutable<SkIPoint> for IPoint {}
#[test]
fn test_ipoint_layout() {
    IPoint::test_layout()
}

impl Neg for IPoint {
    type Output = IPoint;
    fn neg(self) -> Self::Output {
        IPoint::new(-self.x, -self.y)
    }
}

impl Add<IVector> for IPoint {
    type Output = IPoint;
    fn add(self, rhs: IVector) -> Self {
        IPoint::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign<IVector> for IPoint {
    fn add_assign(&mut self, rhs: IVector) {
        self.x += rhs.x;
        self.y += self.y;
    }
}

impl Add<ISize> for IPoint {
    type Output = IPoint;
    fn add(self, rhs: ISize) -> Self::Output {
        IPoint::new(self.x + rhs.width, self.y + rhs.height)
    }
}

impl AddAssign<ISize> for IPoint {
    fn add_assign(&mut self, rhs: ISize) {
        self.x += rhs.width;
        self.y += rhs.height;
    }
}

impl Sub for IPoint {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        IPoint::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign<IVector> for IPoint {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Sub<ISize> for IPoint {
    type Output = IPoint;
    fn sub(self, rhs: ISize) -> Self::Output {
        IPoint::new(self.x - rhs.width, self.y - rhs.height)
    }
}

impl SubAssign<ISize> for IPoint {
    fn sub_assign(&mut self, rhs: ISize) {
        self.x -= rhs.width;
        self.y -= rhs.height;
    }
}

impl IPoint {
    pub fn new(x: i32, y: i32) -> IPoint {
        IPoint { x, y }
    }

    pub fn is_zero(self) -> bool {
        // does not link:
        // unsafe { self.native().isZero() }
        (self.x | self.y) == 0
    }

    pub fn set(&mut self, x: i32, y: i32) {
        *self = IPoint::new(x, y);
    }

    pub fn equals(self, x: i32, y: i32) -> bool {
        self == IPoint::new(x, y)
    }
}

pub type Vector = Point;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Point {
    pub x: scalar,
    pub y: scalar,
}

impl NativeTransmutable<SkPoint> for Point {}

#[test]
fn test_point_layout() {
    Point::test_layout()
}

impl Neg for Point {
    type Output = Point;
    fn neg(self) -> Self::Output {
        Point::new(-self.x, -self.y)
    }
}

impl Add<Vector> for Point {
    type Output = Self;
    fn add(self, rhs: Vector) -> Self {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign<Vector> for Point {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add<Size> for Point {
    type Output = Self;
    fn add(self, rhs: Size) -> Self {
        Point::new(self.x + rhs.width, self.y + rhs.height)
    }
}

impl AddAssign<Size> for Point {
    fn add_assign(&mut self, rhs: Size) {
        self.x += rhs.width;
        self.y += rhs.height;
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign<Vector> for Point {
    fn sub_assign(&mut self, rhs: Vector) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Sub<Size> for Point {
    type Output = Self;
    fn sub(self, rhs: Size) -> Self {
        Point::new(self.x - rhs.width, self.y - rhs.height)
    }
}

impl SubAssign<Size> for Point {
    fn sub_assign(&mut self, rhs: Size) {
        self.x -= rhs.width;
        self.y -= rhs.height;
    }
}

impl Mul<scalar> for Point {
    type Output = Self;
    fn mul(self, rhs: scalar) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<scalar> for Point {
    fn mul_assign(&mut self, rhs: scalar) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Point {
    pub fn new(x: scalar, y: scalar) -> Self {
        Self { x, y }
    }

    pub fn is_zero(self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }

    pub fn set(&mut self, x: scalar, y: scalar) {
        *self = Self::new(x, y);
    }

    pub fn iset(&mut self, p: impl Into<IPoint>) {
        let p = p.into();
        self.x = p.x as scalar;
        self.y = p.y as scalar;
    }

    pub fn set_abs(&mut self, p: impl Into<Point>) {
        let p = p.into();
        self.x = p.x.abs();
        self.y = p.y.abs();
    }

    pub fn offset_points(points: &mut [Point], offset: impl Into<Vector>) {
        let offset = offset.into();
        points.iter_mut().for_each(|p| p.offset(offset));
    }

    pub fn offset(&mut self, d: impl Into<Vector>) {
        *self += d.into();
    }

    pub fn length(self) -> scalar {
        unsafe { self.native().length() }
    }

    pub fn distance_to_origin(self) -> scalar {
        self.length()
    }

    pub fn normalize(&mut self) -> bool {
        unsafe { self.native_mut().normalize() }
    }

    pub fn set_normalize(&mut self, x: scalar, y: scalar) -> bool {
        unsafe { self.native_mut().setNormalize(x, y) }
    }

    pub fn set_length(&mut self, length: scalar) -> bool {
        unsafe { self.native_mut().setLength(length) }
    }

    pub fn set_length_xy(&mut self, x: scalar, y: scalar, length: scalar) -> bool {
        unsafe { self.native_mut().setLength1(x, y, length) }
    }

    #[deprecated(since = "0.12.0", note = "use set_length()")]
    pub fn with_length(mut self, length: scalar) -> Option<Self> {
        unsafe { self.native_mut().setLength(length) }.if_true_some(self)
    }

    #[must_use]
    pub fn scaled(self, scale: scalar) -> Self {
        let mut p = Point::default();
        unsafe { self.native().scale(scale, p.native_mut()) }
        p
    }

    pub fn scale(&mut self, scale: scalar) {
        *self = self.scaled(scale);
    }

    pub fn negate(&mut self) {
        *self = -*self;
    }

    pub fn is_finite(self) -> bool {
        unsafe { self.native().isFinite() }
    }

    pub fn equals(self, x: scalar, y: scalar) -> bool {
        self == Point::new(x, y)
    }

    pub fn length_xy(x: scalar, y: scalar) -> scalar {
        unsafe { SkPoint::Length(x, y) }
    }

    pub fn normalize_vector(v: &mut Vector) -> scalar {
        unsafe { SkPoint::Normalize(v.native_mut()) }
    }

    pub fn distance(a: Self, b: Self) -> scalar {
        unsafe { SkPoint::Distance(a.native(), b.native()) }
    }

    pub fn dot_product(a: Self, b: Self) -> scalar {
        unsafe { SkPoint::DotProduct(a.native(), b.native()) }
    }

    pub fn cross_product(a: Self, b: Self) -> scalar {
        unsafe { SkPoint::CrossProduct(a.native(), b.native()) }
    }

    pub fn cross(self, vec: Vector) -> scalar {
        unsafe { self.native().cross(vec.native()) }
    }

    pub fn dot(self, vec: Vector) -> scalar {
        unsafe { self.native().dot(vec.native()) }
    }
}

//
// From
//

impl From<(i32, i32)> for IPoint {
    fn from(source: (i32, i32)) -> Self {
        IPoint::new(source.0, source.1)
    }
}

impl From<(scalar, scalar)> for Point {
    fn from(source: (scalar, scalar)) -> Self {
        Point::new(source.0, source.1)
    }
}

impl From<IPoint> for Point {
    fn from(source: IPoint) -> Self {
        Self::new(source.x as _, source.y as _)
    }
}

impl From<(i32, i32)> for Point {
    fn from(source: (i32, i32)) -> Self {
        (source.0 as scalar, source.1 as scalar).into()
    }
}
