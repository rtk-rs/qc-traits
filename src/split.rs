//! Split operation, to split in time domain
use hifitime::{Duration, Epoch};

/// Implement [QcSplit] to rearrange datasets timewise.
pub trait QcSplit {
    /// [QcSplit]s [Self] with mutable access, retaining only <= t,
    /// and returning > t.
    fn split_mut(&mut self, t: Epoch) -> Self;

    /// [QcSplit]s Self into a batch of equal [Duration]
    fn split_even_dt(&self, dt: Duration) -> Vec<Self>
    where
        Self: Sized;

    /// [QcSplit]s Self into two at specified [Epoch]
    /// Returns:
    ///  - (a , b) where a <= t and b > t
    fn split(&self, t: Epoch) -> (Self, Self)
    where
        Self: Sized + Clone,
    {
        let mut s = self.clone();
        let rhs = s.split_mut(t);
        (s, rhs)
    }
}
