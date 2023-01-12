use crate::backend_lib::ExWhRa64::*;
use num_rational::*;
use num_traits::ops::checked::*;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub const ZERO: Rational64 = Rational64::new_raw(0, 1);
pub const ONE_OVER_FOUR: Rational64 = Rational64::new_raw(1, 4);
pub const ONE_OVER_TWO: Rational64 = Rational64::new_raw(1, 2);
pub const ONE: Rational64 = Rational64::new_raw(1, 1);
pub const TWO: Rational64 = Rational64::new_raw(2, 1);
pub const THREE: Rational64 = Rational64::new_raw(3, 1);
pub const FOUR: Rational64 = Rational64::new_raw(4, 1);

// use crate::backend_lib::ExWh::*;
// use crate::backend_lib::ExRa::*;

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

// fn r_checked_add(a:&Ratio<i64> , b:&Ratio<i64> ) -> Result<Ratio<i64>, ExWhRa64>{
//     match <Ratio<i64>>::checked_add(a, b){
//         Some(a_b) => Ok(a_b),
//         None => Err(Ov),
//     }
// }

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Clone, Copy)]
pub enum ExWh {
    MInfty,
    Whole(i64),
    Overflow,
    PInfty,
}

impl std::fmt::Display for ExWh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExWh::MInfty => write!(f, "-♾️"),
            ExWh::Whole(n) => write!(f, "{n}"),
            ExWh::Overflow => write!(f, "Overflow"),
            ExWh::PInfty => write!(f, "♾️"),
        }
    }
}

impl std::ops::Add for ExWh {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (ExWh::MInfty, ExWh::PInfty) => panic!("Adding ♾️ to -♾️ is ill defined!"),
            (ExWh::PInfty, ExWh::MInfty) => panic!("Adding -♾️ to ♾️ is ill defined!"),
            (ExWh::MInfty, _) => ExWh::MInfty,
            (_, ExWh::MInfty) => ExWh::MInfty,
            (ExWh::PInfty, _) => ExWh::PInfty,
            (_, ExWh::PInfty) => ExWh::PInfty,
            (ExWh::Overflow, _) => ExWh::Overflow,
            (_, ExWh::Overflow) => ExWh::Overflow,
            (ExWh::Whole(n1), ExWh::Whole(n2)) => {
                let n3 = match i64::checked_add(n1, n2) {
                    Some(n_t) => n_t,
                    None => return ExWh::Overflow,
                };
                ExWh::Whole(n3)
            }
        }
    }
}

impl std::ops::Sub for ExWh {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (ExWh::MInfty, ExWh::MInfty) => panic!("Substracting -♾️ from -♾️ is ill defined!"),
            (ExWh::PInfty, ExWh::PInfty) => panic!("Substracting ♾️ from -♾️ is ill defined!"),
            (ExWh::MInfty, _) => ExWh::MInfty,
            (_, ExWh::MInfty) => ExWh::PInfty,
            (ExWh::PInfty, _) => ExWh::PInfty,
            (_, ExWh::PInfty) => ExWh::MInfty,
            (ExWh::Overflow, _) => ExWh::Overflow,
            (_, ExWh::Overflow) => ExWh::Overflow,
            (ExWh::Whole(n1), ExWh::Whole(n2)) => {
                let n3 = match i64::checked_sub(n1, n2) {
                    Some(n_t) => n_t,
                    None => return ExWh::Overflow,
                };
                ExWh::Whole(n3)
            }
        }
    }
}

