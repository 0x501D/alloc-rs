use libloading::{Library, Symbol};
use std::alloc::{alloc, dealloc, Layout};

pub unsafe fn alloc_cb(n: usize, align: usize) -> *mut u8 {
    println!("allocate {} bytes", n);
    let layout = Layout::from_size_align_unchecked(n, align);
    let ptr = alloc(layout);

    ptr
}

pub unsafe fn free_cb(ptr: *mut u8, n: usize, align: usize) {
    println!("free {} bytes", n);
    let layout = Layout::from_size_align_unchecked(n, align);
    dealloc(ptr, layout);
}

#[repr(C)]
pub struct Demo {
    alloc: unsafe fn(usize, usize) -> *mut u8,
    free: unsafe fn(*mut u8, usize, usize),
}

fn main() {
    unsafe {
        let mut demo = Demo {
            alloc: alloc_cb,
            free: free_cb,
        };
        let lib = Library::new("./c/demo.so").unwrap();
        let api: Symbol<unsafe extern "C" fn(*mut Demo)> = lib.get(b"demo").unwrap();

        api(&mut demo as *mut Demo);
    }
}
