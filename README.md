# at-timer

AtCoderの過去問からランダムに1問出題するCLIツール

## 機能

- 指定した難易度（レーティング）の範囲からランダムに問題を出題
- コンテスト名・問題タイトル・難易度・問題ページのURLを表示
- 解答開始から Enter キーを押すまでの経過時間（クリアタイム）を計測して表示
- [AtCoder Problems](https://kenkoooo.com/atcoder/) が公開しているデータ（難易度モデル・問題一覧）を利用

## 必要環境

- Rust / Cargo（[rust-lang.org](https://www.rust-lang.org/) からインストール）
- インターネット接続（起動時に問題データを取得するため）

`Cargo.toml` にはクロスコンパイル用のターゲットとして `x86_64-pc-windows-gnu` が設定されています。Linux/WSL環境からWindows向けバイナリをビルドする場合は、事前に以下のツールチェインを追加してください。

```sh
rustup target add x86_64-pc-windows-gnu
```

Windows以外の環境で単に動作確認したい場合は、`--target` を指定して実行することでホスト環境向けにビルド・実行できます。

```sh
cargo run --target x86_64-unknown-linux-gnu
```

## 使い方

```sh
cargo build --release
cargo run
```

起動すると `Start` / `Exit` を選択するプロンプトが表示されます。

1. `Start` を選択
2. 出題してほしい難易度の下限（Min difficulty）と上限（Max difficulty）を入力
3. 条件に合う問題がランダムに選ばれ、コンテスト名・タイトル・難易度・URLが表示される
4. 問題を解き終えたら Enter キーを押すとクリアタイムが `[hh:mm:ss]` 形式で表示される
5. 続けて別の問題に挑戦する場合は再度 `Start`、終了する場合は `Exit` を選択

## データソース

- 難易度モデル: `https://kenkoooo.com/atcoder/resources/problem-models.json`
- 問題一覧: `https://kenkoooo.com/atcoder/resources/problems.json`
- 問題ページの存在確認: `https://atcoder.jp/contests/{contest_id}/tasks/{problem_id}`

## プロジェクト構成

```
src/
├── main.rs            # エントリポイント、コマンドループ
├── commands.rs         # Start / Exit コマンド定義
└── problem_mgr.rs       # 問題データの取得・検索ロジック
    └── problem.rs        # Problem / ProblemInfo データモデル
```
