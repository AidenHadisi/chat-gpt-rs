//! Data structures for the request to the OpenAI API amd response from the OpenAI API.

#[derive(serde::Serialize, Default)]
/// gpt-3.5 and gpt-4 models that are supported by the OpenAI API.
pub enum Model {
    /// gpt-3.5-turbo model.
    #[default]
    #[serde(rename = "gpt-3.5-turbo")]
    Gpt35Turbo,

    /// gpt-3.5-turbo-0301 model.
    #[serde(rename = "gpt-3.5-turbo-0301")]
    Gpt35Turbo0301,

    #[serde(rename = "gpt-4")]
    Gpt4,

    #[serde(rename = "gpt-4-0314")]
    Gpt4_0314,

    #[serde(rename = "gpt-4-32k")]
    Gpt4_32k,

    #[serde(rename = "gpt-4-32k-0314")]
    Gpt4_32k0314,

}

#[derive(serde::Deserialize)]
/// A chat completion choice.
pub struct Choice {
    /// index of the message in the request.
    pub index: i32,

    /// text of the chat completion.
    pub message: Message,

    /// finish reason for the chat completion.
    pub finish_reason: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
/// A message in the chat format.
pub struct Message {
    /// role for the message, either "user" or "assistant".
    pub role: String,

    /// message content.
    pub content: String,
}

#[derive(serde::Deserialize)]
/// usage information for the OpenAI API.
pub struct Usage {
    /// how many tokens were used for the prompt.
    pub prompt_tokens: i32,

    /// how many tokens were used for the chat completion.
    pub completion_tokens: i32,

    /// how many tokens were used for the entire request.
    pub total_tokens: i32,
}

#[derive(serde::Serialize, Default)]
/// request to the OpenAI API.
pub struct Request {
    /// ID of the model to use. Currently, only `gpt-3.5-turbo` and `gpt-3.5-turbo-0301` are supported.
    pub model: Model,

    /// Messages to generate chat completions for, in the chat format.
    pub messages: Vec<Message>,

    /// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,

    /// Alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,

    /// How many chat completion choices to generate for each input message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,

    /// Up to 4 sequences where the API will stop generating further tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,

    /// Maximum number of tokens to generate for each chat completion choice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,

    /// A number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,

    /// A number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,

    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(serde::Deserialize)]
/// response from the OpenAI API.
pub struct Response {
    /// ID of the request.
    pub id: String,
    pub object: String,

    /// when the request was created.
    pub created: i64,

    /// list of chat completion choices.
    pub choices: Vec<Choice>,

    /// usage information for the request.
    pub usage: Usage,
}
