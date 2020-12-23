// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::IconTheme;
use crate::StyleContext;
use glib::object::IsA;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    pub struct IconInfo(Object<ffi::GtkIconInfo, ffi::GtkIconInfoClass>);

    match fn {
        get_type => || ffi::gtk_icon_info_get_type(),
    }
}

impl IconInfo {
    #[doc(alias = "gtk_icon_info_new_for_pixbuf")]
    pub fn new_for_pixbuf<P: IsA<IconTheme>>(
        icon_theme: &P,
        pixbuf: &gdk_pixbuf::Pixbuf,
    ) -> IconInfo {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gtk_icon_info_new_for_pixbuf(
                icon_theme.as_ref().to_glib_none().0,
                pixbuf.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_icon_info_get_base_scale")]
    pub fn get_base_scale(&self) -> i32 {
        unsafe { ffi::gtk_icon_info_get_base_scale(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_icon_info_get_base_size")]
    pub fn get_base_size(&self) -> i32 {
        unsafe { ffi::gtk_icon_info_get_base_size(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_icon_info_get_filename")]
    pub fn get_filename(&self) -> Option<std::path::PathBuf> {
        unsafe { from_glib_none(ffi::gtk_icon_info_get_filename(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_icon_info_is_symbolic")]
    pub fn is_symbolic(&self) -> bool {
        unsafe { from_glib(ffi::gtk_icon_info_is_symbolic(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_icon_info_load_icon")]
    pub fn load_icon(&self) -> Result<gdk_pixbuf::Pixbuf, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_icon_info_load_icon(self.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gtk_icon_info_load_icon_async")]
    pub fn load_icon_async<
        P: IsA<gio::Cancellable>,
        Q: FnOnce(Result<gdk_pixbuf::Pixbuf, glib::Error>) + Send + 'static,
    >(
        &self,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn load_icon_async_trampoline<
            Q: FnOnce(Result<gdk_pixbuf::Pixbuf, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret =
                ffi::gtk_icon_info_load_icon_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = load_icon_async_trampoline::<Q>;
        unsafe {
            ffi::gtk_icon_info_load_icon_async(
                self.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn load_icon_async_future(
        &self,
    ) -> Pin<
        Box_<dyn std::future::Future<Output = Result<gdk_pixbuf::Pixbuf, glib::Error>> + 'static>,
    > {
        Box_::pin(gio::GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            obj.load_icon_async(Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    #[doc(alias = "gtk_icon_info_load_surface")]
    pub fn load_surface(
        &self,
        for_window: Option<&gdk::Window>,
    ) -> Result<cairo::Surface, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_icon_info_load_surface(
                self.to_glib_none().0,
                for_window.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gtk_icon_info_load_symbolic")]
    pub fn load_symbolic(
        &self,
        fg: &gdk::RGBA,
        success_color: Option<&gdk::RGBA>,
        warning_color: Option<&gdk::RGBA>,
        error_color: Option<&gdk::RGBA>,
    ) -> Result<(gdk_pixbuf::Pixbuf, bool), glib::Error> {
        unsafe {
            let mut was_symbolic = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_icon_info_load_symbolic(
                self.to_glib_none().0,
                fg.to_glib_none().0,
                success_color.to_glib_none().0,
                warning_color.to_glib_none().0,
                error_color.to_glib_none().0,
                was_symbolic.as_mut_ptr(),
                &mut error,
            );
            let was_symbolic = was_symbolic.assume_init();
            if error.is_null() {
                Ok((from_glib_full(ret), from_glib(was_symbolic)))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gtk_icon_info_load_symbolic_async")]
    pub fn load_symbolic_async<
        P: IsA<gio::Cancellable>,
        Q: FnOnce(Result<(gdk_pixbuf::Pixbuf, bool), glib::Error>) + Send + 'static,
    >(
        &self,
        fg: &gdk::RGBA,
        success_color: Option<&gdk::RGBA>,
        warning_color: Option<&gdk::RGBA>,
        error_color: Option<&gdk::RGBA>,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn load_symbolic_async_trampoline<
            Q: FnOnce(Result<(gdk_pixbuf::Pixbuf, bool), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let mut was_symbolic = mem::MaybeUninit::uninit();
            let ret = ffi::gtk_icon_info_load_symbolic_finish(
                _source_object as *mut _,
                res,
                was_symbolic.as_mut_ptr(),
                &mut error,
            );
            let was_symbolic = was_symbolic.assume_init();
            let result = if error.is_null() {
                Ok((from_glib_full(ret), from_glib(was_symbolic)))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = load_symbolic_async_trampoline::<Q>;
        unsafe {
            ffi::gtk_icon_info_load_symbolic_async(
                self.to_glib_none().0,
                fg.to_glib_none().0,
                success_color.to_glib_none().0,
                warning_color.to_glib_none().0,
                error_color.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn load_symbolic_async_future(
        &self,
        fg: &gdk::RGBA,
        success_color: Option<&gdk::RGBA>,
        warning_color: Option<&gdk::RGBA>,
        error_color: Option<&gdk::RGBA>,
    ) -> Pin<
        Box_<
            dyn std::future::Future<Output = Result<(gdk_pixbuf::Pixbuf, bool), glib::Error>>
                + 'static,
        >,
    > {
        let fg = fg.clone();
        let success_color = success_color.map(ToOwned::to_owned);
        let warning_color = warning_color.map(ToOwned::to_owned);
        let error_color = error_color.map(ToOwned::to_owned);
        Box_::pin(gio::GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            obj.load_symbolic_async(
                &fg,
                success_color.as_ref().map(::std::borrow::Borrow::borrow),
                warning_color.as_ref().map(::std::borrow::Borrow::borrow),
                error_color.as_ref().map(::std::borrow::Borrow::borrow),
                Some(&cancellable),
                move |res| {
                    send.resolve(res);
                },
            );

            cancellable
        }))
    }

    #[doc(alias = "gtk_icon_info_load_symbolic_for_context")]
    pub fn load_symbolic_for_context<P: IsA<StyleContext>>(
        &self,
        context: &P,
    ) -> Result<(gdk_pixbuf::Pixbuf, bool), glib::Error> {
        unsafe {
            let mut was_symbolic = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_icon_info_load_symbolic_for_context(
                self.to_glib_none().0,
                context.as_ref().to_glib_none().0,
                was_symbolic.as_mut_ptr(),
                &mut error,
            );
            let was_symbolic = was_symbolic.assume_init();
            if error.is_null() {
                Ok((from_glib_full(ret), from_glib(was_symbolic)))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gtk_icon_info_load_symbolic_for_context_async")]
    pub fn load_symbolic_for_context_async<
        P: IsA<StyleContext>,
        Q: IsA<gio::Cancellable>,
        R: FnOnce(Result<(gdk_pixbuf::Pixbuf, bool), glib::Error>) + Send + 'static,
    >(
        &self,
        context: &P,
        cancellable: Option<&Q>,
        callback: R,
    ) {
        let user_data: Box_<R> = Box_::new(callback);
        unsafe extern "C" fn load_symbolic_for_context_async_trampoline<
            R: FnOnce(Result<(gdk_pixbuf::Pixbuf, bool), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let mut was_symbolic = mem::MaybeUninit::uninit();
            let ret = ffi::gtk_icon_info_load_symbolic_for_context_finish(
                _source_object as *mut _,
                res,
                was_symbolic.as_mut_ptr(),
                &mut error,
            );
            let was_symbolic = was_symbolic.assume_init();
            let result = if error.is_null() {
                Ok((from_glib_full(ret), from_glib(was_symbolic)))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<R> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = load_symbolic_for_context_async_trampoline::<R>;
        unsafe {
            ffi::gtk_icon_info_load_symbolic_for_context_async(
                self.to_glib_none().0,
                context.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn load_symbolic_for_context_async_future<P: IsA<StyleContext> + Clone + 'static>(
        &self,
        context: &P,
    ) -> Pin<
        Box_<
            dyn std::future::Future<Output = Result<(gdk_pixbuf::Pixbuf, bool), glib::Error>>
                + 'static,
        >,
    > {
        let context = context.clone();
        Box_::pin(gio::GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            obj.load_symbolic_for_context_async(&context, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }
}

impl fmt::Display for IconInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("IconInfo")
    }
}