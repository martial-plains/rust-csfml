use csfml_sys::{
    sfColor, sfVector2f, sfVertex, sfVertexArray, sfVertexArray_append, sfVertexArray_clear,
    sfVertexArray_create, sfVertexArray_destroy, sfVertexArray_getBounds,
    sfVertexArray_getPrimitiveType, sfVertexArray_getVertex, sfVertexArray_getVertexCount,
    sfVertexArray_resize, sfVertexArray_setPrimitiveType, sfVertexBuffer, sfVertexBufferUsage,
    sfVertexBuffer_create, sfVertexBuffer_destroy, sfVertexBuffer_getPrimitiveType,
    sfVertexBuffer_getUsage, sfVertexBuffer_getVertexCount, sfVertexBuffer_isAvailable,
    sfVertexBuffer_update,
};

use crate::{system::Vector2f, types::Result};

use super::{color::Color, primitive_type::PrimitiveType, rect::FloatRect};

/// Define a point with color and texture coordinates.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct Vertex {
    /// Position of the vertex
    pub position: Vector2f,
    /// Color of the vertex
    pub color: Color,
    /// Texture coordinates of the vertex
    pub tex_coords: Vector2f,
}

impl Vertex {
    #[must_use]
    pub fn to_csml(&self) -> sfVertex {
        sfVertex {
            position: sfVector2f::from(self.position),
            color: sfColor::from(self.color),
            texCoords: sfVector2f::from(self.tex_coords),
        }
    }

    /// Allows iterating over a slice of vertices as if they were primitives.
    /// # Panics
    /// Panics if the number of vertices is not a multiple of the primitive type's packed size.
    #[must_use]
    pub fn vertices_as_primitives<'a, T>(
        vertices: &'a [Self],
        primitive_type: PrimitiveType,
    ) -> Vec<T>
    where
        T: Clone + 'a,
    {
        let vertices_len = vertices.len();
        let primitive_size = size_of_val(&primitive_type);

        assert!((vertices_len % primitive_size == 0),"The total number of vertices must be a multiple of the primitive type number of vertices"
            );

        let ret_len = vertices_len / primitive_size;
        let ret: Vec<T> = unsafe {
            let ptr = vertices.as_ptr().cast::<T>();
            std::slice::from_raw_parts(ptr, ret_len).to_vec()
        };

        ret
    }
}

/// A wrapper for SFML's `sfVertexArray` structure.
pub struct VertexArray {
    pub(crate) ptr: *mut sfVertexArray,
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe { sfVertexArray_destroy(self.ptr) };
    }
}

impl VertexArray {
    /// Creates an empty vertex array.
    pub fn create() -> Result<Self> {
        let va = unsafe { sfVertexArray_create() };
        if va.is_null() {
            Err("Failed to create vertex array".into())
        } else {
            Ok(Self { ptr: va })
        }
    }

    /// Creates a vertex array from a slice of vertices.
    pub fn create_from_slice(vertices: &[Vertex], primitive: PrimitiveType) -> Result<Self> {
        let va = unsafe { sfVertexArray_create() };
        if va.is_null() {
            return Err("Failed to create vertex array".into());
        }

        unsafe {
            sfVertexArray_setPrimitiveType(va, primitive.to_csfml());
            sfVertexArray_resize(va, vertices.len());
            for (i, vertex) in vertices.iter().enumerate() {
                sfVertexArray_getVertex(va, i).write(vertex.to_csml());
            }
        }

        Ok(Self { ptr: va })
    }

