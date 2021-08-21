use num_rational::*;
use std::io;

use searching::order_and_occurences_lib::*;

fn main() {
    loop {
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

        let p_q_order_and_occurences = point_order_and_occurences(p_q, 0);

        print_order_and_occurences(p_q, &p_q_order_and_occurences);
    }
}
