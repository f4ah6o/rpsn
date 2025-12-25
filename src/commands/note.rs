use crate::api::{RepsonaClient, endpoints::note::*};
use crate::cli::NoteCommands;
use crate::output::{print, OutputFormat, print_success};
use anyhow::Result;

pub async fn handle(client: &RepsonaClient, command: NoteCommands, json: bool) -> Result<()> {
    let format = if json { OutputFormat::Json } else { OutputFormat::Human };

    match command {
        NoteCommands::List { project_id } => {
            let response = client.list_notes(project_id).await?;
            print(&response.notes, format)?;
        }
        NoteCommands::Get { project_id, note_id } => {
            let response = client.get_note(project_id, note_id).await?;
            print(&response.note, format)?;
        }
        NoteCommands::Create { project_id, name, description, parent, tags, add_to_bottom } => {
            let tags_vec = tags.map(|t| t.split(',').filter_map(|s| s.trim().parse().ok()).collect());
            let request = CreateNoteRequest {
                name,
                description,
                parent,
                tags: tags_vec,
                add_to_bottom: Some(add_to_bottom),
            };
            let response = client.create_note(project_id, &request).await?;
            print(&response.note, format)?;
            print_success(&format!("Note '{}' created", response.note.name));
        }
        NoteCommands::Update { project_id, note_id, name, description, tags } => {
            let tags_vec = tags.map(|t| t.split(',').filter_map(|s| s.trim().parse().ok()).collect());
            let request = UpdateNoteRequest { name, description, tags: tags_vec };
            let response = client.update_note(project_id, note_id, &request).await?;
            print(&response.note, format)?;
            print_success(&format!("Note '{}' updated", response.note.name));
        }
        NoteCommands::Delete { project_id, note_id } => {
            client.delete_note(project_id, note_id).await?;
            print_success("Note deleted");
        }
        NoteCommands::Children { project_id, note_id } => {
            let response = client.get_note_children(project_id, note_id).await?;
            print(&response.notes, format)?;
        }
        NoteCommands::CommentList { project_id, note_id } => {
            let response = client.list_note_comments(project_id, note_id).await?;
            print(&response.note_comments, format)?;
        }
        NoteCommands::CommentAdd { project_id, note_id, comment } => {
            let response = client.add_note_comment(project_id, note_id, comment).await?;
            print(&response.note_comment, format)?;
            print_success("Comment added");
        }
        NoteCommands::CommentUpdate { project_id, note_id, comment_id, comment } => {
            let response = client.update_note_comment(project_id, note_id, comment_id, comment).await?;
            print(&response.note_comment, format)?;
            print_success("Comment updated");
        }
        NoteCommands::CommentDelete { project_id, note_id, comment_id } => {
            client.delete_note_comment(project_id, note_id, comment_id).await?;
            print_success("Comment deleted");
        }
        NoteCommands::Activity { project_id, note_id } => {
            let response = client.get_note_activity(project_id, note_id).await?;
            print(&response.activity, format)?;
        }
        NoteCommands::History { project_id, note_id } => {
            let response = client.get_note_history(project_id, note_id).await?;
            print(&response.history, format)?;
        }
    }

    Ok(())
}