impl std::ops::Mul for ExWh {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (ExWh::Whole(0), ExWh::MInfty) => panic!("Multiplying 0 by -♾️ is undefined!"),
            (ExWh::MInfty, ExWh::Whole(0)) => panic!("Multiplying -♾️ by an 0 is undefined!"),
            (ExWh::Whole(0), ExWh::PInfty) => panic!("Multiplying 0 by ♾️ is undefined!"),
            (ExWh::PInfty, ExWh::Whole(0)) => panic!("Multiplying ♾️ by an 0 is undefined!"),
            (ExWh::Whole(0), _) => ExWh::Whole(0),
            (_, ExWh::Whole(0)) => ExWh::Whole(0),
            (ExWh::MInfty, ExWh::PInfty) => ExWh::MInfty,
            (ExWh::PInfty, ExWh::MInfty) => ExWh::MInfty,
            (ExWh::PInfty, ExWh::PInfty) => ExWh::PInfty,
            (ExWh::MInfty, ExWh::MInfty) => ExWh::PInfty,
            (ExWh::MInfty, ExWh::Overflow) => {
                panic!("Sign of multiplying -♾️ by an overflow is undefined!")
            }
            (ExWh::Overflow, ExWh::MInfty) => {
                panic!("Sign of multiplying an overflow by -♾️ is undefined!")
            }
            (ExWh::PInfty, ExWh::Overflow) => {
                panic!("Sign of multiplying ♾️ by an overflow is undefined!")
            }
            (ExWh::Overflow, ExWh::PInfty) => {
                panic!("Sign of multiplying an overflow by ♾️ is undefined!")
            }
            (ExWh::MInfty, ExWh::Whole(n)) => {
                if n > 0 {
                    ExWh::MInfty
                } else {
                    ExWh::PInfty
                }
            }
            (ExWh::Whole(n), ExWh::MInfty) => {
                if n > 0 {
                    ExWh::MInfty
                } else {
                    ExWh::PInfty
                }
            }
            (ExWh::PInfty, ExWh::Whole(n)) => {
                if n > 0 {
                    ExWh::PInfty
                } else {
                    ExWh::MInfty
                }
            }
            (ExWh::Whole(n), ExWh::PInfty) => {
                if n > 0 {
                    ExWh::PInfty
                } else {
                    ExWh::MInfty
                }
            }
            (ExWh::Overflow, _) => ExWh::Overflow,
            (_, ExWh::Overflow) => ExWh::Overflow,
            (ExWh::Whole(n1), ExWh::Whole(n2)) => {
                let n3 = match i64::checked_mul(n1, n2) {
                    Some(n_t) => n_t,
                    None => return ExWh::Overflow,
                };
                ExWh::Whole(n3)
            }
        }
    }
}

impl std::ops::Div for ExWh {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (ExWh::Whole(0), _) => ExWh::Whole(0),
            (ExWh::Whole(0), ExWh::Whole(0)) => panic!("Dividing 0 by 0 is undefined!"),
            (_, ExWh::Whole(0)) => panic!("Sign of dividing by 0 is undefined!"),
            (ExWh::MInfty, ExWh::PInfty) => panic!("Dividing -♾️ by ♾️ is undefined!"),
            (ExWh::PInfty, ExWh::MInfty) => panic!("Dividing ♾️ by -♾️ is undefined!"),
            (ExWh::MInfty, ExWh::Overflow) => {
                panic!("Sign of dividing -♾️ by an overflow is undefined!")
            }
            (ExWh::Overflow, ExWh::MInfty) => {
                panic!("Sign of dividing an overflow by -♾️ is undefined!")
            }
            (ExWh::PInfty, ExWh::Overflow) => {
                panic!("Sign of dividing ♾️ by an overflow is undefined!")
            }
            (ExWh::Overflow, ExWh::PInfty) => {
                panic!("Sign of dividing an overflow by ♾️ is undefined!")
            }
            (ExWh::MInfty, ExWh::Whole(n)) => {
                if n > 0 {
                    ExWh::MInfty
                } else {
                    ExWh::PInfty
                }
            }
            (ExWh::Whole(n), ExWh::MInfty) => {
                if n > 0 {
                    ExWh::MInfty
                } else {
                    ExWh::PInfty
                }
            }
            (ExWh::PInfty, ExWh::Whole(n)) => {
                if n > 0 {
                    ExWh::PInfty
                } else {
                    ExWh::MInfty
                }
            }
            (ExWh::Whole(n), ExWh::PInfty) => {
                if n > 0 {
                    ExWh::PInfty
                } else {
                    ExWh::MInfty
                }
            }
            (ExWh::Overflow, _) => ExWh::Overflow,
            (_, ExWh::Overflow) => ExWh::Overflow,
            (ExWh::Whole(n1), ExWh::Whole(n2)) => {
                let n3 = match i64::checked_div(n1, n2) {
                    Some(n_t) => n_t,
                    None => return ExWh::Overflow,
                };
                ExWh::Whole(n3)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ExRa {
    MInfty,
    Rational(Rational64),
    Overflow,
    PInfty,
}

impl std::fmt::Display for ExRa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExRa::MInfty => write!(f, "-♾️"),
            ExRa::Rational(pq) => write!(f, "{pq}"),
            ExRa::Overflow => write!(f, "Overflow"),
            ExRa::PInfty => write!(f, "♾️"),
        }
    }
}