    /// Destroys the vertex array.
    pub fn destroy(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                sfVertexArray_destroy(self.ptr);
            }
        }
        self.ptr = std::ptr::null_mut();
    }

    /// Gets the vertex count of the vertex array.
    #[must_use]
    pub fn get_vertex_count(&self) -> usize {
        unsafe { sfVertexArray_getVertexCount(self.ptr) as usize }
    }

    /// Gets a vertex by its index.
    #[must_use]
    pub fn get_vertex(&self, index: usize) -> Option<&Vertex> {
        unsafe {
            if index >= self.get_vertex_count() {
                return None;
            }
            let vertex = sfVertexArray_getVertex(self.ptr, index).cast::<Vertex>();
            vertex.as_ref()
        }
    }

    /// Gets a slice of all the vertices.
    #[must_use]
    pub fn get_slice(&self) -> Vec<Vertex> {
        (0..self.get_vertex_count())
            .filter_map(|i| self.get_vertex(i))
            .copied()
            .collect()
    }

    /// Gets a vertex by its index.
    #[must_use]
    pub fn get_vertex_mut(&self, index: usize) -> Option<&mut Vertex> {
        unsafe {
            if index >= self.get_vertex_count() {
                return None;
            }
            let vertex = sfVertexArray_getVertex(self.ptr, index).cast::<Vertex>();
            vertex.as_mut()
        }
    }

    /// Clears the vertex array.
    pub fn clear(&mut self) {
        unsafe {
            sfVertexArray_clear(self.ptr);
        }
    }

    /// Resizes the vertex array.
    pub fn resize(&mut self, new_size: usize) {
        unsafe {
            sfVertexArray_resize(self.ptr, new_size);
        }
    }

    /// Appends a vertex to the array.
    pub fn append(&mut self, vertex: Vertex) {
        unsafe {
            sfVertexArray_append(self.ptr, vertex.to_csml());
        }
    }

    /// Gets the primitive type of the vertex array.
    #[must_use]
    pub fn get_primitive_type(&self) -> PrimitiveType {
        unsafe {
            let primitive_type = sfVertexArray_getPrimitiveType(self.ptr);
            PrimitiveType::from(primitive_type)
        }
    }

    /// Sets the primitive type of the vertex array.
    pub fn set_primitive_type(&mut self, primitive: PrimitiveType) {
        unsafe {
            sfVertexArray_setPrimitiveType(self.ptr, primitive.to_csfml());
        }
    }

    /// Gets the bounding rectangle of the vertex array.
    #[must_use]
    pub fn get_bounds(&self) -> FloatRect {
        unsafe {
            let bounds = sfVertexArray_getBounds(self.ptr);
            FloatRect::from_csfml(bounds)
        }
    }
}

/// A wrapper for SFML's `sfVertexBuffer` structure.
pub struct VertexBuffer {
    pub(crate) ptr: *mut sfVertexBuffer,
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        self.destroy();
    }
}

/// The usage types for vertex buffers
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Usage {
    Static = 0,
    Dynamic = 1,
    Stream = 2,
}

impl Usage {
    #[must_use]
    pub const fn to_csfml(self) -> sfVertexBufferUsage {
        self as sfVertexBufferUsage
    }
}

impl VertexBuffer {
    /// Creates a vertex buffer from a slice of vertices with a given primitive type and usage.
    pub fn create_from_slice(
        vertices: &[Vertex],
        primitive: PrimitiveType,
        usage: Usage,
    ) -> Result<Self> {
        let ptr = unsafe {
            sfVertexBuffer_create(
                u32::try_from(vertices.len()).map_err(|e| e.to_string())?,
                primitive.to_csfml(),
                usage.to_csfml(),
            )
        };

        if ptr.is_null() {
            return Err("Failed to create vertex buffer".into());
        }

        let update_result = unsafe {
            sfVertexBuffer_update(
                ptr,
                vertices.as_ptr().cast(),
                u32::try_from(vertices.len()).map_err(|e| e.to_string())?,
                0,
            )
        };

        if update_result != 1 {
            unsafe { sfVertexBuffer_destroy(ptr) };
            return Err("Failed to update vertex buffer".into());
        }

        Ok(Self { ptr })
    }

