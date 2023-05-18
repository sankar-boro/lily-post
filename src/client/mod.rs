#![allow(dead_code, unused_variables)]

use crate::AppError;
use log::{error, trace, warn};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str, to_string};

#[derive(Debug)]
pub enum Method<Q, B> {
    Get { query: Q },
    Post { query: Q, body: B },
    Patch { query: Q, body: B },
    Put { query: Q, body: B },
    Delete { query: Q },
}

pub fn add_query_parameters<Query: Serialize>(url: &str, query: &Query) -> Result<String, AppError> {
    // let query = yaup::to_string(query)?;

    // if query.is_empty() {
    //     Ok(url.to_string())
    // } else {
    //     Ok(format!("{url}?{query}"))
    // }
    Ok(url.to_string())

}

pub async fn request<
    Query: Serialize,
    Body: Serialize,
    Output: DeserializeOwned + 'static,
>(
    url: &str,
    apikey: &str,
    method: Method<Query, Body>,
    expected_status_code: u16,
) -> Result<Output, AppError> {
    use isahc::http::header;
    use isahc::http::method::Method as HttpMethod;
    use isahc::*;

    let builder = Request::builder().header(header::USER_AGENT, qualified_version());
    let builder = builder.header(header::AUTHORIZATION, format!("Bearer {apikey}"));

    let mut response = match &method {
        Method::Get { query } => {
            let url = add_query_parameters(url, query)?;

            builder
                .method(HttpMethod::GET)
                .uri(url)
                .body(())
                .map_err(|e| e.to_string())?
                .send_async()
                .await.unwrap()
        }
        Method::Delete { query } => {
            let url = add_query_parameters(url, query)?;

            builder
                .method(HttpMethod::DELETE)
                .uri(url)
                .body(())
                .map_err(|e| e.to_string())?
                .send_async()
                .await.unwrap()
        }
        Method::Post { query, body } => {
            let url = add_query_parameters(url, query)?;

            builder
                .method(HttpMethod::POST)
                .uri(url)
                .header(header::CONTENT_TYPE, "application/json")
                .body(to_string(&body).unwrap())
                .map_err(|e| e.to_string())?
                .send_async()
                .await.unwrap()
        }
        Method::Patch { query, body } => {
            let url = add_query_parameters(url, query)?;

            builder
                .method(HttpMethod::PATCH)
                .uri(url)
                .header(header::CONTENT_TYPE, "application/json")
                .body(to_string(&body).unwrap())
                .map_err(|e| e.to_string())?
                .send_async()
                .await.unwrap()
        }
        Method::Put { query, body } => {
            let url = add_query_parameters(url, query)?;

            builder
                .method(HttpMethod::PUT)
                .uri(url)
                .header(header::CONTENT_TYPE, "application/json")
                .body(to_string(&body).unwrap())
                .map_err(|e| e.to_string())?
                .send_async()
                .await.unwrap()
        }
    };

    let status = response.status().as_u16();

    let mut body = response
        .text()
        .await
        .map_err(|e| e.to_string()).unwrap();

    if body.is_empty() {
        body = "null".to_string();
    }

    parse_response(status, expected_status_code, body)
}

pub async fn stream_request<
    'a,
    Query: Serialize,
    Body: futures_io::AsyncRead + Send + Sync + 'static,
    Output: DeserializeOwned + 'static,
>(
    url: &str,
    apikey: Option<&str>,
    method: Method<Query, Body>,
    content_type: &str,
    expected_status_code: u16,
) -> Result<Output, AppError> {
    use isahc::http::header;
    use isahc::http::method::Method as HttpMethod;
    use isahc::*;

    let builder = Request::builder().header(header::USER_AGENT, qualified_version());
    let builder = match apikey {
        Some(apikey) => builder.header(header::AUTHORIZATION, format!("Bearer {apikey}")),
        None => builder,
    };

    let mut response = match method {
        Method::Get { query } => {
            let url = add_query_parameters(url, &query)?;

            builder
                .method(HttpMethod::GET)
                .uri(url)
                .body(())
                .map_err(|e| e.to_string())?
                .send_async()
                .await.unwrap()
        }
        Method::Delete { query } => {
            let url = add_query_parameters(url, &query)?;

            builder
                .method(HttpMethod::DELETE)
                .uri(url)
                .body(())
                .map_err(|e| e.to_string())?
                .send_async()
                .await.unwrap()
        }
        Method::Post { query, body } => {
            let url = add_query_parameters(url, &query)?;

            builder
                .method(HttpMethod::POST)
                .uri(url)
                .header(header::CONTENT_TYPE, content_type)
                .body(AsyncBody::from_reader(body))
                .map_err(|e| e.to_string())?
                .send_async()
                .await.unwrap()
        }
        Method::Patch { query, body } => {
            let url = add_query_parameters(url, &query)?;

            builder
                .method(HttpMethod::PATCH)
                .uri(url)
                .header(header::CONTENT_TYPE, content_type)
                .body(AsyncBody::from_reader(body))
                .map_err(|e| e.to_string())?
                .send_async()
                .await.unwrap()
        }
        Method::Put { query, body } => {
            let url = add_query_parameters(url, &query)?;

            builder
                .method(HttpMethod::PUT)
                .uri(url)
                .header(header::CONTENT_TYPE, content_type)
                .body(AsyncBody::from_reader(body))
                .map_err(|e| e.to_string())?
                .send_async()
                .await.unwrap()
        }
    };

    let status = response.status().as_u16();

    let mut body = response
        .text()
        .await
        .map_err(|e| e.to_string())?;

    if body.is_empty() {
        body = "".to_string();
    }

    parse_response(status, expected_status_code, body)
}

fn parse_response<Output: DeserializeOwned>(
    status_code: u16,
    expected_status_code: u16,
    body: String,
) -> Result<Output, AppError> {
    if status_code == expected_status_code {
        match from_str::<Output>(&body) {
            Ok(output) => {
                trace!("Request succeed");
                return Ok(output);
            }
            Err(e) => {
                error!("Request succeeded but failed to parse response");
                return Err(AppError::from("").into());
            }
        };
    }
    // TODO: create issue where it is clear what the HTTP error is
    // ParseError(Error("invalid type: null, expected struct MeilisearchError", line: 1, column: 4))

    warn!(
        "Expected response code {}, got {}",
        expected_status_code, status_code
    );
    // match from_str::<MeilisearchError>(&body) {
    //     Ok(e) => Err(Error::from(e)),
    //     Err(e) => Err(Error::ParseError(e)),
    // }
    Err(AppError::from("").into())
}

pub fn qualified_version() -> String {
    const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

    format!("Lily Search (v{})", VERSION.unwrap_or("unknown"))
}

#[cfg(test)]
mod test { 
    use super::*;
    use serde_json::{json, Value};
    use futures::executor::block_on;
    
    #[test]
    fn test_get_request() {
        block_on(async move {
            request::<(), (), Value>(
                &format!("{}/indexes/{}", "http", "uuid"),
                "apiKey",
                Method::Get { query: () },
                200,
            )
            .await.unwrap();
        });
    }

    #[test]
    fn test_post_request() {
        block_on(async move {
            request::<(), Value, ()>(
                &format!("{}/indexes", "http://localhost:7700"),
                "apiKey",
                Method::Post {
                    query: (),
                    body: json!({
                        "uid": "uuid",
                        "primaryKey": "primaryKey",
                    }),
                },
                202,
            ).await.unwrap();
        });
    }
}