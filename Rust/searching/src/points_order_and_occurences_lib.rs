use num_rational::*;
use num_traits::ops::checked::CheckedSub;
use std::cmp::*;
use Ordering::*;

use serde::{Deserialize, Serialize};

//use searching::*;

use crate::common_lib::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PointsOrderAndOccurencesConfig {
    pub maximal_number_of_occurences: i64,
    pub base_manifolds: Vec<TwoDimentionalManifold>,
}

pub fn points_order_and_occurences(
    p_q: Rational64,
    base_manifold: &TwoDimentionalManifold,
    occurences_limit: i64,
) -> ((i64, bool), (Vec<Vec<i64>>, bool)) {
    let points_order_and_first_occurence = determine_points_order(p_q, base_manifold);
    let p_q_order = points_order_and_first_occurence.0;
    let p_q_order_possible_missing = points_order_and_first_occurence.2;
    // let p_q_order = 1;
    let p_q_occurences = search_for_point(p_q, base_manifold, occurences_limit);
    ((p_q_order, p_q_order_possible_missing), p_q_occurences)
}

pub fn determine_points_order(
    p_q: Rational64,
    base_manifold: &TwoDimentionalManifold,
) -> (i64, Vec<i64>, bool) {
    let _zero = Rational64::from_integer(0);
    let _one_over_two = Rational64::new(1, 2);
    let _one_over_four = Rational64::new(1, 4);
    let _one = Rational64::from_integer(1);
    let _two = Rational64::from_integer(2);
    let _three = Rational64::from_integer(3);
    let _four = Rational64::from_integer(4);

    let mut counters: Vec<i64> = vec![];
    let mut p_q_order = -1;
    let mut p_q_first_occurence: Vec<i64> = vec![];

    let p_q = match base_manifold {
        TwoDimentionalManifold::Disk => _two * p_q,
        TwoDimentionalManifold::Sphere => p_q,
        TwoDimentionalManifold::Genus(g) => p_q + 2 * g,
        TwoDimentionalManifold::General {
            handles: h,
            cross_caps: cc,
            boundry_components: bc,
        } => p_q + 2 * h + cc + bc,
    };
    let mut possibly_ommited = false;
    let l = _two * p_q;
    if l.is_integer() {
        let l = l.to_integer();
        if l > 4 {
            return (p_q_order, vec![], false);
        } else {
            while chi_orb(&counters).1 - _one_over_two > p_q {
                counters.push(0);
            }
            match l % 2 {
                0 => {
                    ();
                }
                1 => {
                    counters.push(2);
                }
                -1 => {
                    counters.push(2);
                }
                _ => panic!(),
            }

            p_q_order = match l % 2 {
                0 => (4 - l) / 2,
                1 => (3 - l) / 2,
                -1 => (3 - l) / 2,
                _ => panic!(),
            };
        }
    } else {
        let mut distance = _zero;

        let mut depth = 0 as i64;

        while _two - distance >= p_q + _one {
            distance += _one;
            depth += 1;
        }

        let mut current_point = p_q + distance;
        let mut found = vec![];
        while found.len() == 0 && depth >= 0 {
            let search_result = search_for_point(current_point, &TwoDimentionalManifold::Sphere, 1);
            found = search_result.0;
            if found.len() == 0 {
                possibly_ommited = search_result.1;
                depth -= 1;
                current_point -= _one;
            }
        }
        if found.len() > 0 {
            p_q_first_occurence = found[0].clone();
        }
        p_q_order = depth;
        counters = p_q_first_occurence;
    }
    return (p_q_order, counters, possibly_ommited);
}

