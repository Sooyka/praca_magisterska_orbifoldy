use num_rational::*;
use std::io;

// use std::{thread, time};
use searching::orbifolds_lib::*;

fn main() {
    enum Menu1 {
        Start,
        Options,
    }
    enum Option1 {
        Maximal,
        Exact,
    }
    enum Option2 {
        Exclusive,
        Inclusive,
    }
    loop {
        println!("Start (1) or Options (2).");
        let mut ui = "".to_string();
        io::stdin()
            .read_line(&mut ui)
            .expect("Failed to read the line");
        let ui:i64 = ui.trim().parse().expect("Please type a number!");
        let menu1:Menu1;
        loop {
            let option1: bool = true;

            println!("Maximal (0) or exact (1)?");
            let mut ui = "".to_string();
            io::stdin()
                .read_line(&mut ui)
                .expect("Failed to read the line");
            let ui = ui.trim().parse().expect("Please type a number!");
          
            println!("Enter maximal denumerator.");
            let mut q_max = "".to_string();
            io::stdin()
                .read_line(&mut q_max)
                .expect("Failed to read the line");
            let q_max = q_max.trim().parse().expect("Please type a number!");
            let mut q = 1;
            let mut p_q;
            let mut no_count = 0;
            let mut yes_count = 0;
            let mut no_denominators = "".to_string();
            match ui {
                0 => {}
                1 => {
                    q = q_max;
                }
                _ => {
                    panic!();
                }
            }
            while q <= q_max {
                for p in 1..q {
                    p_q = Rational64::new(-p, q);
                    if *p_q.denom() == q {
                        let p_q_order_and_first_occurence = determine_points_order(p_q);
                        let p_q_order = p_q_order_and_first_occurence.0;
                        match p_q_order {
                            -1 => {
                                no_count += 1;
                            }
                            _ => {
                                yes_count += 1;
                            }
                        }
                        println!(
                            "{}",
                            [&p_q.to_string(), "\t\t", &p_q_order.to_string()].concat()
                        );
                    }
                }
                if no_count > yes_count {
                    println!("STOP");
                    println!(
                        "{}",
                        [
                            "no: ",
                            &no_count.to_string(),
                            " ",
                            "yes: ",
                            &yes_count.to_string()
                        ]
                        .concat()
                    );
                    println!("{}", q.to_string());
                    // let sleep_time = time::Duration::from_millis(5000);
                    // thread::sleep(sleep_time);
                    no_denominators = [no_denominators, q.to_string()].concat();
                }
                no_count = 0;
                yes_count = 0;
                println!("");
                q += 1;
            }
            println!("{}", no_denominators);
        }
    }
}
