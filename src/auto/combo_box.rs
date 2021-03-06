// This file was generated by gir (b7f5189) from gir-files (71d73f0)
// DO NOT EDIT

use Bin;
use CellArea;
use CellEditable;
use CellLayout;
use Container;
use Object;
use ScrollType;
use SensitivityType;
use TreeIter;
use TreeModel;
use Widget;
use ffi;
use gdk;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct ComboBox(Object<ffi::GtkComboBox>): Bin, Container, Widget, CellEditable, CellLayout;

    match fn {
        get_type => || ffi::gtk_combo_box_get_type(),
    }
}

impl ComboBox {
    pub fn new() -> ComboBox {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_combo_box_new()).downcast_unchecked()
        }
    }

    pub fn new_with_area<T: IsA<CellArea>>(area: &T) -> ComboBox {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_combo_box_new_with_area(area.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_area_and_entry<T: IsA<CellArea>>(area: &T) -> ComboBox {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_combo_box_new_with_area_and_entry(area.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_entry() -> ComboBox {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_combo_box_new_with_entry()).downcast_unchecked()
        }
    }

    pub fn new_with_model<T: IsA<TreeModel>>(model: &T) -> ComboBox {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_combo_box_new_with_model(model.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_model_and_entry<T: IsA<TreeModel>>(model: &T) -> ComboBox {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_combo_box_new_with_model_and_entry(model.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait ComboBoxExt {
    fn get_active(&self) -> i32;

    fn get_active_id(&self) -> Option<String>;

    fn get_active_iter(&self) -> Option<TreeIter>;

    fn get_add_tearoffs(&self) -> bool;

    fn get_button_sensitivity(&self) -> SensitivityType;

    fn get_column_span_column(&self) -> i32;

    fn get_entry_text_column(&self) -> i32;

    #[cfg(not(feature = "v3_20"))]
    fn get_focus_on_click(&self) -> bool;

    fn get_has_entry(&self) -> bool;

    fn get_id_column(&self) -> i32;

    fn get_model(&self) -> Option<TreeModel>;

    //fn get_popup_accessible(&self) -> /*Ignored*/Option<atk::Object>;

    fn get_popup_fixed_width(&self) -> bool;

    //fn get_row_separator_func(&self) -> /*Unknown conversion*//*Unimplemented*/TreeViewRowSeparatorFunc;

    fn get_row_span_column(&self) -> i32;

    fn get_title(&self) -> Option<String>;

    fn get_wrap_width(&self) -> i32;

    fn popdown(&self);

    fn popup(&self);

    fn popup_for_device<T: IsA<gdk::Device>>(&self, device: &T);

    fn set_active(&self, index_: i32);

    fn set_active_id<'a, T: Into<Option<&'a str>>>(&self, active_id: T) -> bool;

    fn set_active_iter(&self, iter: Option<&TreeIter>);

    fn set_add_tearoffs(&self, add_tearoffs: bool);

    fn set_button_sensitivity(&self, sensitivity: SensitivityType);

    fn set_column_span_column(&self, column_span: i32);

    fn set_entry_text_column(&self, text_column: i32);

    #[cfg(not(feature = "v3_20"))]
    fn set_focus_on_click(&self, focus_on_click: bool);

    fn set_id_column(&self, id_column: i32);

    fn set_model<T: IsA<TreeModel>>(&self, model: Option<&T>);

    fn set_popup_fixed_width(&self, fixed: bool);

    //fn set_row_separator_func(&self, func: /*Unknown conversion*//*Unimplemented*/TreeViewRowSeparatorFunc, data: /*Unimplemented*/Option<Fundamental: Pointer>, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    fn set_row_span_column(&self, row_span: i32);

    fn set_title(&self, title: &str);

    fn set_wrap_width(&self, width: i32);

    fn get_property_cell_area(&self) -> Option<CellArea>;

    fn set_property_cell_area(&self, cell_area: Option<&CellArea>);

    fn set_property_has_entry(&self, has_entry: bool);

    fn get_property_has_frame(&self) -> bool;

    fn set_property_has_frame(&self, has_frame: bool);

    fn get_property_popup_shown(&self) -> bool;

    fn get_property_tearoff_title(&self) -> Option<String>;

    fn set_property_tearoff_title(&self, tearoff_title: Option<&str>);

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_format_entry_text<F: Fn(&Self, &str) -> String + 'static>(&self, f: F) -> u64;

    fn connect_move_active<F: Fn(&Self, ScrollType) + 'static>(&self, f: F) -> u64;

    fn connect_popdown<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> u64;

    fn connect_popup<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<ComboBox> + IsA<Object>> ComboBoxExt for O {
    fn get_active(&self) -> i32 {
        unsafe {
            ffi::gtk_combo_box_get_active(self.to_glib_none().0)
        }
    }

    fn get_active_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_combo_box_get_active_id(self.to_glib_none().0))
        }
    }

    fn get_active_iter(&self) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_combo_box_get_active_iter(self.to_glib_none().0, iter.to_glib_none_mut().0));
            if ret { Some(iter) } else { None }
        }
    }

    fn get_add_tearoffs(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_combo_box_get_add_tearoffs(self.to_glib_none().0))
        }
    }

    fn get_button_sensitivity(&self) -> SensitivityType {
        unsafe {
            from_glib(ffi::gtk_combo_box_get_button_sensitivity(self.to_glib_none().0))
        }
    }

    fn get_column_span_column(&self) -> i32 {
        unsafe {
            ffi::gtk_combo_box_get_column_span_column(self.to_glib_none().0)
        }
    }

    fn get_entry_text_column(&self) -> i32 {
        unsafe {
            ffi::gtk_combo_box_get_entry_text_column(self.to_glib_none().0)
        }
    }

    #[cfg(not(feature = "v3_20"))]
    fn get_focus_on_click(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_combo_box_get_focus_on_click(self.to_glib_none().0))
        }
    }

    fn get_has_entry(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_combo_box_get_has_entry(self.to_glib_none().0))
        }
    }

    fn get_id_column(&self) -> i32 {
        unsafe {
            ffi::gtk_combo_box_get_id_column(self.to_glib_none().0)
        }
    }

    fn get_model(&self) -> Option<TreeModel> {
        unsafe {
            from_glib_none(ffi::gtk_combo_box_get_model(self.to_glib_none().0))
        }
    }

    //fn get_popup_accessible(&self) -> /*Ignored*/Option<atk::Object> {
    //    unsafe { TODO: call ffi::gtk_combo_box_get_popup_accessible() }
    //}

    fn get_popup_fixed_width(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_combo_box_get_popup_fixed_width(self.to_glib_none().0))
        }
    }

    //fn get_row_separator_func(&self) -> /*Unknown conversion*//*Unimplemented*/TreeViewRowSeparatorFunc {
    //    unsafe { TODO: call ffi::gtk_combo_box_get_row_separator_func() }
    //}

    fn get_row_span_column(&self) -> i32 {
        unsafe {
            ffi::gtk_combo_box_get_row_span_column(self.to_glib_none().0)
        }
    }

    fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_combo_box_get_title(self.to_glib_none().0))
        }
    }

    fn get_wrap_width(&self) -> i32 {
        unsafe {
            ffi::gtk_combo_box_get_wrap_width(self.to_glib_none().0)
        }
    }

    fn popdown(&self) {
        unsafe {
            ffi::gtk_combo_box_popdown(self.to_glib_none().0);
        }
    }

    fn popup(&self) {
        unsafe {
            ffi::gtk_combo_box_popup(self.to_glib_none().0);
        }
    }

    fn popup_for_device<T: IsA<gdk::Device>>(&self, device: &T) {
        unsafe {
            ffi::gtk_combo_box_popup_for_device(self.to_glib_none().0, device.to_glib_none().0);
        }
    }

    fn set_active(&self, index_: i32) {
        unsafe {
            ffi::gtk_combo_box_set_active(self.to_glib_none().0, index_);
        }
    }

    fn set_active_id<'a, T: Into<Option<&'a str>>>(&self, active_id: T) -> bool {
        unsafe {
            from_glib(ffi::gtk_combo_box_set_active_id(self.to_glib_none().0, active_id.into().to_glib_none().0))
        }
    }

    fn set_active_iter(&self, iter: Option<&TreeIter>) {
        unsafe {
            ffi::gtk_combo_box_set_active_iter(self.to_glib_none().0, mut_override(iter.to_glib_none().0));
        }
    }

    fn set_add_tearoffs(&self, add_tearoffs: bool) {
        unsafe {
            ffi::gtk_combo_box_set_add_tearoffs(self.to_glib_none().0, add_tearoffs.to_glib());
        }
    }

    fn set_button_sensitivity(&self, sensitivity: SensitivityType) {
        unsafe {
            ffi::gtk_combo_box_set_button_sensitivity(self.to_glib_none().0, sensitivity.to_glib());
        }
    }

    fn set_column_span_column(&self, column_span: i32) {
        unsafe {
            ffi::gtk_combo_box_set_column_span_column(self.to_glib_none().0, column_span);
        }
    }

    fn set_entry_text_column(&self, text_column: i32) {
        unsafe {
            ffi::gtk_combo_box_set_entry_text_column(self.to_glib_none().0, text_column);
        }
    }

    #[cfg(not(feature = "v3_20"))]
    fn set_focus_on_click(&self, focus_on_click: bool) {
        unsafe {
            ffi::gtk_combo_box_set_focus_on_click(self.to_glib_none().0, focus_on_click.to_glib());
        }
    }

    fn set_id_column(&self, id_column: i32) {
        unsafe {
            ffi::gtk_combo_box_set_id_column(self.to_glib_none().0, id_column);
        }
    }

    fn set_model<T: IsA<TreeModel>>(&self, model: Option<&T>) {
        unsafe {
            ffi::gtk_combo_box_set_model(self.to_glib_none().0, model.to_glib_none().0);
        }
    }

    fn set_popup_fixed_width(&self, fixed: bool) {
        unsafe {
            ffi::gtk_combo_box_set_popup_fixed_width(self.to_glib_none().0, fixed.to_glib());
        }
    }

    //fn set_row_separator_func(&self, func: /*Unknown conversion*//*Unimplemented*/TreeViewRowSeparatorFunc, data: /*Unimplemented*/Option<Fundamental: Pointer>, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_combo_box_set_row_separator_func() }
    //}

    fn set_row_span_column(&self, row_span: i32) {
        unsafe {
            ffi::gtk_combo_box_set_row_span_column(self.to_glib_none().0, row_span);
        }
    }

    fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_combo_box_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    fn set_wrap_width(&self, width: i32) {
        unsafe {
            ffi::gtk_combo_box_set_wrap_width(self.to_glib_none().0, width);
        }
    }

    fn get_property_cell_area(&self) -> Option<CellArea> {
        let mut value = Value::from(None::<&CellArea>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "cell-area".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_property_cell_area(&self, cell_area: Option<&CellArea>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "cell-area".to_glib_none().0, Value::from(cell_area).to_glib_none().0);
        }
    }

    fn set_property_has_entry(&self, has_entry: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "has-entry".to_glib_none().0, Value::from(&has_entry).to_glib_none().0);
        }
    }

    fn get_property_has_frame(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "has-frame".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_has_frame(&self, has_frame: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "has-frame".to_glib_none().0, Value::from(&has_frame).to_glib_none().0);
        }
    }

    fn get_property_popup_shown(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "popup-shown".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_tearoff_title(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "tearoff-title".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_property_tearoff_title(&self, tearoff_title: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "tearoff-title".to_glib_none().0, Value::from(tearoff_title).to_glib_none().0);
        }
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "changed",
                transmute(changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_format_entry_text<F: Fn(&Self, &str) -> String + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str) -> String + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "format-entry-text",
                transmute(format_entry_text_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_move_active<F: Fn(&Self, ScrollType) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, ScrollType) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "move-active",
                transmute(move_active_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_popdown<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "popdown",
                transmute(popdown_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_popup<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "popup",
                transmute(popup_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn changed_trampoline<T>(this: *mut ffi::GtkComboBox, f: glib_ffi::gpointer)
where T: IsA<ComboBox> {
    callback_guard!();
    let f: &Box_<Fn(&T) + 'static> = transmute(f);
    f(&ComboBox::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn format_entry_text_trampoline<T>(this: *mut ffi::GtkComboBox, path: *mut libc::c_char, f: glib_ffi::gpointer) -> *mut libc::c_char
where T: IsA<ComboBox> {
    callback_guard!();
    let f: &Box_<Fn(&T, &str) -> String + 'static> = transmute(f);
    f(&ComboBox::from_glib_none(this).downcast_unchecked(), &String::from_glib_none(path)).to_glib_full()
}

unsafe extern "C" fn move_active_trampoline<T>(this: *mut ffi::GtkComboBox, scroll_type: ffi::GtkScrollType, f: glib_ffi::gpointer)
where T: IsA<ComboBox> {
    callback_guard!();
    let f: &Box_<Fn(&T, ScrollType) + 'static> = transmute(f);
    f(&ComboBox::from_glib_none(this).downcast_unchecked(), from_glib(scroll_type))
}

unsafe extern "C" fn popdown_trampoline<T>(this: *mut ffi::GtkComboBox, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where T: IsA<ComboBox> {
    callback_guard!();
    let f: &Box_<Fn(&T) -> bool + 'static> = transmute(f);
    f(&ComboBox::from_glib_none(this).downcast_unchecked()).to_glib()
}

unsafe extern "C" fn popup_trampoline<T>(this: *mut ffi::GtkComboBox, f: glib_ffi::gpointer)
where T: IsA<ComboBox> {
    callback_guard!();
    let f: &Box_<Fn(&T) + 'static> = transmute(f);
    f(&ComboBox::from_glib_none(this).downcast_unchecked())
}
