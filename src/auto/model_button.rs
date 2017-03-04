// This file was generated by gir (c3b4020) from gir-files (71d73f0)
// DO NOT EDIT

use Actionable;
use Bin;
use Button;
#[cfg(feature = "v3_16")]
use ButtonRole;
use Container;
use Widget;
use ffi;
#[cfg(feature = "v3_16")]
use glib::Value;
use glib::object::Downcast;
use glib::translate::*;
#[cfg(feature = "v3_16")]
use gobject_ffi;
#[cfg(feature = "v3_16")]
use std::mem::transmute;

glib_wrapper! {
    pub struct ModelButton(Object<ffi::GtkModelButton>): Button, Bin, Container, Widget, Actionable;

    match fn {
        get_type => || ffi::gtk_model_button_get_type(),
    }
}

impl ModelButton {
    #[cfg(feature = "v3_16")]
    pub fn new() -> ModelButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_model_button_new()).downcast_unchecked()
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn get_property_active(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "active".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    #[cfg(feature = "v3_16")]
    pub fn set_property_active(&self, active: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "active".to_glib_none().0, Value::from(&active).to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn get_property_centered(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "centered".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    #[cfg(feature = "v3_16")]
    pub fn set_property_centered(&self, centered: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "centered".to_glib_none().0, Value::from(&centered).to_glib_none().0);
        }
    }

    //#[cfg(feature = "v3_16")]
    //pub fn get_property_icon(&self) -> /*Ignored*/Option<gio::Icon> {
    //    let mut value = Value::from(None::<&/*Ignored*/gio::Icon>);
    //    unsafe {
    //        gobject_ffi::g_object_get_property(self.to_glib_none().0, "icon".to_glib_none().0, value.to_glib_none_mut().0);
    //    }
    //    value.get()
    //}

    //#[cfg(feature = "v3_16")]
    //pub fn set_property_icon(&self, icon: /*Ignored*/Option<&gio::Icon>) {
    //    unsafe {
    //        gobject_ffi::g_object_set_property(self.to_glib_none().0, "icon".to_glib_none().0, Value::from(icon).to_glib_none().0);
    //    }
    //}

    #[cfg(feature = "v3_16")]
    pub fn get_property_iconic(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "iconic".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    #[cfg(feature = "v3_16")]
    pub fn set_property_iconic(&self, iconic: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "iconic".to_glib_none().0, Value::from(&iconic).to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn get_property_inverted(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "inverted".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    #[cfg(feature = "v3_16")]
    pub fn set_property_inverted(&self, inverted: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "inverted".to_glib_none().0, Value::from(&inverted).to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn get_property_menu_name(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "menu-name".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    #[cfg(feature = "v3_16")]
    pub fn set_property_menu_name(&self, menu_name: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "menu-name".to_glib_none().0, Value::from(menu_name).to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn get_property_role(&self) -> ButtonRole {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "role".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<i32>().unwrap()))
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn set_property_role(&self, role: ButtonRole) {
        let role = role.to_glib() as i32;
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "role".to_glib_none().0, Value::from(&role).to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn get_property_text(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "text".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    #[cfg(feature = "v3_16")]
    pub fn set_property_text(&self, text: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "text".to_glib_none().0, Value::from(text).to_glib_none().0);
        }
    }
}
