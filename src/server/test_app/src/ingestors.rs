use hyper::StatusCode;
use reqwest::Response;
use serde::Serialize;

use crate::TestApp;

pub struct Scope<'a> {
    pub(crate) parent:     &'a TestApp,
    pub(crate) url_prefix: String,
}

impl Scope<'_> {
    pub async fn create(&self, body: impl Serialize) -> Response {
        self.parent
            .client
            .post(&self.url_prefix)
            .json(&body)
            .send()
            .await
            .expect("failed to execute request")
    }

    pub async fn create_ok(&self, body: impl Serialize) -> view::Ingestor {
        let response = self.create(body).await;
        let status = response.status();
        let text = response
            .text()
            .await
            .expect("failed to read response body as text");

        assert_eq!(status, StatusCode::CREATED, "Body text: {}", &text);

        match serde_json::from_str(&text) {
            | Ok(x) => x,
            | Err(err) => {
                panic!(
                    "failed to deserialize response body text: {}\n{}",
                    err, &text
                )
            },
        }
    }
}
