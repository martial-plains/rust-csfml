use std::ops::Add as _;

use csfml_sys::{
    sfFloatRect, sfVector2f, sfView, sfView_create, sfView_getCenter, sfView_getSize,
    sfView_getViewport, sfView_setCenter, sfView_setSize, sfView_setViewport,
};

use crate::system::Vector2f;

use super::rect::FloatRect;

#[derive(Debug, Clone, Copy)]
pub struct View {
    center: Vector2f,
    size: Vector2f,
    viewport: FloatRect,
}

impl View {
    /// Creates a view from a rectangle
    #[must_use]
    pub fn from_rect(rect: FloatRect) -> Self {
        let mut ret = Self {
            center: rect.get_corner(),
            size: rect.get_size(),
            viewport: FloatRect::new(0.0, 0.0, 1.0, 1.0),
        };
        ret.center = ret.center.add(ret.size.scale(0.5));
        ret
    }

    /// Creates a view from a CSFML object
    /// This is mainly for the inner workings of this wrapper
    #[must_use]
    pub unsafe fn from_csfml(view: *const sfView) -> Self {
        let center = unsafe { Vector2f::from(sfView_getCenter(view)) };
        let size = unsafe { Vector2f::from(sfView_getSize(view)) };
        let viewport = unsafe { FloatRect::from(sfView_getViewport(view)) };
        Self {
            center,
            size,
            viewport,
        }
    }

    /// Converts this view into a CSFML view
    /// This view must be destroyed manually!
    #[must_use]
    pub fn to_csfml(&self) -> *mut sfView {
        let view = unsafe { sfView_create() };
        assert!(!view.is_null(), "Failed to create CSFML view");
        unsafe {
            sfView_setCenter(view, sfVector2f::from(self.center));
            sfView_setSize(view, sfVector2f::from(self.size));
            sfView_setViewport(view, sfFloatRect::from(self.viewport));
        }
        view
    }

    /// Returns the rectangle representing the view
    #[must_use]
    pub fn get_rect(&self) -> FloatRect {
        FloatRect::new(
            self.center.x - self.size.x / 2.0,
            self.center.y - self.size.y / 2.0,
            self.size.x,
            self.size.y,
        )
    }

    /// Sets the size of the view
    pub fn set_size(&mut self, size: Vector2f) {
        self.size = size;
    }

    /// Sets the center of the view
    pub fn set_center(&mut self, center: Vector2f) {
        self.center = center;
    }

    /// Zooms the view by a given factor
    pub fn zoom(&mut self, factor: f32) {
        self.size = Vector2f {
            x: self.size.x * factor,
            y: self.size.y * factor,
        };
    }
}

#[cfg(test)]
mod tests {
    use csfml_sys::{sfView_createFromRect, sfView_destroy};

    use crate::assert_approx_eq;

    use super::*;

    #[test]
    fn test_view_from_rect() {
        let rect = FloatRect::new(10.0, -15.0, 700.0, 600.0);

        let csfml_view = unsafe { sfView_createFromRect(rect.to_csfml()) };
        assert!(!csfml_view.is_null());

        let mut view2 = View::from_rect(rect);
        let center = unsafe { Vector2f::from(sfView_getCenter(csfml_view)) };
        let size = unsafe { Vector2f::from(sfView_getSize(csfml_view)) };

        assert_approx_eq!(center.x, view2.center.x, 0.00001);
        assert_approx_eq!(center.y, view2.center.y, 0.00001);
        assert_approx_eq!(size.x, view2.size.x, 0.00001);
        assert_approx_eq!(size.y, view2.size.y, 0.00001);

        let rect_ret = view2.get_rect();

        assert_approx_eq!(rect.left, rect_ret.left, 0.00001);
        assert_approx_eq!(rect.top, rect_ret.top, 0.00001);
        assert_approx_eq!(rect.width, rect_ret.width, 0.00001);
        assert_approx_eq!(rect.height, rect_ret.height, 0.00001);

        view2.set_center(Vector2f { x: 400.0, y: 300.0 });
        view2.set_size(Vector2f { x: 800.0, y: 600.0 });
        let rect_ret = view2.get_rect();

        assert_approx_eq!(0.0, rect_ret.left, 0.00001);
        assert_approx_eq!(0.0, rect_ret.top, 0.00001);
        assert_approx_eq!(800.0, rect_ret.width, 0.00001);
        assert_approx_eq!(600.0, rect_ret.height, 0.00001);

        unsafe { sfView_destroy(csfml_view) };
    }
}
