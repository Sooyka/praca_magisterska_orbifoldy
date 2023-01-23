use crate::backend_lib::ExRa::*;
use crate::backend_lib::ExWh::*;
use crate::backend_lib::*;
use crate::mathematics_lib::TwoDimentionalManifold::*;
use num_rational::*;
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
    pub r: Vec<ExWh>,                // rotational points
    pub d: Vec<Vec<ExWh>>,           //dihedral points
}

pub fn chi(m: &TwoDimentionalManifold) -> ExWh {
    match m {
        Disk => Whole(1),
        Sphere => Whole(2),
        Genus(g) => Whole(2) - Whole(2) * Whole(*g),
        General { h, c_c, b_c } => Whole(2) - (Whole(2) * Whole(*h) + Whole(*c_c) + Whole(*b_c)),
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
    let mut chi_orb = Rational(TWO);
    for o_p in periods {
        chi_orb = chi_orb - rot_per_dif(*o_p);
    }
    chi_orb
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
        ExWh::Overflow => ExRa::Overflow,
        ExWh::PInfty => Rational(ONE_OVER_TWO),
    }
}
