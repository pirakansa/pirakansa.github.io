# Map Panels

Rust + egui/eframe で構築したリポジトリギャラリー UI です。レスポンシブなレイアウトと YAML で管理するデータセットを組み合わせ、GitHub ライクな "Repositories Map" をデスクトップ/ブラウザ双方に配信できます。

## 主な特徴
- Hero / カルーセル / フッターなど UI コンポーネントを `src/app/components/` に分割し、シンプルにカスタム可能
- `assets/featured.yaml` と `assets/portfolio.yaml` を編集するだけで掲載リポジトリを更新
- Noto Sans JP をバンドルし、日本語フォントを環境差なく表示
- `ResponsiveLayout` により 520px/720px を境界にしたレスポンシブ対応
- `serde` + `persistence` 機能で（リリースビルド時に）検索キーワードなどを保存

## セットアップ
1. Rust stable と `vorbere` を用意します（`rustup default stable` 推奨）。
2. 初回は依存解決とアセット取得を兼ねて `vorbere run build` を実行します。
3. `assets/NotoSansJP-Regular.otf` が存在しない場合は自動でダウンロードされます。フォントのみ取得したい場合は `vorbere run assets`。

### Web 版の前提
- `rustup target add wasm32-unknown-unknown`
- `cargo install trunk`

## よく使う vorbere タスク
- `vorbere run run` : ネイティブアプリを起動
- `vorbere run web` : Trunk で `http://localhost:8080` をホスト（開発用ビルド）
- `vorbere run web-release` : `dist/` に最適化済みアセットを生成
- `vorbere run test` / `vorbere run check` / `vorbere run build` : CI 相当の検証
- `vorbere run fmt` / `vorbere run fmt-check` : rustfmt
- `vorbere run clippy` / `vorbere run audit` : 静的解析と脆弱性チェック
- `vorbere run ci` : 上記主要タスクを一括実行
- `vorbere run clean` : `target/` や生成物を削除

`vorbere tasks list` で全コマンドを確認できます。

## ブラウザ版の手順
1. 上記「Web 版の前提」を満たします。
2. `vorbere run web` を実行すると Trunk が立ち上がり、`index.html` の `<canvas id="canvas_id">` に描画されます。
3. 配布用アセットが必要な場合は `vorbere run web-release` で `dist/` を生成します。

## データの更新方法
- `assets/featured.yaml` : ヒーローセクション（リポジトリ名/説明/タグ/スター数）
- `assets/portfolio.yaml` : セクション配列とカード情報（名称/説明/バッジ/画像 URL など）

YAML を編集したらホットリロードまたは再ビルドで UI に即反映されます。`cargo` の永続化機能はデバッグビルドではオフにしているため、データ更新の確認が容易です。


## 開発フロー
1. `main` からブランチを作成し、ロジックは極力 `src/app/` のモジュールで管理します。
2. 変更時は `vorbere run check && vorbere run test && vorbere run build` を通します。
3. Web 配信を確認したい場合は `vorbere run web` で挙動をチェックし、`dist/` の生成物をレビューに添付してください。

## ライセンス
本リポジトリは MIT License で提供されます。詳細は `LICENSE` を参照してください。
