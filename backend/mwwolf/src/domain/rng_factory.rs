pub trait RngFactory: Send + Sync {
    type RngCore: rand::RngCore + Send + Sync;
    fn create(&self) -> Self::RngCore;
}

#[cfg(test)]
#[derive(new)]
pub struct MockRngFactory {
    initial: u64,
    increment: u64,
}

#[cfg(test)]
impl RngFactory for MockRngFactory {
    type RngCore = rand::rngs::mock::StepRng;
    fn create(&self) -> Self::RngCore {
        Self::RngCore::new(self.initial, self.increment)
    }
}
