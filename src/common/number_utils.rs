pub fn clamp(i: f32, min: f32, max: f32) -> f32 {
    if i > max {
        return max;
    }
    if i < min {
        return min;
    }
    i
}
