use windows::Win32::Foundation::RECT;

pub trait HasLeft {
    fn left(&self) -> i32;
}
pub trait HasRight {
    fn right(&self) -> i32;
}
pub trait HasTop {
    fn top(&self) -> i32;
}
pub trait HasBottom {
    fn bottom(&self) -> i32;
}
impl HasLeft for RECT {
    fn left(&self) -> i32 {
        self.left
    }
}
impl HasRight for RECT {
    fn right(&self) -> i32 {
        self.right
    }
}
impl HasTop for RECT {
    fn top(&self) -> i32 {
        self.top
    }
}
impl HasBottom for RECT {
    fn bottom(&self) -> i32 {
        self.bottom
    }
}

pub trait HasWidth {
    fn width(&self) -> i32;
}
pub trait HasHeight {
    fn height(&self) -> i32;
}

impl<T> HasWidth for T
where
    T: HasRight + HasLeft,
{
    fn width(&self) -> i32 {
        self.right() - self.left()
    }
}
impl<T> HasHeight for T
where
    T: HasBottom + HasTop,
{
    fn height(&self) -> i32 {
        self.bottom() - self.top()
    }
}

pub trait HasTopLeft {
    fn top_left(&self) -> (i32, i32);
}
pub trait HasTopRight {
    fn top_right(&self) -> (i32, i32);
}
pub trait HasBottomLeft {
    fn bottom_left(&self) -> (i32, i32);
}
pub trait HasBottomRight {
    fn bottom_right(&self) -> (i32, i32);
}
impl<T> HasTopLeft for T
where
    T: HasTop + HasLeft,
{
    fn top_left(&self) -> (i32, i32) {
        (self.left(), self.top())
    }
}
impl<T> HasTopRight for T
where
    T: HasTop + HasRight,
{
    fn top_right(&self) -> (i32, i32) {
        (self.right(), self.top())
    }
}
impl<T> HasBottomLeft for T
where
    T: HasBottom + HasLeft,
{
    fn bottom_left(&self) -> (i32, i32) {
        (self.left(), self.bottom())
    }
}
impl<T> HasBottomRight for T
where
    T: HasBottom + HasRight,
{
    fn bottom_right(&self) -> (i32, i32) {
        (self.right(), self.bottom())
    }
}

pub trait FromCorners {
    fn from_corners(p0: (i32, i32), p1: (i32, i32)) -> Self;
}
impl FromCorners for RECT {
    fn from_corners(p0: (i32, i32), p1: (i32, i32)) -> Self {
        RECT {
            left: p0.0.min(p1.0),
            top: p0.1.min(p1.1),
            right: p1.0.max(p0.0),
            bottom: p1.1.max(p0.1),
        }
    }
}

pub trait Translatable {
    fn translate(&self, dx: i32, dy: i32) -> Self;
}
impl Translatable for RECT {
    fn translate(&self, dx: i32, dy: i32) -> Self {
        RECT {
            left: self.left + dx,
            top: self.top + dy,
            right: self.right + dx,
            bottom: self.bottom + dy,
        }
    }
}
impl Translatable for (i32, i32) {
    fn translate(&self, dx: i32, dy: i32) -> Self {
        (self.0 + dx, self.1 + dy)
    }
}
