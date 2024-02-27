#[cfg(windows)]
use windows::Win32::Foundation::{POINT, RECT, SIZE};

pub mod coord {
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Logical;

    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Physical;

    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Screen;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Position<T, Coord> {
    pub x: T,
    pub y: T,
    #[cfg_attr(feature = "serde", serde(skip))]
    _coord: std::marker::PhantomData<Coord>,
}

impl<T, Coord> Position<T, Coord> {
    pub const fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
            _coord: std::marker::PhantomData,
        }
    }
}

impl<T, Coord> From<(T, T)> for Position<T, Coord> {
    #[inline]
    fn from(value: (T, T)) -> Self {
        Position::new(value.0, value.1)
    }
}

pub type PhysicalPosition<T> = Position<T, coord::Physical>;
pub type LogicalPosition<T> = Position<T, coord::Logical>;
pub type ScreenPosition<T> = Position<T, coord::Screen>;

impl<T, Coord> std::ops::Mul<T> for Position<T, Coord>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    type Output = Position<T, Coord>;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl<T, Coord> std::ops::Div<T> for Position<T, Coord>
where
    T: std::ops::Div<Output = T> + Copy,
{
    type Output = Position<T, Coord>;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Size<T, Coord> {
    pub width: T,
    pub height: T,
    #[cfg_attr(feature = "serde", serde(skip))]
    _coord: std::marker::PhantomData<Coord>,
}

impl<T, Coord> Size<T, Coord> {
    pub const fn new(width: T, height: T) -> Self {
        Self {
            width,
            height,
            _coord: std::marker::PhantomData,
        }
    }
}

impl<T, Coord> From<(T, T)> for Size<T, Coord> {
    #[inline]
    fn from(value: (T, T)) -> Self {
        Size::new(value.0, value.1)
    }
}

pub type PhysicalSize<T> = Size<T, coord::Physical>;
pub type LogicalSize<T> = Size<T, coord::Logical>;

impl<T, Coord> std::ops::Mul<T> for Size<T, Coord>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    type Output = Size<T, Coord>;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.width * rhs, self.height * rhs)
    }
}

impl<T, Coord> std::ops::Div<T> for Size<T, Coord>
where
    T: std::ops::Div<Output = T> + Copy,
{
    type Output = Size<T, Coord>;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.width / rhs, self.height / rhs)
    }
}

impl<T, Coord> std::ops::Add<Size<T, Coord>> for Position<T, Coord>
where
    T: std::ops::Add<T, Output = T>,
{
    type Output = Position<T, Coord>;

    #[inline]
    fn add(self, rhs: Size<T, Coord>) -> Self::Output {
        Position::new(self.x + rhs.width, self.y + rhs.height)
    }
}

