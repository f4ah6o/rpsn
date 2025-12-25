# rpsn - Repsona Task Management CLI
# =================================
#
# 方針
# - Repsona REST API に 1:1 で対応
# - 人間向け操作を優先（list/get/create/update）
# - 危険操作は明示的に実行
# - ai サブコマンドは現時点では存在しない
#

# ---------------------------------
# グローバル仕様
# ---------------------------------

rpsn [global options] <command> [subcommand] [args]

Global Options:
  --space <space_id>        Repsona Space ID
  --token <api_key>         API Token
  --profile <name>          Config profile
  --json                    JSON output
  --dry-run                 Show request only
  --yes                     Skip confirmation
  --trace                   Show HTTP trace
  -h, --help
  -v, --version

Env:
  REPSONA_SPACE
  REPSONA_TOKEN

Config:
  ~/.config/rpsn/config.toml


# ---------------------------------
# 0. Utility
# ---------------------------------

rpsn version
rpsn help
rpsn ping


# ---------------------------------
# 1. config
# ---------------------------------

rpsn config init
rpsn config get
rpsn config set --space <space_id> --token <api_key>

rpsn config set-profile <name> \
  --space <space_id> \
  --token <api_key>

rpsn config use <name>

rpsn config whoami


# ---------------------------------
# 2. me (自分)
# ---------------------------------

rpsn me get
rpsn me update

rpsn me tasks
rpsn me tasks responsible
rpsn me tasks ballHolding
rpsn me tasks following

rpsn me tasks-count
rpsn me projects
rpsn me activity


# ---------------------------------
# 3. project
# ---------------------------------

rpsn project list
rpsn project get <projectId>

rpsn project create \
  --name <name> \
  [--full-name <full>] \
  [--purpose <text>]

rpsn project update <projectId> \
  [--name <name>] \
  [--purpose <text>]

rpsn project members list <projectId>
rpsn project members add <projectId> --user <userId>
rpsn project members remove <projectId> --user <userId>

rpsn project activity <projectId>

rpsn project status list <projectId>
rpsn project milestone list <projectId>


# ---------------------------------
# 4. task
# ---------------------------------

rpsn task list <projectId>

rpsn task get <projectId> <taskId>

rpsn task create <projectId> \
  --title <title> \
  [--description <text>] \
  [--status <status>] \
  [--priority <p>] \
  [--due <date>] \
  [--assignee <userId>] \
  [--tags <tag,tag>]

rpsn task update <projectId> <taskId> \
  [--title <title>] \
  [--description <text>] \
  [--status <status>] \
  [--priority <p>] \
  [--due <date>] \
  [--assignee <userId>] \
  [--tags <tag,tag>]

rpsn task done <projectId> <taskId>
rpsn task reopen <projectId> <taskId>

rpsn task children <projectId> <taskId>

rpsn task comment list <projectId> <taskId>

rpsn task comment add <projectId> <taskId> \
  --comment <text> \
  [--reply-to <commentId>]

rpsn task activity <projectId> <taskId>
rpsn task history <projectId> <taskId>


# ---------------------------------
# 5. note
# ---------------------------------

rpsn note list <projectId>
rpsn note get <projectId> <noteId>

rpsn note create <projectId> \
  --name <name> \
  [--description <text>] \
  [--parent <noteId>] \
  [--tags <tag,tag>] \
  [--add-to-bottom]

rpsn note update <projectId> <noteId>
rpsn note delete <projectId> <noteId>

rpsn note children <projectId> <noteId>

rpsn note comment list <projectId> <noteId>
rpsn note comment add <projectId> <noteId> --comment <text>
rpsn note comment update <projectId> <noteId> <commentId>
rpsn note comment delete <projectId> <noteId> <commentId>

rpsn note activity <projectId> <noteId>
rpsn note history <projectId> <noteId>


# ---------------------------------
# 6. file
# ---------------------------------

rpsn file upload <projectId> <path>

rpsn file download \
  --hash <fileHash> \
  [--out <path>]

rpsn file attach <projectId> \
  --model task|task_comment|note|note_comment \
  --id <modelId> \
  --file <fileId>

rpsn file detach <projectId> \
  --model task|task_comment|note|note_comment \
  --id <modelId> \
  --file <fileId>

rpsn file delete <fileId>


# ---------------------------------
# 7. tag
# ---------------------------------

rpsn tag list


# ---------------------------------
# 8. inbox
# ---------------------------------

rpsn inbox list
rpsn inbox update <inboxId>
rpsn inbox read-all
rpsn inbox unread-count


# ---------------------------------
# 9. space / user / invite
# ---------------------------------

rpsn space get

rpsn space invite \
  --email <email> \
  [--role <role>]

rpsn user list
rpsn user get <userId>

rpsn user role set <userId> --role <role>
rpsn user payment set <userId> --type <type>

rpsn user activity <userId>


# ---------------------------------
# 10. webhook / idlink
# ---------------------------------

rpsn webhook list
rpsn webhook create
rpsn webhook update <webhookId>
rpsn webhook delete <webhookId>

rpsn idlink list
rpsn idlink create
rpsn idlink delete <idlinkId>

