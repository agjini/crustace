use hello_macro::HelloMacro;
use hello_macro::hello_macro_derive::{measure, sql};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Blinis {
    #[serde(rename = "nnn")]
    name: String,
}

#[measure]
fn hellop();

fn main() {
    //Blinis::hello_macro();

    let b = Blinis {
        name: "Tapenade".to_string(),
    };
    println!("TOTO \n{}", serde_yaml::to_string(&b).unwrap());

    let sql = sql!(SELECT 1 FROM toto);
    println!("{}", sql)
}
