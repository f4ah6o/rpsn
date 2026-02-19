use crate::api::types::*;
use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateTaskRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dueDate")]
    pub due_date: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "startDate")]
    pub start_date: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "responsibleUser")]
    pub responsible_user: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ballHoldingUser")]
    pub ball_holding_user: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "addToBottom")]
    pub add_to_bottom: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateTaskRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dueDate")]
    pub due_date: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "startDate")]
    pub start_date: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "responsibleUser")]
    pub responsible_user: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ballHoldingUser")]
    pub ball_holding_user: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<u64>>,
}

impl crate::api::RepsonaClient {
    pub async fn list_tasks(
        &self,
        project_id: u64,
        _filter: &super::me::TaskFilter,
    ) -> Result<ApiResponse<TasksData>> {
        self.get(&format!("project/{}/task", project_id)).await
    }

    pub async fn get_task(&self, project_id: u64, task_id: u64) -> Result<ApiResponse<TaskData>> {
        self.get(&format!("project/{}/task/{}", project_id, task_id))
            .await
    }

    pub async fn create_task(
        &self,
        project_id: u64,
        request: &CreateTaskRequest,
    ) -> Result<ApiResponse<TaskData>> {
        self.post(&format!("project/{}/task", project_id), request)
            .await
    }

    pub async fn update_task(
        &self,
        project_id: u64,
        task_id: u64,
        request: &UpdateTaskRequest,
    ) -> Result<ApiResponse<TaskData>> {
        self.patch(&format!("project/{}/task/{}", project_id, task_id), request)
            .await
    }

    pub async fn delete_task(&self, project_id: u64, task_id: u64) -> Result<()> {
        self.delete(&format!("project/{}/task/{}", project_id, task_id))
            .await
    }

    pub async fn set_task_status(
        &self,
        project_id: u64,
        task_id: u64,
        status_id: u64,
    ) -> Result<ApiResponse<TaskData>> {
        self.patch(
            &format!("project/{}/task/{}", project_id, task_id),
            &serde_json::json!({ "status": status_id }),
        )
        .await
    }

    pub async fn get_task_children(
        &self,
        project_id: u64,
        task_id: u64,
    ) -> Result<ApiResponse<TasksData>> {
        self.get(&format!("project/{}/task/{}/children", project_id, task_id))
            .await
    }

    pub async fn list_task_comments(
        &self,
        project_id: u64,
        task_id: u64,
    ) -> Result<ApiResponse<TaskCommentsData>> {
        self.get(&format!(
            "project/{}/task/{}/task_comment",
            project_id, task_id
        ))
        .await
    }

    pub async fn add_task_comment(
        &self,
        project_id: u64,
        task_id: u64,
        comment: String,
        reply_to: Option<u64>,
    ) -> Result<ApiResponse<TaskCommentData>> {
        let body = match reply_to {
            Some(reply_to_id) => serde_json::json!({ "comment": comment, "replyTo": reply_to_id }),
            None => serde_json::json!({ "comment": comment }),
        };
        self.post(
            &format!("project/{}/task/{}/task_comment", project_id, task_id),
            &body,
        )
        .await
    }

    pub async fn update_task_comment(
        &self,
        project_id: u64,
        comment_id: u64,
        comment: String,
    ) -> Result<ApiResponse<TaskCommentData>> {
        self.patch(
            &format!("project/{}/task_comment/{}", project_id, comment_id),
            &serde_json::json!({ "comment": comment }),
        )
        .await
    }

    pub async fn delete_task_comment(&self, project_id: u64, comment_id: u64) -> Result<()> {
        self.delete(&format!(
            "project/{}/task_comment/{}",
            project_id, comment_id
        ))
        .await
    }

    pub async fn get_task_activity(
        &self,
        project_id: u64,
        task_id: u64,
    ) -> Result<ApiResponse<ActivityData>> {
        self.get(&format!("project/{}/task/{}/activity", project_id, task_id))
            .await
    }

    pub async fn get_task_history(
        &self,
        project_id: u64,
        task_id: u64,
    ) -> Result<ApiResponse<HistoryData>> {
        self.get(&format!("project/{}/task/{}/history", project_id, task_id))
            .await
    }
}
