use num_rational::*;
use serde::{Deserialize, Serialize};
// use std::cmp::Ordering::*;
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Clone, Copy)]
pub enum Extended<T> {
    MInfty,
    Base(T),
    Overflow,
    PInfty,
}
pub type ExWh = Extended<i64>;
pub type ExRa = Extended<Ratio<i64>>;

impl<T> From<T> for Extended<T> {
    fn from(value: T) -> Self {
        Self::Base(value)
    }
}
impl<T: std::fmt::Display> std::fmt::Display for Extended<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Extended::MInfty => write!(f, "-♾️"),
            Extended::Base(n) => write!(f, "{n}"),
            Extended::Overflow => write!(f, "Overflow"),
            Extended::PInfty => write!(f, "♾️"),
        }
    }
}

impl From<ExWh> for ExRa {
    fn from(value: ExWh) -> Self {
        match value {
            ExWh::MInfty => ExRa::MInfty,
            ExWh::Base(n) => ExRa::Base(Rational64::from_integer(n)),
            ExWh::Overflow => ExRa::Overflow,
            ExWh::PInfty => ExRa::PInfty,
        }
    }
}

impl<T> std::ops::Add for Extended<T>
where
    T: num_traits::ops::checked::CheckedAdd,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Extended::MInfty, Extended::PInfty) => panic!("Adding ♾️ to -♾️ is ill defined!"),
            (Extended::PInfty, Extended::MInfty) => panic!("Adding -♾️ to ♾️ is ill defined!"),
            (Extended::MInfty, _) => Extended::MInfty,
            (_, Extended::MInfty) => Extended::MInfty,
            (Extended::PInfty, _) => Extended::PInfty,
            (_, Extended::PInfty) => Extended::PInfty,
            (Extended::Overflow, _) => Extended::Overflow,
            (_, Extended::Overflow) => Extended::Overflow,
            (Extended::Base(n1), Extended::Base(n2)) => {
                let n3 = match n1.checked_add(&n2) {
                    Some(n_t) => n_t,
                    None => return Extended::Overflow,
                };
                Extended::Base(n3)
            }
        }
    }
}

impl<T> std::ops::Sub for Extended<T>
where
    T: num_traits::ops::checked::CheckedSub,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Extended::MInfty, Extended::MInfty) => {
                panic!("Substracting -♾️ from -♾️ is ill defined!")
            }
            (Extended::PInfty, Extended::PInfty) => {
                panic!("Substracting ♾️ from -♾️ is ill defined!")
            }
            (Extended::MInfty, _) => Extended::MInfty,
            (_, Extended::MInfty) => Extended::PInfty,
            (Extended::PInfty, _) => Extended::PInfty,
            (_, Extended::PInfty) => Extended::MInfty,
            (Extended::Overflow, _) => Extended::Overflow,
            (_, Extended::Overflow) => Extended::Overflow,
            (Extended::Base(n1), Extended::Base(n2)) => {
                let n3 = match n1.checked_sub(&n2) {
                    Some(n_t) => n_t,
                    None => return Extended::Overflow,
                };
                Extended::Base(n3)
            }
        }
    }
}

