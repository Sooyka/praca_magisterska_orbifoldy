use backend_lib::*;
use backend_lib::ExWh::*;
use mathematics_lib::*;
use searching::*;

fn main() {
    let limit = 10000;
    let mut found = false;
    let mut periods1 = vec![ExWh::PInfty, Whole(4)];
    let mut periods2 = vec![Whole(1), Whole(1)];
    let mut value1 = per_chi_orb(&periods1);
    let mut value2 = per_chi_orb(&periods2);

    for c1 in 2..limit {
        for c2 in 2..limit {
            for c3 in c2..limit {
                periods1 = vec![ExWh::PInfty, Whole(c1)];
                periods2 = vec![Whole(c3), Whole(c2)];
                value1 = per_chi_orb(&periods1);
                value2 = per_chi_orb(&periods2);
                if value1 == value2 {
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
        if found {
            break;
        }
    }
    println!("{:?} {:?} {} {}",periods1, periods2, value1, value2);
}
