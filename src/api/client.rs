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
    use serde::{Deserialize, Serialize};
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    struct TestResponse {
        message: String,
        value: i32,
    }

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
    async fn test_get_success() {
        let mock_server = MockServer::start().await;

        let response_body = TestResponse {
            message: "success".to_string(),
            value: 42,
        };

        Mock::given(method("GET"))
            .and(path("/test"))
            .and(header("authorization", "Bearer test-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let mut client = RepsonaClient::new(
            "test".to_string(),
            "test-token".to_string(),
            false,
            false,
        );
        client.base_url = mock_server.uri();

        let result: Result<TestResponse> = client.get("test").await;
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.message, "success");
        assert_eq!(data.value, 42);
    }

    #[tokio::test]
    async fn test_post_success() {
        let mock_server = MockServer::start().await;

        #[derive(Serialize)]
        struct RequestBody {
            name: String,
        }

        let response_body = TestResponse {
            message: "created".to_string(),
            value: 100,
        };

        Mock::given(method("POST"))
            .and(path("/create"))
            .and(header("authorization", "Bearer test-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let mut client = RepsonaClient::new(
            "test".to_string(),
            "test-token".to_string(),
            false,
            false,
        );
        client.base_url = mock_server.uri();

        let request = RequestBody {
            name: "test".to_string(),
        };

        let result: Result<TestResponse> = client.post("create", &request).await;
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.message, "created");
        assert_eq!(data.value, 100);
    }

    #[tokio::test]
    async fn test_patch_success() {
        let mock_server = MockServer::start().await;

        #[derive(Serialize)]
        struct UpdateBody {
            field: String,
        }

        let response_body = TestResponse {
            message: "updated".to_string(),
            value: 200,
        };

        Mock::given(method("PATCH"))
            .and(path("/update"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let mut client = RepsonaClient::new(
            "test".to_string(),
            "test-token".to_string(),
            false,
            false,
        );
        client.base_url = mock_server.uri();

        let request = UpdateBody {
            field: "value".to_string(),
        };

        let result: Result<TestResponse> = client.patch("update", &request).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().message, "updated");
    }

    #[tokio::test]
    async fn test_delete_success() {
        let mock_server = MockServer::start().await;

        let response_body = TestResponse {
            message: "deleted".to_string(),
            value: 0,
        };

        Mock::given(method("DELETE"))
            .and(path("/remove"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let mut client = RepsonaClient::new(
            "test".to_string(),
            "test-token".to_string(),
            false,
            false,
        );
        client.base_url = mock_server.uri();

        let result: Result<TestResponse> = client.delete("remove").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().message, "deleted");
    }

    #[tokio::test]
    async fn test_api_error_404() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/notfound"))
            .respond_with(ResponseTemplate::new(404).set_body_string("Not found"))
            .mount(&mock_server)
            .await;

        let mut client = RepsonaClient::new(
            "test".to_string(),
            "test-token".to_string(),
            false,
            false,
        );
        client.base_url = mock_server.uri();

        let result: Result<TestResponse> = client.get("notfound").await;
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("404"));
    }

    #[tokio::test]
    async fn test_api_error_500() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/error"))
            .respond_with(
                ResponseTemplate::new(500).set_body_string("Internal server error"),
            )
            .mount(&mock_server)
            .await;

        let mut client = RepsonaClient::new(
            "test".to_string(),
            "test-token".to_string(),
            false,
            false,
        );
        client.base_url = mock_server.uri();

        let result: Result<TestResponse> = client.get("error").await;
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("500"));
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

        let result: Result<TestResponse> = client.post("test", &request).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Dry run"));
    }

    #[tokio::test]
    async fn test_authorization_header() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/auth-test"))
            .and(header("authorization", "Bearer my-secret-token"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(&TestResponse {
                    message: "authorized".to_string(),
                    value: 1,
                }),
            )
            .mount(&mock_server)
            .await;

        let mut client = RepsonaClient::new(
            "test".to_string(),
            "my-secret-token".to_string(),
            false,
            false,
        );
        client.base_url = mock_server.uri();

        let result: Result<TestResponse> = client.get("auth-test").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().message, "authorized");
    }

    #[tokio::test]
    async fn test_malformed_json_response() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/bad-json"))
            .respond_with(ResponseTemplate::new(200).set_body_string("not valid json"))
            .mount(&mock_server)
            .await;

        let mut client = RepsonaClient::new(
            "test".to_string(),
            "test-token".to_string(),
            false,
            false,
        );
        client.base_url = mock_server.uri();

        let result: Result<TestResponse> = client.get("bad-json").await;
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("Failed to parse"));
    }

    #[tokio::test]
    async fn test_rate_limit_headers() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/rate-limited"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(&TestResponse {
                        message: "ok".to_string(),
                        value: 1,
                    })
                    .insert_header("RateLimit-Limit", "100")
                    .insert_header("RateLimit-Remaining", "50")
                    .insert_header("RateLimit-Reset", "60"),
            )
            .mount(&mock_server)
            .await;

        let mut client = RepsonaClient::new(
            "test".to_string(),
            "test-token".to_string(),
            false,
            false,
        );
        client.base_url = mock_server.uri();

        let result: Result<TestResponse> = client.get("rate-limited").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_trace_mode() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/trace-test"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(&TestResponse {
                    message: "traced".to_string(),
                    value: 42,
                }),
            )
            .mount(&mock_server)
            .await;

        let mut client = RepsonaClient::new(
            "test".to_string(),
            "test-token".to_string(),
            false,
            true, // trace enabled
        );
        client.base_url = mock_server.uri();

        let result: Result<TestResponse> = client.get("trace-test").await;
        assert!(result.is_ok());
    }
}