impl std::ops::Add for ExRa {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (ExRa::MInfty, ExRa::PInfty) => panic!("Adding ♾️ to -♾️ is ill defined!"),
            (ExRa::PInfty, ExRa::MInfty) => panic!("Adding -♾️ to ♾️ is ill defined!"),
            (ExRa::MInfty, _) => ExRa::MInfty,
            (_, ExRa::MInfty) => ExRa::MInfty,
            (ExRa::PInfty, _) => ExRa::PInfty,
            (_, ExRa::PInfty) => ExRa::PInfty,
            (ExRa::Overflow, _) => ExRa::Overflow,
            (_, ExRa::Overflow) => ExRa::Overflow,
            (ExRa::Rational(pq1), ExRa::Rational(pq2)) => {
                let pq3 = match <Ratio<i64>>::checked_add(&pq1, &pq2) {
                    Some(pq_t) => pq_t,
                    None => return ExRa::Overflow,
                };
                ExRa::Rational(pq3)
            }
        }
    }
}

impl std::ops::Sub for ExRa {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (ExRa::MInfty, ExRa::MInfty) => panic!("Substracting -♾️ from -♾️ is ill defined!"),
            (ExRa::PInfty, ExRa::PInfty) => panic!("Substracting ♾️ from -♾️ is ill defined!"),
            (ExRa::MInfty, _) => ExRa::MInfty,
            (_, ExRa::MInfty) => ExRa::PInfty,
            (ExRa::PInfty, _) => ExRa::PInfty,
            (_, ExRa::PInfty) => ExRa::MInfty,
            (ExRa::Overflow, _) => ExRa::Overflow,
            (_, ExRa::Overflow) => ExRa::Overflow,
            (ExRa::Rational(pq1), ExRa::Rational(pq2)) => {
                let pq3 = match <Ratio<i64>>::checked_sub(&pq1, &pq2) {
                    Some(pq_t) => pq_t,
                    None => return ExRa::Overflow,
                };
                ExRa::Rational(pq3)
            }
        }
    }
}

impl std::ops::Mul for ExRa {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (ExRa::Rational(ZERO), ExRa::MInfty) => panic!("Multiplying ZERO by -♾️ is undefined!"),
            (ExRa::MInfty, ExRa::Rational(ZERO)) => panic!("Multiplying -♾️ by an ZERO is undefined!"),
            (ExRa::Rational(ZERO), ExRa::PInfty) => panic!("Multiplying ZERO by ♾️ is undefined!"),
            (ExRa::PInfty, ExRa::Rational(ZERO)) => panic!("Multiplying ♾️ by an ZERO is undefined!"),
            (ExRa::Rational(ZERO), _) => ExRa::Rational(ZERO),
            (_, ExRa::Rational(ZERO)) => ExRa::Rational(ZERO),
            (ExRa::MInfty, ExRa::PInfty) => ExRa::MInfty,
            (ExRa::PInfty, ExRa::MInfty) => ExRa::MInfty,
            (ExRa::PInfty, ExRa::PInfty) => ExRa::PInfty,
            (ExRa::MInfty, ExRa::MInfty) => ExRa::PInfty,
            (ExRa::MInfty, ExRa::Overflow) => {
                panic!("Sign of multiplying -♾️ by an overflow is undefined!")
            }
            (ExRa::Overflow, ExRa::MInfty) => {
                panic!("Sign of multiplying an overflow by -♾️ is undefined!")
            }
            (ExRa::PInfty, ExRa::Overflow) => {
                panic!("Sign of multiplying ♾️ by an overflow is undefined!")
            }
            (ExRa::Overflow, ExRa::PInfty) => {
                panic!("Sign of multiplying an overflow by ♾️ is undefined!")
            }
            (ExRa::MInfty, ExRa::Rational(pq)) => {
                if pq > ZERO {
                    ExRa::MInfty
                } else {
                    ExRa::PInfty
                }
            }
            (ExRa::Rational(pq), ExRa::MInfty) => {
                if pq > ZERO {
                    ExRa::MInfty
                } else {
                    ExRa::PInfty
                }
            }
            (ExRa::PInfty, ExRa::Rational(pq)) => {
                if pq > ZERO {
                    ExRa::PInfty
                } else {
                    ExRa::MInfty
                }
            }
            (ExRa::Rational(pq), ExRa::PInfty) => {
                if pq > ZERO {
                    ExRa::PInfty
                } else {
                    ExRa::MInfty
                }
            }
            (ExRa::Overflow, _) => ExRa::Overflow,
            (_, ExRa::Overflow) => ExRa::Overflow,
            (ExRa::Rational(pq1), ExRa::Rational(pq2)) => {
                let pq3 = match <Ratio<i64>>::checked_mul(&pq1, &pq2) {
                    Some(pq_t) => pq_t,
                    None => return ExRa::Overflow,
                };
                ExRa::Rational(pq3)
            }
        }
    }
}

