use num_rational::*;
use std::io;

mod orbifolds_lib;
use orbifolds_lib::*;

fn main() {
    loop {
        println!("Enter maximal denumerator.");
        let mut q_max = "".to_string();
        io::stdin()
            .read_line(&mut q_max)
            .expect("Failed to read the line");
        let q_max = q_max.trim().parse().expect("Please type a number!");
        let mut q = 1;
        let mut p_q;
        while q <= q_max {
            for p in 1..q {
                p_q = Rational64::new(-p, q);
                if *p_q.denom() == q {
                    let p_q_order_and_first_occurence = determine_points_order(p_q);
                    let p_q_order = p_q_order_and_first_occurence.0;
                    let p_q_occurences = search_for_point(p_q, 0);
                    let mut p_q_first_occurence = &vec![];
                    let length = p_q_occurences.len();
                    if length != 0 {
                        p_q_first_occurence = &p_q_occurences[0];
                    }
                    println!(
                        "{}",
                        [
                            &(-p).to_string(),
                            "/",
                            &q.to_string(),
                            "\t\t",
                            &p_q.to_string(),
                            "\t\t",
                            &p_q_order.to_string(),
                            "\t\t",
                            &(&p_q_occurences).len().to_string(),
                            "\t\t",
                            &match length {
                                0 => "none".to_string(),
                                _ => signature_string(p_q_first_occurence),
                            }
                        ]
                        .concat()
                    );
                }
            }
            println!("");
            q += 1;
        }
    }
}
