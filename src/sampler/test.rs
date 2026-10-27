use trng_a::Sfc32;

use crate::{Alias, Bst, Sampler};

const WEIGHTS: &[f32] = &[0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];

#[test]
fn alias_works() {
    let mut rng = Sfc32::from(327984);
    let mut sampler = Alias::new(&mut rng, WEIGHTS);

    if let Err(e) = sampler.build() {
        panic!("{e}");
    }

    let Some(i) = sampler.sample() else {
        panic!("");
    };
    assert_eq!(i, 8);
}

#[test]
fn alias_with_one_positive_weight() {
    let mut rng = Sfc32::from(327984);
    let mut sampler = Alias::new(&mut rng, &[0.0, 1.0]);
    sampler.build().unwrap();

    for _ in 0..100 {
        assert_eq!(sampler.sample(), Some(1));
    }
}

#[test]
fn alias_table_matches_weights() {
    let mut rng = Sfc32::from(327984);
    let mut sampler = Alias::new(&mut rng, WEIGHTS);
    sampler.build().unwrap();

    let prob = sampler.prob.as_ref().unwrap();
    let alias = sampler.alias.as_ref().unwrap();
    let mut mass = vec![0.0; WEIGHTS.len()];
    for (i, &p) in prob.iter().enumerate() {
        assert!((0.0..=1.0).contains(&p));
        mass[i] += p;
        mass[alias[i]] += 1.0 - p;
    }
    let total: f32 = WEIGHTS.iter().sum();
    for (&actual, &weight) in mass.iter().zip(WEIGHTS) {
        let expected = weight * WEIGHTS.len() as f32 / total;
        assert!((actual - expected).abs() < 1e-6);
    }
}

#[test]
fn bst_works() {
    let mut rng = Sfc32::from(327984);
    let mut sampler = Bst::new(&mut rng, WEIGHTS);

    if let Err(e) = sampler.build() {
        panic!("{e}");
    }

    let Some(i) = sampler.sample() else {
        panic!("");
    };
    assert_eq!(i, 9);
}

// Return a chosen point within the range so the selected interval is deterministic.
struct FractionRng(f32);

impl trng::Rng for FractionRng {
    type Word = u32;

    fn nextu(&mut self) -> u32 {
        unreachable!("the test supplies randf directly")
    }

    fn randf(&mut self, min: f32, max: f32) -> f32 {
        min + self.0 * (max - min)
    }
}

#[test]
fn bst_empty_weights_return_none() {
    let mut rng = FractionRng(0.5);
    let mut sampler = Bst::new(&mut rng, &[]);
    sampler.build().unwrap();
    assert_eq!(sampler.sample(), None);
}

#[test]
fn bst_single_weight_always_returns_zero() {
    for fraction in [0.0, 0.5, 0.99] {
        let mut rng = FractionRng(fraction);
        let mut sampler = Bst::new(&mut rng, &[2.0]);
        sampler.build().unwrap();
        assert_eq!(sampler.sample(), Some(0));
    }
}

#[test]
fn bst_selects_weighted_intervals() {
    for (fraction, expected) in [(0.125, 0), (0.375, 1), (0.875, 2)] {
        let mut rng = FractionRng(fraction);
        let mut sampler = Bst::new(&mut rng, &[1.0, 2.0, 1.0]);
        sampler.build().unwrap();
        assert_eq!(sampler.sample(), Some(expected));
    }
}

#[test]
fn bst_skips_zero_weights_inside_intervals() {
    for (fraction, expected) in [(0.2, 1), (0.8, 3)] {
        let mut rng = FractionRng(fraction);
        let mut sampler = Bst::new(&mut rng, &[0.0, 1.0, 0.0, 2.0, 0.0]);
        sampler.build().unwrap();
        assert_eq!(sampler.sample(), Some(expected));
    }
}
