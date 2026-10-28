use std::ops::AddAssign;

use crate::sampler::Sampler;

pub struct Bst<'a, R>
where
    R: trng::Rng,
{
    rng: &'a mut R,
    acc_weights: Vec<crate::Float<R::Word>>,
}

impl<'a, R> Bst<'a, R>
where
    R: trng::Rng,
    crate::Float<R::Word>: Default + AddAssign<crate::Float<R::Word>> + PartialOrd,
{
    pub fn new(rng: &'a mut R, weights: &'a [crate::internal::Float<R::Word>]) -> Self {
        let acc_weights = {
            let mut acc = Vec::with_capacity(weights.len());
            let mut total = Default::default();

            for &w in weights.iter() {
                total += w;
                acc.push(total);
            }

            acc
        };

        Self { rng, acc_weights }
    }
}

impl<'a, R> Sampler for Bst<'a, R>
where
    R: trng::Rng,
    crate::Float<R::Word>: Default + AddAssign<crate::Float<R::Word>> + PartialOrd,
{
    fn build(&mut self) -> crate::SamplerResult<()> {
        Ok(())
    }

    fn sample(&mut self) -> Option<usize> {
        let total = self.acc_weights.last()?.to_owned();
        let r = self.rng.randf(Default::default(), total);

        let min = |i: usize| i.min(self.acc_weights.len() - 1);

        Some(
            self.acc_weights
                .binary_search_by(|c| c.partial_cmp(&r).unwrap())
                .map(min)
                .unwrap_or_else(min),
        )
    }
}
