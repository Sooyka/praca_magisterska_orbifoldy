use num_rational::*;
use num_traits::ops::checked::*;
use std::cmp::*;
use Ordering::*;

use crate::backend_lib::*;
use crate::backend_lib::ExRa::*;
use crate::backend_lib::ExWh::*;
use crate::mathematics_lib::TwoDimentionalManifold::*;
use crate::mathematics_lib::*;
// use crate::searching_algorithm_lib::PointsOrder;

// takes rational number and gives order of accumulation in base_manifold spectrum, as well as a signature confirming said order

// pub enum PointsOrder {
//     NonNegative {
//         order: i64,
//         instance: Vec<TwoDimentionalOrbifold>,
//         pos_omm: bool, //if during some search i64 was overflowed, there is possibility that some result was ommited, its for possibly_ommited
//     },
//     Negative {
//         order: i64,
//         pos_omm: bool, //if during some search i64 was overflowed, there is possibility that some result was ommited, its for possibly_ommited
//         reason: Vec<PosOmm> // reaason for possible ommision
//     },
// }

#[derive(Debug)]
pub struct PointsOrder {
    pub order: i64,
    pub instances: Vec<TwoDimentionalOrbifold>,
    pub omm_inf: PosOmm, //if during some search i64 was overflowed, there is possibility that some result was ommited, its for possibly_ommited
                         // reaason for possible ommision immision info
}

#[derive(Debug)]
pub struct PosOmm {
    pub pos_omm: bool,
    pub pos_rea: Vec<PosOmmRea>,
}

#[derive(Debug)]
pub enum PosOmmRea {
    DenLimit,
    Overflow,
}

pub struct PointsOrbifolds {
    pub orbifolds: Vec<TwoDimentionalOrbifold>,
    pub pos_omm: bool, // some orbifolds were possibly ommited due too the overflow or limits provided
}

pub fn points_order(p_q: Rational64, b_m: TwoDimentionalManifold) -> PointsOrder {
    let mut counters: Vec<ExWh> = vec![];
    let mut order = -1; // order of point outside of the specturm is -1

    let mut pos_omm = false;
    let mut pos_rea = vec![];
    let p_q = match adj_for_b_m(p_q, b_m) {
        Rational(p_q_0) => p_q_0,
        ExRa::Overflow => {
            return PointsOrder {
                order: -1,
                instances: vec![],
                omm_inf: PosOmm {
                    pos_omm: true,
                    pos_rea: vec![PosOmmRea::Overflow],
                },
            }
        }
        _ => panic!("Adjusted p_q should be finite!"),
    };

    // we want to handle p_q = k/2 case
    // let l = match <Ratio<i64>>::checked_mul(&TWO, &p_q) {
    //     Some(l_1) => l_1,
    //     None => {
    //         return Negative {
    //             order: -1,
    //             pos_omm: true,
    //         }
    //     }
    // };
    let l = TWO * p_q;
    if l.is_integer() {
        let l = l.to_integer();
        if l > 4 {
            return PointsOrder {
                order: -1,
                instances: vec![],
                omm_inf: PosOmm {
                    pos_omm: false,
                    pos_rea: vec![],
                },
            };
        }

        let mut con = true;

        order = match l % 2 {
            0 => (4 - l) / 2,
            1 => (3 - l) / 2,
            -1 => (3 - l) / 2,
            _ => panic!("Residue from dividing by 2 must be 0, 1 or -1!"),
        };

        for i in 0..order {
            counters.push(ExWh::PInfty);
        }

        match l % 2 {
            0 => {
                ();
            }
            1 => {
                counters.push(Whole(2));
            }
            -1 => {
                counters.push(Whole(2));
            }
            _ => panic!("Residue from dividing by 2 must be 0, 1 or -1!"),
        }

        return PointsOrder {
            order: order,
            instances: vec![co_to_orb(&counters, b_m)],
            omm_inf: PosOmm { pos_omm, pos_rea },
        };
    }
    let mut dist = ZERO;

    let mut depth: i64 = 0;

    while TWO - dist >= p_q + ONE {
        dist += ONE;
        depth += 1;
    }

    let mut cur_pnt = p_q + dist;
    let mut found = vec![];
    while found.len() == 0 && depth >= 0 {
        let search_result = points_orb(cur_pnt, TwoDimentionalManifold::Sphere, 1, 0);
        found = search_result.orbifolds;
        if found.len() == 0 {
            pos_omm = search_result.pos_omm;
            depth -= 1;
            cur_pnt -= ONE;
        }
    }
    let mut instance: TwoDimentionalOrbifold;
    if found.len() > 0 {
        instance = found[0].clone();
    }
    order = depth;
    counters = instance.r;
    return PointsOrder {
        order: order,
        instances: vec![co_to_orb(&counters, b_m)],
        omm_inf: PosOmm { pos_omm, pos_rea },
    };
    // match b_m {
    //     Disk => {
    //         return NonNegative {
    //             order: order,
    //             instance: TwoDimentionalOrbifold {
    //                 b_m: Disk,
    //                 r: vec![],
    //                 d: vec![counters],
    //             },
    //             pos_omm: pos_omm,
    //         }
    //     }
    //     _ => {
    //         return NonNegative {
    //             order: order,
    //             instance: TwoDimentionalOrbifold {
    //                 b_m: b_m,
    //                 r: counters,
    //                 d: vec![],
    //             },
    //             pos_omm: pos_omm,
    //         }
    //     }
    // }
}

