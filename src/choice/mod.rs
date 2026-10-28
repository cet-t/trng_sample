use trng::{Rng, Word};

#[cfg(test)]
mod test;

pub trait Choice: trng::Rng {
    fn choice<'a, T>(&mut self, items: &'a [T]) -> Option<&'a T>;
}

impl<R> Choice for R
where
    R: Rng,
    <R::Word as Word>::Int: From<u8> + TryFrom<usize> + TryInto<usize>,
{
    fn choice<'a, T>(&mut self, items: &'a [T]) -> Option<&'a T> {
        if items.is_empty() {
            None
        } else {
            let max = (items.len() - 1).try_into().ok()?;
            let i: usize = self.randi(0u8.into(), max).try_into().ok()?;
            Some(&items[i])
        }
    }
}
