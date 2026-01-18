// TODO: implement the necessary traits to make the test compile and pass.
//  You *can't* modify the test.
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WrappingU32 {
    value: u32,
}

// Implementing the add trait for WrappingU32 
// I don't need to add pub trait Add<RHS = Self> because it's already in the std library
impl Add for WrappingU32 {
    // Self here means WrappingU32
    // Output is defined on the original trait so must be implemented
    type Output = Self;

    // RHS is short for right-hand side
    fn add(self, rhs: Self) -> Self::Output {
        // wrapping_add is needed so the MAX test doesnt fail
        Self::new(self.value.wrapping_add(rhs.value))
    }
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let x = WrappingU32::new(42);
        let y = WrappingU32::new(31);
        let z = WrappingU32::new(u32::MAX);
        assert_eq!(x + y + y + z, WrappingU32::new(103));
    }
}
