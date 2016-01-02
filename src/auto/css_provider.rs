// This file was generated by gir (b745e7b) from gir-files (11e0e6d)
// DO NOT EDIT

use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct CssProvider(Object<ffi::GtkCssProvider>);

    match fn {
        get_type => || ffi::gtk_css_provider_get_type(),
    }
}

impl CssProvider {
    pub fn new() -> CssProvider {
        unsafe {
            from_glib_full(ffi::gtk_css_provider_new())
        }
    }

    //pub fn load_from_data(&self, data: /*Unknown conversion*/Unknown rust type: "CArray TypeId { ns_id: 0, id: 3 }", length: Fundamental: SSize, error: /*Ignored*/Option<glib::Error>) -> bool {
    //    unsafe { TODO: call ffi::gtk_css_provider_load_from_data() }
    //}

    //pub fn load_from_file<T: Upcast</*Ignored*/gio::File>>(&self, file: &T, error: /*Ignored*/Option<glib::Error>) -> bool {
    //    unsafe { TODO: call ffi::gtk_css_provider_load_from_file() }
    //}

    //pub fn load_from_path(&self, path: &str, error: /*Ignored*/Option<glib::Error>) -> bool {
    //    unsafe { TODO: call ffi::gtk_css_provider_load_from_path() }
    //}

    #[cfg(gtk_3_16)]
    pub fn load_from_resource(&self, resource_path: &str) {
        unsafe {
            ffi::gtk_css_provider_load_from_resource(self.to_glib_none().0, resource_path.to_glib_none().0);
        }
    }

    pub fn to_string(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_css_provider_to_string(self.to_glib_none().0))
        }
    }

    pub fn get_default() -> Option<CssProvider> {
        unsafe {
            from_glib_none(ffi::gtk_css_provider_get_default())
        }
    }

    pub fn get_named(name: &str, variant: Option<&str>) -> Option<CssProvider> {
        unsafe {
            from_glib_none(ffi::gtk_css_provider_get_named(name.to_glib_none().0, variant.to_glib_none().0))
        }
    }

}
