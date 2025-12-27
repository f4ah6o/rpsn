use anyhow::{Context, Result};
use reqwest::{header, multipart, Client, Method, RequestBuilder, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct RepsonaClient {
    base_url: String,
    api_token: String,
    dry_run: bool,
    trace: bool,
    client: Client,
}

impl RepsonaClient {
    pub fn new(space_id: String, api_token: String, dry_run: bool, trace: bool) -> Self {
        let base_url = format!("https://{}.repsona.com/api", space_id);
        let client = Client::builder().build().unwrap();

        RepsonaClient {
            base_url,
            api_token,
            dry_run,
            trace,
            client,
        }
    }

    fn build_request(&self, method: Method, endpoint: &str) -> RequestBuilder {
        let url = format!("{}/{}", self.base_url, endpoint);
        self.client
            .request(method, &url)
            .header(header::AUTHORIZATION, format!("Bearer {}", self.api_token))
    }

    fn log_trace(&self, method: Method, endpoint: &str, request_body: Option<&serde_json::Value>, response: &Response) {
        if !self.trace {
            return;
        }

        eprintln!("[TRACE] {} {}", method, endpoint);
        if let Some(body) = request_body {
            eprintln!("[TRACE] Request body: {}", serde_json::to_string_pretty(body).unwrap_or_else(|_| "N/A".to_string()));
        }
        eprintln!("[TRACE] Response status: {}", response.status());
    }

    fn handle_rate_limits(&self, headers: &header::HeaderMap) {
        if let Some(limit) = headers.get("RateLimit-Limit") {
            if let Some(remaining) = headers.get("RateLimit-Remaining") {
                if let Ok(limit_str) = limit.to_str() {
                    if let Ok(remaining_str) = remaining.to_str() {
                        eprintln!("[Rate Limit] {}/{}", remaining_str, limit_str);
                    }
                }
            }
        }

        if let Some(reset) = headers.get("RateLimit-Reset") {
            if let Ok(reset_str) = reset.to_str() {
                eprintln!("[Rate Limit] Resets in: {}s", reset_str);
            }
        }
    }

    async fn execute_request<T: DeserializeOwned>(
        &self,
        method: Method,
        endpoint: &str,
        body: Option<&impl Serialize>,
    ) -> Result<T> {
        let method_clone = method.clone();
        let mut builder = self.build_request(method, endpoint);

        if let Some(b) = body {
            builder = builder.json(b);
        }

        if self.dry_run {
            let req_body = body.map(|b| serde_json::to_value(b).ok()).flatten();
            eprintln!("[DRY RUN] {} {}", method_clone, endpoint);
            if let Some(b) = req_body {
                eprintln!("[DRY RUN] Request body: {}", serde_json::to_string_pretty(&b)?);
            }
            return Err(anyhow::anyhow!("Dry run mode - request not executed"));
        }

        let response = builder.send().await.context("Failed to send request")?;

        self.handle_rate_limits(response.headers());

        let request_body = body.map(|b| serde_json::to_value(b).ok()).flatten();
        self.log_trace(method_clone, endpoint, request_body.as_ref(), &response);

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Failed to read error".to_string());
            return Err(anyhow::anyhow!("API error ({}): {}", status, error_text));
        }

        let response_text = response.text().await.context("Failed to read response")?;

        serde_json::from_str(&response_text).context("Failed to parse response")
    }

    pub async fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        self.execute_request::<T>(Method::GET, endpoint, None::<&()>).await
    }

    pub async fn post<T: DeserializeOwned>(&self, endpoint: &str, body: &impl Serialize) -> Result<T> {
        self.execute_request::<T>(Method::POST, endpoint, Some(body)).await
    }

    pub async fn patch<T: DeserializeOwned>(&self, endpoint: &str, body: &impl Serialize) -> Result<T> {
        self.execute_request::<T>(Method::PATCH, endpoint, Some(body)).await
    }

    pub async fn delete<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        self.execute_request::<T>(Method::DELETE, endpoint, None::<&()>).await
    }

    pub async fn post_multipart<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        form: multipart::Form,
    ) -> Result<T> {
        let mut builder = self.build_request(Method::POST, endpoint);

        if self.dry_run {
            eprintln!("[DRY RUN] POST {} (multipart)", endpoint);
            return Err(anyhow::anyhow!("Dry run mode - request not executed"));
        }

        builder = builder.multipart(form);

        let response = builder.send().await.context("Failed to send request")?;

        self.handle_rate_limits(response.headers());

        self.log_trace(Method::POST, endpoint, None, &response);

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Failed to read error".to_string());
            return Err(anyhow::anyhow!("API error ({}): {}", status, error_text));
        }

        let response_text = response.text().await.context("Failed to read response")?;

        serde_json::from_str(&response_text).context("Failed to parse response")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[tokio::test]
    async fn test_client_new() {
        let client = RepsonaClient::new(
            "test-space".to_string(),
            "test-token".to_string(),
            false,
            false,
        );

        assert_eq!(client.base_url, "https://test-space.repsona.com/api");
        assert_eq!(client.api_token, "test-token");
        assert_eq!(client.dry_run, false);
        assert_eq!(client.trace, false);
    }

    #[tokio::test]
    async fn test_dry_run_mode() {
        let client = RepsonaClient::new(
            "test".to_string(),
            "test-token".to_string(),
            true, // dry_run enabled
            false,
        );

        #[derive(Serialize)]
        struct RequestBody {
            test: String,
        }

        let request = RequestBody {
            test: "value".to_string(),
        };

        let result: Result<()> = client.post("test", &request).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Dry run"));
    }
}
