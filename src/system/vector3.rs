use derive_more::derive::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use sfml_sys::sfVector3f;

pub type Vextor3i = Vector3<isize>;
pub type Vector3f = Vector3<f32>;

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

#[cfg(test)]
mod tests {
    use sfml_sys::sfVector3f;

    use crate::{assert_approx_eq, system::Vector3};

    #[test]
    fn cfml_vectors() {
        let vec = Vector3::new(1.0, 3.5, -12.0);
        let cvec: sfVector3f = vec.into();

        assert_approx_eq!(vec.x, cvec.x, 0.0);
        assert_approx_eq!(vec.y, cvec.y, 0.0);
        assert_approx_eq!(vec.z, cvec.z, 0.0);

        let vec2 = Vector3::from(cvec);

        assert_eq!(vec, vec2);
    }
}
