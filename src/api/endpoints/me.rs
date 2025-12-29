use crate::api::types::*;
use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
pub struct MeUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "whatAreYouDoing")]
    pub what_are_you_doing: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct TaskFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestones: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priorities: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "responsibleUsers")]
    pub responsible_users: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ballHoldingUsers")]
    pub ball_holding_users: Option<String>,
}

impl crate::api::RepsonaClient {
    pub async fn get_me(&self) -> Result<ApiResponse<UserData>> {
        self.get("me").await
    }

    pub async fn update_me(&self, updates: MeUpdateRequest) -> Result<ApiResponse<UserData>> {
        self.patch("me", &updates).await
    }

    pub async fn get_me_tasks(&self, _filter: &TaskFilter) -> Result<ApiResponse<TasksData>> {
        self.get("me/tasks").await
    }

    pub async fn get_me_tasks_responsible(&self, _filter: &TaskFilter) -> Result<ApiResponse<TasksData>> {
        self.get("me/task/responsible").await
    }

    pub async fn get_me_tasks_ball_holding(&self, _filter: &TaskFilter) -> Result<ApiResponse<TasksData>> {
        self.get("me/task/ballHolding").await
    }

    pub async fn get_me_tasks_following(&self, _filter: &TaskFilter) -> Result<ApiResponse<TasksData>> {
        self.get("me/task/following").await
    }

    pub async fn get_me_tasks_count(&self) -> Result<ApiResponse<TaskCountData>> {
        self.get("me/task/all/count").await
    }

    pub async fn get_me_projects(&self) -> Result<ApiResponse<ProjectsData>> {
        self.get("me/project").await
    }

    pub async fn get_me_activity(&self) -> Result<ApiResponse<ActivityData>> {
        self.get("feed").await
    }
}
