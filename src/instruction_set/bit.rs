use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bit(pub bool);

impl fmt::Binary for Bit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = if self.0 { 1 } else { 0 };
        write!(f, "{}", value)
    }
}

impl fmt::Display for Bit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Binary::fmt(self, f)
    }
}

impl From<bool> for Bit {
    fn from(value: bool) -> Self {
        Bit(value)
    }
}

impl From<Bit> for bool {
    fn from(value: Bit) -> Self {
        value.0
    }
}

// Implement the logical NOT operation using the `Not` trait
impl Not for Bit {
    type Output = Self;
    fn not(self) -> Self::Output {
        Bit(!self.0)
    }
}

// Implement the AND operation using the `BitAnd` trait
impl BitAnd for Bit {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Bit(self.0 & rhs.0)
    }
}

// Implement the OR operation using the `BitOr` trait
impl BitOr for Bit {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Bit(self.0 | rhs.0)
    }
}

// Implement the XOR operation using the `BitXor` trait
impl BitXor for Bit {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Bit(self.0 ^ rhs.0)
    }
}

// Implement the AND assignment operation using the `BitAndAssign` trait
impl BitAndAssign for Bit {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

// Implement the OR assignment operation using the `BitOrAssign` trait
impl BitOrAssign for Bit {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

// Implement the XOR assignment operation using the `BitXorAssign` trait
impl BitXorAssign for Bit {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

#[cfg(test)]
mod tests {
    use super::Bit;

    // Test for fmt::Binary
    #[test]
    fn test_binary_format() {
        let true_bit = Bit(true);
        assert_eq!(format!("{:b}", true_bit), "1");

        let false_bit = Bit(false);
        assert_eq!(format!("{:b}", false_bit), "0");
    }

    // Test for fmt::Display
    #[test]
    fn test_display_format() {
        let true_bit = Bit(true);
        assert_eq!(format!("{}", true_bit), "1");

        let false_bit = Bit(false);
        assert_eq!(format!("{}", false_bit), "0");
    }

    #[test]
    fn test_copy_trait() {
        let original = Bit(true);
        let mut copy = original; // Copy occurs here

        // Modify the original
        copy = !copy;

        // The copy should remain unchanged
        assert_eq!(original, Bit(true));
        assert_eq!(copy, Bit(false));
    }

    #[test]
    fn test_clone_trait() {
        let original = Bit(true); // replace with the constructor or initialization method for your type
        #[allow(clippy::clone_on_copy)]
        let clone = original.clone();

        // Check that the original and clone are equal in value
        assert_eq!(original, clone);

        // Check that they are different instances in memory
        assert!(!std::ptr::eq(&original, &clone));
    }

    #[test]
    fn test_debug_format() {
        let true_bit = Bit(true);
        assert_eq!(format!("{:?}", true_bit), "Bit(true)");

        let false_bit = Bit(false);
        assert_eq!(format!("{:?}", false_bit), "Bit(false)");
    }

    // Test for From<bool> for Bit
    #[test]
    fn test_from_bool() {
        let true_bit: Bit = true.into();
        assert_eq!(true_bit, Bit(true));

        let false_bit: Bit = false.into();
        assert_eq!(false_bit, Bit(false));
    }

    // Test for From<Bit> for bool
    #[test]
    fn test_from_bit() {
        let true_bool: bool = Bit(true).into();
        assert!(true_bool);

        let false_bool: bool = Bit(false).into();
        assert!(!false_bool);
    }

    #[test]
    fn test_not() {
        let a = Bit(true);
        let b = Bit(false);
        assert_eq!(!a, Bit(false));
        assert_eq!(!b, Bit(true));
    }

    #[test]
    fn test_and() {
        let a = Bit(true);
        let b = Bit(false);
        assert_eq!(a & a, Bit(true));
        assert_eq!(a & b, Bit(false));
        assert_eq!(b & a, Bit(false));
        assert_eq!(b & b, Bit(false));
    }

    #[test]
    fn test_or() {
        let a = Bit(true);
        let b = Bit(false);
        assert_eq!(a | a, Bit(true));
        assert_eq!(a | b, Bit(true));
        assert_eq!(b | a, Bit(true));
        assert_eq!(b | b, Bit(false));
    }

    #[test]
    fn test_xor() {
        let a = Bit(true);
        let b = Bit(false);
        assert_eq!(a ^ a, Bit(false));
        assert_eq!(a ^ b, Bit(true));
        assert_eq!(b ^ a, Bit(true));
        assert_eq!(b ^ b, Bit(false));
    }

    #[test]
    fn test_and_assign() {
        let mut a = Bit(true);
        let b = Bit(true);
        a &= b;
        assert_eq!(a, Bit(true));

        let mut a = Bit(true);
        let b = Bit(false);
        a &= b;
        assert_eq!(a, Bit(false));

        let mut a = Bit(false);
        let b = Bit(true);
        a &= b;
        assert_eq!(a, Bit(false));

        let mut a = Bit(false);
        let b = Bit(false);
        a &= b;
        assert_eq!(a, Bit(false));
    }

    #[test]
    fn test_or_assign() {
        let mut a = Bit(true);
        let b = Bit(true);
        a |= b;
        assert_eq!(a, Bit(true));

        let mut a = Bit(true);
        let b = Bit(false);
        a |= b;
        assert_eq!(a, Bit(true));

        let mut a = Bit(false);
        let b = Bit(true);
        a |= b;
        assert_eq!(a, Bit(true));

        let mut a = Bit(false);
        let b = Bit(false);
        a |= b;
        assert_eq!(a, Bit(false));
    }

    #[test]
    fn test_xor_assign() {
        let mut a = Bit(true);
        let b = Bit(true);
        a ^= b;
        assert_eq!(a, Bit(false));

        let mut a = Bit(true);
        let b = Bit(false);
        a ^= b;
        assert_eq!(a, Bit(true));

        let mut a = Bit(false);
        let b = Bit(true);
        a ^= b;
        assert_eq!(a, Bit(true));

        let mut a = Bit(false);
        let b = Bit(false);
        a ^= b;
        assert_eq!(a, Bit(false));
    }
}
