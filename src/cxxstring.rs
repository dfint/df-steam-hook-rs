#[repr(C)]
pub struct CxxString {
  data: CxxStringContent,
  pub len: usize,
  pub capa: usize,
}

union CxxStringContent {
  buf: [u8; 16],
  ptr: *mut u8,
}

impl CxxString {
  pub unsafe fn new(bytes: *mut u8, size: usize) -> Self {
    if size >= 16 {
      return Self {
        data: CxxStringContent { ptr: bytes },
        len: size,
        capa: size,
      };
    }
    let array_ptr: *const [u8; 16] = bytes as *const [u8; 16];
    Self {
      data: CxxStringContent {
        buf: std::mem::transmute(*array_ptr),
      },
      len: size,
      capa: 15,
    }
  }

  pub unsafe fn to_str(&mut self) -> Result<&'static str, Box<dyn std::error::Error>> {
    let mut data: *const u8 = self.data.buf.as_ptr();
    if self.capa >= 16 {
      data = self.data.ptr;
    }
    match std::ffi::CStr::from_bytes_with_nul(std::slice::from_raw_parts(data, self.len + 1)) {
      Ok(value) => match value.to_str() {
        Ok(value) => Ok(value),
        Err(_err) => Err("fail to form string".into()),
      },
      Err(_err) => Err("fail to form string".into()),
    }
  }

  pub unsafe fn as_ptr(&mut self) -> *const u8 {
    std::mem::transmute(self)
  }
}
