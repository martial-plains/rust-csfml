use std::{
    ffi::{CStr, CString},
    ptr::{self, null_mut},
    str::FromStr,
};

use csfml_sys::{
    sfBool, sfContextSettings, sfRenderWindow, sfRenderWindow_clear, sfRenderWindow_close,
    sfRenderWindow_create, sfRenderWindow_createFromHandle, sfRenderWindow_destroy,
    sfRenderWindow_display, sfRenderWindow_drawCircleShape, sfRenderWindow_drawConvexShape,
    sfRenderWindow_drawRectangleShape, sfRenderWindow_drawSprite, sfRenderWindow_drawText,
    sfRenderWindow_drawVertexArray, sfRenderWindow_drawVertexBuffer, sfRenderWindow_getPosition,
    sfRenderWindow_getSize, sfRenderWindow_getView, sfRenderWindow_isOpen,
    sfRenderWindow_mapCoordsToPixel, sfRenderWindow_mapPixelToCoords, sfRenderWindow_pollEvent,
    sfRenderWindow_setFramerateLimit, sfRenderWindow_setPosition, sfRenderWindow_setSize,
    sfRenderWindow_setTitle, sfRenderWindow_setVerticalSyncEnabled, sfRenderWindow_setView,
    sfRenderWindow_waitEvent, sfVector2f, sfVector2i, sfVector2u, sfWindowHandle,
};

use crate::{
    graphics::{RenderStates, View},
    system::{Vector2f, Vector2i, Vector2u},
    types::Result,
    utils::HasCsfmlPointer,
    window::{ContextSettings, Event, VideoMode},
};

use super::{
    color::Color,
    vertex::{VertexArray, VertexBuffer},
    CircleShape, ConvexShape, RectangleShape, Sprite, Text,
};

pub trait RenderWindowDrawable {
    fn draw_to_render_window(
        &self,
        render_texture: &mut RenderWindow,
        states: Option<&RenderStates>,
    );
}

#[repr(C)]
pub struct RenderWindow {
    ptr: *mut sfRenderWindow,
}

impl Drop for RenderWindow {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl RenderWindow {
    pub fn create(
        mode: VideoMode,
        title: &str,
        style: u32,
        settings: Option<ContextSettings>,
    ) -> Result<Self> {
        unsafe {
            let c_settings = settings.map_or(ptr::null(), |s| {
                let settings = sfContextSettings::from(s);
                &raw const settings
            });
            let title_cstr = CString::from_str(title).unwrap();
            let window_ptr =
                sfRenderWindow_create(mode.to_csfml(), title_cstr.as_ptr(), style, c_settings);

            if window_ptr.is_null() {
                Err("Failed to create RenderWindow".into())
            } else {
                Ok(Self { ptr: window_ptr })
            }
        }
    }

    pub unsafe fn create_from_handle(
        handle: sfWindowHandle,
        settings: Option<ContextSettings>,
    ) -> Result<Self> {
        unsafe {
            let c_settings = settings.map_or(ptr::null(), |s| {
                let settings = sfContextSettings::from(s);
                &raw const settings
            });
            let window_ptr = sfRenderWindow_createFromHandle(handle, c_settings);

            if window_ptr.is_null() {
                Err("Failed to create RenderWindow from handle".into())
            } else {
                Ok(Self { ptr: window_ptr })
            }
        }
    }

