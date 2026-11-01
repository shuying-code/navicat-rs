<p align="center">🌐 <b>言語</b>：<a href="README.md">简体中文</a> · <a href="README.en.md">English</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a> · <a href="README.ru.md">Русский</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a> · <a href="README.de.md">Deutsch</a> · <a href="README.pt-BR.md">Português</a></p>

# navicat-rs

**Rust + egui/eframe** で作られた**クロスプラットフォームの単一ファイル**データベース管理デスクトップクライアントです。UI とワークフローは Navicat Premium 18 を参考にしています。
**すべての機能を無料開放。ログイン・試用・ライセンス/権限の制限はありません。**

<p>
  <img alt="CI" src="https://github.com/shuying-code/navicat-rs/actions/workflows/ci.yml/badge.svg">
  <img alt="build" src="https://github.com/shuying-code/navicat-rs/actions/workflows/build.yml/badge.svg">
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.75%2B-orange">
  <img alt="Platform" src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-2f6fed">
  <img alt="License" src="https://img.shields.io/badge/license-MIT-green">
  <img alt="Release" src="https://img.shields.io/github/v/release/shuying-code/navicat-rs?display_name=tag">
</p>

> 本プロジェクトは**ローカル優先**の独立実装で、クラウド接続・ログイン・サブスク・試用・テレメトリはありません。**すべての機能を誰でも無条件に利用できます。**

![ようこそ](assets/screenshot-welcome.png)

---

## なぜ「無制限」なのか

Navicat の有料 / ログイン制限は主に**クラウド / サブスク**機能（クラウド同期、チーム連携、クラウド自動化など）に対するものです。本ツールは**クラウド非依存のローカル**データベース管理のみを実装しているため、アカウントも課金もクォータも不要です。設計上、ログイン / サブスク / ライセンスの制限を取り除き、すべてのローカル機能を無条件に利用できます。

---

## 機能

- **テーブル**：ページング閲覧、フィルタ、インライン編集、行の追加/削除、並べ替え（ヘッダークリック）、エクスポート。
- **SQL クエリ**：複数ステートメント実行、`Ctrl+Enter`、複数グリッド結果、実行時間 / 行数、Explain 実行計画ツリー、SQL 整形、SQL アウトライン / リファクタ提案、クエリ履歴。
- **設計とモデリング**：テーブル設計（列編集 → DDL）、ER 図、クエリビルダー、モデル設計（概念/論理/物理 · リバースエンジニアリング · 比較 · 同期）。
- **データ移行**：データ転送（列マッピング + WHERE）、データ/構造同期、インポートウィザード（CSV/JSON/SQL）、エクスポートウィザード（CSV/JSON/SQL/HTML）、データポンプ。
- **運用と拡張**：バックアップ / リストア（インメモリ DB の `VACUUM INTO` 含む）、サーバー監視、プロファイリング、スキーマ分析、データ生成、マスキング、チャート、データ辞書、レポート。
- **オブジェクトと検索**：オブジェクト管理（インデックス/トリガー/ビュー）、データベース内検索（テーブル横断の列値検索）、オブジェクトツリーのズームとグループ折りたたみ。
- **その他**：AI アシスタント、自動化 / スケジュールタスク、コードスニペット、ユーザーと権限（すべて開放）、設定（マルチテーマ / ツリーズーム / 自動コミット / 削除確認）。
- **マルチテーマ**：ライト / ダーク / ミッドナイト / フォレスト / サンド / グラファイト。どのテーマでも本文とボタン文字は高コントラストで可読。

## GUI

- **純 Rust + egui/eframe**：単一ファイル exe、WebView 不要、ダブルクリックで起動。CJK フォント内蔵で豆腐（□）なし。
- **3 領域レイアウト**：メニューバー / ツールバー / タブバー / 左の接続とオブジェクトツリー / 右の機能ページ / ステータスバー。
- **オブジェクトツリー**：接続 → テーブル / ビュー / バックアップ / モデル / ユーザー。ズーム可能（`− 100% ＋`、`Ctrl+ホイール`、設定スライダー）、グループ折りたたみ可能。
- **マルチタブ**：テーブル、クエリ、設計、ER、ビルダー、各ツールページを並行して開けます。
- **マルチテーマ**：`表示 → テーマ` または `設定 → テーマ` で切替、選択は保存されます。

## 対応プラットフォーム

| プラットフォーム | 状態 | 説明 |
|---|---|---|
| Windows x64 | ✅ ビルド検証済み | MSVC ツールチェーン推奨。Release にビルド済み exe |
| macOS（Intel / Apple Silicon） | ✅ CI ビルド（universal） | `build.yml` がユニバーサルバイナリを生成 |
| Linux（X11 / Wayland） | ✅ CI ビルド | `build.yml` が x64 アーカイブを生成（システム GUI 依存が必要） |

## ビルド

Rust 1.75+ が必要（stable 推奨）。

```bash
cargo build --release
```

### Windows（MSVC）

```powershell
# Visual Studio Build Tools（C++ と Windows SDK）が必要
cargo build --release
```

### macOS