    /// Destroys the vertex buffer
    pub fn destroy(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                sfVertexBuffer_destroy(self.ptr);
            }
            self.ptr = std::ptr::null_mut();
        }
    }

    /// Updates the vertex buffer with new data
    pub fn update_from_slice(&mut self, vertices: &[Vertex]) -> Result<()> {
        let result = unsafe {
            sfVertexBuffer_update(
                self.ptr,
                vertices.as_ptr().cast(),
                u32::try_from(vertices.len()).map_err(|e| e.to_string())?,
                0,
            )
        };

        (result != 1)
            .then_some(())
            .ok_or_else(|| "Failed to update vertex buffer".into())
    }

    /// Gets the vertex count of the vertex buffer
    #[must_use]
    pub fn get_vertex_count(&self) -> usize {
        unsafe { sfVertexBuffer_getVertexCount(self.ptr) as usize }
    }

    /// Gets the primitive type of the vertex buffer
    #[must_use]
    pub fn get_primitive_type(&self) -> PrimitiveType {
        unsafe {
            let primitive_type = sfVertexBuffer_getPrimitiveType(self.ptr);
            PrimitiveType::from(primitive_type)
        }
    }

    /// Gets the usage of the vertex buffer
    #[must_use]
    pub fn get_usage(&self) -> Usage {
        unsafe {
            let usage = sfVertexBuffer_getUsage(self.ptr);
            std::mem::transmute(usage)
        }
    }

    /// Checks if vertex buffers are available in the system
    #[must_use]
    pub fn is_available() -> bool {
        unsafe { sfVertexBuffer_isAvailable() != 0 }
    }
}

// Test case for the VertexArray
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_array_methods() {
        let vertices = [
            Vertex {
                position: Vector2f::new(-1.0, 0.0),
                color: Color::RED,
                ..Default::default()
            },
            Vertex {
                position: Vector2f::new(1.0, 0.0),
                color: Color::GREEN,
                ..Default::default()
            },
            Vertex {
                position: Vector2f::new(-1.0, 1.0),
                color: Color::BLUE,
                ..Default::default()
            },
        ];

        let mut va = VertexArray::create_from_slice(&vertices, PrimitiveType::Triangles).unwrap();

        va.append(Vertex {
            position: Vector2f::new(1.0, 1.0),
            color: Color::YELLOW,
            ..Default::default()
        });

        va.set_primitive_type(PrimitiveType::Quads);

        assert_eq!(va.get_vertex_count(), 4);

        assert_eq!(va.get_primitive_type(), PrimitiveType::Quads);

        assert_eq!(va.get_bounds(), FloatRect::new(-1.0, 0.0, 2.0, 1.0));

        va.resize(3);
        va.set_primitive_type(PrimitiveType::TriangleFan);
        assert_eq!(va.get_vertex_count(), 3);

        let vert = va.get_vertex(0).unwrap();
        assert_eq!(vert.position, Vector2f::new(-1.0, 0.0));
        assert_eq!(vert.color, Color::RED);

        {
            let vertix = va.get_vertex_mut(1).unwrap();
            vertix.position = Vector2f::new(1.0, 1.0);
            vertix.color = Color::YELLOW;
        }

        let slice = va.get_slice();
        assert_eq!(slice.len(), 3);
        assert_eq!(slice[1].color, Color::YELLOW);

        va.clear();
        assert_eq!(va.get_vertex_count(), 0);

        let va = VertexArray::create().unwrap();
        assert_eq!(va.get_vertex_count(), 0);
    }

    #[test]
    fn test_vertex_buffer_methods() {
        let va_slice = [
            Vertex {
                position: Vector2f::new(-1.0, 0.0),
                color: Color::RED,
                ..Default::default()
            },
            Vertex {
                position: Vector2f::new(1.0, 0.0),
                color: Color::GREEN,
                ..Default::default()
            },
            Vertex {
                position: Vector2f::new(-1.0, 1.0),
                color: Color::BLUE,
                ..Default::default()
            },
        ];

        let vb =
            VertexBuffer::create_from_slice(&va_slice, PrimitiveType::Triangles, Usage::Static)
                .unwrap();

        assert_eq!(vb.get_vertex_count(), 3);

        assert_eq!(vb.get_primitive_type(), PrimitiveType::Triangles);

        assert_eq!(vb.get_usage(), Usage::Static);

        let new_vertices = [
            Vertex {
                position: Vector2f::new(0.0, 0.0),
                color: Color::YELLOW,
                ..Default::default()
            },
            Vertex {
                position: Vector2f::new(2.0, 0.0),
                color: Color::CYAN,
                ..Default::default()
            },
        ];
    }
}
