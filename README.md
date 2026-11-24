# 国公立大 予想問題サイト

Rust 製の静的サイト生成器です。`content/<教科>/*.md` を読み、`dist/` に HTML を出力します。
サーバー処理が無いため、GitHub Pages で無料公開できます。

## ビルド

```
cargo run --release
```

公開 URL を `SITE_URL` に設定すると、canonical・sitemap.xml・構造化データも出力します
（GitHub Actions では自動で設定されます）。

```
SITE_URL=https://<ユーザー名>.github.io/<リポジトリ名> cargo run --release
```

## 問題の追加

- 教科ごとに `content/<教科>/_subject.toml`（`name` / `desc` / `order`）を置く
- 問題ファイルは先頭に `+++` で囲んだフロントマター（`title` / `level = "easy" | "hard"` / `note` / `order`）
- 本文中の `@@解答@@` より後ろは、別ページの解答・解説になる
- 数式は `$...$`（インライン）と `$$...$$`（ディスプレイ）で書く（KaTeX）
