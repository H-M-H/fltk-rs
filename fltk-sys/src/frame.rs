/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Widget {
    _unused: [u8; 0],
}
pub type Fl_Callback = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut Fl_Widget, arg2: *mut ::std::os::raw::c_void),
>;
pub type custom_handler_callback = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: ::std::os::raw::c_int,
        arg2: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn Fl_Widget_callback_with_captures(
        arg1: *mut Fl_Widget,
        cb: Fl_Callback,
        arg2: *mut ::std::os::raw::c_void,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Box {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Box_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Box;
}
extern "C" {
    pub fn Fl_Box_x(arg1: *mut Fl_Box) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Box_y(arg1: *mut Fl_Box) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Box_width(arg1: *mut Fl_Box) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Box_height(arg1: *mut Fl_Box) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Box_label(arg1: *mut Fl_Box) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Box_set_label(arg1: *mut Fl_Box, title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Box_redraw(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_show(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_hide(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_activate(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_deactivate(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_redraw_label(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_resize(
        arg1: *mut Fl_Box,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Box_tooltip(arg1: *mut Fl_Box) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Box_set_tooltip(arg1: *mut Fl_Box, txt: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Box_get_type(arg1: *mut Fl_Box) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Box_set_type(arg1: *mut Fl_Box, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Box_color(arg1: *mut Fl_Box) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Box_set_color(arg1: *mut Fl_Box, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Box_label_color(arg1: *mut Fl_Box) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Box_set_label_color(arg1: *mut Fl_Box, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Box_label_font(arg1: *mut Fl_Box) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Box_set_label_font(arg1: *mut Fl_Box, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Box_label_size(arg1: *mut Fl_Box) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Box_set_label_size(arg1: *mut Fl_Box, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Box_label_type(arg1: *mut Fl_Box) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Box_set_label_type(arg1: *mut Fl_Box, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Box_box(arg1: *mut Fl_Box) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Box_set_box(arg1: *mut Fl_Box, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Box_changed(arg1: *mut Fl_Box) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Box_set_changed(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_clear_changed(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_align(arg1: *mut Fl_Box) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Box_set_align(arg1: *mut Fl_Box, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Box_delete(arg1: *mut Fl_Box);
}
extern "C" {
    pub fn Fl_Box_set_image(arg1: *mut Fl_Box, arg2: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Box_set_handler(
        self_: *mut *mut Fl_Box,
        cb: custom_handler_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Box_set_trigger(arg1: *mut Fl_Box, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Box_image(arg1: *const Fl_Box) -> *mut ::std::os::raw::c_void;
}
