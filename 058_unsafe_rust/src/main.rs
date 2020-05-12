use std::slice;

fn main() {
    deref_raw_pointers();

    unsafe {
        dangerous();
    }

    wrap_unsafe_with_safe_abstraction();
    undefined_behavior_slicing_arbitrary_memory();
    call_external_function();
    global_variable_manipulations();
}

fn deref_raw_pointers() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

unsafe fn dangerous() {
}

fn wrap_unsafe_with_safe_abstraction() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn undefined_behavior_slicing_arbitrary_memory() {
    let address = 0x01234usize;
    let r = address as *mut i32;

    let _slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn call_external_function() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn global_variable_manipulations() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}