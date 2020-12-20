pub trait Relative: Sized {
    fn adjacent(&self) -> Vec<Self>;
}
