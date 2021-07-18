use num_rational::*;
use std::cmp::*;
use std::io;

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

    let p_q = Rational32::new(2, 3);
    // let n = 4 * p_q.to_integer() + 4 + 4;
    // let mut counters: [i32; n];
    let mut counters: Vec<i32> = vec![1];
    let mut pivot = 0;
    let mut flag = Ordering::Greater;

    loop {
        match flag {
            Ordering::Equal => {}
            Ordering::Less => {
                counters[pivot] += 1;
                let b_c = counters[pivot];
                let mut break_condition = true;
                for (i, counter) in counters.iter().enumerate() {
                    if i > pivot {
                        break;
                    }
                    if *counter != 2 {
                        break_condition = false;
                    }
                }
                if break_condition == true {
                    println!("No");
                    break;
                }
                for (i, counter) in counters.iter_mut().enumerate() {
                    if i > pivot {
                        break;
                    }
                    *counter = b_c;
                }
                match chi_orb(&counters).cmp(&p_q) {
                    Ordering::Equal => {
                        
                    }
                    Ordering::Less => {}
                    Ordering::Greater => {}
                }
            }
            Ordering::Greater => {}
        }
    }
    // let _var: i32 = 1 ;
    // let _var = _var as i64;
}

fn chi_orb(counters: &Vec<i32>) -> Rational32 {
    
}
