use num_rational::*;
use std::error::Error;
use std::io;

use backend_lib::*;
use mathematics_lib::ExWh::*;
use mathematics_lib::ExRa::*;
use mathematics_lib::TwoDimentionalManifold::*;
use mathematics_lib::*;
use searching::*;
// use searching_algorithm_lib::*;
fn main() {
    // println!("dupa");
    println!("{}", ExRa::Rational(Rational64::new(7, 3)));
    // for start in vec![30, 300, 3000, 30000, 300000, 3000000] {
    //     let range = (start)..(start + 12);
    //     let timer = std::time::Instant::now();
    //     for a1 in range.clone() {
    //         for a2 in range.clone() {
    //             for a3 in range.clone() {
    //                 for a4 in range.clone() {
    //                     for a5 in range.clone() {
    //                         for a6 in range.clone() {
    //                             // println!("d");
    //                             // println!("{}",per_chi_orb(vec![Natural(a1),Natural(a2),Natural(a3),Natural(a4),Natural(a5),Natural(a6)]))
    //                             per_chi_orb_ref(vec![
    //                                 Natural(a1),
    //                                 Natural(a2),
    //                                 Natural(a3),
    //                                 Natural(a4),
    //                                 Natural(a5),
    //                                 Natural(a6),
    //                             ]);
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     println!("{start:20} {:?}", timer.elapsed());
    // }
    // println!("Orb:");
    // for start in vec![30, 300, 3000, 30000, 300000, 3000000] {
    //     let range = (start)..(start + 12);
    //     let timer = std::time::Instant::now();
    //     for a1 in range.clone() {
    //         for a2 in range.clone() {
    //             for a3 in range.clone() {
    //                 for a4 in range.clone() {
    //                     for a5 in range.clone() {
    //                         for a6 in range.clone() {
    //                             // println!("d");
    //                             // println!("{}",per_chi_orb(vec![Natural(a1),Natural(a2),Natural(a3),Natural(a4),Natural(a5),Natural(a6)]))
    //                             chi_orb(&TwoDimentionalOrbifold {
    //                                 b_m: Sphere,
    //                                 r: vec![
    //                                     Natural(a1),
    //                                     Natural(a2),
    //                                     Natural(a3),
    //                                     Natural(a4),
    //                                     Natural(a5),
    //                                     Natural(a6),
    //                                 ],
    //                                 d: vec![],
    //                             });
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     println!("{start:20} {:?}", timer.elapsed());
    // }
    // println!("Copy:");
    // for start in vec![30, 300, 3000, 30000, 300000, 3000000] {
    //     let range = (start)..(start + 12);
    //     let timer = std::time::Instant::now();
    //     for a1 in range.clone() {
    //         for a2 in range.clone() {
    //             for a3 in range.clone() {
    //                 for a4 in range.clone() {
    //                     for a5 in range.clone() {
    //                         for a6 in range.clone() {
    //                             // println!("d");
    //                             // println!("{}",per_chi_orb(vec![Natural(a1),Natural(a2),Natural(a3),Natural(a4),Natural(a5),Natural(a6)]))
    //                             per_chi_orb(vec![
    //                                 Natural(a1),
    //                                 Natural(a2),
    //                                 Natural(a3),
    //                                 Natural(a4),
    //                                 Natural(a5),
    //                                 Natural(a6),
    //                             ]);
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     println!("{start:20} {:?}", timer.elapsed());
    // }
}