pub fn search_for_point(
    p_q: Rational64,
    base_manifold: &TwoDimentionalManifold,
    occurences_limit: i64,
) -> (Vec<Vec<i64>>, bool) {
    let _zero = Rational64::from_integer(0);
    let _one_over_two = Rational64::new(1, 2);
    let _one_over_four = Rational64::new(1, 4);
    let _one = Rational64::from_integer(1);
    let _two = Rational64::from_integer(2);
    let _three = Rational64::from_integer(3);
    let _four = Rational64::from_integer(4);

    let mut occurences: Vec<Vec<i64>> = vec![];
    let mut counters: Vec<i64> = vec![1];
    let mut occurences_count = 0;
    let mut pivot = 0;
    let mut flag = chi_orb(&counters).1.cmp(&p_q);
    let p_q = match base_manifold {
        TwoDimentionalManifold::Disk => _two * p_q,
        TwoDimentionalManifold::Sphere => p_q,
        TwoDimentionalManifold::Genus(g) => p_q + 2 * g,
        TwoDimentionalManifold::General {
            handles: h,
            cross_caps: cc,
            boundry_components: bc,
        } => p_q + 2 * h + cc + bc,
    };
    // counters[0] = 7312724677;
    // counters.push(405642);
    // counters.push(403);
    // counters.push(33);
    // pivot = 0;
    // flag = Greater;
    let mut possibly_ommited = false;
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
                occurences.push(counters.clone());
                occurences_count += 1;
                if occurences_count == occurences_limit {
                    possibly_ommited = true;
                    return (occurences, possibly_ommited);
                } else {
                    flag = Less;
                    pivot += 1;
                    if counters.len() <= pivot {
                        counters.push(1);
                    }
                    continue;
                }
            }
            Less => {
                counters[pivot] += 1;
                if break_condition(&counters, pivot) {
                    return (occurences, possibly_ommited);
                }
                level_to_b_c(&mut counters, pivot);
                let chi_orb_0 = chi_orb(&counters);
                match if chi_orb_0.0 {
                    flag = Less;
                    pivot += 1;
                    if counters.len() <= pivot {
                        counters.push(1);
                    }
                    possibly_ommited = true;
                    continue;
                } else {
                    chi_orb_0.1
                }
                .cmp(&p_q)
                {
                    Equal => {
                        occurences.push(counters.clone());
                        occurences_count += 1;
                        if occurences_count == occurences_limit {
                            possibly_ommited = true;
                            return (occurences, possibly_ommited);
                        } else {
                            flag = Less;
                            pivot += 1;
                            if counters.len() <= pivot {
                                counters.push(1);
                            }
                            continue;
                        }
                    }
                    Less => {
                        flag = Less;
                        pivot += 1;
                        if counters.len() <= pivot {
                            counters.push(1);
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
                counters[pivot] = 0;
                let chi_orb_0 = chi_orb(&counters);
                match if chi_orb_0.0 {
                    flag = Less;
                    pivot += 1;
                    if counters.len() <= pivot {
                        counters.push(1);
                    }
                    possibly_ommited = true;
                    continue;
                } else {
                    chi_orb_0.1
                }
                .cmp(&p_q)
                {
                    Equal => {
                        occurences.push(counters.clone());
                        occurences_count += 1;
                        if occurences_count == occurences_limit {
                            possibly_ommited = true;
                            return (occurences, possibly_ommited);
                        } else {
                            flag = Less;
                            pivot += 1;
                            if counters.len() <= pivot {
                                counters.push(1);
                            }
                            continue;
                        }
                    }
                    Less => {
                        let chi_orb_0 = chi_orb(&counters);

                        let current_chi_orb = if chi_orb_0.0 {
                            flag = Less;
                            pivot += 1;
                            if counters.len() <= pivot {
                                counters.push(1);
                            }
                            possibly_ommited = true;
                            continue;
                        } else {
                            chi_orb_0.1
                        } + Rational64::new(1, 1);
                        let b_c = b_c_value(current_chi_orb, p_q);
                        if b_c.0 {
                            flag = Less;
                            pivot += 1;
                            if counters.len() <= pivot {
                                counters.push(1);
                            }
                            possibly_ommited = true;
                            continue;
                        } else {
                            counters[pivot] = b_c.1;
                        }
                        let chi_orb_0 = chi_orb(&counters);
                        if if chi_orb_0.0 {
                            flag = Less;
                            pivot += 1;
                            if counters.len() <= pivot {
                                counters.push(1);
                            }
                            possibly_ommited = true;
                            continue;
                        } else {
                            chi_orb_0.1
                        } == p_q
                        {
                            occurences.push(counters.clone());
                            occurences_count += 1;
                            if occurences_count == occurences_limit {
                                possibly_ommited = true;
                                return (occurences, possibly_ommited);
                            } else {
                                flag = Less;
                                pivot += 1;
                                if counters.len() <= pivot {
                                    counters.push(1);
                                }
                                continue;
                            }
                        }
                        level_to_b_c(&mut counters, pivot);
                        let chi_orb_0 = chi_orb(&counters);
                        match if chi_orb_0.0 {
                            flag = Less;
                            pivot += 1;
                            if counters.len() <= pivot {
                                counters.push(1);
                            }
                            possibly_ommited = true;
                            continue;
                        } else {
                            chi_orb_0.1
                        }
                        .cmp(&p_q)
                        {
                            Equal => {
                                occurences.push(counters.clone());
                                occurences_count += 1;
                                if occurences_count == occurences_limit {
                                    possibly_ommited = true;
                                    return (occurences, possibly_ommited);
                                } else {
                                    flag = Less;
                                    pivot += 1;
                                    if counters.len() <= pivot {
                                        counters.push(1);
                                    }
                                    continue;
                                }
                            }
                            Less => {
                                flag = Less;
                                pivot += 1;
                                if counters.len() <= pivot {
                                    counters.push(1);
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
                            counters.push(1);
                        }
                        continue;
                    }
                }
            }
        }
    }
}

pub fn chi_orb(counters: &Vec<i64>) -> (bool, Rational64) {
    let mut chi_orb = Rational64::from_integer(2);
    for counter in counters {
        if *counter == 1 {
            break;
        }
        match chi_orb.checked_sub(&period_to_difference(*counter)) {
            Some(diff) => {
                chi_orb = diff;
            }
            None => {
                return (true, Rational64::from_integer(-1));
            }
        }
        //chi_orb -= period_to_difference(*counter);
    }
    (false, chi_orb)
}

fn break_condition(counters: &Vec<i64>, pivot: usize) -> bool {
    let mut break_condition = true;
    for (i, counter) in counters.iter().enumerate() {
        if i > pivot {
            break;
        }
        if *counter != 2 {
            break_condition = false;
        }
    }
    break_condition
}

fn level_to_b_c(counters: &mut Vec<i64>, pivot: usize) {
    let b_c = counters[pivot];
    for (i, counter) in counters.iter_mut().enumerate() {
        if i > pivot {
            break;
        }
        *counter = b_c;
    }
}

pub fn signature_string(counters: &Vec<i64>, manifold: &TwoDimentionalManifold) -> String {
    let mut signature_string = match manifold {
        TwoDimentionalManifold::Disk => "*".to_string(),
        TwoDimentionalManifold::Sphere => "".to_string(),
        TwoDimentionalManifold::Genus(n) => {
            let mut genus_string = "".to_string();
            for _ in 1..*n {
                genus_string += "o";
            }
            genus_string
        }
        TwoDimentionalManifold::General {
            handles: h,
            cross_caps: cc,
            boundry_components: bc,
        } => {
            let mut genus_string = "".to_string();
            for _ in 1..*h {
                genus_string += "o";
            }
            for _ in 1..*cc {
                genus_string += "x";
            }
            for _ in 1..*bc {
                genus_string += "*";
            }
            genus_string
        }
    } + "\t";
    let mut reversed_counters = counters.clone();
    reversed_counters.reverse();
    let reversed_counters = reversed_counters;
    let len = reversed_counters.len();
    for (i, counter) in (&reversed_counters).iter().enumerate() {
        if *counter == 1 {
            continue;
        }

        signature_string += &match *counter {
            0 => "infty".to_string(),
            _ => counter.to_string(),
        };
        if i != len - 1 {
            signature_string += "\t";
        }
    }
    signature_string
}

fn b_c_value(old_chi_orb: Rational64, p_q: Rational64) -> (bool, i64) {
    let mut b_c = 2;
    let mut a = b_c;
    let mut chi_orb_value;
    match old_chi_orb.checked_sub(&period_to_difference(b_c)) {
        None => {
            return (true, -1);
        }
        Some(result) => {
            chi_orb_value = result;
        }
    }
    while chi_orb_value > p_q {
        a = b_c;
        b_c = 2 * b_c;
        match old_chi_orb.checked_sub(&period_to_difference(b_c)) {
            None => {
                return (true, -1);
            }
            Some(result) => {
                chi_orb_value = result;
            }
        }
    }
    let mut b = b_c;
    if old_chi_orb - period_to_difference(b_c) == p_q {
        return (false, b_c);
    }
    if a == b {
        return (false, b_c);
    }
    loop {
        let diff = (b - a) / 2;
        b_c = a + diff;
        match old_chi_orb.checked_sub(&period_to_difference(b_c)) {
            None => {
                return (true, -1);
            }
            Some(result) => {
                chi_orb_value = result;
            }
        }
        match (chi_orb_value).cmp(&p_q) {
            Equal => {
                return (false, b_c);
            }
            Less => {
                if diff == 1 {
                    return (false, b_c);
                }
                b = b_c;
                continue;
            }
            Greater => {
                if diff == 0 {
                    return (false, b);
                }
                a = b_c;
                continue;
            }
        }
    }
}

pub fn period_to_difference(b_n: i64) -> Rational64 {
    match b_n {
        0 => Rational64::new(1, 1),
        _ => Rational64::new(b_n - 1, b_n),
    }
}

pub fn print_order_and_occurences(
    p_q: Rational64,
    p_q_order_and_occurences: &(i64, Vec<Vec<i64>>),
    manifold: &TwoDimentionalManifold,
    config: &PointsOrderAndOccurencesConfig,
) {
    let p_q_order = p_q_order_and_occurences.0;

    let p_q_orbifolds = &p_q_order_and_occurences.1;

    if p_q_order == -1 {
        println!(
            "{}",
            p_q.to_string()
                + " is not an Euler orbicharacteristic of any "
                + &match manifold {
                    TwoDimentionalManifold::Disk => "disk".to_string(),
                    TwoDimentionalManifold::Sphere => "sphere".to_string(),
                    TwoDimentionalManifold::Genus(n) => "genus-".to_string() + &n.to_string(),
                    TwoDimentionalManifold::General {
                        handles: h,
                        cross_caps: cc,
                        boundry_components: bc,
                    } =>
                        "(".to_string()
                            + &h.to_string()
                            + ","
                            + &cc.to_string()
                            + ","
                            + &bc.to_string()
                            + ")",
                }
                + " orbifold."
        );
    } else {
        let p_q_orbifolds_signatures = signature_strings(p_q_orbifolds, manifold);
        let len = p_q_orbifolds.len();
        let number_of_p_q_orbifolds = len.to_string();
        println!(
            "{}",
            p_q.to_string()
                + " "
                + "is an Euler orbicharacteristic of "
                + {
                    if len as i64 == config.maximal_number_of_occurences {
                        "at least "
                    } else {
                        ""
                    }
                }
                + &number_of_p_q_orbifolds
                + " "
                + &match manifold {
                    TwoDimentionalManifold::Disk => "disk".to_string(),
                    TwoDimentionalManifold::Sphere => "sphere".to_string(),
                    TwoDimentionalManifold::Genus(g) => "genus-".to_string() + &g.to_string(),
                    TwoDimentionalManifold::General {
                        handles: h,
                        cross_caps: cc,
                        boundry_components: bc,
                    } =>
                        "(".to_string()
                            + &h.to_string()
                            + ","
                            + &cc.to_string()
                            + ","
                            + &bc.to_string()
                            + ")",
                }
                + match len {
                    1 => " orbifold with signature: \n",
                    _ => " orbifolds with signatures: \n",
                }
                + &p_q_orbifolds_signatures
        );
        println!(
            "{}",
            "and it is an accumulation point of order ".to_string()
                + &(p_q_order.to_string())
                + "."
        );
    }
}

pub fn points_order_and_occurences_string(
    p_q: Rational64,
    p_q_order_and_occurences: &((i64, bool), (Vec<Vec<i64>>, bool)),
    manifold: &TwoDimentionalManifold,
    _maximal_number_of_occurences: i64,
) -> String {
    let p_q_order = p_q_order_and_occurences.0;

    let p_q_orbifolds = &p_q_order_and_occurences.1;

    if p_q_order.0 == -1 {
        p_q.to_string()
            + " is"
            + if p_q_order.1 { " possibly " } else { " " }
            + "not an Euler orbicharacteristic of any "
            + &match manifold {
                TwoDimentionalManifold::Disk => "disk".to_string(),
                TwoDimentionalManifold::Sphere => "sphere".to_string(),
                TwoDimentionalManifold::Genus(n) => "genus-".to_string() + &n.to_string(),
                TwoDimentionalManifold::General {
                    handles: h,
                    cross_caps: cc,
                    boundry_components: bc,
                } => {
                    "(".to_string()
                        + &h.to_string()
                        + ","
                        + &cc.to_string()
                        + ","
                        + &bc.to_string()
                        + ")"
                }
            }
            + " orbifold."
    } else {
        let p_q_orbifolds_signatures = signature_strings(&p_q_orbifolds.0, manifold);
        let len = p_q_orbifolds.0.len();
        let number_of_p_q_orbifolds = len.to_string();
        p_q.to_string()
            + " "
            + "is an Euler orbicharacteristic of "
            + {
                if p_q_orbifolds.1 {
                    "at least "
                } else {
                    ""
                }
            }
            + &number_of_p_q_orbifolds
            + " "
            + &match manifold {
                TwoDimentionalManifold::Disk => "disk".to_string(),
                TwoDimentionalManifold::Sphere => "sphere".to_string(),
                TwoDimentionalManifold::Genus(g) => "genus-".to_string() + &g.to_string(),
                TwoDimentionalManifold::General {
                    handles: h,
                    cross_caps: cc,
                    boundry_components: bc,
                } => {
                    "(".to_string()
                        + &h.to_string()
                        + ","
                        + &cc.to_string()
                        + ","
                        + &bc.to_string()
                        + ")"
                }
            }
            + match len {
                1 => " orbifold with signature: \n",
                _ => " orbifolds with signatures: \n",
            }
            + &p_q_orbifolds_signatures
            + "\n"
            + &"and it is an accumulation point of order ".to_string()
            + if p_q_order.1 { "at least " } else { "" }
            + &(p_q_order.0.to_string())
            + "."
    }
}

pub fn signature_strings(signatures: &Vec<Vec<i64>>, manifold: &TwoDimentionalManifold) -> String {
    let len = signatures.len();
    let mut orbifolds_signatures = "".to_string();
    for (i, orbifold) in signatures.iter().enumerate() {
        orbifolds_signatures.push_str(&signature_string(&orbifold, manifold));
        if i != len - 1 && len != 0 {
            orbifolds_signatures.push_str("\n");
        }
    }
    return orbifolds_signatures;
}