// add that this is sphere based, change name to naormalise to sphere add function normalising to given manifold
fn adj_for_b_m(p_q: Rational64, b_m: TwoDimentionalManifold) -> ExRa {
    let chi = chi(&b_m);
    match chi {
        Whole(chi) => match b_m {
            Disk => match <Ratio<i64>>::checked_mul(&TWO, &p_q) {
                Some(p_q_0) => return Rational(p_q_0),
                None => return ExRa::Overflow,
            },
            Sphere => Rational(p_q),
            Genus(g) => {
                let chi_1 = match <i64>::checked_sub(2, chi) {
                    Some(chi_1) => chi_1,
                    None => return ExRa::Overflow,
                };
                match <Ratio<i64>>::checked_add(&p_q, &Rational64::from_integer(chi_1)) {
                    Some(chi_2) => return Rational(chi_2),
                    None => return ExRa::Overflow,
                }
            }
            General { h, c_c, b_c } => {
                if b_c == 0 {
                    // Rational(p_q + (2 - chi))
                    let chi_1 = match i64::checked_sub(2, chi) {
                        Some(chi_1) => chi_1,
                        None => return ExRa::Overflow,
                    };
                    match <Ratio<i64>>::checked_add(&p_q, &Rational64::from_integer(chi_1)) {
                        Some(chi_2) => return Rational(chi_2),
                        None => return ExRa::Overflow,
                    }
                } else {
                    // Rational(TWO * (p_q + (1 - chi)))
                    let chi_1 = match i64::checked_sub(1, chi) {
                        Some(chi_1) => chi_1,
                        None => return ExRa::Overflow,
                    };
                    let chi_2 =
                        match <Ratio<i64>>::checked_add(&p_q, &Rational64::from_integer(chi_1)) {
                            Some(chi_2) => chi_2,
                            None => return ExRa::Overflow,
                        };
                    match <Ratio<i64>>::checked_mul(&TWO, &chi_2) {
                        Some(chi_3) => return Rational(chi_3),
                        None => return ExRa::Overflow,
                    }
                }
            }
        },
        ExWh::Overflow => ExRa::Overflow,
        _ => panic!("Euler characteristic should be finite."),
    }
}

fn co_to_orb(counters: &Vec<ExWh>, b_m: TwoDimentionalManifold) -> TwoDimentionalOrbifold {
    match b_m {
        Disk => TwoDimentionalOrbifold {
            b_m: Disk,
            r: vec![],
            d: vec![*counters],
        },
        General { h, c_c, b_c } => {
            if b_c == 0 {
                TwoDimentionalOrbifold {
                    b_m: b_m,
                    r: *counters,
                    d: vec![],
                }
            } else {
                TwoDimentionalOrbifold {
                    b_m: b_m,
                    r: vec![],
                    d: vec![*counters],
                }
            }
        }
        _ => TwoDimentionalOrbifold {
            b_m: b_m,
            r: *counters,
            d: vec![],
        },
    }
}


