const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

fn main() {
    let n: f32 = 42.42;

    let (sign, exponent, fraction) = to_parts(n);
    let (sign_, mantissa, exp_) = decode(sign, exponent, fraction);
    let n_ = sign_ * mantissa * exp_;
    println!("{} -> {}", n, n_);
    println!("field | as bits | as real number");
    println!("sign | {:01b} | {}", sign, sign_);
    // println!("exponent | {:08b} | {}", exp_, exp_);
    println!("mantissa | {:023b} | {}", fraction, mantissa);

}

fn to_parts(n: f32) -> (u32, u32, u32) {
    let bits = n.to_bits();

    let sign = (bits >> 31) & 1;
    let exponent = (bits >> 23) & 0xff;
    let fraction = bits & 0x7fffff;

    (sign, exponent, fraction)
}

fn decode(sign: u32, exponent: u32, fraction: u32) -> (f32, f32, f32) {
    let signed_1 = (-1.0_f32).powf(sign as f32);

    let exponent = (exponent as i32) - BIAS;
    let exponent = RADIX.powf(exponent as f32);

    let mut mantissa = 1.0;

    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = fraction & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2.0_f32.powf(i_ - 23.0);
            mantissa += weight
        }
    }

    (signed_1, mantissa, exponent)
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct Q7(i8);

impl From<f64> for Q7 {
    fn from(n: f64) -> Self {
        if n >= 1.0 {
            Q7(127)
        } else if n <= -1.0 {
            Q7(-128)
        } else {
            Q7((n * 128.0) as i8)
        }
    }
}

impl From<Q7> for f64 {
    fn from(n: Q7) -> Self {
        (n.0 as f64) / 128.0
    }
}

impl From<f32> for Q7 {
    fn from(n: f32) -> Self {
        Q7::from(n as f64)
    }
}

impl From<Q7> for f32 {
    fn from(n: Q7) -> Self {
        f64::from(n) as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_from_f32() {
        let n: f32 = 0.7;
        let q: Q7 = n.into();
        // Assert they are close within epsilon
        let epsilon = 1.0 / 128.0;
        let n_ = f32::from(q);
        assert!((n - n_).abs() < epsilon);
    }
}