    pub fn destroy(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                sfRenderWindow_destroy(self.ptr);
                self.ptr = null_mut();
            }
        }
    }

    #[must_use]
    pub fn is_open(&self) -> bool {
        unsafe { sfRenderWindow_isOpen(self.ptr) != 0 }
    }

    pub fn close(&mut self) {
        unsafe { sfRenderWindow_close(self.ptr) }
    }

    pub fn poll_event(&mut self) -> Option<Event> {
        let mut event = unsafe { std::mem::zeroed() };
        if unsafe { sfRenderWindow_pollEvent(self.ptr, &mut event) } != 0 {
            Event::from_csfml(event).ok()
        } else {
            None
        }
    }

    pub fn wait_event(&mut self) -> Option<Event> {
        let mut event = unsafe { std::mem::zeroed() };
        if unsafe { sfRenderWindow_waitEvent(self.ptr, &mut event) } != 0 {
            Event::from_csfml(event).ok()
        } else {
            None
        }
    }

    pub fn clear(&mut self, color: Color) {
        unsafe {
            sfRenderWindow_clear(self.ptr, color.to_csfml());
        }
    }

    pub fn display(&mut self) {
        unsafe {
            sfRenderWindow_display(self.ptr);
        }
    }

    pub fn draw<T: RenderWindowDrawable>(&mut self, drawable: &T, states: Option<&RenderStates>) {
        drawable.draw_to_render_window(self, states);
    }

    #[must_use]
    pub fn get_view(&self) -> View {
        unsafe {
            let view_ptr = sfRenderWindow_getView(self.ptr);
            View::from_csfml(view_ptr)
        }
    }

    pub fn set_view(&mut self, view: &View) {
        unsafe {
            let cview = view.to_csfml();
            sfRenderWindow_setView(self.ptr, cview);
        }
    }

    #[must_use]
    pub fn get_size(&self) -> Vector2u {
        unsafe { Vector2u::from(sfRenderWindow_getSize(self.ptr)) }
    }

    pub fn set_size(&mut self, size: Vector2u) {
        unsafe {
            sfRenderWindow_setSize(self.ptr, sfVector2u::from(size));
        }
    }

    #[must_use]
    pub fn get_position(&self) -> Vector2i {
        unsafe { Vector2i::from(sfRenderWindow_getPosition(self.ptr)) }
    }

    pub fn set_position(&mut self, position: Vector2i) {
        unsafe {
            sfRenderWindow_setPosition(self.ptr, sfVector2i::from(position));
        }
    }

    pub fn set_title(&mut self, title: &str) {
        let title_cstr = CStr::from_bytes_with_nul(title.as_bytes()).unwrap();
        unsafe {
            sfRenderWindow_setTitle(self.ptr, title_cstr.as_ptr());
        }
    }

    pub fn set_framerate_limit(&mut self, fps: u32) {
        unsafe {
            sfRenderWindow_setFramerateLimit(self.ptr, fps);
        }
    }

    pub fn set_vertical_sync_enabled(&mut self, enabled: bool) {
        unsafe {
            sfRenderWindow_setVerticalSyncEnabled(self.ptr, sfBool::from(enabled));
        }
    }

    pub fn map_pixel_to_coords(&self, pixel: Vector2i, view: Option<&View>) -> Vector2f {
        unsafe {
            let view_ptr = view.map_or(null_mut(), super::view::View::to_csfml);
            Vector2f::from(sfRenderWindow_mapPixelToCoords(
                self.ptr,
                sfVector2i::from(pixel),
                view_ptr,
            ))
        }
    }

    pub fn map_coords_to_pixel(&self, coords: Vector2f, view: Option<&View>) -> Vector2i {
        unsafe {
            let view_ptr = view.map_or(null_mut(), super::view::View::to_csfml);
            Vector2i::from(sfRenderWindow_mapCoordsToPixel(
                self.ptr,
                sfVector2f::from(coords),
                view_ptr,
            ))
        }
    }
}

impl RenderWindowDrawable for Sprite {
    fn draw_to_render_window(
        &self,
        render_texture: &mut RenderWindow,
        states: Option<&RenderStates>,
    ) {
        let states = states.map_or_else(ptr::null, |state| {
            let cstate = state.to_csfml();
            &raw const cstate
        });

        unsafe {
            sfRenderWindow_drawSprite(render_texture.ptr, self.ptr, states);
        }
    }
}

impl RenderWindowDrawable for Text {
    fn draw_to_render_window(
        &self,
        render_window: &mut RenderWindow,
        states: Option<&RenderStates>,
    ) {
        let states = states.map_or_else(ptr::null, |state| {
            let cstate = state.to_csfml();
            &raw const cstate
        });

        unsafe {
            sfRenderWindow_drawText(render_window.ptr, self.ptr, states);
        }
    }
}

impl RenderWindowDrawable for CircleShape {
    fn draw_to_render_window(
        &self,
        render_window: &mut RenderWindow,
        states: Option<&RenderStates>,
    ) {
        let states = states.map_or_else(ptr::null, |state| {
            let cstate = state.to_csfml();
            &raw const cstate
        });

        unsafe {
            sfRenderWindow_drawCircleShape(render_window.ptr, self.mut_ptr(), states);
        }
    }
}

impl RenderWindowDrawable for ConvexShape {
    fn draw_to_render_window(
        &self,
        render_window: &mut RenderWindow,
        states: Option<&RenderStates>,
    ) {
        let states = states.map_or_else(ptr::null, |state| {
            let cstate = state.to_csfml();
            &raw const cstate
        });

        unsafe {
            sfRenderWindow_drawConvexShape(render_window.ptr, self.mut_ptr(), states);
        }
    }
}

impl RenderWindowDrawable for RectangleShape {
    fn draw_to_render_window(
        &self,
        render_window: &mut RenderWindow,
        states: Option<&RenderStates>,
    ) {
        let states = states.map_or_else(ptr::null, |state| {
            let cstate = state.to_csfml();
            &raw const cstate
        });

        unsafe {
            sfRenderWindow_drawRectangleShape(render_window.ptr, self.mut_ptr(), states);
        }
    }
}

impl RenderWindowDrawable for VertexArray {
    fn draw_to_render_window(
        &self,
        render_window: &mut RenderWindow,
        states: Option<&RenderStates>,
    ) {
        let states = states.map_or_else(ptr::null, |state| {
            let cstate = state.to_csfml();
            &raw const cstate
        });

        unsafe {
            sfRenderWindow_drawVertexArray(render_window.ptr, self.ptr, states);
        }
    }
}

impl RenderWindowDrawable for VertexBuffer {
    fn draw_to_render_window(
        &self,
        render_window: &mut RenderWindow,
        states: Option<&RenderStates>,
    ) {
        let states = states.map_or_else(ptr::null, |state| {
            let cstate = state.to_csfml();
            &raw const cstate
        });

        unsafe {
            sfRenderWindow_drawVertexBuffer(render_window.ptr, self.ptr, states);
        }
    }
}
