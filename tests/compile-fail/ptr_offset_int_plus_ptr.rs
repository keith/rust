// error-pattern: invalid use of 1 as a pointer

fn main() {
    let ptr = Box::into_raw(Box::new(0u32));
    // Can't start with an integer pointer and get to something usable
    unsafe {
        let _val = (1 as *mut u8).offset(ptr as isize);
    }
}
