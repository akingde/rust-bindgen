/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug)]
pub struct Struct_HandleWithDtor<T> {
    pub ptr: *mut T,
}
pub type HandleValue = Struct_HandleWithDtor<::std::os::raw::c_int>;
#[repr(C)]
#[derive(Debug)]
pub struct Struct_WithoutDtor {
    pub shouldBeWithDtor: HandleValue,
}
#[test]
fn bindgen_test_layout_Struct_WithoutDtor() {
    assert_eq!(::std::mem::size_of::<Struct_WithoutDtor>() , 8usize);
    assert_eq!(::std::mem::align_of::<Struct_WithoutDtor>() , 8usize);
}
