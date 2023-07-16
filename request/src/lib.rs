use reqwest::Error as ReqwestError;

#[derive(Debug)]
pub enum HttpClientError {
    RequestFailed(ReqwestError),
    InvalidStatusCode(u16),
    JsonParsingFailed(serde_json::Error),
}

impl From<ReqwestError> for HttpClientError {
    fn from(error: ReqwestError) -> Self {
        HttpClientError::RequestFailed(error)
    }
}

impl From<serde_json::Error> for HttpClientError {
    fn from(error: serde_json::Error) -> Self {
        HttpClientError::JsonParsingFailed(error)
    }
}

pub struct HttpClient {
    client: reqwest::Client,
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpClient {
    pub fn new() -> Self {
        let client = reqwest::Client::new();
        HttpClient { client }
    }

    pub async fn get<T>(&self, url: &str) -> Result<T, HttpClientError>
    where
        T: serde::de::DeserializeOwned,
    {
        let response = self.client.get(url).send().await?;
        let status = response.status();

        if !status.is_success() {
            return Err(HttpClientError::InvalidStatusCode(status.as_u16()));
        }

        let json = response.json::<T>().await?;

        Ok(json)
    }

    pub async fn post<T>(&self, url: &str, body: &T) -> Result<(), HttpClientError>
    where
        T: serde::Serialize,
    {
        let response = self.client.post(url).json(body).send().await?;
        let status = response.status();

        if !status.is_success() {
            return Err(HttpClientError::InvalidStatusCode(status.as_u16()));
        }

        Ok(())
    }

    pub async fn put<T>(&self, url: &str, body: &T) -> Result<(), HttpClientError>
    where
        T: serde::Serialize,
    {
        let response = self.client.put(url).json(body).send().await?;
        let status = response.status();

        if !status.is_success() {
            return Err(HttpClientError::InvalidStatusCode(status.as_u16()));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get() {
        let url = "https://jsonplaceholder.typicode.com/posts/1";
        let client = HttpClient::default();
        let result: Result<Post, HttpClientError> = client.get(url).await;
        assert!(result.is_ok());
        let post = result.unwrap();
        assert_eq!(post.id, 1);
    }

    #[tokio::test]
    async fn test_post() {
        let url = "https://jsonplaceholder.typicode.com/posts";
        let client = HttpClient::default();
        let post = Post {
            id: 1,
            title: "Test Post".to_string(),
            body: "This is a test post.".to_string(),
        };
        let result: Result<(), HttpClientError> = client.post(url, &post).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_put() {
        let url = "https://jsonplaceholder.typicode.com/posts/1";
        let client = HttpClient::default();
        let post = Post {
            id: 1,
            title: "Updated Post".to_string(),
            body: "This post has been updated.".to_string(),
        };
        let result: Result<(), HttpClientError> = client.put(url, &post).await;
        assert!(result.is_ok());
    }

    #[derive(Debug, serde::Deserialize, serde::Serialize)]
    struct Post {
        id: u32,
        title: String,
        body: String,
    }
}
