use crate::backend_lib::*;
use crate::backend_lib::ExWh::*;
use crate::backend_lib::ExRa::*;
use crate::mathematics_lib::TwoDimentionalManifold::*;
use num_rational::*;
use num_traits::ops::checked::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum TwoDimentionalManifold {
    Disk,
    Sphere,
    Genus(i64),
    General {
        h: i64,   //handles
        c_c: i64, //cross-caps
        b_c: i64, //boundry components
    },
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TwoDimentionalOrbifold {
    pub b_m: TwoDimentionalManifold, // base manifold
    pub r: Vec<ExWh>,              // rotational points
    pub d: Vec<Vec<ExWh>>,         //dihedral points
}

pub fn chi(m: &TwoDimentionalManifold) -> ExWh {
    match m {
        Disk => Whole(1),
        Sphere => Whole(2),
        Genus(g) => match i64::checked_mul(2, *g) {
            Some(g_2) => match i64::checked_sub(2, g_2) {
                Some(chi) => Whole(chi),
                None => ExWh::Overflow,
            },
            None => ExWh::Overflow,
        },
        General { h, c_c, b_c } => {
            let h_chi = match i64::checked_mul(2, *h) {
                Some(h_two) => h_two,
                None => return ExWh::Overflow,
            };
            let mut sum: i64 = match i64::checked_add(h_chi, *c_c) {
                Some(s) => s,
                None => return ExWh::Overflow,
            };
            sum = match i64::checked_add(sum, *b_c) {
                Some(s) => s,
                None => return ExWh::Overflow,
            };
            sum = match i64::checked_sub(2, sum) {
                Some(s) => s,
                None => return ExWh::Overflow,
            };
            Whole(sum)
        }
    }
}


pub fn chi_orb(o: &TwoDimentionalOrbifold) -> ExRa {
    let mut chi_orb = match chi(&o.b_m) {
        ExWh::MInfty => return ExRa::MInfty,
        Whole(chi) => Rational64::from_integer(chi),
        ExWh::Overflow => return ExRa::Overflow,
        ExWh::PInfty => return ExRa::PInfty,
    };

    for o_p in &o.r {
        let diff = match rot_per_dif(*o_p) {
            ExRa::MInfty => panic!("Difference in Euler orbicharacteristic caused a single orbipoint can not be infinite!"),
            Rational(r) => r,
            ExRa::Overflow => return ExRa::Overflow, 
            ExRa::PInfty => panic!("Difference in Euler orbicharacteristic caused a single orbipoint can not be infinite!"),
        };
        chi_orb = match <Ratio<i64>>::checked_sub(&chi_orb, &diff) {
            Some(chi_orb_1) => chi_orb_1,
            None => return ExRa::Overflow,
        };
    }
    for b_c in &o.d {
        for o_p in b_c {
            let diff = match dih_per_dif(*o_p) {
            ExRa::MInfty => panic!("Difference in Euler orbicharacteristic caused a single orbipoint can not be infinite!"),
            Rational(r) => r,
            ExRa::Overflow => return ExRa::Overflow,
            ExRa::PInfty => panic!("Difference in Euler orbicharacteristic caused a single orbipoint can not be infinite!"),
        };
            chi_orb = match <Ratio<i64>>::checked_sub(&chi_orb, &diff) {
                Some(chi_orb_1) => chi_orb_1,
                None => return ExRa::Overflow,
            };
        }
    }
    Rational(chi_orb)
}

pub fn per_chi_orb(periods: &Vec<ExWh>) -> ExRa {
    let mut chi_orb = TWO;
    for o_p in periods {
        let diff = match rot_per_dif(*o_p) {
            ExRa::MInfty => panic!("Difference in Euler orbicharacteristic caused a single orbipoint can not be infinite!"),
            Rational(r) => r,
            ExRa::Overflow => return ExRa::Overflow,
            ExRa::PInfty => panic!("Difference in Euler orbicharacteristic caused a single orbipoint can not be infinite!"),
        };
        chi_orb = match <Ratio<i64>>::checked_sub(&chi_orb, &diff) {
            Some(chi_orb_1) => chi_orb_1,
            None => return ExRa::Overflow,
        };
    }
    Rational(chi_orb)
}

pub fn rot_per_dif(n: ExWh) -> ExRa {
    match n {
        ExWh::MInfty => panic!("Orbipoint can not have anorder equal to -♾️"),
        Whole(0) => panic!("Orbipoint can not have an order equal to 0!"),
        Whole(n_0) => match i64::checked_sub(n_0, 1) {
            Some(n_1) => Rational(Rational64::new(n_1, n_0)),
            None => ExRa::Overflow,
        },
        ExWh::Overflow => ExRa::Overflow,
        ExWh::PInfty => Rational(ONE),
    }
}

pub fn dih_per_dif(n: ExWh) -> ExRa {
    match n {
        ExWh::MInfty => panic!("Orbipoint can not have anorder equal to -♾️"),
        Whole(0) => panic!("Orbipoint can not have an order equal to 0!"),
        Whole(n_0) => {
            let n_1 = match i64::checked_sub(n_0, 1) {
                Some(n_1_1) => n_1_1,
                None => return ExRa::Overflow,
            };
            let n_2 = match i64::checked_mul(2, n_0) {
                Some(n_2_1) => n_2_1,
                None => return ExRa::Overflow,
            };
            Rational(Rational64::new(n_1, n_2))
        }
        ExWh::PInfty => Rational(ONE_OVER_TWO),
        ExWh::Overflow => ExRa::Overflow,
    }
}
