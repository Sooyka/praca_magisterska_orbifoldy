use crate::backend_lib::Extended::*;
use crate::backend_lib::*;
use crate::mathematics_lib::TwoDimentionalManifold::*;
use num_rational::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct TwoDimentionalOrbifold {
    pub b_m: TwoDimentionalManifold, // base manifold
    pub r: Vec<ExWh>,                // rotational points
    pub d: Vec<Vec<ExWh>>,           //dihedral points
}

pub fn chi(m: &TwoDimentionalManifold) -> ExWh {
    match m {
        Disk => Base(1),
        Sphere => Base(2),
        Genus(g) => Base(2) - Base(2) * Base(*g),
        General { h, c_c, b_c } => Base(2) - (Base(2) * Base(*h) + Base(*c_c) + Base(*b_c)),
    }
}

pub fn chi_orb(o: &TwoDimentionalOrbifold) -> ExRa {
    let mut chi_orb: ExRa = chi(&o.b_m).into();

    for o_p in &o.r {
        chi_orb = chi_orb - rot_per_dif(*o_p);
    }
    for b_c in &o.d {
        for o_p in b_c {
            chi_orb = chi_orb - dih_per_dif(*o_p);
        }
    }
    chi_orb
}

pub fn per_chi_orb(periods: &Vec<ExWh>) -> ExRa {
    let mut chi_orb = Base(TWO);
    for o_p in periods {
        chi_orb = chi_orb - rot_per_dif(*o_p);
    }
    chi_orb
}

pub fn rot_per_dif<T: Into<ExWh>>(n: T) -> ExRa {
    match n.into() {
        Extended::MInfty => panic!("Orbipoint can not have anorder equal to -♾️"),
        Base(0) => panic!("Orbipoint can not have an order equal to 0!"),
        Base(n_0) => match i64::checked_sub(n_0, 1) {
            Some(n_1) => Base(Rational64::new(n_1, n_0)),
            None => Extended::Overflow,
        },
        Extended::Overflow => Extended::Overflow,
        Extended::PInfty => Base(ONE),
    }
}

pub fn dih_per_dif<T: Into<ExWh>>(n: T) -> ExRa {
    match n.into() {
        Extended::MInfty => panic!("Orbipoint can not have anorder equal to -♾️"),
        Base(0) => panic!("Orbipoint can not have an order equal to 0!"),
        Base(n_0) => {
            let n_1 = match i64::checked_sub(n_0, 1) {
                Some(n_1_1) => n_1_1,
                None => return Extended::Overflow,
            };
            let n_2 = match i64::checked_mul(2, n_0) {
                Some(n_2_1) => n_2_1,
                None => return Extended::Overflow,
            };
            Base(Rational64::new(n_1, n_2))
        }
        Extended::PInfty => Base(ONE_OVER_TWO),
        Extended::Overflow => Extended::Overflow,
    }
}
