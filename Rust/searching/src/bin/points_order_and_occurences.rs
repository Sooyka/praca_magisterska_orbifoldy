use num_rational::*;
use std::io;

use searching::points_order_and_occurences_lib::*;
use searching::*;

use crate::common_lib::*;

fn main() {
    loop {
        let config: &PointsOrderAndOccurencesConfig =
            &match read_config("points_order_and_occurences".to_string()) {
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
        println!("Enter a rational number.");

        let mut p_q = String::new();
        //let p_q: Vec<i64> = vec![];
        io::stdin()
            .read_line(&mut p_q)
            .expect("Failed to read the line");

        let p_q: Vec<i64> = p_q
            .split('/')
            .map(|number_string| number_string.trim().parse().expect("Please type a number!"))
            .collect();

        let p_q = Rational64::new(p_q[0], p_q[1]);
        loop {
            let config: &PointsOrderAndOccurencesConfig =
                &match read_config("points_order_and_occurences".to_string()) {
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

        match 

        let p_q_order_and_occurences = points_order_and_occurences(p_q, 0);
        let disk: TwoDimentionalManifold = TwoDimentionalManifold {
            handles: 0,
            cross_caps: 0,
            boundry_components: 1,
        };
        let sphere: TwoDimentionalManifold = TwoDimentionalManifold {
            handles: 0,
            cross_caps: 0,
            boundry_components: 0,
        };
        print_order_and_occurences(p_q, &p_q_order_and_occurences, &disk);
    }
}
