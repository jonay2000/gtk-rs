// This file was generated by gir (dc20488) from gir-files (11e0e6d)
// DO NOT EDIT

use EventController;
use Gesture;
use GestureSingle;
#[cfg(feature = "3.14")]
use Rectangle;
#[cfg(feature = "3.14")]
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct GestureMultiPress(Object<ffi::GtkGestureMultiPress>): GestureSingle, Gesture, EventController;

    match fn {
        get_type => || ffi::gtk_gesture_multi_press_get_type(),
    }
}

impl GestureMultiPress {
    #[cfg(feature = "3.14")]
    pub fn new<T: IsA<Widget>>(widget: &T) -> GestureMultiPress {
        skip_assert_initialized!();
        unsafe {
            Gesture::from_glib_full(ffi::gtk_gesture_multi_press_new(widget.to_glib_none().0)).downcast_unchecked()
        }
    }

    #[cfg(feature = "3.14")]
    pub fn get_area(&self) -> Option<Rectangle> {
        unsafe {
            let mut rect = Rectangle::uninitialized();
            let ret = from_glib(ffi::gtk_gesture_multi_press_get_area(self.to_glib_none().0, rect.to_glib_none_mut().0));
            if ret { Some(rect) } else { None }
        }
    }

    #[cfg(feature = "3.14")]
    pub fn set_area(&self, rect: Option<&Rectangle>) {
        unsafe {
            ffi::gtk_gesture_multi_press_set_area(self.to_glib_none().0, rect.to_glib_none().0);
        }
    }
}
