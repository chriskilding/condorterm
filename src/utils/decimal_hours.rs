use crate::utils::fractions::into_fraction;
use chrono::NaiveTime;

/// Convert decimal-hours to a Time object
pub fn into_time(decimal_hour: f32) -> NaiveTime {
    let (hour, decimal) = into_fraction(decimal_hour);
    let min_sec = 60.0 * decimal;
    let min = min_sec as u32;
    let sec = (60.0 * (min_sec - (min as f32))) as u32;
    NaiveTime::from_hms_opt(hour, min, sec).unwrap_or(NaiveTime::MIN)
}

#[cfg(test)]
mod tests {
    use chrono::Timelike;

    use super::*;

    #[test]
    fn should_convert_hours() {
        let result = into_time(12.0);
        assert_eq!(result.hour(), 12);
        assert_eq!(result.minute(), 0);
        assert_eq!(result.second(), 0);
    }

    #[test]
    fn should_convert_hours_and_minutes() {
        let result = into_time(13.5);
        assert_eq!(result.hour(), 13);
        assert_eq!(result.minute(), 30);
        assert_eq!(result.second(), 0);
    }

    #[test]
    fn should_convert_hours_and_minutes_and_seconds() {
        let result = into_time(14.76);
        assert_eq!(result.hour(), 14);
        assert_eq!(result.minute(), 45);
        assert_eq!(result.second(), 36);
    }
}
