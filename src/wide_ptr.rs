//! Yet another wide pointer.

use std::fmt::Debug;
use std::marker::PhantomData;

#[repr(C)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct WidePointer {
    pub ptr: usize,
    pub trivia: usize,
    _phantom: PhantomData<*const ()>
}

impl WidePointer {
    pub const fn new(ptr: usize, trivia: usize) -> Self {
        Self {
            ptr,
            trivia,
            _phantom: PhantomData
        }
    }
}

impl Debug for WidePointer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WidePointer(0x{:X}, 0x{:X})", self.ptr, self.trivia)
    }
}

impl Default for WidePointer {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

#[cfg(test)]
mod test {
    use crate::wide_ptr::WidePointer;

    #[test]
    fn test_wide_pointer_size() {
        trait UselessTrait {}

        assert_eq!(std::mem::size_of::<WidePointer>(),
                   std::mem::size_of::<*mut dyn UselessTrait>());
        assert_eq!(std::mem::align_of::<WidePointer>(),
                   std::mem::align_of::<*mut dyn UselessTrait>());
    }

    #[test]
    fn test_wide_pointer_layout() {
        trait UselessTrait {}
        struct MeinStrukt();

        impl UselessTrait for MeinStrukt {}

        let s = MeinStrukt();
        let ptr: *const MeinStrukt = &s as *const MeinStrukt;
        let wide_ptr: *const dyn UselessTrait = &s as &dyn UselessTrait as *const dyn UselessTrait;
        let wide_ptr: WidePointer = unsafe { std::mem::transmute::<>(wide_ptr) };

        assert_eq!(wide_ptr.ptr, ptr as usize);
    }

    #[test]
    fn test_wide_pointer_layout2() {
        let slice: &[i32; 4] = &[114, 514, 1919, 810];
        let ptr: *const i32 = &slice[0] as *const i32;
        let wide_ptr: *const [i32] = slice as *const [i32];
        let wide_ptr: WidePointer = unsafe { std::mem::transmute::<>(wide_ptr) };

        assert_eq!(wide_ptr.ptr, ptr as usize);
        assert_eq!(wide_ptr.trivia, 4);
    }
}