// add reason to overflow
pub fn points_orb(
    // Points orbifolds
    p_q: Rational64,
    b_m: TwoDimentionalManifold, //base manifold
    occ_lim: i64,                //occurences limit, zero for no limit
    deg_lim: i64,                // degree limit, zero for no limit
) -> PointsOrbifolds {
    let mut occurences: Vec<Vec<ExWh>> = vec![];
    let mut counters: Vec<ExWh> = vec![Whole(1)];
    let mut occ_co: i64 = 0; //occurences count
    let mut pivot = 0;
    let mut flag = if let Rational(chi_orb) = per_chi_orb(&counters) {
        chi_orb
    } else {
        panic!("After initialising Euler orbicharacteristic should be equal to 1.")
    }
    .cmp(&p_q);

    let mut possibly_ommited = false;
    let p_q = match adj_for_b_m(p_q, b_m) {
        Rational(p_q_0) => p_q_0,
        ExRa::Overflow => {
            return PointsOrbifolds {
                orbifolds: cos_to_orb(&occurences, b_m),
                pos_omm: true,
            };
        }
        _ => panic!("Adjusted p_q should be finite!"),
    };
    // let p_q = match b_m {
    //     Disk => TWO * p_q,
    //     Sphere => p_q,
    //     Genus(g) => p_q + 2 * g,
    //     General { h, c_c, b_c } => p_q + 2 * h + c_c + b_c,
    // };
    // counters[0] = 7312724677;
    // counters.push(405642);
    // counters.push(403);
    // counters.push(33);
    // pivot = 0;
    // flag = Greater;
    loop {
        // println!(
        //     "{}",
        //     &(signature_string(&counters, base_manifold)
        //         + "\n"
        //         + &pivot.to_string()
        //         + "\n"
        //         + &match flag {
        //             Equal => "Equal",
        //             Less => "Less",
        //             Greater => "Greater",
        //         })
        // );

        match flag {
            Equal => {
                panic!("Flag should not be left on 'Equal' at the main loop!");
            }
            Less => {
                match counters[pivot] {
                    ExWh::MInfty => panic!("Counters should not be equal to -♾️"),
                    Whole(n) => {
                        counters[pivot] = Whole(n + 1);
                    }
                    ExWh::Overflow => {
                        panic!("Counters should not be left on the Overflow state after main loop!")
                    }

                    ExWh::PInfty => todo!("Counter should not be qual to ♾️ in this place!"),
                }
                if break_condition(&counters, pivot) {
                    return PointsOrbifolds {
                        orbifolds: cos_to_orb(&occurences, b_m),
                        pos_omm: possibly_ommited,
                    };
                }
                level_to_pivot(&mut counters, pivot);
                let chi_orb_0 = match per_chi_orb(&counters) {
                    Rational(chi_orb) => chi_orb,
                    ExRa::Overflow => {
                        flag = Less;
                        pivot += 1;
                        if counters.len() <= pivot {
                            counters.push(Whole(1));
                        }
                        possibly_ommited = true;
                        continue;
                    }
                    _ => panic!("Euler Orbicharacteristic should be finite."),
                };
                match chi_orb_0.cmp(&p_q) {
                    Equal => {
                        occurences.push(counters.clone());
                        occ_co += 1;
                        if occ_co == occ_lim {
                            possibly_ommited = true;
                            return PointsOrbifolds {
                                orbifolds: cos_to_orb(&occurences, b_m),
                                pos_omm: possibly_ommited,
                            };
                        } else {
                            flag = Less;
                            pivot += 1;
                            if counters.len() <= pivot {
                                counters.push(Whole(1));
                            }
                            continue;
                        }
                    }
                    Less => {
                        flag = Less;
                        pivot += 1;
                        if counters.len() <= pivot {
                            counters.push(Whole(1));
                        }
                        continue;
                    }
                    Greater => {
                        flag = Greater;
                        pivot = 0;
                        continue;
                    }
                }
            }
            Greater => {
                counters[pivot] = ExWh::PInfty;
                let chi_orb_0 = match per_chi_orb(&counters) {
                    Rational(chi_orb) => chi_orb,
                    ExRa::Overflow => {
                        possibly_ommited = true;
                        return PointsOrbifolds {
                            orbifolds: cos_to_orb(&occurences, b_m),
                            pos_omm: possibly_ommited,
                        };
                    }
                    _ => panic!("Euler Orbicharacteristic should be finite."),
                };
                match chi_orb_0.cmp(&p_q) {
                    Equal => {
                        occurences.push(counters.clone());
                        occ_co += 1;
                        if occ_co == occ_lim {
                            possibly_ommited = true;
                            return PointsOrbifolds {
                                orbifolds: cos_to_orb(&occurences, b_m),
                                pos_omm: possibly_ommited,
                            };
                        } else {
                            flag = Less;
                            pivot += 1;
                            if counters.len() <= pivot {
                                counters.push(Whole(1));
                            }
                            continue;
                        }
                    }
                    Less => {
                        let current_chi_orb = chi_orb_0 + ONE; // for now at pivot counter there is ♾️, so we need to compasate it by adding 1
                        let b_c = b_c_value(current_chi_orb, p_q); // smallest b_c such that Euler orbicharacterictic would be smaller or equal p/q
                        match b_c {
                            ExWh::MInfty => panic!("Counters value should not be equal to -♾️"),
                            Whole(n) => {
                                counters[pivot] = b_c;
                            }
                            ExWh::Overflow => {
                                flag = Less;
                                pivot += 1;
                                if counters.len() <= pivot {
                                    counters.push(Whole(1));
                                }
                                possibly_ommited = true;
                                continue;
                            }
                            ExWh::PInfty => {
                                counters[pivot] = b_c;
                            }
                        }
                        let chi_orb_0 = match per_chi_orb(&counters) {
                            Rational(chi_orb) => chi_orb,
                            ExRa::Overflow => {
                                flag = Less;
                                pivot += 1;
                                if counters.len() <= pivot {
                                    counters.push(Whole(1));
                                }
                                possibly_ommited = true;
                                continue;
                            }
                            _ => panic!("Euler Orbicharacteristic should be finite."),
                        };
                        if chi_orb_0 == p_q {
                            occurences.push(counters.clone());
                            occ_co += 1;
                            if occ_co == occ_lim {
                                possibly_ommited = true;
                                return PointsOrbifolds {
                                    orbifolds: cos_to_orb(&occurences, b_m),
                                    pos_omm: possibly_ommited,
                                };
                            } else {
                                flag = Less;
                                // pivot += 1;
                                // if counters.len() <= pivot {
                                //     counters.push(Whole(1));
                                // }
                                continue;
                            }
                        }
                        level_to_pivot(&mut counters, pivot);
                        let chi_orb_0 = match per_chi_orb(&counters) {
                            Rational(chi_orb) => chi_orb,
                            ExRa::Overflow => {
                                flag = Less;
                                pivot += 1;
                                if counters.len() <= pivot {
                                    counters.push(Whole(1));
                                }
                                possibly_ommited = true;
                                continue;
                            }
                            _ => panic!("Euler Orbicharacteristic should be finite."),
                        };
                        match chi_orb_0.cmp(&p_q) {
                            Equal => {
                                occurences.push(counters.clone());
                                occ_co += 1;
                                if occ_co == occ_lim {
                                    possibly_ommited = true;
                                    return PointsOrbifolds {
                                        orbifolds: cos_to_orb(&occurences, b_m),
                                        pos_omm: possibly_ommited,
                                    };
                                } else {
                                    flag = Less;
                                    pivot += 1;
                                    if counters.len() <= pivot {
                                        counters.push(Whole(1));
                                    }
                                    continue;
                                }
                            }
                            Less => {
                                flag = Less;
                                pivot += 1;
                                if counters.len() <= pivot {
                                    counters.push(Whole(1));
                                }
                                continue;
                            }
                            Greater => {
                                flag = Greater;
                                pivot = 0;
                                continue;
                            }
                        }
                    }
                    Greater => {
                        flag = Greater;
                        pivot += 1;
                        if counters.len() <= pivot {
                            // panic!("Pivot should not reach end of counters at the greater loop!");
                            counters.push(Whole(1));
                        }
                        continue;
                    }
                }
            }
        }
    }
}

