use crate::frame::tanh::TanhKer;

const LOW: f32 = -9.0;
const HIGH: f32 = 9.0;
const ALPHA_13: f32 = -2.76076847742355e-16;
const ALPHA_11: f32 = 2.00018790482477e-13;
const ALPHA_9: f32 = -8.60467152213735e-11;
const ALPHA_7: f32 = 5.12229709037114e-08;
const ALPHA_5: f32 = 1.48572235717979e-05;
const ALPHA_3: f32 = 6.37261928875436e-04;
const ALPHA_1: f32 = 4.89352455891786e-03;
const BETA_6: f32 = 1.19825839466702e-06;
const BETA_4: f32 = 1.18534705686654e-04;
const BETA_2: f32 = 2.26843463243900e-03;
const BETA_0: f32 = 4.89352518554385e-03;

pub fn stanh(x: f32) -> f32 {
    let x = x.max(LOW).min(HIGH);

    let x2 = x * x;

    let p = x2 * ALPHA_13 + ALPHA_11;
    let p = x2 * p + ALPHA_9;
    let p = x2 * p + ALPHA_7;
    let p = x2 * p + ALPHA_5;
    let p = x2 * p + ALPHA_3;
    let p = x2 * p + ALPHA_1;
    let p = p * x;

    let q = x2 * BETA_6 + BETA_4;
    let q = x2 * q + BETA_2;
    let q = x2 * q + BETA_0;

    p / q
}

#[derive(Clone, Debug)]
pub struct STanh4;

impl TanhKer<f32> for STanh4 {
    fn name() -> &'static str {
        "generic"
    }

    fn alignment_bytes() -> usize {
        16
    }

    fn nr() -> usize {
        4
    }

    fn run(x: &mut [f32]) {
        debug_assert!(x.len() % Self::nr() == 0);
        debug_assert!(x.as_ptr() as usize % Self::alignment_bytes() == 0);
        x.iter_mut().for_each(|px| *px = stanh(*px))
    }
}

#[cfg(test)]
#[macro_use]
pub mod test {
    tanh_frame_tests!(true, crate::generic::tanh::STanh4);
}
