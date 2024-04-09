use rand::prelude::*;
use rayon::prelude::*;
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use coz::*;

fn median_n(n: u32, rng: &mut SmallRng) -> f64 {
    let mut nums: Vec<f64>  = vec![rng.gen(); n as usize];
    nums.par_sort_by(|a, b| { coz::thread_init(); a.partial_cmp(b).unwrap()} );
    coz::scope!("median_n");
    nums[(n / 2) as usize]
}

const POWER: u32 = 9;
const ITERATIONS: u64 = 10_u64.pow(POWER);
const BATCH_SIZE: u64 = 10_u64.pow(3);
const NUM_NS: u32 = 10;

fn main() {
    coz::thread_init();
    let mut thread_rng = thread_rng();
    let mut batch_vec = vec![0.0; BATCH_SIZE as usize];
    let mut small_rngs: Vec<SmallRng> = (0..BATCH_SIZE)
        .map(|_| SmallRng::from_rng(&mut thread_rng).unwrap())
        .collect();
    println!("Initialization Complete");

    let ns = (0..NUM_NS).map(|n| n * 2 + 1).collect::<Vec<_>>();
    for n in ns {
        coz::begin!("loop");
        let medians = small_rngs.par_iter_mut() 
            .map(|rng| {
                coz::thread_init();
                (0..(ITERATIONS / BATCH_SIZE)).into_iter()
                    .map(|_| median_n(n, rng))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<f64>>();
        /* 
        for _ in 0..(ITERATIONS / BATCH_SIZE) {
            small_rngs.par_iter_mut()
                .map(|rng| median_n(n, rng))
                .collect_into_vec(&mut batch_vec);
            medians.append(&mut batch_vec);     
        }
        */
        coz::end!("loop");
        let expected = medians.into_iter().sum::<f64>() / ITERATIONS as f64;
        println!("Determined expected median with n={n}, batch size {BATCH_SIZE} in {ITERATIONS} iterations. E = {expected}");
    }
    coz::scope!("main");
}