impl<T, Coord> std::ops::Add<Position<T, Coord>> for Size<T, Coord>
where
    T: std::ops::Add<T, Output = T>,
{
    type Output = Position<T, Coord>;

    #[inline]
    fn add(self, rhs: Position<T, Coord>) -> Self::Output {
        Position::new(self.width + rhs.x, self.height + rhs.y)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rect<T, Coord> {
    pub left: T,
    pub top: T,
    pub right: T,
    pub bottom: T,
    #[cfg_attr(feature = "serde", serde(skip))]
    _coord: std::marker::PhantomData<Coord>,
}

impl<T, Coord> Rect<T, Coord> {
    pub const fn new(left: T, top: T, right: T, bottom: T) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
            _coord: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn from_positions(left_top: impl Into<Position<T, Coord>>, right_bottom: impl Into<Position<T, Coord>>) -> Self {
        let left_top = left_top.into();
        let right_bottom = right_bottom.into();
        Self {
            left: left_top.x,
            top: left_top.y,
            right: right_bottom.x,
            bottom: right_bottom.y,
            _coord: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn from_position_size(position: impl Into<Position<T, Coord>>, size: impl Into<Size<T, Coord>>) -> Self
    where
        T: std::ops::Add<T, Output = T> + Clone,
    {
        let position = position.into();
        let size = size.into();
        Self {
            left: position.x.clone(),
            top: position.y.clone(),
            right: position.x + size.width,
            bottom: position.y + size.height,
            _coord: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn left_top(&self) -> Position<T, Coord>
    where
        T: Clone,
    {
        Position::new(self.left.clone(), self.top.clone())
    }

    #[inline]
    pub fn left_bottom(&self) -> Position<T, Coord>
    where
        T: Clone,
    {
        Position::new(self.left.clone(), self.bottom.clone())
    }

    #[inline]
    pub fn right_top(&self) -> Position<T, Coord>
    where
        T: Clone,
    {
        Position::new(self.right.clone(), self.top.clone())
    }

    #[inline]
    pub fn right_bottom(&self) -> Position<T, Coord>
    where
        T: Clone,
    {
        Position::new(self.right.clone(), self.bottom.clone())
    }

    #[inline]
    pub fn size(&self) -> Size<T, Coord>
    where
        T: Clone + std::ops::Sub<Output = T>,
    {
        Size::new(
            self.right.clone() - self.left.clone(),
            self.bottom.clone() - self.top.clone(),
        )
    }
}

impl<T, Coord> From<(T, T, T, T)> for Rect<T, Coord> {
    #[inline]
    fn from(value: (T, T, T, T)) -> Self {
        Rect::new(value.0, value.1, value.2, value.3)
    }
}

pub type PhysicalRect<T> = Rect<T, coord::Physical>;
pub type LogicalRect<T> = Rect<T, coord::Logical>;

impl<T, Coord> std::ops::Mul<T> for Rect<T, Coord>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    type Output = Rect<T, Coord>;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Self::new(
            self.left * rhs,
            self.top * rhs,
            self.right * rhs,
            self.bottom * rhs,
        )
    }
}

impl<T, Coord> std::ops::Div<T> for Rect<T, Coord>
where
    T: std::ops::Div<Output = T> + Copy,
{
    type Output = Rect<T, Coord>;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Self::new(
            self.left / rhs,
            self.top / rhs,
            self.right / rhs,
            self.bottom / rhs,
        )
    }
}

impl<T, Coord> std::ops::Add<Position<T, Coord>> for Rect<T, Coord>
where
    T: std::ops::Add<Output = T> + Copy,
{
    type Output = Rect<T, Coord>;

    #[inline]
    fn add(self, rhs: Position<T, Coord>) -> Self::Output {
        Self::new(
            self.left + rhs.x,
            self.top + rhs.y,
            self.right + rhs.x,
            self.bottom + rhs.y,
        )
    }
}

impl<T, Coord> std::ops::Add<Rect<T, Coord>> for Position<T, Coord>
where
    T: std::ops::Add<Output = T> + Copy,
{
    type Output = Rect<T, Coord>;

    #[inline]
    fn add(self, rhs: Rect<T, Coord>) -> Self::Output {
        Rect::new(
            rhs.left + self.x,
            rhs.top + self.y,
            rhs.right + self.x,
            rhs.bottom + self.y,
        )
    }
}

pub const DEFAULT_DPI: u32 = 96;

#[cfg(windows)]
impl From<PhysicalPosition<i32>> for POINT {
    #[inline]
    fn from(value: PhysicalPosition<i32>) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

#[cfg(windows)]
impl From<POINT> for PhysicalPosition<i32> {
    #[inline]
    fn from(value: POINT) -> Self {
        Self::new(value.x, value.y)
    }
}

#[cfg(windows)]
impl From<PhysicalSize<u32>> for SIZE {
    #[inline]
    fn from(value: PhysicalSize<u32>) -> SIZE {
        SIZE {
            cx: value.width as i32,
            cy: value.height as i32,
        }
    }
}

#[cfg(windows)]
impl From<SIZE> for PhysicalSize<u32> {
    #[inline]
    fn from(value: SIZE) -> Self {
        PhysicalSize::new(value.cx as u32, value.cy as u32)
    }
}

#[cfg(windows)]
impl From<PhysicalRect<i32>> for RECT {
    #[inline]
    fn from(value: PhysicalRect<i32>) -> Self {
        RECT {
            left: value.left,
            top: value.top,
            right: value.right,
            bottom: value.bottom,
        }
    }
}

#[cfg(windows)]
impl From<RECT> for PhysicalRect<i32> {
    #[inline]
    fn from(value: RECT) -> Self {
        Self::new(value.left, value.top, value.right, value.bottom)
    }
}

pub trait ToLogical<T> {
    type Output<U>;

    fn to_logical(&self, dpi: T) -> Self::Output<T>;
}

pub trait ToPhysical<T> {
    type Output<U>;

    fn to_physical(&self, dpi: T) -> Self::Output<T>;
}

fn to_logical_value<T>(a: T, dpi: T) -> T
where
    T: num::Num + num::NumCast,
{
    a * num::cast(DEFAULT_DPI).unwrap() / dpi
}

fn to_physical_value<T>(a: T, dpi: T) -> T
where
    T: num::Num + num::NumCast,
{
    a * dpi / num::cast(DEFAULT_DPI).unwrap()
}

impl<T> ToLogical<T> for LogicalPosition<T>
where
    T: Copy,
{
    type Output<U> = LogicalPosition<U>;

    #[inline]
    fn to_logical(&self, _dpi: T) -> Self::Output<T> {
        *self
    }
}

impl<T> ToLogical<T> for LogicalSize<T>
where
    T: Copy,
{
    type Output<U> = LogicalSize<U>;

    #[inline]
    fn to_logical(&self, _dpi: T) -> Self::Output<T> {
        *self
    }
}

impl<T> ToLogical<T> for LogicalRect<T>
where
    T: Copy,
{
    type Output<U> = LogicalRect<U>;

    #[inline]
    fn to_logical(&self, _dpi: T) -> Self::Output<T> {
        *self
    }
}

impl<T> ToLogical<T> for PhysicalPosition<T>
where
    T: num::Num + num::NumCast + Copy,
{
    type Output<U> = LogicalPosition<U>;

    #[inline]
    fn to_logical(&self, dpi: T) -> Self::Output<T> {
        Position::new(to_logical_value(self.x, dpi), to_logical_value(self.y, dpi))
    }
}

impl<T> ToLogical<T> for PhysicalSize<T>
where
    T: num::Num + num::NumCast + Copy,
{
    type Output<U> = LogicalSize<U>;

    #[inline]
    fn to_logical(&self, dpi: T) -> Self::Output<T> {
        Size::new(
            to_logical_value(self.width, dpi),
            to_logical_value(self.height, dpi),
        )
    }
}

impl<T> ToLogical<T> for PhysicalRect<T>
where
    T: num::Num + num::NumCast + Copy,
{
    type Output<U> = LogicalRect<T>;

    #[inline]
    fn to_logical(&self, dpi: T) -> Self::Output<T> {
        Rect::new(
            to_logical_value(self.left, dpi),
            to_logical_value(self.top, dpi),
            to_logical_value(self.right, dpi),
            to_logical_value(self.bottom, dpi),
        )
    }
}

impl<T> ToPhysical<T> for LogicalPosition<T>
where
    T: num::Num + num::NumCast + Copy,
{
    type Output<U> = PhysicalPosition<U>;

    #[inline]
    fn to_physical(&self, dpi: T) -> Self::Output<T> {
        Position::new(
            to_physical_value(self.x, dpi),
            to_physical_value(self.y, dpi),
        )
    }
}

impl<T> ToPhysical<T> for LogicalSize<T>
where
    T: num::Num + num::NumCast + Copy,
{
    type Output<U> = PhysicalSize<U>;

    #[inline]
    fn to_physical(&self, dpi: T) -> Self::Output<T> {
        Size::new(
            to_physical_value(self.width, dpi),
            to_physical_value(self.height, dpi),
        )
    }
}

impl<T> ToPhysical<T> for LogicalRect<T>
where
    T: num::Num + num::NumCast + Copy,
{
    type Output<U> = PhysicalRect<U>;

    #[inline]
    fn to_physical(&self, dpi: T) -> Self::Output<T> {
        Rect::new(
            to_physical_value(self.left, dpi),
            to_physical_value(self.top, dpi),
            to_physical_value(self.right, dpi),
            to_physical_value(self.bottom, dpi),
        )
    }
}

impl<T> ToPhysical<T> for PhysicalPosition<T>
where
    T: Copy,
{
    type Output<U> = PhysicalPosition<U>;

    #[inline]
    fn to_physical(&self, _dpi: T) -> Self::Output<T> {
        *self
    }
}

impl<T> ToPhysical<T> for PhysicalSize<T>
where
    T: Copy,
{
    type Output<U> = PhysicalSize<U>;

    #[inline]
    fn to_physical(&self, _dpi: T) -> Self::Output<T> {
        *self
    }
}

impl<T> ToPhysical<T> for PhysicalRect<T>
where
    T: Copy,
{
    type Output<U> = PhysicalRect<U>;

    fn to_physical(&self, _dpi: T) -> Self::Output<T> {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logical_to_logical_position() {
        let src = LogicalPosition::new(128, 256);
        let dest = src.to_logical(DEFAULT_DPI * 2);
        assert!(src == dest);
    }

    #[test]
    fn logical_to_physical_position() {
        let src = LogicalPosition::new(128, 256);
        let dest = src.to_physical(DEFAULT_DPI * 2);
        assert!(src.x * 2 == dest.x);
        assert!(src.y * 2 == dest.y);
    }

    #[test]
    fn physical_to_logical_position() {
        let src = PhysicalPosition::new(128, 256);
        let dest = src.to_logical(DEFAULT_DPI * 2);
        assert!(src.x == dest.x * 2);
        assert!(src.y == dest.y * 2);
    }

    #[test]
    fn physical_to_physical_position() {
        let src = PhysicalPosition::new(128, 256);
        let dest = src.to_physical(DEFAULT_DPI * 2);
        assert!(src == dest);
    }

    #[test]
    fn logical_to_logical_size() {
        let src = LogicalSize::new(128, 256);
        let dest = src.to_logical(DEFAULT_DPI * 2);
        assert!(src == dest);
    }

    #[test]
    fn logical_to_physical_size() {
        let src = LogicalSize::new(128, 256);
        let dest = src.to_physical(DEFAULT_DPI * 2);
        assert!(src.width * 2 == dest.width);
        assert!(src.height * 2 == dest.height);
    }

    #[test]
    fn physical_to_logical_size() {
        let src = PhysicalSize::new(128, 256);
        let dest = src.to_logical(DEFAULT_DPI * 2);
        assert!(src.width == dest.width * 2);
        assert!(src.height == dest.height * 2);
    }

    #[test]
    fn physical_to_physical_size() {
        let src = PhysicalSize::new(128, 256);
        let dest = src.to_physical(DEFAULT_DPI * 2);
        assert!(src == dest);
    }

    #[test]
    fn logical_to_logical_rect() {
        let src = LogicalRect::new(6, 128, 256, 64);
        let dest = src.to_logical(DEFAULT_DPI * 2);
        assert!(src == dest);
    }

    #[test]
    fn logical_to_physical_rect() {
        let src = LogicalRect::new(6, 128, 256, 64);
        let dest = src.to_physical(DEFAULT_DPI * 2);
        assert!(src.left * 2 == dest.left);
        assert!(src.top * 2 == dest.top);
        assert!(src.right * 2 == dest.right);
        assert!(src.bottom * 2 == dest.bottom);
    }

    #[test]
    fn physical_to_logical_rect() {
        let src = PhysicalRect::new(6, 128, 256, 64);
        let dest = src.to_logical(DEFAULT_DPI * 2);
        assert!(src.left == dest.left * 2);
        assert!(src.top == dest.top * 2);
        assert!(src.right == dest.right * 2);
        assert!(src.bottom == dest.bottom * 2);
    }

    #[test]
    fn physical_to_physical_rect() {
        let src = PhysicalRect::new(6, 128, 256, 64);
        let dest = src.to_physical(DEFAULT_DPI * 2);
        assert!(src == dest);
    }

    #[test]
    fn position_mul() {
        let src = LogicalPosition::new(3, 4);
        let dest = src * 3;
        assert!(dest == LogicalPosition::new(9, 12));
    }

    #[test]
    fn size_mul() {
        let src = LogicalSize::new(3, 4);
        let dest = src * 3;
        assert!(dest == LogicalSize::new(9, 12));
    }

    #[test]
    fn rect_mul() {
        let src = LogicalRect::new(1, 2, 3, 4);
        let dest = src * 3;
        assert!(dest == LogicalRect::new(3, 6, 9, 12));
    }

    #[test]
    fn position_div() {
        let src = LogicalPosition::new(3, 6);
        let dest = src / 3;
        assert!(dest == LogicalPosition::new(1, 2));
    }

    #[test]
    fn size_div() {
        let src = LogicalSize::new(3, 6);
        let dest = src / 3;
        assert!(dest == LogicalSize::new(1, 2));
    }

    #[test]
    fn rect_div() {
        let src = LogicalRect::new(3, 6, 9, 12);
        let dest = src / 3;
        assert!(dest == LogicalRect::new(1, 2, 3, 4));
    }

    #[test]
    fn add_position_and_size() {
        let lhs = LogicalPosition::new(1, 2);
        let rhs = LogicalSize::new(10, 11);
        let dest = lhs + rhs;
        assert!(dest == LogicalPosition::new(11, 13));
    }

    #[test]
    fn add_size_and_position() {
        let lhs = LogicalSize::new(10, 11);
        let rhs = LogicalPosition::new(1, 2);
        let dest = lhs + rhs;
        assert!(dest == LogicalPosition::new(11, 13));
    }

    #[test]
    fn add_rect_and_position() {
        let lhs = LogicalRect::new(1, 2, 3, 4);
        let rhs = LogicalPosition::new(10, 11);
        let dest = lhs + rhs;
        assert!(dest == LogicalRect::new(11, 13, 13, 15));
    }

    #[test]
    fn add_position_and_rect() {
        let lhs = LogicalPosition::new(10, 11);
        let rhs = LogicalRect::new(1, 2, 3, 4);
        let dest = lhs + rhs;
        assert!(dest == LogicalRect::new(11, 13, 13, 15));
    }
}
