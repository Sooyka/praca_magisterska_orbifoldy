use num_rational::*;
use std::io;

// use std::{thread, time};
use searching::denominators_lib::*;
use searching::points_order_and_occurences_lib::*;
use searching::*;

use crate::common_lib::TwoDimentionalManifold::*;

use searching::denominators_lib::DenominatorsMaximalExact::*;

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
            Maximal => {
                println!("Enter maximal denumerator.");
            }
            Exact => {
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
        if config.maximal_exact == Exact {
            q = q_max;
        }
        while q <= q_max {
            for p in 1..q {
                let mut is_sphere: bool = false;
                let mut is_disk: bool = false;
                p_q = Rational64::new(-p, q);
                let mut p_q_order = -1;
                let mut number_of_p_q_occurences = 0;
                let mut p_q_order_and_occurences_outside: ((i64, bool), (Vec<Vec<i64>>, bool)) =
                    ((-1, true), (vec![], true));
                let mut p_q_order_and_occurences;
                let mut p_q_order_possible_missing = true;
                for base_manifold in &config.base_manifolds {
                    if *p_q.denom() == q || config.only_relatively_prime_numerators == false {
                        // let mut p_q_occurences = vec![];
                        if config.order_and_occurences > -1  && *base_manifold == Sphere {
                            p_q_order_and_occurences = points_order_and_occurences(
                                p_q,
                                base_manifold,
                                config.order_and_occurences,
                            );
                            p_q_order = p_q_order_and_occurences.0 .0;
                            number_of_p_q_occurences = p_q_order_and_occurences.1 .0.len() as i64;
                            // p_q_occurences = p_q_order_and_occurences.1;
                        } else {
                            let p_q_order_and_first_occurence =
                                determine_points_order(p_q, base_manifold);
                            p_q_order = p_q_order_and_first_occurence.0;
                            number_of_p_q_occurences = if p_q_order == -1 { 0 } else { 1 };
                            p_q_order_and_occurences = ((-1, true), (vec![], true))
                        }
                        // let p_q_occurences = p_q_occurences;
                        let p_q_order = p_q_order;
                        let number_of_p_q_occurences = number_of_p_q_occurences;
                        let p_q_order_and_occurences = p_q_order_and_occurences;
                        p_q_order_possible_missing = p_q_order_and_occurences.0.1.clone();
                        p_q_order_and_occurences_outside = p_q_order_and_occurences.clone();
                        if *base_manifold == Sphere && p_q_order > -1 {
                            is_sphere = true;
                        }
                        if *base_manifold == Disk && p_q_order > -1 {
                            is_disk = true;
                        }
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
                        if output.provided_p_q
                            || output.p_q
                            || output.p_q_order
                            || output.number_of_p_q_occurences
                        {
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
                                if output.provided_p_q {
                                    (-p).to_string() + "/" + &q.to_string() + "\t\t\t"
                                } else {
                                    "".to_string()
                                } + &if output.p_q {
                                    p_q.to_string() + "\t\t\t"
                                } else {
                                    "".to_string()
                                } + &if output.p_q_order {
                                    if p_q_order_and_occurences.0 .1 {
                                        "at least "
                                    } else {
                                        ""
                                    }
                                    .to_string()
                                        + &p_q_order.to_string()
                                        + "\t\t\t"
                                } else {
                                    "".to_string()
                                } + &if output.number_of_p_q_occurences {
                                    number_of_p_q_occurences.to_string() + "\t\t\t"
                                } else {
                                    "".to_string()
                                } + &if config.output.order_and_occurences {
                                    "\n\n".to_string()
                                        + "occurences:"
                                        + "\n"
                                        + &points_order_and_occurences_string(
                                            p_q,
                                            &(p_q_order_and_occurences),
                                            base_manifold,
                                            config.order_and_occurences,
                                        )
                                } else {
                                    "".to_string()
                                }
                            );
                            println!("\n\n");
                        }
                    }
                }
                let disk_sphere_output = true;
                if disk_sphere_output {
                    if is_sphere && !is_disk {
                        println!(
                            "{}",
                            if output.provided_p_q || disk_sphere_output {
                                "provided_p_q:".to_string() + "\t\t"
                            } else {
                                "".to_string()
                            } + &if output.p_q || disk_sphere_output {
                                "p_q:".to_string() + "\t\t"
                            } else {
                                "".to_string()
                            } + "sphere"
                                + "\t\t"
                                + "disk"
                                + "\t\t"
                                + &"p_q_order:".to_string()
                                + "\t\t"
                                + &"number_of_p_q_occurences:".to_string()
                        );
                        println!(
                            "{}",
                            if output.provided_p_q || disk_sphere_output {
                                (-p).to_string() + "/" + &q.to_string() + "\t\t\t"
                            } else {
                                "".to_string()
                            } + &if output.p_q || disk_sphere_output {
                                p_q.to_string() + "\t\t"
                            } else {
                                "".to_string()
                            } + &is_sphere.to_string()
                                + "\t\t"
                                + &is_disk.to_string()
                                + "\t\t\t"
                                + if p_q_order_possible_missing {
                                    "at least "
                                } else {
                                    ""
                                }
                                + &p_q_order.to_string()
                                + "\t\t\t"
                                + &number_of_p_q_occurences.to_string()
                                + "\n"
                                + "occurences:"
                                + "\n"
                                + &points_order_and_occurences_string(
                                    p_q,
                                    &(p_q_order_and_occurences_outside),
                                    &Sphere,
                                    config.order_and_occurences,
                                )
                        );
                    }
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
                println!("\n\n\n\n");
            }
            if config.yes_no_counting {
                no_count = 0;
                yes_count = 0;
            }
            q += 1;
        }
    }
}
