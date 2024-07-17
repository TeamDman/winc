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
pub trait HasWidth {
    fn width(&self) -> i32;
}
pub trait HasHeight {
    fn height(&self) -> i32;
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
