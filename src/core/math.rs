pub fn clamp<T: std::cmp::PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        return min;
    } else if value > max {
        return max;
    } else {
        return value;
    }
}
