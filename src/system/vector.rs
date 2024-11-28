use std::ops::Mul;

use derive_more::derive::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use csfml_sys::{sfVector2f, sfVector2i, sfVector2u, sfVector3f};

pub type Vector2i = Vector2<i32>;
pub type Vector2u = Vector2<u32>;
pub type Vector2f = Vector2<f32>;
pub type Vextor3i = Vector3<isize>;
pub type Vector3f = Vector3<f32>;

#[repr(C)]
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Sub,
    AddAssign,
    SubAssign,
    Add,
    Mul,
    MulAssign,
    Div,
    DivAssign,
)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> {
    #[inline]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    #[must_use]
    pub fn scale(self, scalar: T) -> Self
    where
        T: Mul<Output = T> + Copy,
    {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl<T> From<(T, T)> for Vector2<T> {
    fn from((x, y): (T, T)) -> Self {
        Self { x, y }
    }
}

impl From<sfVector2f> for Vector2f {
    fn from(sfVector2f { x, y }: sfVector2f) -> Self {
        Self { x, y }
    }
}

impl From<Vector2f> for sfVector2f {
    fn from(Vector2f { x, y }: Vector2f) -> Self {
        Self { x, y }
    }
}

impl From<sfVector2i> for Vector2i {
    fn from(sfVector2i { x, y }: sfVector2i) -> Self {
        Self { x, y }
    }
}

impl From<Vector2i> for sfVector2i {
    fn from(Vector2i { x, y }: Vector2i) -> Self {
        Self { x, y }
    }
}

impl From<sfVector2u> for Vector2u {
    fn from(sfVector2u { x, y }: sfVector2u) -> Self {
        Self { x, y }
    }
}

impl From<Vector2u> for sfVector2u {
    fn from(Vector2u { x, y }: Vector2u) -> Self {
        Self { x, y }
    }
}

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Sub,
    AddAssign,
    SubAssign,
    Add,
    Mul,
    MulAssign,
    Div,
    DivAssign,
)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T> {
    #[inline]
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T> From<(T, T, T)> for Vector3<T> {
    fn from((x, y, z): (T, T, T)) -> Self {
        Self { x, y, z }
    }
}

impl From<sfVector3f> for Vector3f {
    fn from(sfVector3f { x, y, z }: sfVector3f) -> Self {
        Self { x, y, z }
    }
}

impl From<Vector3f> for sfVector3f {
    fn from(Vector3f { x, y, z }: Vector3f) -> Self {
        Self { x, y, z }
    }
}

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Sub,
    AddAssign,
    SubAssign,
    Add,
    Mul,
    MulAssign,
    Div,
    DivAssign,
)]
pub struct Vector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Vector4<T> {
    #[inline]
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
}

impl<T> From<(T, T, T, T)> for Vector4<T> {
    fn from((x, y, z, w): (T, T, T, T)) -> Self {
        Self { x, y, z, w }
    }
}

#[cfg(test)]
mod tests {
    use csfml_sys::{sfVector2i, sfVector3f};

    use crate::{assert_approx_eq, system::Vector3};

    use super::Vector2;

    #[test]
    fn cfml_vector2() {
        let vec = Vector2::new(1, 3);
        let cvec: sfVector2i = vec.into();

        assert_eq!(vec.x, cvec.x);
        assert_eq!(vec.y, cvec.y);

        let vec2 = Vector2::from(cvec);
        assert_eq!(vec, vec2);
    }

    #[test]
    fn cfml_vector3() {
        let vec = Vector3::new(1.0, 3.5, -12.0);
        let cvec: sfVector3f = vec.into();

        assert_approx_eq!(vec.x, cvec.x, 0.0);
        assert_approx_eq!(vec.y, cvec.y, 0.0);
        assert_approx_eq!(vec.z, cvec.z, 0.0);

        let vec2 = Vector3::from(cvec);

        assert_eq!(vec, vec2);
    }
}
