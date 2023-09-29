#![allow(dead_code)]

use std::alloc::{alloc_zeroed, realloc, Layout};
use std::ops::{Index, IndexMut};

#[cfg(target_os = "linux")]
#[repr(C)]
pub struct CxxString {
  pub ptr: *mut u8,
  pub len: usize,
  pub sso: CxxSSO,
}

#[cfg(target_os = "linux")]
#[repr(C)]
pub union CxxSSO {
  pub capa: usize,
  pub buf: [u8; 16],
}

#[cfg(target_os = "linux")]
impl CxxString {
  pub unsafe fn new<T>(ptr: *mut T, size: usize) -> Self {
    if size >= 16 {
      return Self {
        ptr: ptr as *mut u8,
        len: size,
        sso: CxxSSO { capa: size },
      };
    }
    let array_ptr: *const [u8; 16] = ptr as *const [u8; 16];
    Self {
      ptr: ptr as *mut u8,
      len: size,
      sso: CxxSSO {
        buf: std::mem::transmute(*array_ptr),
      },
    }
  }

  // TODO: maybe wrong, not tested
  pub unsafe fn resize(&mut self, size: usize) {
    if size > self.len {
      match (size >= 16, self.len) {
        (true, v) if v < 16 => {
          let new_array = alloc_zeroed(Layout::array::<u8>(32).unwrap());
          std::ptr::copy_nonoverlapping(self.sso.buf.as_ptr(), new_array, 16);
          self.ptr = new_array;
          self.sso.capa = 32;
        }
        (true, v) if v >= 16 && size > self.sso.capa => {
          self.ptr = realloc(
            self.ptr,
            Layout::array::<u8>(self.sso.capa).unwrap(),
            self.sso.capa + 16,
          );
          self.sso.capa += 16;
        }
        (_, _) => (),
      }
    } else {
      match (size >= 16, self.len) {
        (true, _) => {
          let target = self.ptr as usize + size;
          let slice = std::slice::from_raw_parts_mut(target as *mut u8, 1);
          slice[0] = 0;
        }
        (false, v) if v >= 16 => {
          std::ptr::copy_nonoverlapping(self.ptr, self.sso.buf.as_mut_ptr(), 16);
        }
        (_, _) => {}
      }
    }
    self.len = size;
  }

  pub unsafe fn from_ptr(ptr: *const u8) -> &'static mut Self {
    std::mem::transmute(ptr)
  }

  pub unsafe fn as_ptr(&mut self) -> *const u8 {
    std::mem::transmute(self)
  }

  pub unsafe fn as_mut_ptr(&mut self) -> *mut u8 {
    std::mem::transmute(self)
  }

  pub unsafe fn to_str(&mut self) -> Result<&'static str, Box<dyn std::error::Error>> {
    match std::ffi::CStr::from_bytes_with_nul(std::slice::from_raw_parts(self.ptr, self.len + 1)) {
      Ok(value) => match value.to_str() {
        Ok(value) => Ok(value),
        Err(err) => Err(err.into()),
      },
      Err(err) => Err(err.into()),
    }
  }

  pub fn size(&self) -> usize {
    self.len
  }

  pub unsafe fn pop_back(&mut self) {
    let index = self.len;
    self[index] = 0;
    self.resize(self.len - 1);
  }

  pub unsafe fn push_back(&mut self, symbol: u8) {
    let index = self.len;
    self.resize(index + 1);
    self[index] = symbol;
  }
}

#[cfg(target_os = "linux")]
impl Index<usize> for CxxString {
  type Output = u8;

  fn index(&self, index: usize) -> &Self::Output {
    unsafe {
      let mut data: *const u8 = self.sso.buf.as_ptr();
      if self.len >= 16 {
        data = self.ptr;
      }
      let target = data as usize + index;
      let slice = std::slice::from_raw_parts(target as *const u8, 1);
      &slice[0]
    }
  }
}

#[cfg(target_os = "linux")]
impl IndexMut<usize> for CxxString {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    unsafe {
      let mut data: *mut u8 = self.sso.buf.as_mut_ptr();
      if self.len >= 16 {
        data = self.ptr;
      }
      let target = data as usize + index;
      let slice = std::slice::from_raw_parts_mut(target as *mut u8, 1);
      &mut slice[0]
    }
  }
}