impl<T> std::ops::Mul for Extended<T>
where
    T: num_traits::ops::checked::CheckedMul,
    T: num_traits::identities::Zero,
    T: num_traits::sign::Signed,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Extended::Base(n), Extended::MInfty) if n.is_zero() => {
                panic!("Multiplying 0 by -♾️ is undefined!")
            }
            (Extended::MInfty, Extended::Base(n)) if n.is_zero() => {
                panic!("Multiplying -♾️ by an 0 is undefined!")
            }
            (Extended::Base(n), Extended::PInfty) if n.is_zero() => {
                panic!("Multiplying 0 by ♾️ is undefined!")
            }
            (Extended::PInfty, Extended::Base(n)) if n.is_zero() => {
                panic!("Multiplying ♾️ by an 0 is undefined!")
            }
            (Extended::Base(n), _) if n.is_zero() => Extended::Base(T::zero()),
            (_, Extended::Base(n)) if n.is_zero() => Extended::Base(T::zero()),
            (Extended::MInfty, Extended::PInfty) => Extended::MInfty,
            (Extended::PInfty, Extended::MInfty) => Extended::MInfty,
            (Extended::PInfty, Extended::PInfty) => Extended::PInfty,
            (Extended::MInfty, Extended::MInfty) => Extended::PInfty,
            (Extended::MInfty, Extended::Overflow) => {
                panic!("Sign of multiplying -♾️ by an overflow is undefined!")
            }
            (Extended::Overflow, Extended::MInfty) => {
                panic!("Sign of multiplying an overflow by -♾️ is undefined!")
            }
            (Extended::PInfty, Extended::Overflow) => {
                panic!("Sign of multiplying ♾️ by an overflow is undefined!")
            }
            (Extended::Overflow, Extended::PInfty) => {
                panic!("Sign of multiplying an overflow by ♾️ is undefined!")
            }
            (Extended::MInfty, Extended::Base(n)) => {
                if n.is_positive() {
                    Extended::MInfty
                } else {
                    Extended::PInfty
                }
            }
            (Extended::Base(n), Extended::MInfty) => {
                if n.is_positive() {
                    Extended::MInfty
                } else {
                    Extended::PInfty
                }
            }
            (Extended::PInfty, Extended::Base(n)) => {
                if n.is_positive() {
                    Extended::PInfty
                } else {
                    Extended::MInfty
                }
            }
            (Extended::Base(n), Extended::PInfty) => {
                if n.is_positive() {
                    Extended::PInfty
                } else {
                    Extended::MInfty
                }
            }
            (Extended::Overflow, _) => Extended::Overflow,
            (_, Extended::Overflow) => Extended::Overflow,
            (Extended::Base(n1), Extended::Base(n2)) => {
                let n3 = match n1.checked_mul(&n2) {
                    Some(n_t) => n_t,
                    None => return Extended::Overflow,
                };
                Extended::Base(n3)
            }
        }
    }
}

impl<T> std::ops::Div for Extended<T>
where
    T: num_traits::ops::checked::CheckedDiv,
    T: num_traits::identities::Zero,
    T: num_traits::sign::Signed,
{
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (Extended::Base(n), Extended::Base(m)) if n.is_zero() && m.is_zero() => {
                panic!("Dividing 0 by 0 is undefined!")
            }
            (Extended::Base(n), _) if n.is_zero() => Extended::Base(T::zero()),
            (_, Extended::Base(n)) if n.is_zero() => panic!("Sign of dividing by 0 is undefined!"),
            (Extended::MInfty, Extended::PInfty) => panic!("Dividing -♾️ by ♾️ is undefined!"),
            (Extended::PInfty, Extended::MInfty) => panic!("Dividing ♾️ by -♾️ is undefined!"),
            (Extended::PInfty, Extended::PInfty) => panic!("Dividing ♾️ by ♾️ is undefined!"),
            (Extended::MInfty, Extended::MInfty) => panic!("Dividing -♾️ by -♾️ is undefined!"),
            (Extended::MInfty, Extended::Overflow) => {
                panic!("Sign of dividing -♾️ by an overflow is undefined!")
            }
            (Extended::Overflow, Extended::MInfty) => {
                panic!("Sign of dividing an overflow by -♾️ is undefined!")
            }
            (Extended::PInfty, Extended::Overflow) => {
                panic!("Sign of dividing ♾️ by an overflow is undefined!")
            }
            (Extended::Overflow, Extended::PInfty) => {
                panic!("Sign of dividing an overflow by ♾️ is undefined!")
            }
            (Extended::MInfty, Extended::Base(n)) => {
                if n.is_positive() {
                    Extended::MInfty
                } else {
                    Extended::PInfty
                }
            }
            (Extended::Base(n), Extended::MInfty) => {
                if n.is_positive() {
                    Extended::MInfty
                } else {
                    Extended::PInfty
                }
            }
            (Extended::PInfty, Extended::Base(n)) => {
                if n.is_positive() {
                    Extended::PInfty
                } else {
                    Extended::MInfty
                }
            }
            (Extended::Base(n), Extended::PInfty) => {
                if n.is_positive() {
                    Extended::PInfty
                } else {
                    Extended::MInfty
                }
            }
            (Extended::Overflow, _) => Extended::Overflow,
            (_, Extended::Overflow) => Extended::Overflow,
            (Extended::Base(n1), Extended::Base(n2)) => {
                let n3 = match n1.checked_div(&n2) {
                    Some(n_t) => n_t,
                    None => return Extended::Overflow,
                };
                Extended::Base(n3)
            }
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
pub struct Config {}
