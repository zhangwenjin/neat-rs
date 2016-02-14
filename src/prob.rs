use rand::{Rng, Closed01};

pub fn is_probable<R: Rng>(prob: &Closed01<f32>, rng: &mut R) -> bool {
    if prob.0 < 1.0 {
        let v: f32 = rng.gen(); // half open [0, 1)
        debug_assert!(v >= 0.0 && v < 1.0);
        v < prob.0
    } else {
        true
    }
}

/// Takes the fractional part of `num` as probability to
/// round up or down to the next integral value.
pub fn probabilistic_round<R: Rng>(num: f64, rng: &mut R) -> f64 {
    assert!(num >= 0.0);
    let p: f64 = rng.gen(); // half open [0, 1)
    debug_assert!(p >= 0.0 && p < 1.0);
    if p < num.fract() {
        num.trunc() + 1.0
    } else {
        num.trunc()
    }
}