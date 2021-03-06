// This file was generated by gir (b7f5189) from gir-files (71d73f0)
// DO NOT EDIT

use Container;
use StackTransitionType;
use Widget;
use ffi;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use gobject_ffi;
use std::mem::transmute;

glib_wrapper! {
    pub struct Stack(Object<ffi::GtkStack>): Container, Widget;

    match fn {
        get_type => || ffi::gtk_stack_get_type(),
    }
}

impl Stack {
    #[cfg(feature = "v3_10")]
    pub fn new() -> Stack {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_stack_new()).downcast_unchecked()
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn add_named<T: IsA<Widget>>(&self, child: &T, name: &str) {
        unsafe {
            ffi::gtk_stack_add_named(self.to_glib_none().0, child.to_glib_none().0, name.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn add_titled<T: IsA<Widget>>(&self, child: &T, name: &str, title: &str) {
        unsafe {
            ffi::gtk_stack_add_titled(self.to_glib_none().0, child.to_glib_none().0, name.to_glib_none().0, title.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_12")]
    pub fn get_child_by_name(&self, name: &str) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_stack_get_child_by_name(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn get_hhomogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_stack_get_hhomogeneous(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn get_homogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_stack_get_homogeneous(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_18")]
    pub fn get_interpolate_size(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_stack_get_interpolate_size(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn get_transition_duration(&self) -> u32 {
        unsafe {
            ffi::gtk_stack_get_transition_duration(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v3_12")]
    pub fn get_transition_running(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_stack_get_transition_running(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn get_transition_type(&self) -> StackTransitionType {
        unsafe {
            from_glib(ffi::gtk_stack_get_transition_type(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn get_vhomogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_stack_get_vhomogeneous(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn get_visible_child(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_stack_get_visible_child(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn get_visible_child_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_stack_get_visible_child_name(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn set_hhomogeneous(&self, hhomogeneous: bool) {
        unsafe {
            ffi::gtk_stack_set_hhomogeneous(self.to_glib_none().0, hhomogeneous.to_glib());
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn set_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::gtk_stack_set_homogeneous(self.to_glib_none().0, homogeneous.to_glib());
        }
    }

    #[cfg(feature = "v3_18")]
    pub fn set_interpolate_size(&self, interpolate_size: bool) {
        unsafe {
            ffi::gtk_stack_set_interpolate_size(self.to_glib_none().0, interpolate_size.to_glib());
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn set_transition_duration(&self, duration: u32) {
        unsafe {
            ffi::gtk_stack_set_transition_duration(self.to_glib_none().0, duration);
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn set_transition_type(&self, transition: StackTransitionType) {
        unsafe {
            ffi::gtk_stack_set_transition_type(self.to_glib_none().0, transition.to_glib());
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn set_vhomogeneous(&self, vhomogeneous: bool) {
        unsafe {
            ffi::gtk_stack_set_vhomogeneous(self.to_glib_none().0, vhomogeneous.to_glib());
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn set_visible_child<T: IsA<Widget>>(&self, child: &T) {
        unsafe {
            ffi::gtk_stack_set_visible_child(self.to_glib_none().0, child.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn set_visible_child_full(&self, name: &str, transition: StackTransitionType) {
        unsafe {
            ffi::gtk_stack_set_visible_child_full(self.to_glib_none().0, name.to_glib_none().0, transition.to_glib());
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn set_visible_child_name(&self, name: &str) {
        unsafe {
            ffi::gtk_stack_set_visible_child_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    pub fn get_property_homogeneous(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "homogeneous".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_property_homogeneous(&self, homogeneous: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "homogeneous".to_glib_none().0, Value::from(&homogeneous).to_glib_none().0);
        }
    }

    pub fn get_property_interpolate_size(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "interpolate-size".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_property_interpolate_size(&self, interpolate_size: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "interpolate-size".to_glib_none().0, Value::from(&interpolate_size).to_glib_none().0);
        }
    }

    pub fn get_property_transition_duration(&self) -> u32 {
        let mut value = Value::from(&0u32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "transition-duration".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_property_transition_duration(&self, transition_duration: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "transition-duration".to_glib_none().0, Value::from(&transition_duration).to_glib_none().0);
        }
    }

    pub fn get_property_transition_running(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "transition-running".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn get_property_transition_type(&self) -> StackTransitionType {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "transition-type".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<i32>().unwrap()))
        }
    }

    pub fn set_property_transition_type(&self, transition_type: StackTransitionType) {
        let transition_type = transition_type.to_glib() as i32;
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "transition-type".to_glib_none().0, Value::from(&transition_type).to_glib_none().0);
        }
    }

    pub fn get_property_visible_child(&self) -> Option<Widget> {
        let mut value = Value::from(None::<&Widget>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "visible-child".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    pub fn set_property_visible_child(&self, visible_child: Option<&Widget>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "visible-child".to_glib_none().0, Value::from(visible_child).to_glib_none().0);
        }
    }

    pub fn get_property_visible_child_name(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "visible-child-name".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    pub fn set_property_visible_child_name(&self, visible_child_name: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "visible-child-name".to_glib_none().0, Value::from(visible_child_name).to_glib_none().0);
        }
    }

    pub fn get_child_icon_name<T: IsA<Widget>>(&self, item: &T) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "icon-name".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    pub fn set_child_icon_name<T: IsA<Widget>>(&self, item: &T, icon_name: Option<&str>) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "icon-name".to_glib_none().0, Value::from(icon_name).to_glib_none().0);
        }
    }

    pub fn get_child_name<T: IsA<Widget>>(&self, item: &T) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "name".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    pub fn set_child_name<T: IsA<Widget>>(&self, item: &T, name: Option<&str>) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "name".to_glib_none().0, Value::from(name).to_glib_none().0);
        }
    }

    pub fn get_child_needs_attention<T: IsA<Widget>>(&self, item: &T) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "needs-attention".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_child_needs_attention<T: IsA<Widget>>(&self, item: &T, needs_attention: bool) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "needs-attention".to_glib_none().0, Value::from(&needs_attention).to_glib_none().0);
        }
    }

    pub fn get_child_position<T: IsA<Widget>>(&self, item: &T) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "position".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_child_position<T: IsA<Widget>>(&self, item: &T, position: i32) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "position".to_glib_none().0, Value::from(&position).to_glib_none().0);
        }
    }

    pub fn get_child_title<T: IsA<Widget>>(&self, item: &T) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "title".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    pub fn set_child_title<T: IsA<Widget>>(&self, item: &T, title: Option<&str>) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "title".to_glib_none().0, Value::from(title).to_glib_none().0);
        }
    }
}
