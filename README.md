# ChatGPT Rust

This Rust crate provides an API client for communicating with OpenAI's GPT-3.5 and GPT-4 (ChatGPT) API. The Api struct provides methods for sending requests and receiving responses from the API.

## Installation
Add this to your Cargo.toml:
```toml
[dependencies]
chat_gpt_rs = "1.4.0"

```

## Usage
    
```rs
use chat_gpt_rs::prelude::*;

#[tokio::main]
async fn main() {
    let token = Token::new("YOUR_API_KEY");
    let api = Api::new(token);
    let request = Request {
        model: Model::Gpt35Turbo,
        messages: vec![Message {
            role: "user".to_string(),
            content: "Hello, how's it going?".to_string(),
        }],
        ..Default::default()
    };
    let response = api.chat(request).await;
    if let Ok(response) = response {
        println!("{:?}", response.choices[0].message.content);
    } else {
        println!("Error: {:?}", response.err());
    }
} 
```

## Additional Configuration

Following configuration options are available in the `Request` struct:
- `Model`: ID of the model to use. All GPT3 and GPT4 models are supported.
- `Messages`: messages to generate chat completions for, in the chat format.
- `Temperature`: what sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.
- `TopP`: an alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass.
- `N`: how many chat completion choices to generate for each input message.
- `Stop`: up to 4 sequences where the API will stop generating further tokens.
- `MaxTokens`: maximum number of tokens to generate for each chat completion choice.
- `PresencePenalty`: a number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics.
- `FrequencyPenalty`: a number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim.
- `User`: a unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.



## Providing Organization ID

You may provide your OpenAI organization ID when creating the API instance.
```rs
    let api = Api::new(token).with_organization_id("YOUR_ORGANIZATION_ID")
```

