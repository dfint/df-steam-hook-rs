pub struct CxString {
  data: CxBuf,
  len: usize,
  capa: usize,
  pad: usize,
}

union CxBuf {
  buf: [u8; 16],
  ptr: *mut u8,
}

impl CxString {
  pub unsafe fn to_str(&mut self) -> String {
    let mut data: *const u8 = self.data.buf.as_ptr();
    if self.capa >= 16 {
      data = self.data.ptr;
    }

    match CStr::from_bytes_with_nul(slice::from_raw_parts(data, self.len + 1)) {
      Ok(value) => match value.to_str() {
        Ok(value) => String::from(value),
        Err(err) => err.to_string(),
      },
      Err(err) => err.to_string(),
    }
  }

  pub unsafe fn write_byte(&mut self, bytes: *const u8, size: usize) {
    self.len = size;
    if size >= 16 {
      self.capa = size;
      std::ptr::copy(bytes, self.data.ptr, size);
    } else {
      let mut a = [0; 16];
      std::ptr::copy(bytes, a.as_mut_ptr(), size);
      std::ptr::copy(a.as_ptr(), self.data.buf.as_mut_ptr(), size);
      self.capa = 15;
    }
  }

  pub unsafe fn as_ptr(&mut self) -> *const u8 {
    std::mem::transmute(self)
  }
}
