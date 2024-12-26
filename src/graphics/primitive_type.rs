use std::mem;

use csfml_sys::sfPrimitiveType;

use super::vertex::Vertex;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PrimitiveType {
    Points,
    Lines,
    LineStrip,
    Triangles,
    TriangleStrip,
    TriangleFan,
    Quads,
}

impl PrimitiveType {
    #[must_use]
    pub fn to_csfml(self) -> sfPrimitiveType {
        unsafe { mem::transmute::<Self, u32>(self) }
    }

    /// Returns the corresponding primitive type for iteration.
    #[must_use]
    pub fn r#type(self) -> Primitive {
        match self {
            Self::Points => Primitive::PointPrimitive(Vertex::default()),
            Self::Lines => Primitive::LinePrimitive(Vertex::default(), Vertex::default()),
            Self::Triangles => Primitive::TrianglePrimitive(
                Vertex::default(),
                Vertex::default(),
                Vertex::default(),
            ),
            Self::Quads => Primitive::QuadPrimitive(
                Vertex::default(),
                Vertex::default(),
                Vertex::default(),
                Vertex::default(),
            ),
            _ => panic!("Primitive type not supported"),
        }
    }

    /// Returns how many vertices each primitive consists of.
    #[must_use]
    pub fn vertex_count(self) -> usize {
        match self {
            Self::Points => 1,
            Self::Lines => 2,
            Self::Triangles => 3,
            Self::Quads => 4,
            _ => panic!("Primitive type not supported"),
        }
    }
}

impl From<sfPrimitiveType> for PrimitiveType {
    fn from(value: sfPrimitiveType) -> Self {
        unsafe { mem::transmute(value) }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Primitive {
    PointPrimitive(Vertex),
    LinePrimitive(Vertex, Vertex),
    TrianglePrimitive(Vertex, Vertex, Vertex),
    QuadPrimitive(Vertex, Vertex, Vertex, Vertex),
}
