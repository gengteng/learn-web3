use crate::handler::{Handler, RouterBuilder};
use bytes::Bytes;
use http::{Method, Request, Response};
use std::collections::HashMap;
use std::path::PathBuf;

pub struct Directory {
    handlers: HashMap<(Method, String), Box<dyn Handler>>,
}

impl Directory {
    pub async fn new(base_route: &str, dir: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let mut handlers: HashMap<_, Box<dyn Handler>> = HashMap::new();

        let dir = dir.into();

        let mut read_dir = tokio::fs::read_dir(&dir).await?;
        while let Some(entry) = read_dir.next_entry().await? {
            let file_path = entry.path();
            let path = file_path.strip_prefix(&dir)?;
            let method = Method::GET;
            let route = format!(
                "{}/{}",
                if base_route == "/" { "" } else { base_route },
                path.to_string_lossy().to_string()
            );
            handlers.insert((method, route), Box::new(File(file_path.to_path_buf())));

            if path == PathBuf::from("index.html") {
                handlers.insert(
                    (Method::GET, base_route.to_string()),
                    Box::new(File(file_path.to_path_buf())),
                );
            }
        }

        Ok(Self { handlers })
    }
}

impl From<Directory> for RouterBuilder {
    fn from(value: Directory) -> Self {
        RouterBuilder {
            handlers: value.handlers,
        }
    }
}

pub struct File(PathBuf);

#[async_trait::async_trait]
impl Handler for File {
    async fn handle(&self, req: Request<Bytes>) -> Response<Bytes> {
        tracing::info!(?req, "Handling file: {:?}", self.0);
        let body = tokio::fs::read(&self.0).await.unwrap_or_default();
        // check the file extension and set the content type
        let content_type = mime_guess::from_path(&self.0)
            .first_or_octet_stream()
            .to_string();
        Response::builder()
            .header("Content-Type", content_type)
            .header("Content-Length", body.len())
            .body(body.into())
            .expect("failed to build response")
    }
}
