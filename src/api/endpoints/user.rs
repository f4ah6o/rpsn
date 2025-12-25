use crate::api::types::*;
use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SetUserRoleRequest {
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct SetPaymentRequest {
    #[serde(rename = "type")]
    pub payment_type: String,
}

impl crate::api::RepsonaClient {
    pub async fn list_users(&self) -> Result<ApiResponse<Vec<User>>> {
        self.get("user").await
    }

    pub async fn get_user(&self, user_id: u64) -> Result<ApiResponse<User>> {
        self.get(&format!("user/{}", user_id)).await
    }

    pub async fn set_user_role(&self, user_id: u64, request: &SetUserRoleRequest) -> Result<ApiResponse<User>> {
        self.patch(&format!("user/{}/role", user_id), request).await
    }

    pub async fn set_user_payment(&self, user_id: u64, request: &SetPaymentRequest) -> Result<ApiResponse<User>> {
        self.patch(&format!("user/{}/payment", user_id), request).await
    }

    pub async fn get_user_activity(&self, user_id: u64) -> Result<ApiResponse<Vec<Activity>>> {
        self.get(&format!("user/{}/activity", user_id)).await
    }
}
