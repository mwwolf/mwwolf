use rand::RngCore;

#[cfg(not(test))]
pub use not_test::*;

#[cfg(test)]
pub use mock::*;

#[cfg(not(test))]
mod not_test {
    use super::*;

    use rand::thread_rng;
    pub struct RngCoreFactory;

    impl RngCoreFactory {
        pub fn create(&self) -> impl RngCore {
            thread_rng()
        }
    }
}

#[cfg(test)]
mod mock {
    use super::*;
    use derive_new::new;
    use rand::rngs::mock::StepRng;

    #[derive(new)]
    pub struct RngCoreFactory {
        initial: u64,
        increment: u64,
    }

    impl RngCoreFactory {
        pub fn create(&self) -> impl RngCore {
            StepRng::new(self.initial, self.increment)
        }
    }
}
