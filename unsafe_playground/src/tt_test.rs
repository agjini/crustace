use std::fmt::Display;

struct Toto {
    x: i32,
    y: i32,
    rr: *const i32,
}

unsafe impl Send for Toto {}

union MyUnion {
    f1: u32,
    f2: f32,
}

union IntOrFloat {
    i: u32,
    f: f32,
}

fn main2() {
    let mut u = IntOrFloat { i: 45 };
    // Reading the fields of a union is always unsafe
    assert_eq!(unsafe { u.i }, 1065353216);
    // Updating through any of the field will modify all of them
    u.i = 1073741824;
    assert_eq!(unsafe { u.f }, 2.0);

    let i = 156;
    let toto = Toto {
        x: 0,
        y: 0,
        rr: i as *const i32,
    };
    ff(toto);
    //print_line(toto);
}

fn ff<T: Send>(toto: T) {
    todo!()
}

fn print_line<T: Display>(toto: T) {
    todo!()
}

impl Display for Toto {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "TOTO")
    }
}
