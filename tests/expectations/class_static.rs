/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy)]
pub struct Struct_MyClass;
impl ::std::clone::Clone for Struct_MyClass {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    #[link_name = "_ZN7MyClass7exampleE"]
    pub static mut Struct_MyClass_consts_example:
               *const ::std::os::raw::c_int;
    #[link_name = "_ZN7MyClass26example_check_no_collisionE"]
    pub static mut Struct_MyClass_consts_example_check_no_collision:
               *const ::std::os::raw::c_int;
}
