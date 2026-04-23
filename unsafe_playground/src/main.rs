use std::slice;

unsafe extern "C" {
    fn abs(input: i32) -> bool;
}

fn main() {
    unsafe {
        println!("Absolute value of -1 according to C: {}", abs(-1));
    }

    split_at_mut(&mut [1, 2, 3, 4, 5], 3);

    let address = 0x01234usize;
    let r = address as *mut i32;

    let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };

    // println!("values: {:?}", values);
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        let toto = slice::from_raw_parts_mut(ptr, mid);
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
