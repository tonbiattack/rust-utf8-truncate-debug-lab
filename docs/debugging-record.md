# デバッグ記録: UTF-8ラベルの文字数省略

## 実行環境と再現境界

| 項目 | 内容 |
| --- | --- |
| 言語 | Rust 1.85以上、標準ライブラリのみ |
| 公開境界 | `abbreviate_for_log(text, max_chars)` |
| 入力 | `text = "障害対応手順"`、`max_chars = 5` |
| 期待 | `障害対応手…`を返し、パニックしない |
| 別の観測 | 文字数、バイト数、UTF-8文字境界、返却値 |

## 最初に観測した事実

`e43f633`で、次を実行しました。

```bash
cargo test japanese_label_must_be_truncated_without_panicking_at_a_character_boundary -- --nocapture
```

```text
[truncate] input="障害対応手順", chars=6, bytes=18, requested_chars=5, byte_boundary=false
end byte index 5 is not a char boundary; it is inside '害' (bytes 3..6 of string)
```

入力は6個のUnicodeスカラー値ですが、UTF-8では18バイトです。5は2文字目の途中にあり、文字列スライスの作成でパニックしました。

## 競合仮説と検証

| 仮説 | 検証 | 結果 |
| --- | --- | --- |
| 入力が短いため誤った分岐へ入る | `chars().count()`を出力する | 6 > 5のため除外 |
| 要求した範囲がUTF-8境界でない | `is_char_boundary(5)`を出力する | `false`で支持 |
| 省略記号の追加が原因 | GDBでスライス式の直前に停止する | スライス時点で原因を確定 |

GDBで`src/lib.rs:20`に停止すると、`max_chars=5`と`character_count=6`を確認できました。スタックはテストから`abbreviate_for_log`へ到達しており、範囲式`&text[..max_chars]`が次の実行箇所です。

## 確定した原因

`str::len()`と範囲添字はバイト単位です。一方、呼び出し側が渡す`max_chars`は文字数です。`&text[..max_chars]`は文字数をバイトオフセットとして解釈し、UTF-8文字の途中を切ろうとします。`str`の有効なスライスはUTF-8文字境界に限られるため、範囲式はパニックします。[1] [2]

## 最小修正

バイト範囲を作らず、Unicodeスカラー値を最大数だけ収集します。

```rust
let prefix: String = text.chars().take(max_chars).collect();
format!("{prefix}…")
```

修正コミット`e4a5ada`後のログでは、同じ入力でも`byte_boundary=false`のままパニックせず、`障害対応手…`を返しました。`chars()`はUnicodeスカラー値を反復するため、この契約に一致します。[2]

## 回帰保証

| テスト | 守る契約 |
| --- | --- |
| `japanese_label_must_be_truncated_without_panicking_at_a_character_boundary` | 日本語を5文字へ安全に省略する |
| `ascii_label_is_truncated_to_the_requested_character_count` | ASCII文字列を指定文字数へ省略する |
| `short_label_is_preserved` | 上限未満の入力を変更しない |

修正済み状態で`cargo fmt --check`と`cargo test -- --nocapture`が成功しました。

## 再現手順

```bash
# 修正済み状態
cargo fmt --check
cargo test -- --nocapture

# バグ状態。作業中の変更は先に退避する
git switch --detach e43f633
cargo test japanese_label_must_be_truncated_without_panicking_at_a_character_boundary -- --nocapture
git switch main
```

## スコープと注意点

本ラボの「文字」はRustの`char`、すなわちUnicodeスカラー値です。結合文字や複数コードポイントから成る絵文字を、一つの見た目の文字として数える書記素クラスタの契約には対応しません。必要なら、要件に応じてUnicode書記素クラスタを扱う専用ライブラリを検討してください。[2]

## References

[1] [Rust Book: UTF-8文字列のスライス](https://doc.rust-lang.org/book/ch08-02-strings.html)

[2] [Rust標準ライブラリの`str`ドキュメント](https://doc.rust-lang.org/std/primitive.str.html)