#[cfg(target_os = "windows")]
#[repr(C)]
pub struct CxxString {
  pub data: CxxStringContent,
  pub len: usize,
  pub capa: usize,
}

#[cfg(target_os = "windows")]
#[repr(C)]
pub union CxxStringContent {
  pub buf: [u8; 16],
  pub ptr: *mut u8,
}

#[cfg(target_os = "windows")]
impl CxxString {
  pub unsafe fn new<T>(ptr: *mut T, size: usize) -> Self {
    if size >= 16 {
      return Self {
        data: CxxStringContent { ptr: ptr as *mut u8 },
        len: size,
        capa: size,
      };
    }
    let array_ptr: *const [u8; 16] = ptr as *const [u8; 16];
    Self {
      data: CxxStringContent {
        buf: std::mem::transmute(*array_ptr),
      },
      len: size,
      capa: 15,
    }
  }

  pub unsafe fn resize(&mut self, size: usize) {
    if size > self.len {
      match (size >= 16, self.capa) {
        (true, v) if v == 15 => {
          let new_array = alloc_zeroed(Layout::array::<u8>(32).unwrap());
          std::ptr::copy_nonoverlapping(self.data.buf.as_ptr(), new_array, 16);
          self.data.ptr = new_array;
          self.capa = 32;
        }
        (true, v) if v < size => {
          self.data.ptr = realloc(self.data.ptr, Layout::array::<u8>(self.capa).unwrap(), self.capa + 16);
          self.capa += 16;
        }
        (_, _) => (),
      }
    } else {
      if self.capa >= 16 {
        let target = self.data.ptr as usize + size;
        let slice = std::slice::from_raw_parts_mut(target as *mut u8, 1);
        slice[0] = 0;
      } else {
        self.data.buf[size] = 0;
      }
    }
    self.len = size;
  }

  pub unsafe fn from_ptr(ptr: *const u8) -> &'static mut Self {
    std::mem::transmute(ptr)
  }

  pub unsafe fn to_str(&mut self) -> Result<&'static str, Box<dyn std::error::Error>> {
    let mut data: *const u8 = self.data.buf.as_ptr();
    if self.capa >= 16 {
      data = self.data.ptr;
    }
    match std::ffi::CStr::from_bytes_with_nul(std::slice::from_raw_parts(data, self.len + 1)) {
      Ok(value) => match value.to_str() {
        Ok(value) => Ok(value),
        Err(err) => Err(err.into()),
      },
      Err(err) => Err(err.into()),
    }
  }

  pub unsafe fn as_ptr(&mut self) -> *const u8 {
    std::mem::transmute(self)
  }

  pub unsafe fn as_mut_ptr(&mut self) -> *mut u8 {
    std::mem::transmute(self)
  }

  pub fn size(&self) -> usize {
    self.len
  }

  pub unsafe fn pop_back(&mut self) {
    let index = self.len;
    self[index] = 0;
    self.resize(self.len - 1);
  }

  pub unsafe fn push_back(&mut self, symbol: u8) {
    let index = self.len;
    self.resize(index + 1);
    self[index] = symbol;
  }
}

#[cfg(target_os = "windows")]
impl Index<usize> for CxxString {
  type Output = u8;

  fn index(&self, index: usize) -> &Self::Output {
    unsafe {
      let mut data: *const u8 = self.data.buf.as_ptr();
      if self.capa >= 16 {
        data = self.data.ptr;
      }
      let target = data as usize + index;
      let slice = std::slice::from_raw_parts(target as *const u8, 1);
      &slice[0]
    }
  }
}

#[cfg(target_os = "windows")]
impl IndexMut<usize> for CxxString {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    unsafe {
      let mut data: *mut u8 = self.data.buf.as_mut_ptr();
      if self.capa >= 16 {
        data = self.data.ptr;
      }
      let target = data as usize + index;
      let slice = std::slice::from_raw_parts_mut(target as *mut u8, 1);
      &mut slice[0]
    }
  }
}
