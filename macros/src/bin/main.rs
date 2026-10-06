use std::collections::HashMap;

#[macro_export]
macro_rules! vvec {
    // ( $( $x:expr ),* ) => {
    //     {
    //         let mut temp_vec = Vec::new();
    //         $(
    //             temp_vec.push($x);
    //         )*
    //         temp_vec
    //     }
    // };
    ($y:ty) => {{
        let temp_vec: HashMap<i32, $y> = HashMap::new();
        temp_vec
    }}; //
        // ( $( $x:literal : $y:ty ),* ) => {
        //     {
        //         let mut temp_vec = HashMap::new();
        //         $(
        //             temp_vec.insert($x, $y);
        //         )*
        //         temp_vec
        //     }
        // };
}

#[derive(Debug)]
struct Bibiche(String);

fn main() {
    println!("Hello, world!");

    // poubelle();

    let bibi = "3";

    let bibiche = Bibiche("jeanne".to_string());

    let mut v = vvec!(Bibiche);

    v.insert(43, bibiche);

    // let vv = vvec!(
    //     "44": 676
    // );

    dbg!(v);
    //
    // println!("taille : {}", size_of::<Vec<()>>());
    //println!("taille : {}", size_of_val(&v));
    //println!("taille : {}", v.capacity() * size_of::<()>());

    //fouasse();
}

fn poubelle() -> ! {
    loop {
        continue;
        let x = 0;
    }
}

fn pipotage() -> &'static str {
    "1"
}

// fn fouasse() {
//     let v2 = {
//         let mut temp_vec = Vec::new();
//         temp_vec.push(1);
//         temp_vec.push(2);
//         temp_vec.push(3);
//         temp_vec.push(4);
//         temp_vec.push("5");
//         temp_vec
//     };
// }
