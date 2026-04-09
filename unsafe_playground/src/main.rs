fn main() {
    let mut array_original: [i32; 6] = [32, 34, 468, 1, 2, 3];
    // let mut array: [i32; 6] = 0x7fff92065f08;

    let pointer: *mut i32 = 0x7fff92065f08 as *mut i32;

    println!("Addresse du pointeur en mémoire {:?}", pointer);
    println!("Valeur présente à cette addresse '{:?}'", unsafe {
        *pointer
    });

    unsafe {
        *pointer = 666;
    }

    println!("Addresse du pointeur en mémoire {:?}", pointer);
    println!("Valeur présente à cette addresse '{:?}'", unsafe {
        *pointer
    });
}
