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
pub struct Fl_Browser {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Browser_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Browser;
}
extern "C" {
    pub fn Fl_Browser_x(arg1: *mut Fl_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Browser_y(arg1: *mut Fl_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Browser_width(arg1: *mut Fl_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Browser_height(arg1: *mut Fl_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Browser_label(arg1: *mut Fl_Browser) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Browser_set_label(arg1: *mut Fl_Browser, title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Browser_redraw(arg1: *mut Fl_Browser);
}
extern "C" {
    pub fn Fl_Browser_show(arg1: *mut Fl_Browser);
}
extern "C" {
    pub fn Fl_Browser_hide(arg1: *mut Fl_Browser);
}
extern "C" {
    pub fn Fl_Browser_activate(arg1: *mut Fl_Browser);
}
extern "C" {
    pub fn Fl_Browser_deactivate(arg1: *mut Fl_Browser);
}
extern "C" {
    pub fn Fl_Browser_redraw_label(arg1: *mut Fl_Browser);
}
extern "C" {
    pub fn Fl_Browser_resize(
        arg1: *mut Fl_Browser,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Browser_tooltip(arg1: *mut Fl_Browser) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Browser_set_tooltip(arg1: *mut Fl_Browser, txt: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Browser_get_type(arg1: *mut Fl_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Browser_set_type(arg1: *mut Fl_Browser, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Browser_color(arg1: *mut Fl_Browser) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Browser_set_color(arg1: *mut Fl_Browser, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Browser_label_color(arg1: *mut Fl_Browser) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Browser_set_label_color(arg1: *mut Fl_Browser, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Browser_label_font(arg1: *mut Fl_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Browser_set_label_font(arg1: *mut Fl_Browser, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Browser_label_size(arg1: *mut Fl_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Browser_set_label_size(arg1: *mut Fl_Browser, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Browser_label_type(arg1: *mut Fl_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Browser_set_label_type(arg1: *mut Fl_Browser, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Browser_box(arg1: *mut Fl_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Browser_set_box(arg1: *mut Fl_Browser, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Browser_changed(arg1: *mut Fl_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Browser_set_changed(arg1: *mut Fl_Browser);
}
extern "C" {
    pub fn Fl_Browser_clear_changed(arg1: *mut Fl_Browser);
}
extern "C" {
    pub fn Fl_Browser_align(arg1: *mut Fl_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Browser_set_align(arg1: *mut Fl_Browser, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Browser_delete(arg1: *mut Fl_Browser);
}
extern "C" {
    pub fn Fl_Browser_set_image(arg1: *mut Fl_Browser, arg2: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Browser_set_handler(
        self_: *mut *mut Fl_Browser,
        cb: custom_handler_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Browser_set_trigger(arg1: *mut Fl_Browser, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Browser_image(arg1: *const Fl_Browser) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Browser_remove(arg1: *mut Fl_Browser, line: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Browser_add(arg1: *mut Fl_Browser, newtext: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Browser_insert(
        arg1: *mut Fl_Browser,
        line: ::std::os::raw::c_int,
        newtext: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Browser_move(
        arg1: *mut Fl_Browser,
        to: ::std::os::raw::c_int,
        from: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Browser_swap(
        arg1: *mut Fl_Browser,
        a: ::std::os::raw::c_int,
        b: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Browser_clear(arg1: *mut Fl_Browser);
}
extern "C" {
    pub fn Fl_Browser_size(arg1: *const Fl_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Browser_set_size(
        arg1: *mut Fl_Browser,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Browser_select(
        arg1: *mut Fl_Browser,
        line: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Browser_selected(
        arg1: *const Fl_Browser,
        line: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Browser_text(
        arg1: *const Fl_Browser,
        line: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Browser_set_text(
        arg1: *mut Fl_Browser,
        line: ::std::os::raw::c_int,
        newtext: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Browser_load_file(arg1: *mut Fl_Browser, file: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Browser_text_size(arg1: *mut Fl_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Browser_set_text_size(arg1: *mut Fl_Browser, s: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Browser_set_icon(
        arg1: *mut Fl_Browser,
        line: ::std::os::raw::c_int,
        icon: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Browser_icon(
        arg1: *const Fl_Browser,
        line: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Browser_remove_icon(arg1: *mut Fl_Browser, line: ::std::os::raw::c_int);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Hold_Browser {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Hold_Browser_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Hold_Browser;
}
extern "C" {
    pub fn Fl_Hold_Browser_x(arg1: *mut Fl_Hold_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Hold_Browser_y(arg1: *mut Fl_Hold_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Hold_Browser_width(arg1: *mut Fl_Hold_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Hold_Browser_height(arg1: *mut Fl_Hold_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Hold_Browser_label(arg1: *mut Fl_Hold_Browser) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Hold_Browser_set_label(
        arg1: *mut Fl_Hold_Browser,
        title: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Hold_Browser_redraw(arg1: *mut Fl_Hold_Browser);
}
extern "C" {
    pub fn Fl_Hold_Browser_show(arg1: *mut Fl_Hold_Browser);
}
extern "C" {
    pub fn Fl_Hold_Browser_hide(arg1: *mut Fl_Hold_Browser);
}
extern "C" {
    pub fn Fl_Hold_Browser_activate(arg1: *mut Fl_Hold_Browser);
}
extern "C" {
    pub fn Fl_Hold_Browser_deactivate(arg1: *mut Fl_Hold_Browser);
}
extern "C" {
    pub fn Fl_Hold_Browser_redraw_label(arg1: *mut Fl_Hold_Browser);
}
extern "C" {
    pub fn Fl_Hold_Browser_resize(
        arg1: *mut Fl_Hold_Browser,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Hold_Browser_tooltip(arg1: *mut Fl_Hold_Browser) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Hold_Browser_set_tooltip(
        arg1: *mut Fl_Hold_Browser,
        txt: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Hold_Browser_get_type(arg1: *mut Fl_Hold_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Hold_Browser_set_type(arg1: *mut Fl_Hold_Browser, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Hold_Browser_color(arg1: *mut Fl_Hold_Browser) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Hold_Browser_set_color(arg1: *mut Fl_Hold_Browser, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Hold_Browser_label_color(arg1: *mut Fl_Hold_Browser) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Hold_Browser_set_label_color(
        arg1: *mut Fl_Hold_Browser,
        color: ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn Fl_Hold_Browser_label_font(arg1: *mut Fl_Hold_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Hold_Browser_set_label_font(arg1: *mut Fl_Hold_Browser, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Hold_Browser_label_size(arg1: *mut Fl_Hold_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Hold_Browser_set_label_size(arg1: *mut Fl_Hold_Browser, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Hold_Browser_label_type(arg1: *mut Fl_Hold_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Hold_Browser_set_label_type(arg1: *mut Fl_Hold_Browser, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Hold_Browser_box(arg1: *mut Fl_Hold_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Hold_Browser_set_box(arg1: *mut Fl_Hold_Browser, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Hold_Browser_changed(arg1: *mut Fl_Hold_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Hold_Browser_set_changed(arg1: *mut Fl_Hold_Browser);
}
extern "C" {
    pub fn Fl_Hold_Browser_clear_changed(arg1: *mut Fl_Hold_Browser);
}
extern "C" {
    pub fn Fl_Hold_Browser_align(arg1: *mut Fl_Hold_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Hold_Browser_set_align(arg1: *mut Fl_Hold_Browser, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Hold_Browser_delete(arg1: *mut Fl_Hold_Browser);
}
extern "C" {
    pub fn Fl_Hold_Browser_set_image(arg1: *mut Fl_Hold_Browser, arg2: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_Hold_Browser_set_handler(
        self_: *mut *mut Fl_Hold_Browser,
        cb: custom_handler_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Hold_Browser_set_trigger(arg1: *mut Fl_Hold_Browser, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Hold_Browser_image(arg1: *const Fl_Hold_Browser) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Hold_Browser_remove(arg1: *mut Fl_Hold_Browser, line: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Hold_Browser_add(arg1: *mut Fl_Hold_Browser, newtext: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Hold_Browser_insert(
        arg1: *mut Fl_Hold_Browser,
        line: ::std::os::raw::c_int,
        newtext: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Hold_Browser_move(
        arg1: *mut Fl_Hold_Browser,
        to: ::std::os::raw::c_int,
        from: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Hold_Browser_swap(
        arg1: *mut Fl_Hold_Browser,
        a: ::std::os::raw::c_int,
        b: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Hold_Browser_clear(arg1: *mut Fl_Hold_Browser);
}
extern "C" {
    pub fn Fl_Hold_Browser_size(arg1: *const Fl_Hold_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Hold_Browser_set_size(
        arg1: *mut Fl_Hold_Browser,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Hold_Browser_select(
        arg1: *mut Fl_Hold_Browser,
        line: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Hold_Browser_selected(
        arg1: *const Fl_Hold_Browser,
        line: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Hold_Browser_text(
        arg1: *const Fl_Hold_Browser,
        line: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Hold_Browser_set_text(
        arg1: *mut Fl_Hold_Browser,
        line: ::std::os::raw::c_int,
        newtext: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Hold_Browser_load_file(
        arg1: *mut Fl_Hold_Browser,
        file: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Hold_Browser_text_size(arg1: *mut Fl_Hold_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Hold_Browser_set_text_size(arg1: *mut Fl_Hold_Browser, s: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Hold_Browser_set_icon(
        arg1: *mut Fl_Hold_Browser,
        line: ::std::os::raw::c_int,
        icon: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Hold_Browser_icon(
        arg1: *const Fl_Hold_Browser,
        line: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Hold_Browser_remove_icon(arg1: *mut Fl_Hold_Browser, line: ::std::os::raw::c_int);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Select_Browser {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Select_Browser_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Select_Browser;
}
extern "C" {
    pub fn Fl_Select_Browser_x(arg1: *mut Fl_Select_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Select_Browser_y(arg1: *mut Fl_Select_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Select_Browser_width(arg1: *mut Fl_Select_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Select_Browser_height(arg1: *mut Fl_Select_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Select_Browser_label(arg1: *mut Fl_Select_Browser) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Select_Browser_set_label(
        arg1: *mut Fl_Select_Browser,
        title: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Select_Browser_redraw(arg1: *mut Fl_Select_Browser);
}
extern "C" {
    pub fn Fl_Select_Browser_show(arg1: *mut Fl_Select_Browser);
}
extern "C" {
    pub fn Fl_Select_Browser_hide(arg1: *mut Fl_Select_Browser);
}
extern "C" {
    pub fn Fl_Select_Browser_activate(arg1: *mut Fl_Select_Browser);
}
extern "C" {
    pub fn Fl_Select_Browser_deactivate(arg1: *mut Fl_Select_Browser);
}
extern "C" {
    pub fn Fl_Select_Browser_redraw_label(arg1: *mut Fl_Select_Browser);
}
extern "C" {
    pub fn Fl_Select_Browser_resize(
        arg1: *mut Fl_Select_Browser,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Select_Browser_tooltip(arg1: *mut Fl_Select_Browser)
        -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Select_Browser_set_tooltip(
        arg1: *mut Fl_Select_Browser,
        txt: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Select_Browser_get_type(arg1: *mut Fl_Select_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Select_Browser_set_type(arg1: *mut Fl_Select_Browser, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Select_Browser_color(arg1: *mut Fl_Select_Browser) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Select_Browser_set_color(arg1: *mut Fl_Select_Browser, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Select_Browser_label_color(arg1: *mut Fl_Select_Browser) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Select_Browser_set_label_color(
        arg1: *mut Fl_Select_Browser,
        color: ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn Fl_Select_Browser_label_font(arg1: *mut Fl_Select_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Select_Browser_set_label_font(
        arg1: *mut Fl_Select_Browser,
        font: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Select_Browser_label_size(arg1: *mut Fl_Select_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Select_Browser_set_label_size(
        arg1: *mut Fl_Select_Browser,
        sz: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Select_Browser_label_type(arg1: *mut Fl_Select_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Select_Browser_set_label_type(
        arg1: *mut Fl_Select_Browser,
        typ: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Select_Browser_box(arg1: *mut Fl_Select_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Select_Browser_set_box(arg1: *mut Fl_Select_Browser, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Select_Browser_changed(arg1: *mut Fl_Select_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Select_Browser_set_changed(arg1: *mut Fl_Select_Browser);
}
extern "C" {
    pub fn Fl_Select_Browser_clear_changed(arg1: *mut Fl_Select_Browser);
}
extern "C" {
    pub fn Fl_Select_Browser_align(arg1: *mut Fl_Select_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Select_Browser_set_align(arg1: *mut Fl_Select_Browser, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Select_Browser_delete(arg1: *mut Fl_Select_Browser);
}
extern "C" {
    pub fn Fl_Select_Browser_set_image(
        arg1: *mut Fl_Select_Browser,
        arg2: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Select_Browser_set_handler(
        self_: *mut *mut Fl_Select_Browser,
        cb: custom_handler_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Select_Browser_set_trigger(arg1: *mut Fl_Select_Browser, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Select_Browser_image(arg1: *const Fl_Select_Browser) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Select_Browser_remove(arg1: *mut Fl_Select_Browser, line: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Select_Browser_add(
        arg1: *mut Fl_Select_Browser,
        newtext: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Select_Browser_insert(
        arg1: *mut Fl_Select_Browser,
        line: ::std::os::raw::c_int,
        newtext: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Select_Browser_move(
        arg1: *mut Fl_Select_Browser,
        to: ::std::os::raw::c_int,
        from: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Select_Browser_swap(
        arg1: *mut Fl_Select_Browser,
        a: ::std::os::raw::c_int,
        b: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Select_Browser_clear(arg1: *mut Fl_Select_Browser);
}
extern "C" {
    pub fn Fl_Select_Browser_size(arg1: *const Fl_Select_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Select_Browser_set_size(
        arg1: *mut Fl_Select_Browser,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Select_Browser_select(
        arg1: *mut Fl_Select_Browser,
        line: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Select_Browser_selected(
        arg1: *const Fl_Select_Browser,
        line: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Select_Browser_text(
        arg1: *const Fl_Select_Browser,
        line: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Select_Browser_set_text(
        arg1: *mut Fl_Select_Browser,
        line: ::std::os::raw::c_int,
        newtext: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Select_Browser_load_file(
        arg1: *mut Fl_Select_Browser,
        file: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Select_Browser_text_size(arg1: *mut Fl_Select_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Select_Browser_set_text_size(arg1: *mut Fl_Select_Browser, s: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Select_Browser_set_icon(
        arg1: *mut Fl_Select_Browser,
        line: ::std::os::raw::c_int,
        icon: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Select_Browser_icon(
        arg1: *const Fl_Select_Browser,
        line: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Select_Browser_remove_icon(arg1: *mut Fl_Select_Browser, line: ::std::os::raw::c_int);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Multi_Browser {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Multi_Browser_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Multi_Browser;
}
extern "C" {
    pub fn Fl_Multi_Browser_x(arg1: *mut Fl_Multi_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multi_Browser_y(arg1: *mut Fl_Multi_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multi_Browser_width(arg1: *mut Fl_Multi_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multi_Browser_height(arg1: *mut Fl_Multi_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multi_Browser_label(arg1: *mut Fl_Multi_Browser) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Multi_Browser_set_label(
        arg1: *mut Fl_Multi_Browser,
        title: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Multi_Browser_redraw(arg1: *mut Fl_Multi_Browser);
}
extern "C" {
    pub fn Fl_Multi_Browser_show(arg1: *mut Fl_Multi_Browser);
}
extern "C" {
    pub fn Fl_Multi_Browser_hide(arg1: *mut Fl_Multi_Browser);
}
extern "C" {
    pub fn Fl_Multi_Browser_activate(arg1: *mut Fl_Multi_Browser);
}
extern "C" {
    pub fn Fl_Multi_Browser_deactivate(arg1: *mut Fl_Multi_Browser);
}
extern "C" {
    pub fn Fl_Multi_Browser_redraw_label(arg1: *mut Fl_Multi_Browser);
}
extern "C" {
    pub fn Fl_Multi_Browser_resize(
        arg1: *mut Fl_Multi_Browser,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Multi_Browser_tooltip(arg1: *mut Fl_Multi_Browser) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Multi_Browser_set_tooltip(
        arg1: *mut Fl_Multi_Browser,
        txt: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Multi_Browser_get_type(arg1: *mut Fl_Multi_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multi_Browser_set_type(arg1: *mut Fl_Multi_Browser, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Multi_Browser_color(arg1: *mut Fl_Multi_Browser) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Multi_Browser_set_color(arg1: *mut Fl_Multi_Browser, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_Multi_Browser_label_color(arg1: *mut Fl_Multi_Browser) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_Multi_Browser_set_label_color(
        arg1: *mut Fl_Multi_Browser,
        color: ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn Fl_Multi_Browser_label_font(arg1: *mut Fl_Multi_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multi_Browser_set_label_font(
        arg1: *mut Fl_Multi_Browser,
        font: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Multi_Browser_label_size(arg1: *mut Fl_Multi_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multi_Browser_set_label_size(arg1: *mut Fl_Multi_Browser, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Multi_Browser_label_type(arg1: *mut Fl_Multi_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multi_Browser_set_label_type(arg1: *mut Fl_Multi_Browser, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Multi_Browser_box(arg1: *mut Fl_Multi_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multi_Browser_set_box(arg1: *mut Fl_Multi_Browser, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Multi_Browser_changed(arg1: *mut Fl_Multi_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multi_Browser_set_changed(arg1: *mut Fl_Multi_Browser);
}
extern "C" {
    pub fn Fl_Multi_Browser_clear_changed(arg1: *mut Fl_Multi_Browser);
}
extern "C" {
    pub fn Fl_Multi_Browser_align(arg1: *mut Fl_Multi_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multi_Browser_set_align(arg1: *mut Fl_Multi_Browser, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Multi_Browser_delete(arg1: *mut Fl_Multi_Browser);
}
extern "C" {
    pub fn Fl_Multi_Browser_set_image(
        arg1: *mut Fl_Multi_Browser,
        arg2: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Multi_Browser_set_handler(
        self_: *mut *mut Fl_Multi_Browser,
        cb: custom_handler_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Multi_Browser_set_trigger(arg1: *mut Fl_Multi_Browser, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Multi_Browser_image(arg1: *const Fl_Multi_Browser) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Multi_Browser_remove(arg1: *mut Fl_Multi_Browser, line: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Multi_Browser_add(
        arg1: *mut Fl_Multi_Browser,
        newtext: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Multi_Browser_insert(
        arg1: *mut Fl_Multi_Browser,
        line: ::std::os::raw::c_int,
        newtext: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Multi_Browser_move(
        arg1: *mut Fl_Multi_Browser,
        to: ::std::os::raw::c_int,
        from: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Multi_Browser_swap(
        arg1: *mut Fl_Multi_Browser,
        a: ::std::os::raw::c_int,
        b: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Multi_Browser_clear(arg1: *mut Fl_Multi_Browser);
}
extern "C" {
    pub fn Fl_Multi_Browser_size(arg1: *const Fl_Multi_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multi_Browser_set_size(
        arg1: *mut Fl_Multi_Browser,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Multi_Browser_select(
        arg1: *mut Fl_Multi_Browser,
        line: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multi_Browser_selected(
        arg1: *const Fl_Multi_Browser,
        line: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multi_Browser_text(
        arg1: *const Fl_Multi_Browser,
        line: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Multi_Browser_set_text(
        arg1: *mut Fl_Multi_Browser,
        line: ::std::os::raw::c_int,
        newtext: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Multi_Browser_load_file(
        arg1: *mut Fl_Multi_Browser,
        file: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Multi_Browser_text_size(arg1: *mut Fl_Multi_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multi_Browser_set_text_size(arg1: *mut Fl_Multi_Browser, s: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Multi_Browser_set_icon(
        arg1: *mut Fl_Multi_Browser,
        line: ::std::os::raw::c_int,
        icon: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_Multi_Browser_icon(
        arg1: *const Fl_Multi_Browser,
        line: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_Multi_Browser_remove_icon(arg1: *mut Fl_Multi_Browser, line: ::std::os::raw::c_int);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_File_Browser {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_File_Browser_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_File_Browser;
}
extern "C" {
    pub fn Fl_File_Browser_x(arg1: *mut Fl_File_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_File_Browser_y(arg1: *mut Fl_File_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_File_Browser_width(arg1: *mut Fl_File_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_File_Browser_height(arg1: *mut Fl_File_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_File_Browser_label(arg1: *mut Fl_File_Browser) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_File_Browser_set_label(
        arg1: *mut Fl_File_Browser,
        title: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_File_Browser_redraw(arg1: *mut Fl_File_Browser);
}
extern "C" {
    pub fn Fl_File_Browser_show(arg1: *mut Fl_File_Browser);
}
extern "C" {
    pub fn Fl_File_Browser_hide(arg1: *mut Fl_File_Browser);
}
extern "C" {
    pub fn Fl_File_Browser_activate(arg1: *mut Fl_File_Browser);
}
extern "C" {
    pub fn Fl_File_Browser_deactivate(arg1: *mut Fl_File_Browser);
}
extern "C" {
    pub fn Fl_File_Browser_redraw_label(arg1: *mut Fl_File_Browser);
}
extern "C" {
    pub fn Fl_File_Browser_resize(
        arg1: *mut Fl_File_Browser,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_File_Browser_tooltip(arg1: *mut Fl_File_Browser) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_File_Browser_set_tooltip(
        arg1: *mut Fl_File_Browser,
        txt: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_File_Browser_get_type(arg1: *mut Fl_File_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_File_Browser_set_type(arg1: *mut Fl_File_Browser, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_File_Browser_color(arg1: *mut Fl_File_Browser) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_File_Browser_set_color(arg1: *mut Fl_File_Browser, color: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn Fl_File_Browser_label_color(arg1: *mut Fl_File_Browser) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn Fl_File_Browser_set_label_color(
        arg1: *mut Fl_File_Browser,
        color: ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn Fl_File_Browser_label_font(arg1: *mut Fl_File_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_File_Browser_set_label_font(arg1: *mut Fl_File_Browser, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_File_Browser_label_size(arg1: *mut Fl_File_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_File_Browser_set_label_size(arg1: *mut Fl_File_Browser, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_File_Browser_label_type(arg1: *mut Fl_File_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_File_Browser_set_label_type(arg1: *mut Fl_File_Browser, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_File_Browser_box(arg1: *mut Fl_File_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_File_Browser_set_box(arg1: *mut Fl_File_Browser, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_File_Browser_changed(arg1: *mut Fl_File_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_File_Browser_set_changed(arg1: *mut Fl_File_Browser);
}
extern "C" {
    pub fn Fl_File_Browser_clear_changed(arg1: *mut Fl_File_Browser);
}
extern "C" {
    pub fn Fl_File_Browser_align(arg1: *mut Fl_File_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_File_Browser_set_align(arg1: *mut Fl_File_Browser, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_File_Browser_delete(arg1: *mut Fl_File_Browser);
}
extern "C" {
    pub fn Fl_File_Browser_set_image(arg1: *mut Fl_File_Browser, arg2: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn Fl_File_Browser_set_handler(
        self_: *mut *mut Fl_File_Browser,
        cb: custom_handler_callback,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_File_Browser_set_trigger(arg1: *mut Fl_File_Browser, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_File_Browser_image(arg1: *const Fl_File_Browser) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_File_Browser_remove(arg1: *mut Fl_File_Browser, line: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_File_Browser_add(arg1: *mut Fl_File_Browser, newtext: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_File_Browser_insert(
        arg1: *mut Fl_File_Browser,
        line: ::std::os::raw::c_int,
        newtext: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_File_Browser_move(
        arg1: *mut Fl_File_Browser,
        to: ::std::os::raw::c_int,
        from: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_File_Browser_swap(
        arg1: *mut Fl_File_Browser,
        a: ::std::os::raw::c_int,
        b: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_File_Browser_clear(arg1: *mut Fl_File_Browser);
}
extern "C" {
    pub fn Fl_File_Browser_size(arg1: *const Fl_File_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_File_Browser_set_size(
        arg1: *mut Fl_File_Browser,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_File_Browser_select(
        arg1: *mut Fl_File_Browser,
        line: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_File_Browser_selected(
        arg1: *const Fl_File_Browser,
        line: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_File_Browser_text(
        arg1: *const Fl_File_Browser,
        line: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_File_Browser_set_text(
        arg1: *mut Fl_File_Browser,
        line: ::std::os::raw::c_int,
        newtext: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_File_Browser_load_file(
        arg1: *mut Fl_File_Browser,
        file: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_File_Browser_text_size(arg1: *mut Fl_File_Browser) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_File_Browser_set_text_size(arg1: *mut Fl_File_Browser, s: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_File_Browser_set_icon(
        arg1: *mut Fl_File_Browser,
        line: ::std::os::raw::c_int,
        icon: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Fl_File_Browser_icon(
        arg1: *const Fl_File_Browser,
        line: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Fl_File_Browser_remove_icon(arg1: *mut Fl_File_Browser, line: ::std::os::raw::c_int);
}
