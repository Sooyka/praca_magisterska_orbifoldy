use num_rational::*;
use std::io;

// use std::{thread, time};
use searching::denominators_lib::*;
use searching::points_order_and_occurences_lib::*;
use searching::*;

fn main() {
    loop {
        let config: &DenominatorsConfig = &match read_config("denominators".to_string()) {
            Ok(c) => c,
            Err(err) => {
                println!(
                    "{}",
                    err.to_string() + "\nPress Enter when the issue is resolved."
                );
                let mut ui = "".to_string();
                io::stdin()
                    .read_line(&mut ui)
                    .expect("Failed to read the line");
                continue;
                // break;
            }
        };
        match &config.maximal_exact {
            DenominatorsMaximalExact::Maximal => {
                println!("Enter maximal denumerator.");
            }
            DenominatorsMaximalExact::Exact => {
                println!("Enter denumerator.");
            }
        }
        let mut q_max = "".to_string();
        io::stdin()
            .read_line(&mut q_max)
            .expect("Failed to read the line");
        let q_max = q_max.trim().parse().expect("Please type a number!");
        let mut q = 1;
        let mut p_q;
        let mut no_count = 0;
        let mut yes_count = 0;
        let config: DenominatorsConfig;
        loop {
            config = match read_config("denominators".to_string()) {
                Ok(c) => c,
                Err(err) => {
                    println!(
                        "{}",
                        err.to_string() + "\nPress Enter when the issue is resolved."
                    );
                    let mut ui = "".to_string();
                    io::stdin()
                        .read_line(&mut ui)
                        .expect("Failed to read the line");
                    continue;
                    // break;
                }
            };
            break;
        }
        let output = &config.output;
        if config.maximal_exact == DenominatorsMaximalExact::Maximal {
            q = q_max;
        }
        let base_manifold = &config.base_manifold;
        while q <= q_max {
            for p in 1..q {
                p_q = Rational64::new(-p, q);
                if *p_q.denom() == q || config.only_relatively_prime_numerators == false {
                    let p_q_order;
                    // let mut p_q_occurences = vec![];
                    let number_of_p_q_occurences;
                    let mut p_q_order_and_occurences = (-1, vec![]);
                    if config.order_and_occurences > -1 {
                        p_q_order_and_occurences = points_order_and_occurences(
                            p_q,
                            base_manifold,
                            config.order_and_occurences,
                        );
                        p_q_order = p_q_order_and_occurences.0;
                        number_of_p_q_occurences = p_q_order_and_occurences.1.len() as i64;
                        // p_q_occurences = p_q_order_and_occurences.1;
                    } else {
                        let p_q_order_and_first_occurence =
                            determine_points_order(p_q, base_manifold);
                        p_q_order = p_q_order_and_first_occurence.0;
                        number_of_p_q_occurences = if p_q_order == -1 { 0 } else { 1 };
                    }
                    // let p_q_occurences = p_q_occurences;
                    let p_q_order_and_occurences = p_q_order_and_occurences;
                    if config.yes_no_counting {
                        match p_q_order {
                            -1 => {
                                no_count += 1;
                            }
                            _ => {
                                yes_count += 1;
                            }
                        }
                    }
                    println!(
                        "{}",
                        if output.provided_p_q {
                            "provided_p_q:".to_string() + "\t\t"
                        } else {
                            "".to_string()
                        } + &if output.p_q {
                            "p_q:".to_string() + "\t\t"
                        } else {
                            "".to_string()
                        } + &if output.p_q_order {
                            "p_q_order:".to_string() + "\t\t"
                        } else {
                            "".to_string()
                        } + &if output.number_of_p_q_occurences {
                            "number_of_p_q_occurences:".to_string() + "\t\t"
                        } else {
                            "".to_string()
                        }
                    );
                    println!(
                        "{}",
                        match output.provided_p_q {
                            true => (-p).to_string() + "/" + &q.to_string() + "\t\t\t",
                            false => "".to_string(),
                        } + &match output.p_q {
                            true => p_q.to_string() + "\t\t\t",
                            false => "".to_string(),
                        } + &match output.p_q_order {
                            true => p_q_order.to_string() + "\t\t\t",
                            false => "".to_string(),
                        } + &match output.number_of_p_q_occurences {
                            true => number_of_p_q_occurences.to_string() + "\t\t\t",
                            false => "".to_string(),
                        } + &match config.output.order_and_occurences {
                            true =>
                                "\n\n".to_string()
                                    + "occurences:"
                                    + "\n"
                                    + &points_order_and_occurences_string(
                                        p_q,
                                        &(p_q_order_and_occurences),
                                        base_manifold,
                                        config.order_and_occurences
                                    ),
                            false => "".to_string(),
                        }
                    );
                    println!("\n\n");
                }
            }
            if config.yes_no_counting && config.output.yes_no_counting {
                println!(
                    "{}",
                    "no: ".to_string()
                        + &no_count.to_string()
                        + " "
                        + "yes: "
                        + &yes_count.to_string()
                );
            }
            println!("\n\n\n\n");
            if config.yes_no_counting {
                no_count = 0;
                yes_count = 0;
            }
            q += 1;
        }
    }
}
