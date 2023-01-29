use num_rational::*;
use std::cmp::*;
use std::collections::HashSet;
use std::hash::Hash;
use Ordering::*;

use crate::backend_lib::Extended::*;
use crate::backend_lib::*;
use crate::mathematics_lib::TwoDimentionalManifold::*;
use crate::mathematics_lib::*;

#[derive(Debug)]
pub struct PointsOrder {
    pub order: i64,
    pub instances: Vec<TwoDimentionalOrbifold>,
    pub omm_inf: HashSet<PosOmmRea>, //if during some search i64 was overflowed, there is possibility that some result was ommited, its for possibly_ommited
                                     // reaason for possible ommision immision info
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PosOmmRea {
    OccLimit,
    DenLimit,
    Overflow,
}

#[derive(Debug)]
pub struct PointsOrbifolds {
    pub orbifolds: Vec<TwoDimentionalOrbifold>,
    pub omm_inf: HashSet<PosOmmRea>, // some orbifolds were possibly ommited due too the overflow or limits provided
}



pub fn points_order(p_q: Rational64, b_m: TwoDimentionalManifold) -> PointsOrder {
    let mut counters: Vec<ExWh> = vec![];
    let mut order = -1; // order of point outside of the specturm is -1

    let mut omm_inf: HashSet<PosOmmRea> = HashSet::new();
    let p_q = match adj_for_b_m(p_q, b_m) {
        Base(p_q_0) => p_q_0,
        Overflow => {
            omm_inf.insert(PosOmmRea::Overflow);
            return PointsOrder {
                order,
                instances: vec![],
                omm_inf,
            };
        }
        _ => panic!("Adjusted p_q should be finite!"),
    };

    // we want to handle p_q = k/2 case
    let l = TWO * p_q;
    if l.is_integer() {
        let l = l.to_integer();
        if l > 4 {
            return PointsOrder {
                order: -1,
                instances: vec![],
                omm_inf,
            };
        }

        order = match l % 2 {
            0 => (4 - l) / 2,
            1 => (3 - l) / 2,
            -1 => (3 - l) / 2,
            _ => panic!("Residue from dividing by 2 must be 0, 1 or -1!"),
        };

        for _i in 0..order {
            counters.push(ExWh::PInfty);
        }

        match l % 2 {
            0 => {
                ();
            }
            1 => {
                counters.push(Base(2));
            }
            -1 => {
                counters.push(Base(2));
            }
            _ => panic!("Residue from dividing by 2 must be 0, 1 or -1!"),
        }

        return PointsOrder {
            order,
            instances: vec![co_to_orb(&counters, b_m)],
            omm_inf,
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
            // omm_inf = omm_inf
            //     .union(&search_result.omm_inf)
            //     .map(|x| x.to_owned())
            //     .collect();
            omm_inf = &omm_inf | &search_result.omm_inf;
            depth -= 1;
            cur_pnt -= ONE;
        }
    }

    let mut instances: Vec<TwoDimentionalOrbifold> = vec![];
    if found.len() > 0 {
        let instance: TwoDimentionalOrbifold;
        match b_m {
            Disk => {
                instance = TwoDimentionalOrbifold {
                    b_m: Disk,
                    r: vec![],
                    d: vec![found[0].r.clone()],
                };
                instances.push(instance);
            }
            Sphere => {
                instances = found;
            }
            Genus(g) => {
                instance = TwoDimentionalOrbifold {
                    b_m: Genus(g),
                    r: found[0].r.clone(),
                    d: vec![],
                };
                instances.push(instance);
            }
            General { h, c_c, b_c } => {
                if b_c == 0 {
                    instance = TwoDimentionalOrbifold {
                        b_m: General { h, c_c, b_c },
                        r: found[0].r.clone(),
                        d: vec![],
                    };
                } else {
                    instance = TwoDimentionalOrbifold {
                        b_m: General { h, c_c, b_c },
                        r: vec![],
                        d: vec![found[0].r.clone()],
                    };
                }
                instances.push(instance);
            }
        }
    }
    return PointsOrder {
        order: depth,
        instances,
        omm_inf,
    };
}

// add that this is sphere based, change name to naormalise to sphere add function normalising to given manifold
fn adj_for_b_m(p_q: Rational64, b_m: TwoDimentionalManifold) -> ExRa {
    let chi = chi(&b_m);
    match chi {
        Base(_) => match b_m {
            Disk => Base(TWO) * Base(p_q),
            Sphere => Base(p_q),
            Genus(_) => Base(p_q) + (Base(TWO) - chi.into()),
            General { h: _, c_c: _, b_c } => {
                if b_c == 0 {
                    // Base(p_q + (2 - chi))
                    Base(p_q) + (Base(TWO) - chi.into())
                } else {
                    // Base(TWO * (p_q + (1 - chi)))
                    Base(TWO) * (Base(p_q) + (Base(ONE) - chi.into()))
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
            d: vec![counters.clone()],
        },
        General { h: _, c_c: _, b_c } => {
            if b_c == 0 {
                TwoDimentionalOrbifold {
                    b_m: b_m,
                    r: counters.clone(),
                    d: vec![],
                }
            } else {
                TwoDimentionalOrbifold {
                    b_m: b_m,
                    r: vec![],
                    d: vec![counters.clone()],
                }
            }
        }
        _ => TwoDimentionalOrbifold {
            b_m: b_m,
            r: counters.clone(),
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
    _deg_lim: i64,               // degree limit, zero for no limit
) -> PointsOrbifolds {
    let mut occurences: Vec<Vec<ExWh>> = vec![];
    let mut counters: Vec<ExWh> = vec![Base(1)];
    let mut occ_co: i64 = 0; //occurences count
    let mut pivot = 0;
    let mut omm_inf = HashSet::new();
    let mut flag = if let Base(chi_orb) = per_chi_orb(&counters) {
        chi_orb
    } else {
        panic!("After initialising Euler orbicharacteristic should be equal to 1.")
    }
    .cmp(&p_q);

    let p_q = match adj_for_b_m(p_q, b_m) {
        Base(p_q_0) => p_q_0,
        ExRa::Overflow => {
            omm_inf.insert(PosOmmRea::Overflow);
            return PointsOrbifolds {
                orbifolds: cos_to_orb(&occurences, b_m),
                omm_inf,
            };
        }
        _ => panic!("Adjusted p_q should be finite!"),
    };
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
                counters[pivot] = counters[pivot] + Base(1);
                match counters[pivot] {
                    ExWh::MInfty => panic!("Counters should not be equal to -♾️"),
                    Base(_) => {},
                    ExWh::Overflow => {
                        // panic!("Counters should not be left on the Overflow state after main loop!")
                        flag = Less;
                        pivot += 1;
                        if counters.len() <= pivot {
                            counters.push(Base(1));
                        }
                        omm_inf.insert(PosOmmRea::Overflow);
                        continue;
                    }

                    ExWh::PInfty => todo!("Counter should not be qual to ♾️ in this place!"),
                }
                if break_condition(&counters, pivot) {
                    return PointsOrbifolds {
                        orbifolds: cos_to_orb(&occurences, b_m),
                        omm_inf,
                    };
                }
                level_to_pivot(&mut counters, pivot);
                let chi_orb_0 = match per_chi_orb(&counters) {
                    Base(chi_orb) => chi_orb,
                    ExRa::Overflow => {
                        flag = Less;
                        pivot += 1;
                        if counters.len() <= pivot {
                            counters.push(Base(1));
                        }
                        omm_inf.insert(PosOmmRea::Overflow);
                        continue;
                    }
                    _ => panic!("Euler Orbicharacteristic should be finite."),
                };
                match chi_orb_0.cmp(&p_q) {
                    Equal => {
                        occurences.push(counters.clone());
                        occ_co += 1;
                        if occ_co == occ_lim {
                            omm_inf.insert(PosOmmRea::OccLimit);
                            return PointsOrbifolds {
                                orbifolds: cos_to_orb(&occurences, b_m),
                                omm_inf,
                            };
                        } else {
                            flag = Less;
                            pivot += 1;
                            if counters.len() <= pivot {
                                counters.push(Base(1));
                            }
                            continue;
                        }
                    }
                    Less => {
                        flag = Less;
                        pivot += 1;
                        if counters.len() <= pivot {
                            counters.push(Base(1));
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
                    Base(chi_orb) => chi_orb,
                    ExRa::Overflow => {
                        omm_inf.insert(PosOmmRea::Overflow);
                        return PointsOrbifolds {
                            orbifolds: cos_to_orb(&occurences, b_m),
                            omm_inf,
                        };
                    }
                    _ => panic!("Euler Orbicharacteristic should be finite."),
                };
                match chi_orb_0.cmp(&p_q) {
                    Equal => {
                        occurences.push(counters.clone());
                        occ_co += 1;
                        if occ_co == occ_lim {
                            omm_inf.insert(PosOmmRea::OccLimit);
                            return PointsOrbifolds {
                                orbifolds: cos_to_orb(&occurences, b_m),
                                omm_inf,
                            };
                        } else {
                            flag = Less;
                            pivot += 1;
                            if counters.len() <= pivot {
                                counters.push(Base(1));
                            }
                            continue;
                        }
                    }
                    Less => {
                        let current_chi_orb = chi_orb_0 + ONE; // for now at pivot counter there is ♾️, so we need to compasate it by adding 1
                        let b_c = b_c_value(current_chi_orb, p_q); // smallest b_c such that Euler orbicharacterictic would be smaller or equal p/q
                        match b_c {
                            ExWh::MInfty => panic!("Counters value should not be equal to -♾️"),
                            Base(_) => {
                                counters[pivot] = b_c;
                            }
                            ExWh::Overflow => {
                                flag = Less;
                                pivot += 1;
                                if counters.len() <= pivot {
                                    counters.push(Base(1));
                                }
                                omm_inf.insert(PosOmmRea::Overflow);
                                continue;
                            }
                            ExWh::PInfty => {
                                counters[pivot] = b_c;
                            }
                        }
                        let chi_orb_0 = match per_chi_orb(&counters) {
                            Base(chi_orb) => chi_orb,
                            ExRa::Overflow => {
                                flag = Less;
                                pivot += 1;
                                if counters.len() <= pivot {
                                    counters.push(Base(1));
                                }
                                omm_inf.insert(PosOmmRea::Overflow);
                                continue;
                            }
                            _ => panic!("Euler Orbicharacteristic should be finite."),
                        };
                        if chi_orb_0 == p_q {
                            occurences.push(counters.clone());
                            occ_co += 1;
                            if occ_co == occ_lim {
                                omm_inf.insert(PosOmmRea::OccLimit);
                                return PointsOrbifolds {
                                    orbifolds: cos_to_orb(&occurences, b_m),
                                    omm_inf,
                                };
                            } else {
                                flag = Less;
                                // the following lines are not here on purpose:
                                // pivot += 1;
                                // if counters.len() <= pivot {
                                //     counters.push(Base(1));
                                // }
                                continue;
                            }
                        }
                        level_to_pivot(&mut counters, pivot);
                        let chi_orb_0 = match per_chi_orb(&counters) {
                            Base(chi_orb) => chi_orb,
                            ExRa::Overflow => {
                                flag = Less;
                                pivot += 1;
                                if counters.len() <= pivot {
                                    counters.push(Base(1));
                                }
                                omm_inf.insert(PosOmmRea::Overflow);
                                continue;
                            }
                            _ => panic!("Euler Orbicharacteristic should be finite."),
                        };
                        match chi_orb_0.cmp(&p_q) {
                            Equal => {
                                occurences.push(counters.clone());
                                occ_co += 1;
                                if occ_co == occ_lim {
                                    omm_inf.insert(PosOmmRea::OccLimit);
                                    return PointsOrbifolds {
                                        orbifolds: cos_to_orb(&occurences, b_m),
                                        omm_inf,
                                    };
                                } else {
                                    flag = Less;
                                    pivot += 1;
                                    if counters.len() <= pivot {
                                        counters.push(Base(1));
                                    }
                                    continue;
                                }
                            }
                            Less => {
                                flag = Less;
                                pivot += 1;
                                if counters.len() <= pivot {
                                    counters.push(Base(1));
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
                            counters.push(Base(1));
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
        if *counter != Base(2) {
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
    let mut b_c = Base(2);
    let mut a = b_c;
    let mut chi_orb_value;
    match Base(old_chi_orb) - rot_per_dif(b_c) {
        ExRa::Overflow => {
            return ExWh::Overflow;
        }
        Base(pq) => {
            chi_orb_value = pq;
        }
        _ => {
            panic!("This should be finite!")
        }
    }
    while chi_orb_value > p_q {
        a = b_c;
        b_c = Base(2) * b_c;
        match Base(old_chi_orb) - rot_per_dif(b_c) {
            ExRa::Overflow => {
                return ExWh::Overflow;
            }
            Base(pq) => {
                chi_orb_value = pq;
            }
            _ => {
                panic!("This should be finite!")
            }
        }
    }
    let mut b = b_c;
    if Base(old_chi_orb) - rot_per_dif(b_c) == Base(p_q) {
        return b_c;
    }
    if a == b {
        return b_c;
    }
    loop {
        let diff = (b - a) / Base(2);
        b_c = a + diff;
        match Base(old_chi_orb) - rot_per_dif(b_c) {
            ExRa::Overflow => {
                return ExWh::Overflow;
            }
            Base(pq) => {
                chi_orb_value = pq;
            }
            _ => {
                panic!("This should be finite!")
            }
        }
        match (chi_orb_value).cmp(&p_q) {
            Equal => {
                return b_c;
            }
            Less => {
                if diff == Base(1) {
                    return b_c;
                }
                b = b_c;
                continue;
            }
            Greater => {
                if diff == Base(0) {
                    return b;
                }
                a = b_c;
                continue;
            }
        }
    }
}
