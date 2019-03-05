// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use TlsPasswordFlags;
use gio_sys;
use glib::GString;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct TlsPassword(Object<gio_sys::GTlsPassword, gio_sys::GTlsPasswordClass, TlsPasswordClass>);

    match fn {
        get_type => || gio_sys::g_tls_password_get_type(),
    }
}

impl TlsPassword {
    pub fn new(flags: TlsPasswordFlags, description: &str) -> TlsPassword {
        unsafe {
            from_glib_full(gio_sys::g_tls_password_new(flags.to_glib(), description.to_glib_none().0))
        }
    }
}

pub const NONE_TLS_PASSWORD: Option<&TlsPassword> = None;

pub trait TlsPasswordExt: 'static {
    fn get_description(&self) -> Option<GString>;

    fn get_flags(&self) -> TlsPasswordFlags;

    fn get_warning(&self) -> Option<GString>;

    fn set_description(&self, description: &str);

    fn set_flags(&self, flags: TlsPasswordFlags);

    //fn set_value_full(&self, value: &[u8]);

    fn set_warning(&self, warning: &str);

    fn connect_property_description_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_warning_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TlsPassword>> TlsPasswordExt for O {
    fn get_description(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gio_sys::g_tls_password_get_description(self.as_ref().to_glib_none().0))
        }
    }

    fn get_flags(&self) -> TlsPasswordFlags {
        unsafe {
            from_glib(gio_sys::g_tls_password_get_flags(self.as_ref().to_glib_none().0))
        }
    }

    fn get_warning(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gio_sys::g_tls_password_get_warning(self.as_ref().to_glib_none().0))
        }
    }

    fn set_description(&self, description: &str) {
        unsafe {
            gio_sys::g_tls_password_set_description(self.as_ref().to_glib_none().0, description.to_glib_none().0);
        }
    }

    fn set_flags(&self, flags: TlsPasswordFlags) {
        unsafe {
            gio_sys::g_tls_password_set_flags(self.as_ref().to_glib_none().0, flags.to_glib());
        }
    }

    //fn set_value_full(&self, value: &[u8]) {
    //    unsafe { TODO: call gio_sys:g_tls_password_set_value_full() }
    //}

    fn set_warning(&self, warning: &str) {
        unsafe {
            gio_sys::g_tls_password_set_warning(self.as_ref().to_glib_none().0, warning.to_glib_none().0);
        }
    }

    fn connect_property_description_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::description\0".as_ptr() as *const _,
                Some(transmute(notify_description_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::flags\0".as_ptr() as *const _,
                Some(transmute(notify_flags_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_warning_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::warning\0".as_ptr() as *const _,
                Some(transmute(notify_warning_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_description_trampoline<P, F: Fn(&P) + 'static>(this: *mut gio_sys::GTlsPassword, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<TlsPassword> {
    let f: &F = &*(f as *const F);
    f(&TlsPassword::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_flags_trampoline<P, F: Fn(&P) + 'static>(this: *mut gio_sys::GTlsPassword, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<TlsPassword> {
    let f: &F = &*(f as *const F);
    f(&TlsPassword::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_warning_trampoline<P, F: Fn(&P) + 'static>(this: *mut gio_sys::GTlsPassword, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<TlsPassword> {
    let f: &F = &*(f as *const F);
    f(&TlsPassword::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for TlsPassword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TlsPassword")
    }
}
