/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Struct_RandomTemplate<T> {
    pub _phantom0: ::std::marker::PhantomData<T>,
}
pub enum Struct_Wat { }
pub enum Struct_Wat3 { }
#[repr(C)]
pub struct ShouldBeOpaque;
pub type ShouldNotBeOpaque = Struct_RandomTemplate<::std::os::raw::c_int>;
