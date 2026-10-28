mod alias;
mod bst;

pub use alias::Alias;
pub use bst::Bst;

#[derive(Debug, thiserror::Error)]
pub enum SamplerError {
    #[error("items and weights size are not match. items: {items} != weights: {weights}")]
    ElementSizeError { items: usize, weights: usize },
}

pub type SamplerResult<T> = Result<T, SamplerError>;

pub trait Sampler {
    fn build(&mut self) -> SamplerResult<()>;
    fn sample(&mut self) -> Option<usize>;
}
