pub trait Verifiable {
    fn verify(&self) -> bool;
}