```bash
xcode-select --install   # 初回に必要な場合あり
cargo build --release
```

### Linux（Debian/Ubuntu）

```bash
sudo apt-get install -y build-essential pkg-config \
  libxkbcommon-dev libgtk-3-dev libgl1-mesa-dev \
  libx11-dev libxi-dev libxrandr-dev libxcursor-dev libxinerama-dev \
  libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev
cargo build --release
```

## ビルド済み / クロスプラットフォーム成果物

- **Releases**: <https://github.com/shuying-code/navicat-rs/releases>（Windows x64 exe/zip、macOS universal tar.gz、Linux x64 tar.gz。`build.yml` が生成）。
- **CI マトリクス**: `.github/workflows/ci.yml` が `ubuntu-latest / macos-latest / windows-latest` でビルドし `--selftest` を実行。
- **コンテナイメージ（GHCR）**:

```bash
docker pull ghcr.io/shuying-code/navicat-rs:latest
docker run --rm ghcr.io/shuying-code/navicat-rs:latest --selftest
```

## 使い方

```bash
navicat-rs              # GUI を起動
navicat-rs --selftest   # ヘッドレス自己テスト（SQLite エンジン + 主要機能、22 項目）
```

初回起動時に「サンプルデータベース (SQLite)」（4 テーブル + 1 ビュー + デモデータ）を自動作成します。オブジェクトツリーでテーブルをダブルクリックして閲覧。上部ツールバーからクエリ / テーブル設計 / ER / クエリビルダー / 転送 / インポート / エクスポート / バックアップ / 監視 / チャート / プロファイリング / AI など全ページを開けます。

## ワークスペース（ローカル設定）

設定と状態（接続、履歴、お気に入り、テーマ、設定）は各プラットフォームのユーザー設定ディレクトリに保存されます：

| プラットフォーム | パス |
|---|---|
| Windows | `%APPDATA%\navicat-rs\navicat-rs.json` |
| macOS | `~/Library/Application Support/navicat-rs/navicat-rs.json` |
| Linux | `$XDG_CONFIG_HOME/navicat-rs/navicat-rs.json` または `~/.config/navicat-rs/navicat-rs.json` |

## 検証

- `--selftest` がエンジンと主要機能 **22 項目すべて合格**。
- CI が 3 プラットフォーム（Windows / macOS / Ubuntu）でビルド + セルフテスト **すべて success**。
- 6 テーマをスクリーンショットで本文 / 太字タイトル / ヘッダー / ボタン文字の可読性を確認。

## データベースバックエンド

| エンジン | 状態 |
|---|---|
| SQLite | **完全に実物**：接続、クエリ、CRUD、DDL、スクリプト、外部キー、PRAGMA（`rusqlite` bundled、クロスプラットフォーム） |
| PostgreSQL | **実ネットワークプロトコル**：接続テスト + 実クエリ（純 Rust `postgres` ドライバ） |
| MySQL / MariaDB / Oracle / SQL Server / MongoDB / Redis / Snowflake / Dameng / ODBC | 接続設定 + オフラインデモ schema（UI/ワークフローは完備。ドライバ層は抽象化済みで `src/realdb.rs` に沿って追加可能） |

## コマンドライン引数

| 引数 | 説明 |
|---|---|
| （なし） | GUI を起動 |
| `--selftest` | ヘッドレス自己テストを実行し `[PASS]/[FAIL]` を出力、終了コードで結果を返す（0=全合格） |

## 環境変数（任意）

| 変数 | 説明 |
|---|---|
| `NAVICAT_RS_THEME` | 起動時テーマ：`light`/`dark`/`midnight`/`forest`/`sand`/`graphite` |
| `NAVICAT_RS_DEMO` | `1` でサンプルクエリとテーブルを開く；`all` で全機能ページを開く（スクリーンショット/検証用） |
| `NAVICAT_RS_TAB` | demo と併用し、タイトルにこの部分文字列を含むタブへ切替 |

## よくある質問（FAQ）

**Q：Navicat のアカウントやライセンスは必要？** 不要です。起動するとすぐメイン UI に入り、ログイン / アクティベーション / 試用のプロンプトはありません。

**Q：データはどこに保存される？** 「ワークスペース」参照。Windows は `%APPDATA%\navicat-rs\navicat-rs.json`、データベース自体は接続する SQLite ファイル次第です。

**Q：macOS バイナリはなぜ CI で作る？** 本機は Windows のため、macOS ユニバーサルバイナリは GitHub Actions の `macos-latest` が生成します（`build.yml` 参照）。

**Q：非 SQLite エンジンは本当に接続できる？** SQLite は完全に実物、PostgreSQL は実ネットワークプロトコル。その他は接続設定 + オフラインデモ DB で、`src/realdb.rs` に沿って拡張できます。

## フィードバック（Issues）

GitHub Issues をご利用ください：<https://github.com/shuying-code/navicat-rs/issues/new/choose>。投稿前に **[フィードバックガイド](docs/FEEDBACK.md)** をお読みください。メンテナは通常週 1 回まとめて対応します。

## ライセンス

[MIT](LICENSE)
