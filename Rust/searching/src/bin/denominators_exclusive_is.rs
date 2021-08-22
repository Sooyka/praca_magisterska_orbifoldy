use num_rational::*;
use std::io;

// use std::{thread, time};
use searching::denominators_lib::*;
use searching::order_and_occurences_lib::*;
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
        let output = &config.output;
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

        if config.maximal_exact == DenominatorsMaximalExact::Maximal {
            q = q_max;
        }
        while q <= q_max {
            for p in 1..q {
                p_q = Rational64::new(-p, q);
                if *p_q.denom() == q
                    || config.exclusive_inclusive == DenominatorsExclusiveInclusive::Inclusive
                {
                    let p_q_order_and_first_occurence = determine_points_order(p_q);
                    let p_q_order = p_q_order_and_first_occurence.0;
                    let mut p_q_occurences = vec![];
                    if config.occurences {
                        p_q_occurences = search_for_point(p_q, config.output.occurences);
                    }
                    let p_q_occurences = p_q_occurences;
                    let number_of_p_q_occurences = p_q_occurences.len();
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
                        match output.provided_p_q {
                            true => "provided_p_q:".to_string() + "\t\t",
                            false => "".to_string(),
                        } + &match output.p_q {
                            true => "p_q:".to_string() + "\t\t",
                            false => "".to_string(),
                        } + &match output.p_q_order {
                            true => "p_q_order:".to_string() + "\t\t",
                            false => "".to_string(),
                        } + &match output.number_of_p_q_occurences {
                            true => "number_of_p_q_occurences:".to_string() + "\t\t",
                            false => "".to_string(),
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
                        } + &match config.occurences {
                            true =>
                                "\n\n".to_string()
                                    + "occurences:"
                                    + "\n"
                                    + &match number_of_p_q_occurences {
                                        0 => "none".to_string(),
                                        _ => signature_strings(&p_q_occurences),
                                    },
                            false => "".to_string(),
                        }
                    );
                }
                println!("\n\n");
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
