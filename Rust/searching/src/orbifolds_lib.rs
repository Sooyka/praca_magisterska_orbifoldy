use num_rational::*;
use num_traits::ops::checked::CheckedSub;
use std::cmp::*;
use Ordering::*;


pub fn point_order_and_occurences(p_q: Rational64, occurences_limit: i64) -> (i64, Vec<Vec<i64>>) {
    let points_order_and_first_occurence = determine_points_order(p_q);
    let p_q_order = points_order_and_first_occurence.0;
    let p_q_occurences = search_for_point(p_q, occurences_limit);
    (p_q_order, p_q_occurences)
}


pub fn determine_points_order(p_q: Rational64) -> (i64, Vec<i64>) {
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
    let l = _four * p_q;
    if l.is_integer() {
        let l = l.to_integer();
        if l > 4 {
            return (p_q_order, vec![]);
        } else {
            while chi_orb(&counters) - _one_over_four > p_q {
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

        while _one - distance >= p_q + _one_over_two {
            distance += _one_over_two;
            depth += 1;
        }

        let mut current_point = p_q + distance;
        let mut found = vec![];
        while found.len() == 0 && depth >= 0 {
            found = search_for_point(current_point, 1);
            if found.len() == 0 {
                depth -= 1;
                current_point -= _one_over_two;
            }
        }
        if found.len() > 0 {
            p_q_first_occurence = found[0].clone();
        }
        p_q_order = depth;
        counters = p_q_first_occurence;
    }
    return (p_q_order, counters);
}


pub fn search_for_point(p_q: Rational64, occurences_limit: i64) -> Vec<Vec<i64>> {
    let mut occurences: Vec<Vec<i64>> = vec![];
    let mut counters: Vec<i64> = vec![1];
    let mut occurences_count = 0;
    let mut pivot = 0;
    let mut flag = chi_orb(&counters).cmp(&p_q);
    loop {
        match flag {
            Equal => {
                occurences.push(counters.clone());
                occurences_count += 1;
                if occurences_count == occurences_limit {
                    return occurences;
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
                    return occurences;
                }
                level_to_b_c(&mut counters, pivot);
                match chi_orb(&counters).cmp(&p_q) {
                    Equal => {
                        occurences.push(counters.clone());
                        occurences_count += 1;
                        if occurences_count == occurences_limit {
                            return occurences;
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
                match chi_orb(&counters).cmp(&p_q) {
                    Equal => {
                        occurences.push(counters.clone());
                        occurences_count += 1;
                        if occurences_count == occurences_limit {
                            return occurences;
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
                        let current_chi_orb = chi_orb(&counters) + Rational64::new(1, 2);
                        let b_c = b_c_value(current_chi_orb, p_q);
                        if b_c.0 {
                            flag = Less;
                            pivot += 1;
                            if counters.len() <= pivot {
                                counters.push(1);
                            }
                            continue;
                        } else {
                            counters[pivot] = b_c.1;
                        }
                        if chi_orb(&counters) == p_q {
                            occurences.push(counters.clone());
                            occurences_count += 1;
                            if occurences_count == occurences_limit {
                                return occurences;
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
                        match chi_orb(&counters).cmp(&p_q) {
                            Equal => {
                                occurences.push(counters.clone());
                                occurences_count += 1;
                                if occurences_count == occurences_limit {
                                    return occurences;
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


pub fn chi_orb(counters: &Vec<i64>) -> Rational64 {
    let mut chi_orb = Rational64::from_integer(1);
    for counter in counters {
        if *counter == 1 {
            break;
        }
        chi_orb -= period_to_difference(*counter);
    }
    chi_orb
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


pub fn signature_string(counters: &Vec<i64>) -> String {
    let mut signature_string = String::from("*");
    let mut reversed_counters = counters.clone();
    reversed_counters.reverse();
    for counter in &reversed_counters {
        if *counter == 1 {
            continue;
        }

        signature_string = signature_string
            + "\t"
            + &match *counter {
                0 => "infty".to_string(),
                _ => counter.to_string(),
            };
    }
    signature_string
}

fn b_c_value(old_chi_orb: Rational64, p_q: Rational64) -> (bool, i64) {
    let mut b_c = 2;
    let mut a = b_c;
    let mut difference = period_to_difference(b_c);
    let mut chi_orb_value_option = old_chi_orb.checked_sub(&difference);
    let mut chi_orb_value;
    match chi_orb_value_option {
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
        difference = period_to_difference(b_c);
        chi_orb_value_option = old_chi_orb.checked_sub(&difference);
        match chi_orb_value_option {
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
        difference = period_to_difference(b_c);
        chi_orb_value_option = old_chi_orb.checked_sub(&difference);
        match chi_orb_value_option {
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
        0 => Rational64::new(1, 2),
        _ => Rational64::new(b_n - 1, 2 * b_n),
    }
}


pub fn print_order_and_occurences(p_q: Rational64, p_q_order_and_occurences: (i64, Vec<Vec<i64>>)) {
    let p_q_order = p_q_order_and_occurences.0;

    let p_q_orbifolds = p_q_order_and_occurences.1;

    if p_q_order == -1 {
        println!(
            "{}",
            [
                &p_q.to_string(),
                " ",
                " is not an Euler orbicharacteristic of any orbifold."
            ]
            .concat()
        );
    } else {
        let mut p_q_orbifolds_signatures = String::from("");
        let number_of_p_q_orbifolds = &p_q_orbifolds.len().to_string();
        for orbifold in p_q_orbifolds {
            p_q_orbifolds_signatures.push_str(&signature_string(&orbifold));
            p_q_orbifolds_signatures.push_str("\n");
        }
        println!(
            "{}",
            [
                &p_q.to_string(),
                " ",
                "is an Euler orbicharacteristic of ",
                number_of_p_q_orbifolds,
                " orbifolds with signatures: \n",
                &p_q_orbifolds_signatures
            ]
            .concat()
        );
        println!(
            "{}",
            [
                "and it is an accumulation point of order ",
                &(p_q_order.to_string()),
                "."
            ]
            .concat()
        );
    }
}
