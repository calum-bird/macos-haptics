extern {
    fn performHapticFeedbackMultipleTimes(count: i32);
}

fn main() {
    unsafe {
        performHapticFeedbackMultipleTimes(5); // triggers haptic feedback 5 times
    }
}