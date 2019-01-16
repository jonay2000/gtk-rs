// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Cancellable;
use Error;
use IOStream;
use ProxyAddress;
use ffi;
#[cfg(feature = "futures")]
use futures_core;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(feature = "futures")]
use std::boxed::Box as Box_;
use std::fmt;
use std::ptr;

glib_wrapper! {
    pub struct Proxy(Interface<ffi::GProxy>);

    match fn {
        get_type => || ffi::g_proxy_get_type(),
    }
}

impl Proxy {
    pub fn get_default_for_protocol(protocol: &str) -> Option<Proxy> {
        unsafe {
            from_glib_full(ffi::g_proxy_get_default_for_protocol(protocol.to_glib_none().0))
        }
    }
}

pub const NONE_PROXY: Option<&Proxy> = None;

pub trait ProxyExt: 'static {
    fn connect<'a, P: IsA<IOStream>, Q: IsA<ProxyAddress>, R: IsA<Cancellable> + 'a, S: Into<Option<&'a R>>>(&self, connection: &P, proxy_address: &Q, cancellable: S) -> Result<IOStream, Error>;

    fn connect_async<'a, P: IsA<IOStream>, Q: IsA<ProxyAddress>, R: IsA<Cancellable> + 'a, S: Into<Option<&'a R>>, T: FnOnce(Result<IOStream, Error>) + Send + 'static>(&self, connection: &P, proxy_address: &Q, cancellable: S, callback: T);

    #[cfg(feature = "futures")]
    fn connect_async_future<P: IsA<IOStream> + Clone + 'static, Q: IsA<ProxyAddress> + Clone + 'static>(&self, connection: &P, proxy_address: &Q) -> Box_<futures_core::Future<Item = (Self, IOStream), Error = (Self, Error)>> where Self: Sized + Clone;

    fn supports_hostname(&self) -> bool;
}

impl<O: IsA<Proxy>> ProxyExt for O {
    fn connect<'a, P: IsA<IOStream>, Q: IsA<ProxyAddress>, R: IsA<Cancellable> + 'a, S: Into<Option<&'a R>>>(&self, connection: &P, proxy_address: &Q, cancellable: S) -> Result<IOStream, Error> {
        let cancellable = cancellable.into();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_proxy_connect(self.as_ref().to_glib_none().0, connection.as_ref().to_glib_none().0, proxy_address.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn connect_async<'a, P: IsA<IOStream>, Q: IsA<ProxyAddress>, R: IsA<Cancellable> + 'a, S: Into<Option<&'a R>>, T: FnOnce(Result<IOStream, Error>) + Send + 'static>(&self, connection: &P, proxy_address: &Q, cancellable: S, callback: T) {
        let cancellable = cancellable.into();
        let user_data: Box<Box<T>> = Box::new(Box::new(callback));
        unsafe extern "C" fn connect_async_trampoline<T: FnOnce(Result<IOStream, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let ret = ffi::g_proxy_connect_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<T>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = connect_async_trampoline::<T>;
        unsafe {
            ffi::g_proxy_connect_async(self.as_ref().to_glib_none().0, connection.as_ref().to_glib_none().0, proxy_address.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn connect_async_future<P: IsA<IOStream> + Clone + 'static, Q: IsA<ProxyAddress> + Clone + 'static>(&self, connection: &P, proxy_address: &Q) -> Box_<futures_core::Future<Item = (Self, IOStream), Error = (Self, Error)>> where Self: Sized + Clone {
        use GioFuture;
        use fragile::Fragile;

        let connection = connection.clone();
        let proxy_address = proxy_address.clone();
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.connect_async(
                 &connection,
                 &proxy_address,
                 Some(&cancellable),
                 move |res| {
                     let obj = obj_clone.into_inner();
                     let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                     let _ = send.into_inner().send(res);
                 },
            );

            cancellable
        })
    }

    fn supports_hostname(&self) -> bool {
        unsafe {
            from_glib(ffi::g_proxy_supports_hostname(self.as_ref().to_glib_none().0))
        }
    }
}

impl fmt::Display for Proxy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Proxy")
    }
}
