use std::alloc::{dealloc, Layout};

#[repr(C)]
pub struct CxxSet<T> {
  pub head: *mut CxxSetNode<T>,
  pub size: usize,
}

#[repr(C)]
pub struct CxxSetNode<T> {
  pub left: *mut CxxSetNode<T>,
  pub parent: *mut CxxSetNode<T>,
  pub right: *mut CxxSetNode<T>,
  pub color: bool,
  pub is_nil: bool,
  pub key: T,
}

impl<T: std::cmp::PartialOrd + std::fmt::Display> CxxSet<T> {
  pub unsafe fn from_ptr(ptr: *const u8) -> &'static mut Self {
    std::mem::transmute(ptr)
  }

  pub unsafe fn contains(&self, key: T) -> bool {
    let mut current = &*(&*self.head).parent;
    while !current.is_nil {
      if current.key == key {
        return true;
      }
      current = match current.key > key {
        true => &*current.left,
        false => &*current.right,
      };
    }
    false
  }

  pub unsafe fn clear(&mut self) {
    let current = (&*self.head).parent;
    self.dealloc_subtree(current);
    (&mut *self.head).parent = self.head;
    (&mut *self.head).left = self.head;
    (&mut *self.head).right = self.head;
    self.size = 0;
  }

  unsafe fn dealloc_subtree(&mut self, node: *mut CxxSetNode<T>) {
    let mut current = node;
    while !(&*current).is_nil {
      self.dealloc_subtree((&*current).right);
      let to_destroy = current;
      current = (&*current).left;
      dealloc(to_destroy as *mut u8, Layout::new::<CxxSetNode<T>>());
    }
  }
}