impl std::ops::Div for ExRa {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (ExRa::Rational(ZERO), _) => ExRa::Rational(ZERO),
            (ExRa::Rational(ZERO), ExRa::Rational(ZERO)) => panic!("Dividing ZERO by ZERO is undefined!"),
            (_, ExRa::Rational(ZERO)) => panic!("Sign of dividing by ZERO is undefined!"),
            (ExRa::MInfty, ExRa::PInfty) => panic!("Dividing -♾️ by ♾️ is undefined!"),
            (ExRa::PInfty, ExRa::MInfty) => panic!("Dividing ♾️ by -♾️ is undefined!"),
            (ExRa::MInfty, ExRa::Overflow) => {
                panic!("Sign of dividing -♾️ by an overflow is undefined!")
            }
            (ExRa::Overflow, ExRa::MInfty) => {
                panic!("Sign of dividing an overflow by -♾️ is undefined!")
            }
            (ExRa::PInfty, ExRa::Overflow) => {
                panic!("Sign of dividing ♾️ by an overflow is undefined!")
            }
            (ExRa::Overflow, ExRa::PInfty) => {
                panic!("Sign of dividing an overflow by ♾️ is undefined!")
            }
            (ExRa::MInfty, ExRa::Rational(pq)) => {
                if pq > ZERO {
                    ExRa::MInfty
                } else {
                    ExRa::PInfty
                }
            }
            (ExRa::Rational(pq), ExRa::MInfty) => {
                if pq > ZERO {
                    ExRa::MInfty
                } else {
                    ExRa::PInfty
                }
            }
            (ExRa::PInfty, ExRa::Rational(pq)) => {
                if pq > ZERO {
                    ExRa::PInfty
                } else {
                    ExRa::MInfty
                }
            }
            (ExRa::Rational(pq), ExRa::PInfty) => {
                if pq > ZERO {
                    ExRa::PInfty
                } else {
                    ExRa::MInfty
                }
            }
            (ExRa::Overflow, _) => ExRa::Overflow,
            (_, ExRa::Overflow) => ExRa::Overflow,
            (ExRa::Rational(pq1), ExRa::Rational(pq2)) => {
                let pq3 = match <Ratio<i64>>::checked_div(&pq1, &pq2) {
                    Some(pq_t) => pq_t,
                    None => return ExRa::Overflow,
                };
                ExRa::Rational(pq3)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExCo64 {
    ex: Rational64,
    co: Rational64,
}

#[derive(Debug, Clone)]
pub enum ExWhRa64 {
    Ep(Vec<ExCo64>),
    Wh(i64),
    Ra(Rational64),
    Ov,
    Om(Vec<ExCo64>),
    Un,
}

impl std::ops::Add for ExWhRa64 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Ep(_), Ep(_)) => todo!(),
            (Ep(_), Wh(_)) => todo!(),
            (Ep(_), Ra(_)) => todo!(),
            (Ep(_), Ov) => todo!(),
            (Ep(_), Om(_)) => todo!(),
            (Wh(_), Ep(_)) => todo!(),
            (Wh(_), Wh(_)) => todo!(),
            (Wh(_), Ra(_)) => todo!(),
            (Wh(_), Ov) => todo!(),
            (Wh(_), Om(_)) => todo!(),
            (Ra(_), Ep(_)) => todo!(),
            (Ra(_), Wh(_)) => todo!(),
            (Ra(_), Ra(_)) => todo!(),
            (Ra(_), Ov) => todo!(),
            (Ra(_), Om(_)) => todo!(),
            (Ov, Ep(_)) => todo!(),
            (Ov, Wh(_)) => todo!(),
            (Ov, Ra(_)) => todo!(),
            (Ov, Ov) => todo!(),
            (Ov, Om(_)) => todo!(),
            (Om(_), Ep(_)) => todo!(),
            (Om(_), Wh(_)) => todo!(),
            (Om(_), Ra(_)) => todo!(),
            (Om(_), Ov) => todo!(),
            (Om(_), Om(_)) => todo!(),
            (Ep(_), Un) => todo!(),
            (Wh(_), Un) => todo!(),
            (Ra(_), Un) => todo!(),
            (Ov, Un) => todo!(),
            (Om(_), Un) => todo!(),
            (Un, Ep(_)) => todo!(),
            (Un, Wh(_)) => todo!(),
            (Un, Ra(_)) => todo!(),
            (Un, Ov) => todo!(),
            (Un, Om(_)) => todo!(),
            (Un, Un) => todo!(),
        }
    }
}

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
