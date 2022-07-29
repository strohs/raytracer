// use rand::prelude::*;
//
// pub fn pi_estimate() {
//     const N: i32 = 1000;
//     let mut inside_circle = 0;
//     let mut rng = thread_rng();
//
//     for _ in 0..N {
//         let x = rng.gen_range(-1.0..1.0);
//         let y = rng.gen_range(-1.0..1.0);
//
//         if x * x + y * y < 1.0 {
//             inside_circle += 1;
//         }
//     }
//
//     let estimate = (4 * inside_circle) as f64 / N as f64;
//     println!("Estimate of Pi = {:.12}", estimate);
// }
//
// pub fn pi_estimate_convergence() {
//     let mut inside_circle = 0;
//     let mut runs = 0;
//     let mut rng = thread_rng();
//
//     loop {
//         runs += 1;
//         let x = rng.gen_range(-1.0..1.0);
//         let y = rng.gen_range(-1.0..1.0);
//
//         if x * x + y * y < 1.0 {
//             inside_circle += 1;
//         }
//
//         if runs % 100_000 == 0 {
//             let estimate = (4 * inside_circle) as f64 / runs as f64;
//             println!("Estimate of Pi = {:.12}", estimate);
//         }
//     }
// }
//
// pub fn pi_estimate_stratified() {
//     let sqrt_n = 10_000;
//     let mut inside_circle = 0;
//     let mut inside_circle_stratified = 0;
//     let mut rng = thread_rng();
//
//     for i in 0..sqrt_n {
//         for j in 0..sqrt_n {
//             let mut x = rng.gen_range(-1.0..1.0);
//             let mut y = rng.gen_range(-1.0..1.0);
//
//             if x * x + y * y < 1.0 {
//                 inside_circle += 1;
//             }
//
//             x = 2.0 * ((i as f64 + rng.gen::<f64>()) / sqrt_n as f64) - 1.0;
//             y = 2.0 * ((j as f64 + rng.gen::<f64>()) / sqrt_n as f64) - 1.0;
//             if x * x + y * y < 1.0 {
//                 inside_circle_stratified += 1;
//             }
//         }
//     }
//
//     let reg_estimate = (4 * inside_circle) as f64 / (sqrt_n * sqrt_n) as f64;
//     let strat_estimate = (4 * inside_circle_stratified) as f64 / (sqrt_n * sqrt_n) as f64;
//     println!("Reg. Estimate of Pi = {:.12}", reg_estimate);
//     println!("Stratified Estimate of Pi = {:.12}", strat_estimate);
// }
//
// #[cfg(test)]
// mod tests {
//     use crate::montecarlo::{pi_estimate, pi_estimate_convergence, pi_estimate_stratified};
//
//     #[test]
//     fn estimate_pi() {
//         pi_estimate();
//     }
//
//     // #[test]
//     // fn estimate_pi_conv() {
//     //     pi_estimate_convergence();
//     // }
//
//     // #[test]
//     // fn estimate_pi_stratified() {
//     //     pi_estimate_stratified();
//     // }
// }
