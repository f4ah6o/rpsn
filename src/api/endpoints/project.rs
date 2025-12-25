use crate::api::types::*;
use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateProjectRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fullName")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateProjectRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fullName")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
}

impl crate::api::RepsonaClient {
    pub async fn list_projects(&self) -> Result<ApiResponse<Vec<Project>>> {
        self.get("project").await
    }

    pub async fn get_project(&self, project_id: u64) -> Result<ApiResponse<Project>> {
        self.get(&format!("project/{}", project_id)).await
    }

    pub async fn create_project(&self, request: &CreateProjectRequest) -> Result<ApiResponse<Project>> {
        self.post("project", request).await
    }

    pub async fn update_project(&self, project_id: u64, request: &UpdateProjectRequest) -> Result<ApiResponse<Project>> {
        self.patch(&format!("project/{}", project_id), request).await
    }

    pub async fn list_project_members(&self, project_id: u64) -> Result<ApiResponse<Vec<User>>> {
        self.get(&format!("project/{}/users", project_id)).await
    }

    pub async fn add_project_member(&self, project_id: u64, user_id: u64) -> Result<ApiResponse<Project>> {
        self.post(&format!("project/{}/user", project_id), &serde_json::json!({ "user": user_id })).await
    }

    pub async fn remove_project_member(&self, project_id: u64, user_id: u64) -> Result<ApiResponse<Project>> {
        self.delete(&format!("project/{}/user/{}", project_id, user_id)).await
    }

    pub async fn get_project_activity(&self, project_id: u64) -> Result<ApiResponse<Vec<Activity>>> {
        self.get(&format!("project/{}/activity", project_id)).await
    }

    pub async fn list_project_statuses(&self, project_id: u64) -> Result<ApiResponse<Vec<Status>>> {
        self.get(&format!("project/{}/status", project_id)).await
    }

    pub async fn list_project_milestones(&self, project_id: u64) -> Result<ApiResponse<Vec<Milestone>>> {
        self.get(&format!("project/{}/milestone", project_id)).await
    }
}
