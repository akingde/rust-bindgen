/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[derive(Copy, Debug)]
#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl <T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self { __BindgenUnionField(::std::marker::PhantomData) }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T { ::std::mem::transmute(self) }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T { ::std::mem::transmute(self) }
}
impl <T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self { Self::new() }
}
impl <T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self { Self::new() }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Union_foo {
    pub a: __BindgenUnionField<::std::os::raw::c_int>,
    pub foo_union_with_anon_struct_bitfield_h_unnamed_1: __BindgenUnionField<Struct_foo_union_with_anon_struct_bitfield_h_unnamed_1>,
    pub _bindgen_data_: u32,
}
impl Union_foo {
    pub unsafe fn a(&mut self) -> *mut ::std::os::raw::c_int {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn foo_union_with_anon_struct_bitfield_h_unnamed_1(&mut self)
     -> *mut Struct_foo_union_with_anon_struct_bitfield_h_unnamed_1 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_foo {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Union_foo() {
    assert_eq!(::std::mem::size_of::<Union_foo>() , 4usize);
    assert_eq!(::std::mem::align_of::<Union_foo>() , 4usize);
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Struct_foo_union_with_anon_struct_bitfield_h_unnamed_1 {
    pub _bitfield_1: ::std::os::raw::c_int,
}
impl Struct_foo_union_with_anon_struct_bitfield_h_unnamed_1 {
    #[inline]
    pub fn b(&self) -> ::std::os::raw::c_int {
        (self._bitfield_1 & (127usize as ::std::os::raw::c_int)) >> 0usize
    }
    #[inline]
    pub fn set_b(&mut self, val: u8) {
        self._bitfield_1 &= !(127usize as ::std::os::raw::c_int);
        self._bitfield_1 |=
            ((val as ::std::os::raw::c_int) << 0usize) &
                (127usize as ::std::os::raw::c_int);
    }
    #[inline]
    pub fn c(&self) -> ::std::os::raw::c_int {
        (self._bitfield_1 & (4294967168usize as ::std::os::raw::c_int)) >>
            7usize
    }
    #[inline]
    pub fn set_c(&mut self, val: u32) {
        self._bitfield_1 &= !(4294967168usize as ::std::os::raw::c_int);
        self._bitfield_1 |=
            ((val as ::std::os::raw::c_int) << 7usize) &
                (4294967168usize as ::std::os::raw::c_int);
    }
    #[inline]
    pub fn new_bitfield_1(b: u8, c: u32) -> ::std::os::raw::c_int {
        0 | ((b as ::std::os::raw::c_int) << 0u32) |
            ((c as ::std::os::raw::c_int) << 7u32)
    }
}
impl ::std::clone::Clone for
 Struct_foo_union_with_anon_struct_bitfield_h_unnamed_1 {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Struct_foo_union_with_anon_struct_bitfield_h_unnamed_1() {
    assert_eq!(::std::mem::size_of::<Struct_foo_union_with_anon_struct_bitfield_h_unnamed_1>()
               , 4usize);
    assert_eq!(::std::mem::align_of::<Struct_foo_union_with_anon_struct_bitfield_h_unnamed_1>()
               , 4usize);
}
