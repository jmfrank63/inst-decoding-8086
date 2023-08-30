#[cfg(test)]
pub fn preprocess_listing(listing: &str) -> String {
    listing
        .lines()
        .filter(|line| !line.starts_with(';') && !line.is_empty() && !line.starts_with("bit 16"))
        .collect::<Vec<_>>()
        .join("\n")
        .replace("\r\n", "\n")
}
