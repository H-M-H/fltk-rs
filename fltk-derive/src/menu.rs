use crate::utils::get_fl_name;
use proc_macro::TokenStream;
use quote::*;
use syn::*;


pub fn impl_menu_trait(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_str = get_fl_name(name.to_string());
    let add = Ident::new(format!("{}_{}", name_str, "add").as_str(), name.span());
    let insert = Ident::new(format!("{}_{}", name_str, "insert").as_str(), name.span());
    let get_item = Ident::new(format!("{}_{}", name_str, "get_item").as_str(), name.span());
    let text_font = Ident::new(
        format!("{}_{}", name_str, "text_font").as_str(),
        name.span(),
    );
    let set_text_font = Ident::new(
        format!("{}_{}", name_str, "set_text_font").as_str(),
        name.span(),
    );
    let text_color = Ident::new(
        format!("{}_{}", name_str, "text_color").as_str(),
        name.span(),
    );
    let set_text_color = Ident::new(
        format!("{}_{}", name_str, "set_text_color").as_str(),
        name.span(),
    );
    let text_size = Ident::new(
        format!("{}_{}", name_str, "text_size").as_str(),
        name.span(),
    );
    let set_text_size = Ident::new(
        format!("{}_{}", name_str, "set_text_size").as_str(),
        name.span(),
    );
    let add_choice = Ident::new(
        format!("{}_{}", name_str, "add_choice").as_str(),
        name.span(),
    );
    let get_choice = Ident::new(
        format!("{}_{}", name_str, "get_choice").as_str(),
        name.span(),
    );

    let gen = quote! {
        impl MenuExt for #name {
            fn add(&mut self, name: &str, shortcut: Shortcut, flag: MenuFlag, mut cb: Box<dyn FnMut()>) {
                debug_assert!(
                    self.top_window().unwrap().takes_events() && self.takes_events(), 
                    "Handling events requires that the window and widget be active!"
                );
                let temp = CString::new(name).unwrap();
                unsafe {
                    unsafe extern "C" fn shim(_wid: *mut Fl_Widget, data: *mut raw::c_void) {
                        let a: *mut Box<dyn FnMut()> = mem::transmute(data);
                        let f: &mut (dyn FnMut()) = &mut **a;
                        f();
                    }
                    let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(cb));
                    let data: *mut raw::c_void = mem::transmute(a);
                    let callback: Fl_Callback = Some(shim);
                    #add(self._inner, temp.as_ptr() as *const raw::c_char, shortcut as i32, callback, data, flag as i32);
                }
            }

            fn insert(&mut self, idx: u32, name: &str, shortcut: Shortcut, flag: MenuFlag, cb: Box<dyn FnMut()>) {
                debug_assert!(
                    self.top_window().unwrap().takes_events() && self.takes_events(), 
                    "Handling events requires that the window and widget be active!"
                );
                let temp = CString::new(name).unwrap();
                unsafe {
                    unsafe extern "C" fn shim(_wid: *mut Fl_Widget, data: *mut raw::c_void) {
                        let a: *mut Box<dyn FnMut()> = mem::transmute(data);
                        let f: &mut (dyn FnMut()) = &mut **a;
                        f();
                    }
                    let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(cb));
                    let data: *mut raw::c_void = mem::transmute(a);
                    let callback: Fl_Callback = Some(shim);
                    #insert(self._inner, idx as i32, temp.into_raw() as *const raw::c_char, shortcut as i32, callback, data, flag as i32);
                }
            }

            fn item(&self, name: &str) -> Option<MenuItem> {
                let name = CString::new(name).unwrap().clone();
                unsafe {
                    let menu_item = #get_item(
                        self._inner,
                        name.into_raw() as *const raw::c_char);
                    if menu_item.is_null() {
                        None
                    } else {
                        Some(MenuItem {
                            _inner: menu_item,
                        })
                    }
                }
            }

            fn text_font(&self) -> Font {
                unsafe {
                    mem::transmute(#text_font(self._inner))
                }
            }

            fn set_text_font(&mut self, c: Font) {
                unsafe {
                    #set_text_font(self._inner, c as i32)
                }
            }

            fn text_size(&self) -> u32 {
                unsafe {
                    #text_size(self._inner) as u32
                }
            }

            fn set_text_size(&mut self, c: u32) {
                unsafe {
                    debug_assert!(c <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                    #set_text_size(self._inner, c as i32)
                }
            }

            fn text_color(&self) -> Color {
                unsafe {
                    mem::transmute(#text_color(self._inner))
                }
            }

            fn set_text_color(&mut self, c: Color) {
                unsafe {
                    #set_text_color(self._inner, c as u32)
                }
            }

            fn add_choice(&mut self, text: &str) {
                unsafe {
                    let arg2 = CString::new(text).unwrap();
                    #add_choice(self._inner, arg2.into_raw() as *mut raw::c_char)
                }
            }

            fn choice(&self) -> Option<String> {
                unsafe {
                    let choice_ptr = #get_choice(self._inner);
                    if choice_ptr.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(choice_ptr as *mut raw::c_char).to_string_lossy().to_string())
                    }
                }
            }
        }
    };
    gen.into()
}