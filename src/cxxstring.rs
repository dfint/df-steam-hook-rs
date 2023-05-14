use std::alloc::{realloc, Layout};
use std::ops::{Index, IndexMut};

#[repr(C)]
pub struct CxxString {
  pub data: CxxStringContent,
  pub len: usize,
  pub capa: usize,
}

#[repr(C)]
pub union CxxStringContent {
  pub buf: [u8; 16],
  pub ptr: *mut u8,
}

impl CxxString {
  pub unsafe fn new(ptr: *mut u8, size: usize) -> Self {
    if size >= 16 {
      return Self {
        data: CxxStringContent { ptr },
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
      self.len = size;
      if size >= 16 {
        if self.capa < 16 {
          log::trace!("{:?}", self.data.buf);
          self.data.ptr = realloc(self.data.buf.as_mut_ptr(), Layout::new::<u8>(), size + 16);
          self.capa = size + 16;
          self.capa = size + 16;
        } else if self.capa < size {
          let mut v = Vec::<u8>::from_raw_parts(self.data.ptr, self.capa, size + 32);
          self.data.ptr = v.as_mut_ptr();
          self.capa = size + 32;
        }
      }
    } else {
      self.len = size;
      if size >= 16 {
        self.capa = size;
        let target = self.data.ptr as usize + size;
        let slice = std::slice::from_raw_parts_mut(target as *mut u8, 1);
        slice[0] = 0;
      } else {
        self.data.buf[size] = 0;
      }
    }
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
}

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
