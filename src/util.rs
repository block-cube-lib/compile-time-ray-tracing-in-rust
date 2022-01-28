pub const fn sqrt(s: f64) -> f64 {
    if s == 0.0 {
        return 0.0;
    }

    let mut x = s / 2.0;
    let mut _last_x = 0.0;

    while x != _last_x {
        _last_x = x;
        x = (x + s / x) / 2.0;
    }
    x
}

pub const fn abs(s: f64) -> f64 {
    if s >= 0.0 {
        s
    } else {
        -s
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sqrt_test() {
        let positive = 4.0_f64;
        let negative = -4.0_f64;
        let negative_zero = -0.0_f64;

        let abs_difference = (positive.sqrt() - 2.0).abs();

        assert!(abs_difference < 1e-10);
        assert!(negative.sqrt().is_nan());
        assert!(negative_zero.sqrt() == negative_zero);
    }

    #[test]
    fn abs_test() {
        for i in -100..100 {
            let f = i as f64;
            assert_eq!(f.abs(), abs(f));
        }
    }
}
