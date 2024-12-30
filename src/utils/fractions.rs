/// Split a float into its components before and after the decimal point
pub fn into_fraction(x: f32) -> (u32, f32) {
    let integer = x.floor();
    let decimal = x - integer;
    (integer as u32, decimal)
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn should_convert_whole_numbers() {
        let (integer, decimal) = into_fraction(12.0);
        assert_eq!(integer, 12);
        assert_relative_eq!(decimal, 0.0);
    }

    #[test]
    fn should_convert_floats() {
        let (integer, decimal) = into_fraction(12.75);
        assert_eq!(integer, 12);
        assert_relative_eq!(decimal, 0.75);
    }
}
