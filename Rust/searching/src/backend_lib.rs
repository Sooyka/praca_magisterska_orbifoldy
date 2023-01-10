use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

// #[macro_export]
// macro_rules! no_ex {
//     (i64) => {
//         searching::mathematics_lib::ExWh
//     };
//     (Ratio<i64>) => {
//         searching::mathematics_lib::ExRa
//     };
// }

// #[macro_export]
// macro_rules! fu_ty {
//     (i64) => {
//         i64
//     };
//     (Ratio<i64>) => {
//         use num_rational::Ratio<i64>
//     };
// }

// #[macro_export]
// macro_rules! ty_ref {
//     (i64, $exp1:expr, $exp2:expr) => {
//         ($exp1, $exp2)
//     };
//     (Ratio<i64>, $exp1:expr, $exp2:expr) => {
//         (&($exp1), &($exp2))
//     };
// }

// #[macro_export]
// macro_rules! ch_op {
//     (+) => {
//         checked_add
//     };
//     (-) => {
//         checked_sub
//     };
//     (*) => {
//         checked_mul
//     };
//     (/) => {
//         checked_div
//     };
// }

// #[macro_export]
// macro_rules! ch_re {
//     ($typ:ty, $exp1:expr, $op:tt, $exp2:expr) => {
//         match <fu_ty!($typ)>:: ch_op!($tt) ty_ref!($typ, $exp1, $exp2) {
//             Some(exp3) => exp3,
//             None => return <no_ex!($typ)>::Overflow,
//         }
//     };
// }

pub fn read_config<C: serde::de::DeserializeOwned>(lib: String) -> Result<C, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = match File::open(Path::new(&("config/".to_string() + &lib + ".json"))) {
        Ok(f) => f,
        Err(_) => {
            // println!("{}", err.to_string());
            File::open(Path::new(&(lib + ".json")))?
        }
    };
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as a config
    let u = serde_json::from_reader(reader)?;

    // Return the config.
    Ok(u)
}
#[derive(Serialize, Deserialize)]
pub enum Config {
    A,
}
