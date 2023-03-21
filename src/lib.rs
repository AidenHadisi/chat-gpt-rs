//! An API Client to communicate with the OpenAI's GPT-3.5 and GPT-4 aka ChatGPT API.
//!
//! # Example
//! ```rs
//! use chat_gpt_rs::prelude::*;
//! 
//! #[tokio::main]
//! async fn main() {
//!     let token = Token::new("YOUR_API_KEY");
//!     let api = Api::new(token);
//!     let request = Request {
//!         model: Model::Gpt35Turbo,
//!         messages: vec![Message {
//!             role: "user".to_string(),
//!             content: "Hello, how's it going?".to_string(),
//!         }],
//!         ..Default::default()
//!     };
//!     let response = api.chat(request).await;
//!     if let Ok(response) = response {
//!         println!("{:?}", response.choices[0].message.content);
//!     } else {
//!         println!("Error: {:?}", response.err());
//!     }
//! }
//! ```
//! 
//! ## Additional Configuration
//! Additional configuration can be added to the [request::Request] struct.
//! For more information, see the [OpenAI API documentation](https://platform.openai.com/docs/api-reference/completions).

pub mod api;
pub mod error;
pub mod request;
pub mod token;

pub mod prelude {
    pub use crate::api::Api;
    pub use crate::error::{Error, Result};
    pub use crate::request::{Message, Model, Request, Response};
    pub use crate::token::Token;
}
