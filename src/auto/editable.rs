// This file was generated by gir (8343e00) from gir-files (71d73f0)
// DO NOT EDIT

use ffi;
use glib::object::IsA;
use glib::translate::*;
use std::mem;

glib_wrapper! {
    pub struct Editable(Object<ffi::GtkEditable>);

    match fn {
        get_type => || ffi::gtk_editable_get_type(),
    }
}

pub trait EditableExt {
    fn copy_clipboard(&self);

    fn cut_clipboard(&self);

    fn delete_selection(&self);

    fn delete_text(&self, start_pos: i32, end_pos: i32);

    fn get_chars(&self, start_pos: i32, end_pos: i32) -> Option<String>;

    fn get_editable(&self) -> bool;

    fn get_position(&self) -> i32;

    fn get_selection_bounds(&self) -> Option<(i32, i32)>;

    fn insert_text(&self, new_text: &str, new_text_length: i32, position: &mut i32);

    fn paste_clipboard(&self);

    fn select_region(&self, start_pos: i32, end_pos: i32);

    fn set_editable(&self, is_editable: bool);

    fn set_position(&self, position: i32);
}

impl<O: IsA<Editable>> EditableExt for O {
    fn copy_clipboard(&self) {
        unsafe {
            ffi::gtk_editable_copy_clipboard(self.to_glib_none().0);
        }
    }

    fn cut_clipboard(&self) {
        unsafe {
            ffi::gtk_editable_cut_clipboard(self.to_glib_none().0);
        }
    }

    fn delete_selection(&self) {
        unsafe {
            ffi::gtk_editable_delete_selection(self.to_glib_none().0);
        }
    }

    fn delete_text(&self, start_pos: i32, end_pos: i32) {
        unsafe {
            ffi::gtk_editable_delete_text(self.to_glib_none().0, start_pos, end_pos);
        }
    }

    fn get_chars(&self, start_pos: i32, end_pos: i32) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_editable_get_chars(self.to_glib_none().0, start_pos, end_pos))
        }
    }

    fn get_editable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_editable_get_editable(self.to_glib_none().0))
        }
    }

    fn get_position(&self) -> i32 {
        unsafe {
            ffi::gtk_editable_get_position(self.to_glib_none().0)
        }
    }

    fn get_selection_bounds(&self) -> Option<(i32, i32)> {
        unsafe {
            let mut start_pos = mem::uninitialized();
            let mut end_pos = mem::uninitialized();
            let ret = from_glib(ffi::gtk_editable_get_selection_bounds(self.to_glib_none().0, &mut start_pos, &mut end_pos));
            if ret { Some((start_pos, end_pos)) } else { None }
        }
    }

    fn insert_text(&self, new_text: &str, new_text_length: i32, position: &mut i32) {
        unsafe {
            ffi::gtk_editable_insert_text(self.to_glib_none().0, new_text.to_glib_none().0, new_text_length, position);
        }
    }

    fn paste_clipboard(&self) {
        unsafe {
            ffi::gtk_editable_paste_clipboard(self.to_glib_none().0);
        }
    }

    fn select_region(&self, start_pos: i32, end_pos: i32) {
        unsafe {
            ffi::gtk_editable_select_region(self.to_glib_none().0, start_pos, end_pos);
        }
    }

    fn set_editable(&self, is_editable: bool) {
        unsafe {
            ffi::gtk_editable_set_editable(self.to_glib_none().0, is_editable.to_glib());
        }
    }

    fn set_position(&self, position: i32) {
        unsafe {
            ffi::gtk_editable_set_position(self.to_glib_none().0, position);
        }
    }
}
