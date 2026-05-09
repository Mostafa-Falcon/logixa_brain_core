pub fn normalize_fts_query(input: &str) -> String {
    input
        .split_whitespace()
        .take(24)
        .map(clean_token)
        .filter(|s| s.chars().count() >= 2)
        .map(|token| format!("{}*", token))
        .collect::<Vec<_>>()
        .join(" OR ")
}

fn clean_token(input: &str) -> String {
    input
        .chars()
        .filter(|ch| !is_query_punctuation(*ch))
        .collect::<String>()
        .trim()
        .to_string()
}

fn is_query_punctuation(ch: char) -> bool {
    ch.is_ascii_punctuation()
        || matches!(
            ch,
            '،' | '؛' | '؟' | 'ـ' | '“' | '”' | '‘' | '’' | '«' | '»' | '…'
        )
}
