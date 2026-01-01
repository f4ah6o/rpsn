use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// AIで生成されたタスク
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedTask {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u32>,
}

/// AI APIからのレスポンス構造
#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    content: Vec<ContentBlock>,
}

#[derive(Debug, Deserialize)]
struct ContentBlock {
    #[serde(rename = "type")]
    _content_type: String,
    text: Option<String>,
}

/// AI生成タスクのラッパー
#[derive(Debug, Deserialize)]
struct TasksWrapper {
    tasks: Vec<GeneratedTask>,
}

/// AIクライアントトレイト
#[async_trait]
pub trait AiClient: Send + Sync {
    /// 目標からタスクを生成する
    async fn generate_tasks_from_goal(
        &self,
        goal: &str,
        count: usize,
    ) -> anyhow::Result<Vec<GeneratedTask>>;

    /// プロバイダー名を取得
    #[allow(dead_code)]
    fn provider_name(&self) -> &str;
}

/// Anthropic APIクライアント
pub struct AnthropicClient {
    api_key: String,
    model: String,
    client: reqwest::Client,
}

impl AnthropicClient {
    /// 新しいAnthropicクライアントを作成
    pub fn new(api_key: String, model: Option<String>) -> Self {
        let model = model.unwrap_or_else(|| "claude-3-5-sonnet-20241022".to_string());
        Self {
            api_key,
            model,
            client: reqwest::Client::new(),
        }
    }

    /// APIキーを検証
    pub fn validate_api_key(&self) -> anyhow::Result<()> {
        if self.api_key.is_empty() {
            return Err(anyhow::anyhow!("Anthropic API key is not set"));
        }
        if !self.api_key.starts_with("sk-ant-") {
            return Err(anyhow::anyhow!(
                "Invalid Anthropic API key format (expected sk-ant-...)"
            ));
        }
        Ok(())
    }

    /// プロンプトを構築
    fn build_prompt(&self, goal: &str, count: usize) -> String {
        format!(
            "あなたはプロジェクト管理の専門家です。以下の目標を達成するための{}個のタスクを生成してください。

目標: {}

要件:
1. 各タスクは具体的で実行可能であること
2. タスク間に論理的な依存関係を考慮すること
3. 各タスクには優先度（1-5、5が最高）を推定すること

出力形式はJSONで、以下の構造に従ってください:
{{
  \"tasks\": [
    {{
      \"title\": \"タスク名\",
      \"description\": \"詳細な説明\",
      \"priority\": 1-5
    }}
  ]
}}

JSONのみを出力してください。他の説明は不要です。",
            count, goal
        )
    }

    /// Anthropic APIを呼び出し
    async fn call_api(&self, prompt: &str) -> anyhow::Result<String> {
        self.validate_api_key()?;

        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&serde_json::json!({
                "model": self.model,
                "max_tokens": 4096,
                "messages": [
                    {
                        "role": "user",
                        "content": prompt
                    }
                ]
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow::anyhow!(
                "Anthropic API error ({}): {}",
                status,
                error_text
            ));
        }

        let anthropic_response: AnthropicResponse = response.json().await?;
        let text = anthropic_response
            .content
            .first()
            .and_then(|b| b.text.as_ref())
            .ok_or_else(|| anyhow::anyhow!("Empty response from AI"))?;

        Ok(text.clone())
    }

    /// JSONレスポンスをパース
    fn parse_tasks(&self, text: &str) -> anyhow::Result<Vec<GeneratedTask>> {
        // JSONコードブロックを抽出
        let json_str = if let Some(start) = text.find("```json") {
            let start = start + 7;
            let end = text[start..].find("```").unwrap_or(text[start..].len());
            &text[start..start + end]
        } else if let Some(start) = text.find('{') {
            let end = text
                .rfind('}')
                .ok_or_else(|| anyhow::anyhow!("Invalid JSON"))?;
            &text[start..=end]
        } else {
            text
        };

        let wrapper: TasksWrapper = serde_json::from_str(json_str)?;
        Ok(wrapper.tasks)
    }
}

#[async_trait]
impl AiClient for AnthropicClient {
    async fn generate_tasks_from_goal(
        &self,
        goal: &str,
        count: usize,
    ) -> anyhow::Result<Vec<GeneratedTask>> {
        let prompt = self.build_prompt(goal, count);
        let response_text = self.call_api(&prompt).await?;
        self.parse_tasks(&response_text)
    }

    fn provider_name(&self) -> &str {
        "anthropic"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generated_task_serialization() {
        let task = GeneratedTask {
            title: "Test Task".to_string(),
            description: Some("Test Description".to_string()),
            priority: Some(3),
        };

        let json = serde_json::to_string(&task).unwrap();
        assert!(json.contains("Test Task"));
        assert!(json.contains("Test Description"));
        assert!(json.contains("3"));

        let deserialized: GeneratedTask = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.title, "Test Task");
        assert_eq!(
            deserialized.description,
            Some("Test Description".to_string())
        );
        assert_eq!(deserialized.priority, Some(3));
    }

    #[test]
    fn test_anthropic_client_validate_api_key_empty() {
        let client = AnthropicClient::new("".to_string(), None);
        assert!(client.validate_api_key().is_err());
    }

    #[test]
    fn test_anthropic_client_validate_api_key_invalid_format() {
        let client = AnthropicClient::new("invalid-key".to_string(), None);
        assert!(client.validate_api_key().is_err());
    }

    #[test]
    fn test_anthropic_client_validate_api_key_valid() {
        let client = AnthropicClient::new("sk-ant-api123-test".to_string(), None);
        assert!(client.validate_api_key().is_ok());
    }

    #[test]
    fn test_parse_tasks_with_json_block() {
        let client = AnthropicClient::new("sk-ant-test".to_string(), None);
        let json = r#"Here are the tasks:
```json
{
  "tasks": [
    {
      "title": "Task 1",
      "description": "Description 1",
      "priority": 5
    },
    {
      "title": "Task 2",
      "description": "Description 2",
      "priority": 3
    }
  ]
}
```"#;

        let tasks = client.parse_tasks(json).unwrap();
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].title, "Task 1");
        assert_eq!(tasks[1].title, "Task 2");
    }

    #[test]
    fn test_parse_tasks_plain_json() {
        let client = AnthropicClient::new("sk-ant-test".to_string(), None);
        let json = r#"{"tasks": [{"title": "Single Task", "description": "Desc", "priority": 1}]}"#;

        let tasks = client.parse_tasks(json).unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].title, "Single Task");
    }

    #[test]
    fn test_build_prompt() {
        let client = AnthropicClient::new("sk-ant-test".to_string(), None);
        let prompt = client.build_prompt("Build a house", 5);

        assert!(prompt.contains("Build a house"));
        assert!(prompt.contains("5個"));
        assert!(prompt.contains("priority"));
    }
}
