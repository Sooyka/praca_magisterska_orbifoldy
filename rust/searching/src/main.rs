use num_rational::*;
use std::cmp::*;
use std::io;
use Ordering::*;

// enum Flag {
//     Dreater,
//     Smaller,
// }

fn main() {
    println!("Enter a rational number.");

    let mut p_q = String::new();

    io::stdin()
        .read_line(&mut p_q)
        .expect("Failed to read the line");

    let p_q: Vec<i32> = p_q
        .split('/')
        .map(|number_string| number_string.trim().parse().expect("Please type a number!"))
        .collect();

    // let p_q: Vec<&i32> = p_q

    let p_q = Rational32::new(p_q[0], p_q[1]);
    // let n = 4 * p_q.to_integer() + 4 + 4;
    // let mut counters: [i32; n];
    let mut counters: Vec<i32> = vec![1];
    let mut pivot = 0;
    let mut flag = Greater;
    let l = Rational32::from_integer(4 as i32) * p_q;
    if l.is_integer() {
        let l = l.to_integer();
        // match l.cmp(&4) {
        //     Greater => {

        //     }
        //     _ => {

        //     }
        // }
        if l > 4 {
            println!("No");
            return ();
        } else {
            if l % 2 == 0 {
                counters[0] = 0;
            }
            while chi_orb(&counters) > p_q {
                counters.push(0);
            }
            println!("{}", ["This rational number is an Euler orbicharacteristic of an orbifold with signature ", &signature_string(&counters)].concat());
            println!(
                "{}",
                [
                    "and it is a condensation point of order ",
                    &(if l % 2 == 0 { (l - 4) / 2 } else { (l - 3) / 2 }).to_string()
                ]
                .concat()
            );
            return ();
        }
    }
    search_point()
    loop {
        match flag {
            Equal => {
                println!("{}", ["Yes, ", &signature_string(&counters)].concat());
                break;
            }
            Less => {
                counters[pivot] += 1;
                if break_condition(&counters, pivot) == true {
                    println!("No");
                    return ();
                }
                level_to_b_c(&mut counters, pivot);
                match chi_orb(&counters).cmp(&p_q) {
                    Equal => {
                        println!("{}", ["Yes, ", &signature_string(&counters)].concat());
                        return ();
                    }
                    Less => {
                        flag = Less;
                        pivot += 1;
                        if counters.len() <= pivot {
                            counters.push(1 as i32);
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
                        println!("{}", ["Yes, ", &signature_string(&counters)].concat());
                        return ();
                    }
                    Less => {
                        let current_chi_orb = chi_orb(&counters) + Rational32::new(1, 2);
                        counters[pivot] = b_c_value(current_chi_orb, p_q);
                        if chi_orb(&counters) == p_q {
                            println!("{}", ["Yes, ", &signature_string(&counters)].concat());
                            break;
                        }
                        level_to_b_c(&mut counters, pivot);
                        match chi_orb(&counters).cmp(&p_q) {
                            Equal => {
                                println!("{}", ["Yes, ", &signature_string(&counters)].concat());
                                return ();
                            }
                            Less => {
                                flag = Less;
                                pivot += 1;
                                if counters.len() <= pivot {
                                    counters.push(1 as i32);
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
                            counters.push(1 as i32);
                        }
                        continue;
                    }
                }
            }
        }
    }
    // let _var: i32 = 1 ;
    // let _var = _var as i64;
}

fn chi_orb(counters: &Vec<i32>) -> Rational32 {
    let mut chi_orb = Rational32::from_integer(1 as i32);
    for counter in counters {
        if *counter == 1 {
            break;
        }
        chi_orb -= period_to_difference(*counter);
    }
    chi_orb
}

fn break_condition(counters: &Vec<i32>, pivot: usize) -> bool {
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

fn level_to_b_c(counters: &mut Vec<i32>, pivot: usize) {
    let b_c = counters[pivot];
    for (i, counter) in counters.iter_mut().enumerate() {
        if i > pivot {
            break;
        }
        *counter = b_c;
    }
}

fn signature_string(counters: &Vec<i32>) -> String {
    let mut signature_string = String::from("*");
    for counter in counters {
        if *counter == 1 {
            break;
        }
        signature_string = [&signature_string, " ", &(*counter).to_string()].concat();
    }
    signature_string
}

fn b_c_value(old_chi_orb: Rational32, p_q: Rational32) -> i32 {
    let mut b_c = 2;
    // let mut new_chi_orb = old_chi_orb;
    let mut a = b_c;
    while old_chi_orb - period_to_difference(b_c) > p_q {
        a = b_c;
        b_c = 2 * b_c;
    }
    let mut b = b_c;
    if old_chi_orb - period_to_difference(b_c) == p_q {
        return b_c;
    }
    loop {
        let diff = (b - a) / 2;
        b_c = a + diff;
        match (old_chi_orb - period_to_difference(b_c)).cmp(&p_q) {
            Equal => {
                return b_c;
            }
            Less => {
                if diff == 1 {
                    return b_c;
                }
                b = b_c;
                continue;
            }
            Greater => {
                a = b_c;
                continue;
            }
        }
    }
}

fn period_to_difference(b_n: i32) -> Rational32 {
    match b_n {
        0 => Rational32::new(1, 2),
        _ => Rational32::new(b_n - 1, 2 * b_n),
    }
}
