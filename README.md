# Rustで日本語ラベルの省略がパニックする問題を切り分ける

Rust標準ライブラリだけで、日本語などのUTF-8文字列を「最大N文字」として省略する処理がパニックする不具合を再現します。文字数をバイト添字として使うコードを、HTTPや外部サービスに依存せず、テスト・ログ・GDB・一次資料で調査します。

## この題材で守る契約

| 入力 | 最大文字数 | 期待する出力 |
| --- | ---: | --- |
| `障害対応手順` | 5 | `障害対応手…` |
| `incident-report` | 8 | `incident…` |
| `監視` | 4 | `監視` |

`max_chars`はUTF-8のバイト数ではなく、Unicodeスカラー値として数えます。したがって日本語入力でもパニックせず、指定された文字数までを残します。絵文字のような書記素クラスタを一つの見た目の文字として数える契約は、このラボの対象外です。[1]

## 最短の開始手順

```bash
cargo fmt --check
cargo test -- --nocapture
```

## バグを再現する

バグ状態は`e43f633`です。作業中の変更を退避してから、次を実行してください。

```bash
git switch --detach e43f633
cargo test japanese_label_must_be_truncated_without_panicking_at_a_character_boundary -- --nocapture
git switch main
```

`max_chars=5`を`&text[..max_chars]`へ渡すため、`障害対応手順`の2文字目の途中を切ろうとしてパニックします。

## 構成

```text
src/lib.rs                  省略関数と回帰テスト
README.md                   実行・バグ再現の導線
docs/topic-brief.md         契約と仮説
docs/debugging-record.md    観測・原因・修正・回帰範囲
```

## References

[1] [Rust標準ライブラリの`str`ドキュメント](https://doc.rust-lang.org/std/primitive.str.html)
