pub fn preprocess_listing(listing: &str) -> String {
    listing
        .lines()
        .filter(|line| !line.starts_with(';') && !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
        .replace("\r\n", "\n")
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_preprocess_listing_removes_comments() {
        let input = "; this is a comment";
        let expected = "";
        assert_eq!(preprocess_listing(input), expected);
    }

    #[test]
    fn test_preprocess_listing_keeps_bits_directive() {
        let input = "bits 16";
        let expected = "bits 16";
        assert_eq!(preprocess_listing(input), expected);
    }

    #[test]
    fn test_preprocess_listing_keeps_single_instruction() {
        let input = "mov ax, 0x1234";
        let expected = "mov ax, 0x1234";
        assert_eq!(preprocess_listing(input), expected);
    }

    #[test]
    fn test_preprocess_listing_keeps_multiple_instructions() {
        let input = "mov ax, 0x1234\nmov bx, 0x5678";
        let expected = "mov ax, 0x1234\nmov bx, 0x5678";
        assert_eq!(preprocess_listing(input), expected);
    }

    #[test]
    fn test_preprocess_listing_removes_extra_newlines() {
        let input = "mov ax, 0x1234\n\nmov bx, 0x5678";
        let expected = "mov ax, 0x1234\nmov bx, 0x5678";
        assert_eq!(preprocess_listing(input), expected);
    }
}
