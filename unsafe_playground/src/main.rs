use std::slice;

unsafe extern "C" {
    fn abs(input: i32) -> u32;
}

#[unsafe(no_mangle)]
pub extern "C" fn augustin() {
    println!("Just called a Rust function from C!");
}

#[link(name = "m")] // libm.so → "m" (sans le préfixe lib ni l'extension)
unsafe extern "C" {
    fn cos(x: f64) -> f64;
}

static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("value is: {HELLO_WORLD}");
    let c = unsafe { cos(0.0) };
    // println!("Absolute value of -3 according to C: {}", unsafe {
    //     let s = String::from("salut");
    //     let s2 = s;
    //
    //     println!("s2 is {}", s);
    //
    //     abs(-3)
    // });
    //
    // split_at_mut(&mut [1, 2, 3, 4, 5], 3);
    //
    // let address = 0x01234usize;
    // let r = address as *mut i32;
    //
    // let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
    //
    // let address = 0x01234 + 120;
    // let address = address as *const i32;

    let address = get_raw_ptr() as usize + 8;
    let address = address as *const u8;
    println!("value @ {:p} : {}", address, unsafe { *address });
    let mut_address = address as *mut usize;
    unsafe {
        *mut_address = 0;
    }
    println!("value @ {:p} : {}", address, unsafe { *address });

    // let address = &value;
    // println!("value @ {:p} : {}", address, *address);

    // println!("values: {:?}", values);
}

fn get_raw_ptr() -> *const u8 {
    let mut value = String::from(
        "abcdefghijklmsssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssss",
    );
    value.push_str("dffffffffffffffffffffffffffffffffffffff");
    value.as_ptr()
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
