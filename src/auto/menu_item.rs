// This file was generated by gir (b745e7b) from gir-files (11e0e6d)
// DO NOT EDIT

use Actionable;
use Bin;
use Buildable;
use Container;
use Menu;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::Upcast;
use glib::translate::*;

glib_wrapper! {
    pub struct MenuItem(Object<ffi::GtkMenuItem>): Widget, Container, Bin, Actionable, Buildable;

    match fn {
        get_type => || ffi::gtk_menu_item_get_type(),
    }
}

impl MenuItem {
    pub fn new() -> MenuItem {
        unsafe {
            Widget::from_glib_none(ffi::gtk_menu_item_new()).downcast_unchecked()
        }
    }

    pub fn new_with_label(label: &str) -> MenuItem {
        unsafe {
            Widget::from_glib_none(ffi::gtk_menu_item_new_with_label(label.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_mnemonic(label: &str) -> MenuItem {
        unsafe {
            Widget::from_glib_none(ffi::gtk_menu_item_new_with_mnemonic(label.to_glib_none().0)).downcast_unchecked()
        }
    }

}

pub trait MenuItemExt {
    fn activate(&self);
    fn deselect(&self);
    fn get_accel_path(&self) -> Option<String>;
    fn get_label(&self) -> Option<String>;
    fn get_reserve_indicator(&self) -> bool;
    fn get_right_justified(&self) -> bool;
    fn get_submenu(&self) -> Option<Widget>;
    fn get_use_underline(&self) -> bool;
    fn select(&self);
    fn set_accel_path(&self, accel_path: Option<&str>);
    fn set_label(&self, label: &str);
    fn set_reserve_indicator(&self, reserve: bool);
    fn set_right_justified(&self, right_justified: bool);
    fn set_submenu<T: Upcast<Menu>>(&self, submenu: Option<&T>);
    fn set_use_underline(&self, setting: bool);
    fn toggle_size_allocate(&self, allocation: i32);
    fn toggle_size_request(&self, requisition: &mut i32);
}

impl<O: Upcast<MenuItem>> MenuItemExt for O {
    fn activate(&self) {
        unsafe {
            ffi::gtk_menu_item_activate(self.to_glib_none().0);
        }
    }

    fn deselect(&self) {
        unsafe {
            ffi::gtk_menu_item_deselect(self.to_glib_none().0);
        }
    }

    fn get_accel_path(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_menu_item_get_accel_path(self.to_glib_none().0))
        }
    }

    fn get_label(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_menu_item_get_label(self.to_glib_none().0))
        }
    }

    fn get_reserve_indicator(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_menu_item_get_reserve_indicator(self.to_glib_none().0))
        }
    }

    fn get_right_justified(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_menu_item_get_right_justified(self.to_glib_none().0))
        }
    }

    fn get_submenu(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_menu_item_get_submenu(self.to_glib_none().0))
        }
    }

    fn get_use_underline(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_menu_item_get_use_underline(self.to_glib_none().0))
        }
    }

    fn select(&self) {
        unsafe {
            ffi::gtk_menu_item_select(self.to_glib_none().0);
        }
    }

    fn set_accel_path(&self, accel_path: Option<&str>) {
        unsafe {
            ffi::gtk_menu_item_set_accel_path(self.to_glib_none().0, accel_path.to_glib_none().0);
        }
    }

    fn set_label(&self, label: &str) {
        unsafe {
            ffi::gtk_menu_item_set_label(self.to_glib_none().0, label.to_glib_none().0);
        }
    }

    fn set_reserve_indicator(&self, reserve: bool) {
        unsafe {
            ffi::gtk_menu_item_set_reserve_indicator(self.to_glib_none().0, reserve.to_glib());
        }
    }

    fn set_right_justified(&self, right_justified: bool) {
        unsafe {
            ffi::gtk_menu_item_set_right_justified(self.to_glib_none().0, right_justified.to_glib());
        }
    }

    fn set_submenu<T: Upcast<Menu>>(&self, submenu: Option<&T>) {
        unsafe {
            ffi::gtk_menu_item_set_submenu(self.to_glib_none().0, submenu.to_glib_none().0);
        }
    }

    fn set_use_underline(&self, setting: bool) {
        unsafe {
            ffi::gtk_menu_item_set_use_underline(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn toggle_size_allocate(&self, allocation: i32) {
        unsafe {
            ffi::gtk_menu_item_toggle_size_allocate(self.to_glib_none().0, allocation);
        }
    }

    fn toggle_size_request(&self, requisition: &mut i32) {
        unsafe {
            ffi::gtk_menu_item_toggle_size_request(self.to_glib_none().0, requisition);
        }
    }

}
