/// 監査ログに表示するラベルを、最大`max_chars`文字へ省略する想定の関数です。
///
/// バグ状態では`max_chars`を文字数として受け取りながら、`str`のバイト範囲に渡しています。
pub fn abbreviate_for_log(text: &str, max_chars: usize) -> String {
    let character_count = text.chars().count();
    if character_count <= max_chars {
        eprintln!(
            "[truncate] input={text:?}, chars={character_count}, bytes={}, output=unchanged",
            text.len()
        );
        return text.to_owned();
    }

    eprintln!(
        "[truncate] input={text:?}, chars={character_count}, bytes={}, requested_chars={max_chars}, byte_boundary={}",
        text.len(),
        text.is_char_boundary(max_chars)
    );

    let prefix: String = text.chars().take(max_chars).collect();
    format!("{prefix}…")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn japanese_label_must_be_truncated_without_panicking_at_a_character_boundary() {
        let source = "障害対応手順";
        let result = std::panic::catch_unwind(|| abbreviate_for_log(source, 5));

        assert_eq!(
            result.expect("日本語ラベルの省略でパニックしてはいけません"),
            "障害対応手…"
        );
    }

    #[test]
    fn ascii_label_is_truncated_to_the_requested_character_count() {
        assert_eq!(abbreviate_for_log("incident-report", 8), "incident…");
    }

    #[test]
    fn short_label_is_preserved() {
        assert_eq!(abbreviate_for_log("監視", 4), "監視");
    }
}
