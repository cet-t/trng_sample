use std::ops::{AddAssign, Div, Mul, Sub, SubAssign};

use crate::{Float, Int, sampler::Sampler};

pub struct Alias<'a, R>
where
    R: trng::Rng,
{
    rng: &'a mut R,
    weights: &'a [crate::Float<R::Word>],

    prob: Option<Vec<crate::Float<R::Word>>>,
    alias: Option<Vec<usize>>,
}

impl<'a, R> Alias<'a, R>
where
    R: trng::Rng,
{
    pub fn new(rng: &'a mut R, weights: &'a [crate::Float<R::Word>]) -> Self {
        Self {
            rng,
            weights,
            prob: None,
            alias: None,
        }
    }
}

impl<'a, R> Sampler for Alias<'a, R>
where
    R: trng::Rng,
    crate::Int<R::Word>: Default + TryFrom<usize> + TryInto<usize>,
    Float<R::Word>: Default
        + Sub<Output = Float<R::Word>>
        + Div<Output = Float<R::Word>>
        + Mul<Output = Float<R::Word>>
        + AddAssign
        + SubAssign
        + From<f32>
        + PartialOrd,
{
    fn build(&mut self) -> super::SamplerResult<()> {
        let n = self.weights.len();

        let total = {
            let mut total = Float::<R::Word>::default();
            for &w in self.weights.iter() {
                total += w;
            }
            total
        };
        let scale = {
            let n = Float::<R::Word>::from(n as f32);
            n / total
        };

        let mut prob = vec![Float::<R::Word>::default(); n];
        let mut alias = vec![usize::default(); n];
        let mut small = Vec::with_capacity(n);
        let mut large = Vec::with_capacity(n);
        let mut p: Vec<_> = self.weights.iter().map(|&w| w * scale).collect();

        for (i, &pi) in p.iter().enumerate() {
            if pi < Float::<R::Word>::from(1.0) {
                small.push(i);
            } else {
                large.push(i);
            }
        }

        while let (Some(&s), Some(&l)) = (small.last(), large.last()) {
            small.pop();
            prob[s] = p[s];
            alias[s] = l;
            let ps = p[s];
            p[l] -= Float::<R::Word>::from(1.0) - ps;
            if p[l] < Float::<R::Word>::from(1.0) {
                large.pop();
                small.push(l);
            }
        }

        for i in large {
            prob[i] = 1.0.into();
        }

        for i in small {
            prob[i] = 1.0.into();
        }

        self.prob = Some(prob);
        self.alias = Some(alias);

        Ok(())
    }

    fn sample(&mut self) -> Option<usize> {
        let prob = self.prob.as_ref()?;
        let alias = self.alias.as_ref()?;

        let n = prob.len();
        let i: usize = {
            let max_n = n.checked_sub(1)?.try_into().ok()?;
            self.rng
                .randi(Int::<R::Word>::default(), max_n)
                .try_into()
                .ok()?
        };
        let u = self.rng.randf(0.0.into(), 1.0.into());

        let res = if u < prob[i] { i } else { alias[i] };

        Some(res)
    }
}
