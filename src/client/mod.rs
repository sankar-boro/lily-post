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
>(
    url: &str,
    apikey: &str,
    method: Method<Query, Body>,
    expected_status_code: u16,
    expected_message: &str
) -> Result<(), AppError> {
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

    let status_code = response.status().as_u16();

    let body = response
    .text()
    .await
    .map_err(|e| e.to_string())?;

    if status_code == expected_status_code {
        if body == expected_message {
            return Ok(());
        }
    }

    Err(AppError::from("").into())

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

    let body = response
        .text()
        .await
        .map_err(|e| e.to_string())?;

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
                println!("error: {}", e);
                return Err(AppError::from(e).into());
            }
        };
    }
    warn!(
        "Expected response code {}, got {}",
        expected_status_code, status_code
    );
    Err(AppError::from("").into())
}

pub fn qualified_version() -> String {
    const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

    format!("Lily Search (v{})", VERSION.unwrap_or("unknown"))
}