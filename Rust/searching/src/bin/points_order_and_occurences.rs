use num_rational::*;
use std::error::Error;
use std::io;

use searching::points_order_and_occurences_lib::*;
use searching::*;

fn main() {
    let _zero = Rational64::from_integer(0);
    let _one_over_two = Rational64::new(1, 2);
    let _one_over_four = Rational64::new(1, 4);
    let _one = Rational64::from_integer(1);
    let _two = Rational64::from_integer(2);
    let _three = Rational64::from_integer(3);
    let _four = Rational64::from_integer(4);
    loop {
        match read_config("points_order_and_occurences".to_string())
            as Result<PointsOrderAndOccurencesConfig, Box<dyn Error>>
        {
            Ok(_) => {}
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
            }
        };
        println!("Enter a rational number.");

        let mut p_q = String::new();
        io::stdin()
            .read_line(&mut p_q)
            .expect("Failed to read the line");

        let p_q: Vec<i64> = p_q
            .split('/')
            .map(|number_string| number_string.trim().parse().expect("Please type a number!"))
            .collect();

        let p_q = Rational64::new(p_q[0], p_q[1]);
        let config: PointsOrderAndOccurencesConfig;
        loop {
            config = match read_config("points_order_and_occurences".to_string()) {
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
                }
            };
            break;
        }
        for base_manifold in &config.base_manifolds {
            let p_q_order_and_occurences = points_order_and_occurences(
                p_q,
                &base_manifold,
                config.maximal_number_of_occurences,
            );

            println!(
                "{}",
                &points_order_and_occurences_string(
                    p_q,
                    &p_q_order_and_occurences,
                    &base_manifold,
                    config.maximal_number_of_occurences
                )
            );
        }
    }
}
