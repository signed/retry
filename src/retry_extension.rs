use retry::delay::Fixed;

pub trait FixedExt {
    fn from_seconds(seconds: u64) -> Self;
}

impl FixedExt for Fixed {
    fn from_seconds(seconds: u64) -> Self {
        Fixed::from_millis(seconds * 1000)
    }
}