fn cos_to_orb(
    counters: &Vec<Vec<ExWh>>,
    b_m: TwoDimentionalManifold,
) -> Vec<TwoDimentionalOrbifold> {
    let mut orbifolds: Vec<TwoDimentionalOrbifold> = vec![];
    for counter in counters.iter() {
        orbifolds.push(co_to_orb(counter, b_m));
    }
    orbifolds
}

fn break_condition(counters: &Vec<ExWh>, pivot: usize) -> bool {
    let mut break_condition = true;
    for (i, counter) in counters.iter().enumerate() {
        if i > pivot {
            break;
        }
        if *counter != Whole(2) {
            break_condition = false;
        }
    }
    break_condition
}

fn level_to_pivot(counters: &mut Vec<ExWh>, pivot: usize) {
    let b_c = counters[pivot];
    for (i, counter) in counters.iter_mut().enumerate() {
        if i > pivot {
            break;
        }
        *counter = b_c;
    }
}

fn b_c_value(old_chi_orb: Rational64, p_q: Rational64) -> ExWh {
    let mut b_c = 2;
    let mut a = b_c;
    let mut chi_orb_value;
    match old_chi_orb.checked_sub(&rot_per_dif(b_c)) {
        None => {
            return ExWh::Overflow;
        }
        Some(result) => {
            chi_orb_value = result;
        }
    }
    while chi_orb_value > p_q {
        a = b_c;
        b_c = 2 * b_c;
        match old_chi_orb.checked_sub(&rot_per_dif(b_c)) {
            None => {
                return ExWh::Overflow;
            }
            Some(result) => {
                chi_orb_value = result;
            }
        }
    }
    let mut b = b_c;
    if old_chi_orb - rot_per_dif(b_c) == p_q {
        return Whole(b_c);
    }
    if a == b {
        return Whole(b_c);
    }
    loop {
        let diff = (b - a) / 2;
        b_c = a + diff;
        match old_chi_orb.checked_sub(&rot_per_dif(b_c)) {
            None => {
                return ExWh::Overflow;
            }
            Some(result) => {
                chi_orb_value = result;
            }
        }
        match (chi_orb_value).cmp(&p_q) {
            Equal => {
                return Whole(b_c);
            }
            Less => {
                if diff == 1 {
                    return Whole(b_c);
                }
                b = b_c;
                continue;
            }
            Greater => {
                if diff == 0 {
                    return Whole(b);
                }
                a = b_c;
                continue;
            }
        }
    }
}
