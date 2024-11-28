use crate::system::{Vector2, Vector3, Vector4};

// Define vector types using nalgebra's fixed-size vector types
pub type FVec2 = Vector2<f32>;
pub type FVec3 = Vector3<f32>;
pub type FVec4 = Vector4<f32>;

pub type IVec2 = Vector2<i32>;
pub type IVec3 = Vector3<i32>;
pub type IVec4 = Vector4<i32>;

pub type BVec2 = Vector2<bool>;
pub type BVec3 = Vector3<bool>;
pub type BVec4 = Vector4<bool>;

// Define matrices as fixed-size arrays or use nalgebra's Matrix types
pub type Mat3 = [f32; 3 * 3];
pub type Mat4 = [f32; 3 * 3];
