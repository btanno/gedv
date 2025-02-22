use super::*;

pub trait Collision<T> {
    fn is_crossing(&self, rhs: &T) -> bool;
    fn contains(&self, inner: &T) -> bool;
}

#[inline]
pub fn is_crossing<T, U>(lhs: &T, rhs: &U) -> bool
where
    T: Collision<U>,
{
    lhs.is_crossing(rhs)
}

#[inline]
pub fn contains<T, U>(outer: &T, inner: &U) -> bool
where
    T: Collision<U>,
{
    outer.contains(inner)
}

impl<T, Coord> Collision<Rect<T, Coord>> for Position<T, Coord>
where
    T: num::Num + PartialOrd,
{
    #[inline]
    fn is_crossing(&self, rhs: &Rect<T, Coord>) -> bool {
        self.x >= rhs.left && self.y >= rhs.top && self.x <= rhs.right && self.y <= rhs.bottom
    }

    #[inline]
    fn contains(&self, inner: &Rect<T, Coord>) -> bool {
        self.is_crossing(inner)
    }
}

impl<T, Coord> Collision<Position<T, Coord>> for Rect<T, Coord>
where
    T: num::Num + PartialOrd,
{
    #[inline]
    fn is_crossing(&self, rhs: &Position<T, Coord>) -> bool {
        rhs.is_crossing(self)
    }

    #[inline]
    fn contains(&self, inner: &Position<T, Coord>) -> bool {
        self.is_crossing(inner)
    }
}

impl<T, Coord> Collision<Rect<T, Coord>> for Rect<T, Coord>
where
    T: num::Num + PartialOrd,
{
    #[inline]
    fn is_crossing(&self, rhs: &Rect<T, Coord>) -> bool {
        self.left <= rhs.right
            && self.top <= rhs.bottom
            && self.right >= rhs.left
            && self.bottom >= rhs.top
    }

    #[inline]
    fn contains(&self, inner: &Rect<T, Coord>) -> bool {
        self.left <= inner.left
            && self.top <= inner.top
            && self.right >= inner.right
            && self.bottom >= inner.bottom
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect_is_crossing_point() {
        let r = LogicalRect::from_position_size((10, 10), (10, 10));
        assert!(is_crossing(&r, &LogicalPosition::new(10, 10)));
        assert!(is_crossing(&r, &LogicalPosition::new(10, 20)));
        assert!(is_crossing(&r, &LogicalPosition::new(20, 10)));
        assert!(is_crossing(&r, &LogicalPosition::new(20, 20)));
        assert!(is_crossing(&r, &LogicalPosition::new(15, 15)));
        assert!(!is_crossing(&r, &LogicalPosition::new(9, 10)));
        assert!(is_crossing(&LogicalPosition::new(15, 15), &r));
    }

    #[test]
    fn rect_contains_point() {
        let r = LogicalRect::from_position_size((10, 10), (10, 10));
        assert!(contains(&r, &LogicalPosition::new(10, 10)));
        assert!(contains(&r, &LogicalPosition::new(10, 20)));
        assert!(contains(&r, &LogicalPosition::new(20, 10)));
        assert!(contains(&r, &LogicalPosition::new(20, 20)));
        assert!(contains(&r, &LogicalPosition::new(15, 15)));
        assert!(!contains(&r, &LogicalPosition::new(9, 10)));
    }

    #[test]
    fn point_contains_rect() {
        let p = LogicalPosition::new(10, 10);
        assert!(contains(
            &p,
            &LogicalRect::from_position_size((10, 10), (11, 11))
        ));
        assert!(contains(
            &p,
            &LogicalRect::from_position_size((9, 9), (10, 10))
        ));
        assert!(contains(
            &p,
            &LogicalRect::from_position_size((1, 1), (20, 20))
        ));
        assert!(!contains(
            &p,
            &LogicalRect::from_position_size((11, 11), (12, 12))
        ));
    }

    #[test]
    fn rect_contains_rect() {
        let r = LogicalRect::from_position_size((10, 10), (10, 10));
        assert!(contains(
            &r,
            &LogicalRect::from_position_size((10, 10), (10, 10))
        ));
        assert!(contains(
            &r,
            &LogicalRect::from_position_size((10, 10), (1, 1))
        ));
        assert!(contains(
            &r,
            &LogicalRect::from_position_size((19, 10), (1, 1))
        ));
        assert!(contains(
            &r,
            &LogicalRect::from_position_size((10, 19), (1, 1))
        ));
        assert!(contains(
            &r,
            &LogicalRect::from_position_size((19, 19), (1, 1))
        ));
        assert!(contains(
            &r,
            &LogicalRect::from_position_size((15, 15), (1, 1))
        ));
        assert!(!contains(
            &r,
            &LogicalRect::from_position_size((9, 9), (1, 1))
        ));
        assert!(!contains(
            &r,
            &LogicalRect::from_position_size((20, 20), (1, 1))
        ));
    }
}
