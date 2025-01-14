use crate::prelude::*;
use skia_bindings::{
    C_SkDataTable_MakeCopyArray, C_SkDataTable_MakeCopyArrays, C_SkDataTable_MakeEmpty,
    SkDataTable, SkRefCntBase,
};
use std::convert::TryInto;
use std::ffi::{c_void, CStr};
use std::ops::Index;
use std::{mem, slice};

pub type DataTable = RCHandle<SkDataTable>;
unsafe impl Send for DataTable {}

impl NativeRefCountedBase for SkDataTable {
    type Base = SkRefCntBase;
    fn ref_counted_base(&self) -> &Self::Base {
        &self._base._base
    }
}

impl Index<usize> for DataTable {
    type Output = [u8];

    fn index(&self, index: usize) -> &Self::Output {
        self.at(index)
    }
}

impl RCHandle<SkDataTable> {
    pub fn is_empty(&self) -> bool {
        // does not link:
        // unsafe { self.native().isEmpty() }
        self.count() == 0
    }

    pub fn count(&self) -> usize {
        unsafe { self.native().count().try_into().unwrap() }
    }

    pub fn at_size(&self, index: usize) -> usize {
        assert!(index < self.count());
        unsafe { self.native().atSize(index.try_into().unwrap()) }
    }

    pub fn at(&self, index: usize) -> &[u8] {
        unsafe { self.at_t(index) }
    }

    pub unsafe fn at_t<T: Copy>(&self, index: usize) -> &[T] {
        assert!(index < self.count());
        let mut size = usize::default();
        let ptr = self.native().at(index.try_into().unwrap(), &mut size);
        let element_size = mem::size_of::<T>();
        assert_eq!(size % element_size, 0);
        let elements = size / element_size;
        slice::from_raw_parts(ptr as _, elements)
    }

    pub fn at_str(&self, index: usize) -> &CStr {
        let bytes = self.at(index);
        CStr::from_bytes_with_nul(bytes).unwrap()
    }

    pub fn new_empty() -> Self {
        DataTable::from_ptr(unsafe { C_SkDataTable_MakeEmpty() }).unwrap()
    }

    pub fn from_slices(slices: &[&[u8]]) -> Self {
        let ptrs: Vec<*const c_void> = slices.iter().map(|s| s.as_ptr() as _).collect();
        let sizes: Vec<usize> = slices.iter().map(|s| s.len()).collect();
        unsafe {
            DataTable::from_ptr(C_SkDataTable_MakeCopyArrays(
                ptrs.as_ptr(),
                sizes.as_ptr(),
                slices.len().try_into().unwrap(),
            ))
            .unwrap()
        }
    }

    pub fn from_slice<T: Copy>(slice: &[T]) -> Self {
        unsafe {
            DataTable::from_ptr(C_SkDataTable_MakeCopyArray(
                slice.as_ptr() as _,
                mem::size_of::<T>(),
                slice.len().try_into().unwrap(),
            ))
            .unwrap()
        }
    }

    // TODO: wrap MakeArrayProc()
}

impl RCHandle<SkDataTable> {
    pub fn iter(&self) -> Iter {
        Iter {
            table: self,
            count: self.count(),
            current: 0,
        }
    }
}

pub struct Iter<'a> {
    table: &'a DataTable,
    count: usize,
    current: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.count {
            let r = Some(self.table.at(self.current));
            self.current += 1;
            r
        } else {
            None
        }
    }
}
