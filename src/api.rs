//! OpenAI API client.
//!
//! Communicates with the OpenAI API to send chat requests and receive chat completions.

use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client,
};

use crate::{
    error::{Error, OpenAIErrorResponse, Result},
    request::{Request, Response},
    token::Token,
};

/// OpenAI API client.
pub struct Api {
    organization_id: Option<String>,
    client: Client,
}

impl Api {
    /// Base URL for the OpenAI API.
    const BASE_URL: &'static str = "https://api.openai.com/v1";

    /// URL for the chat endpoint.
    const CHAT_URL: &'static str = "/chat/completions";

    /// Create a new API client.
    pub fn new(key: Token) -> Api {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, key.into());
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        Api {
            organization_id: None,
            client: Client::builder().default_headers(headers).build().unwrap(),
        }
    }

    /// Set the organization ID for the API client.
    pub fn with_organization_id(mut self, organization_id: String) -> Self {
        self.organization_id = Some(organization_id);
        self
    }

    /// Send a chat request to the OpenAI API.
    pub async fn chat(&self, r: Request) -> Result<Response> {
        let url = format!("{}{}", Self::BASE_URL, Self::CHAT_URL);
        let mut req = self.client.post(&url);
        if let Some(organization_id) = &self.organization_id {
            req = req.header("OpenAI-Organization", organization_id);
        }
        let res = req.json(&r).send().await?;

        //if status is ok, return as Response json, otherwise return as ApiError
        match res.status().is_success() {
            true => Ok(res.json::<Response>().await?),
            false => Err(Error::ApiError(res.json::<OpenAIErrorResponse>().await?)),
        }
    }
}
