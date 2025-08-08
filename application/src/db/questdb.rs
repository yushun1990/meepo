use base64::Engine;
use questdb::ingress::{Protocol, Sender, SenderBuilder};
use reqwest::ClientBuilder;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct QdbConfig {
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub token: Option<String>,
}

pub struct QdbClient {
    pub writer: Sender,
    pub reader: reqwest::Client,
}

pub trait QuestDB {
    fn config(&self) -> QdbConfig;
    fn setup(&self) -> anyhow::Result<QdbClient>;
}

impl QuestDB for QdbConfig {
    fn config(&self) -> QdbConfig {
        self.clone()
    }

    fn setup(&self) -> anyhow::Result<QdbClient> {
        let mut writer_builder =
            SenderBuilder::new(Protocol::Http, self.host.clone(), self.port.clone());

        if let Some(username) = self.username.clone() {
            writer_builder = writer_builder.username(username.as_str())?;
        }

        if let Some(password) = self.password.clone() {
            writer_builder = writer_builder.password(password.as_str())?;
        }

        if let Some(token) = self.token.clone() {
            writer_builder = writer_builder.token(token.as_str())?;
        }

        Ok(QdbClient {
            writer: writer_builder.build()?,
            reader: ClientBuilder::new()
                .http2_adaptive_window(true)
                .http2_prior_knowledge()
                .gzip(true)
                .default_headers({
                    let mut headers = reqwest::header::HeaderMap::new();
                    if let Some(auth) = self.get_auth() {
                        headers.insert(reqwest::header::AUTHORIZATION, auth.parse()?);
                    }

                    headers
                })
                .build()?,
        })
    }
}

impl QdbConfig {
    fn get_auth(&self) -> Option<String> {
        if let Some(username) = self.username.clone()
            && let Some(password) = self.password.clone()
        {
            Some(format!(
                "Basic {}",
                base64::engine::general_purpose::STANDARD.encode(format!("{username}:{password}"))
            ))
        } else if let Some(token) = self.token.clone() {
            Some(format!("Bearer {token}"))
        } else {
            None
        }
    }
}
