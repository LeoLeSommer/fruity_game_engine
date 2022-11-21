use std::f32::consts::PI;
use std::ops::Range;

/// Take a radian angle and normalize it between [-PI, PI[
///
/// # Arguments
/// * `angle` - The input angle
///
pub fn normalize_angle(angle: f32) -> f32 {
    if angle < -PI {
        normalize_angle(angle + 2.0 * PI)
    } else if angle >= PI {
        normalize_angle(angle - 2.0 * PI)
    } else {
        angle
    }
}

/// Take a radian angle range and normalize each born between [-PI, PI[
/// If the range length is 2PI, returns simply -PI..PI
///
/// # Arguments
/// * `range` - The input range
///
pub fn normalize_angle_range(range: Range<f32>) -> Range<f32> {
    if range.start == range.end {
        return 0.0..0.0;
    }

    let angle1 = normalize_angle(range.start);
    let angle2 = normalize_angle(range.end);

    let start = f32::min(angle1, angle2);
    let end = f32::max(angle1, angle2);

    if start == end {
        -PI..PI
    } else {
        start..end
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::math::normalize_angle;
    use crate::utils::math::normalize_angle_range;
    use std::f32::consts::PI;

    #[test]
    fn normalize_angle_test() {
        assert_eq!(normalize_angle(3.0 * PI / 4.0), 3.0 * PI / 4.0);
        assert_eq!(normalize_angle(6.0 * PI / 4.0), -2.0 * PI / 4.0);
        assert_eq!(normalize_angle(PI), -PI);
        assert_eq!(normalize_angle(-PI), -PI);
    }

    #[test]
    fn normalize_angle_range_test() {
        assert_eq!(
            normalize_angle_range((3.0 * PI / 4.0)..(6.0 * PI / 4.0)),
            (-2.0 * PI / 4.0)..(3.0 * PI / 4.0)
        );
        assert_eq!(
            normalize_angle_range(0.0..(6.0 * PI / 4.0)),
            0.0..(2.0 * PI / 4.0)
        );
        assert_eq!(normalize_angle_range(0.0..(2.0 * PI)), -PI..PI);
        assert_eq!(
            normalize_angle_range((3.0 * PI / 4.0)..(3.0 * PI / 4.0)),
            -PI..PI
        );
        assert_eq!(normalize_angle_range(0.0..0.0), 0.0..0.0);
    }
}
