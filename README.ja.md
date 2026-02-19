# rpsn - Repsona タスク管理 CLI

[Repsona](https://repsona.com) タスク管理システム用のコマンドラインインターフェース。

## 特徴

- **1:1 API マッピング** - Repsona REST API と直接対応
- **人間にやさしい** - 直感的なコマンド: list, get, create, update, delete
- **複数の出力形式** - 人間が読みやすいテーブル形式、またはスクリプト用の JSON
- **プロファイル対応** - 名前付きプロファイルで複数のワークスペースを管理
- **シェル補完** - bash, zsh, fish などの自動補完
- **エージェントスキル** - AI アシスタント用のスキルファイルを自動生成

## インストール

### ソースからビルド

```bash
git clone https://github.com/your-org/rpsn.git
cd rpsn
cargo build --release
cp target/release/rpsn ~/.local/bin/
```

### 必要環境

- Rust 1.70 以降
- API アクセス権を持つ Repsona アカウント

## クイックスタート

### 1. 設定の初期化

```bash
rpsn config init
```

### 2. 認証情報の設定

```bash
rpsn config set --space あなたのスペースID --token あなたのAPIトークン
```

スペース ID と API トークンは Repsona の設定ページで確認できます。

### 3. 接続の確認

```bash
rpsn config whoami
```

## コマンド

### グローバルオプション

| オプション | 説明 |
|--------|-------------|
| `--space <id>` | Repsona スペース ID を上書き |
| `--token <token>` | API トークンを上書き |
| `--profile <name>` | 指定したプロファイルを使用 |
| `--json` | JSON 形式で出力 |
| `--dry-run` | リクエストの表示のみ（実行しない） |
| `--yes` | 確認プロンプトをスキップ |
| `--trace` | デバッグ用に HTTP トレースを表示 |

### ユーティリティコマンド

```bash
rpsn util version      # バージョン情報を表示
rpsn util help         # ヘルプを表示
rpsn util ping         # API への接続を確認
```

### 設定管理

```bash
rpsn config init                                      # 設定を初期化
rpsn config get                                       # 現在の設定を表示
rpsn config set --space <id> --token <token>          # 認証情報を設定
rpsn config set-profile <name> --space <id> --token <token>  # 名前付きプロファイルを作成
rpsn config use <name>                                # プロファイルを切り替え
rpsn config whoami                                    # 現在のユーザー情報を表示
```

### 個人操作 (me)

```bash
rpsn me get                # 自分のユーザー情報を取得
rpsn me update             # プロフィールを更新
rpsn me tasks              # 自分のタスク一覧
rpsn me tasks-responsible  # 担当しているタスク一覧
rpsn me tasks-ball-holding # ボールを持っているタスク一覧
rpsn me tasks-following    # フォローしているタスク一覧
rpsn me tasks-count        # タスク数を取得
rpsn me projects           # 参加プロジェクト一覧
rpsn me activity           # アクティビティログを取得
```

### プロジェクト管理

```bash
rpsn project list                                     # 全プロジェクト一覧
rpsn project get <project_id>                         # プロジェクト詳細を取得
rpsn project create --name <name>                     # 新規プロジェクトを作成
rpsn project update <project_id> --name <name>        # プロジェクトを更新
rpsn project delete <project_id>                      # プロジェクトを削除
rpsn project members-list <project_id>                # プロジェクトメンバー一覧
rpsn project members-add <project_id> --user <id>     # メンバーを追加
rpsn project members-remove <project_id> --user <id>  # メンバーを削除
rpsn project activity <project_id>                    # プロジェクトのアクティビティを取得
rpsn project status-list <project_id>                 # ステータス一覧
rpsn project milestone-list <project_id>              # マイルストーン一覧
```

### タスク操作

```bash
rpsn task list <project_id>                           # プロジェクト内のタスク一覧
rpsn task get <project_id> <task_id>                  # タスク詳細を取得
rpsn task create <project_id> --title <title>         # タスクを作成
rpsn task update <project_id> <task_id> --title <t>   # タスクを更新
rpsn task done <project_id> <task_id>                 # タスクを完了にする
rpsn task reopen <project_id> <task_id>               # タスクを再開する
rpsn task delete <project_id> <task_id>               # タスクを削除する
rpsn task children <project_id> <task_id>             # サブタスク一覧
rpsn task comment-list <project_id> <task_id>         # タスクのコメント一覧
rpsn task comment-add <project_id> <task_id> --comment <text>  # コメントを追加
rpsn task comment-update <project_id> <comment_id> --comment <text>  # コメントを更新
rpsn task comment-delete <project_id> <comment_id>    # コメントを削除
rpsn task activity <project_id> <task_id>             # タスクのアクティビティを取得
rpsn task history <project_id> <task_id>              # タスクの履歴を取得
```

### ノート操作

```bash
rpsn note list <project_id>                           # プロジェクト内のノート一覧
rpsn note get <project_id> <note_id>                  # ノート詳細を取得
rpsn note create <project_id> --name <name>           # ノートを作成
rpsn note update <project_id> <note_id> --name <n>    # ノートを更新
rpsn note delete <project_id> <note_id>               # ノートを削除
rpsn note children <project_id> <note_id>             # サブノート一覧
rpsn note comment-list <project_id> <note_id>         # ノートのコメント一覧
rpsn note comment-add <project_id> <note_id> --comment <text>  # コメントを追加
rpsn note comment-update <project_id> <note_id> <comment_id> --comment <text>
rpsn note comment-delete <project_id> <note_id> <comment_id>
rpsn note activity <project_id> <note_id>             # ノートのアクティビティを取得
rpsn note history <project_id> <note_id>              # ノートの履歴を取得
```

### ファイル操作

```bash
rpsn file upload <project_id> <path>                  # プロジェクトにファイルをアップロード
rpsn file download --hash <hash> --out <path>         # ファイルをダウンロード
rpsn file attach <project_id> --model task --id <id> --file <file_id>  # ファイルを添付
rpsn file detach <project_id> --model task --id <id> --file <file_id>  # ファイルを解除
rpsn file delete <file_id>                            # ファイルを削除
```

### その他の操作

```bash
# タグ
rpsn tag list                                         # 全タグ一覧

# 受信箱
rpsn inbox list                                       # 受信箱の項目一覧
rpsn inbox update <inbox_id>                          # 受信箱の項目を更新
rpsn inbox read-all                                   # 全て既読にする
rpsn inbox unread-count                               # 未読数を取得

# スペース
rpsn space get                                        # スペース情報を取得
rpsn space invite --email <email>                     # ユーザーをスペースに招待

# ユーザー
rpsn user list                                        # 全ユーザー一覧
rpsn user get <user_id>                               # ユーザー詳細を取得
rpsn user role-set <user_id> --role <role>            # ユーザーのロールを設定
rpsn user payment-set <user_id> --type <type>         # 支払いタイプを設定
rpsn user activity <user_id>                          # ユーザーのアクティビティを取得

# Webhook
rpsn webhook list                                     # Webhook 一覧
rpsn webhook create --name <n> --url <u> --events <e> # Webhook を作成
rpsn webhook update <id> --name <name>                # Webhook を更新
rpsn webhook delete <id>                              # Webhook を削除

# ID リンク
rpsn idlink list                                      # ID リンク一覧
rpsn idlink create --name <name> --url <url>          # ID リンクを作成
rpsn idlink delete <id>                               # ID リンクを削除
```

### シェル補完

```bash
# 補完スクリプトを生成
rpsn completion bash   # Bash 用
rpsn completion zsh    # Zsh 用
rpsn completion fish   # Fish 用
rpsn completion elvish # Elvish 用
rpsn completion powershell  # PowerShell 用

# インストール（Bash）
rpsn completion bash > ~/.local/share/bash-completion/completions/rpsn

# インストール（Zsh）
rpsn completion zsh > ~/.zsh/completion/_rpsn

# インストール（Fish）
rpsn completion fish > ~/.config/fish/completions/rpsn.fish
```

### エージェントスキル生成

AI アシスタント用のスキルファイルを生成:

```bash
rpsn skill-generate                    # デフォルトの場所に生成
rpsn skill-generate --output ./SKILL.md  # カスタムパスに生成
```

## 設定ファイル

設定は `~/.config/rpsn/config.toml` に保存されます。

### 設定例

```toml
[default]
space_id = "your-space-id"
api_token = "your-api-token"

[work]
space_id = "work-space-id"
api_token = "work-api-token"

[personal]
space_id = "personal-space-id"
api_token = "personal-api-token"
```

### 環境変数

| 変数 | 説明 |
|----------|-------------|
| `REPSONA_SPACE` | スペース ID を上書き |
| `REPSONA_TOKEN` | API トークンを上書き |

## 使用例

### 詳細を指定してタスクを作成

```bash
rpsn task create 123 \
  --title "ログイン機能を実装" \
  --description "OAuth2 ログインサポートを追加" \
  --priority 3 \
  --assignee 456 \
  --tags "feature,auth"
```

### JSON 形式でタスク一覧を取得

```bash
rpsn --json task list 123
```

### リクエストのプレビュー（ドライラン）

```bash
rpsn --dry-run task create 123 --title "テストタスク"
```

### API 呼び出しをデバッグ

```bash
rpsn --trace task get 123 456
```

### 別のプロファイルを使用

```bash
rpsn --profile work project list
```

## エラーハンドリング

- レート制限は指数バックオフで自動的に処理されます
- 一般的な問題に対する詳細なエラーメッセージと提案
- API の問題をデバッグするには `--trace` を使用

## ライセンス

MIT ライセンス。詳細は [LICENSE](LICENSE) を参照してください。

## 関連リンク

- [Repsona](https://repsona.com) - タスク管理サービス
- [Repsona API ドキュメント](https://repsona.com/api/docs)
