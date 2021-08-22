use num_rational::*;
use std::io;

use searching::*;
use searching::points_order_and_occurences_lib::*;

fn main() {
    let p_q_set = false;
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

        io::stdin()
            .read_line(&mut p_q)
            .expect("Failed to read the line");

        let p_q: Vec<i64> = p_q
            .split('/')
            .map(|number_string| number_string.trim().parse().expect("Please type a number!"))
            .collect();

        let p_q = Rational64::new(p_q[0], p_q[1]);
        p_q_set = true;

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
        let p_q_order_and_occurences = points_order_and_occurences(p_q, 0);

        print_order_and_occurences(p_q, &p_q_order_and_occurences);
    }
}
