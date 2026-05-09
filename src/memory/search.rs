pub fn normalize_fts_query(input: &str) -> String {
    input
        .split_whitespace()
        .take(24)
        .map(|s| s.replace(['"', '\'', ':', ';'], ""))
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" OR ")
}
