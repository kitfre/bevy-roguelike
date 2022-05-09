pub(crate) mod map;

pub(crate) trait Generator {
    type Output;

    fn generate(&mut self) -> Self::Output;
}
