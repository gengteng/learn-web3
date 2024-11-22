use bytes::Bytes;
use http::{Method, Request, Response};
use std::collections::HashMap;
use std::fmt::Debug;
use std::future::Future;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait Handler: Send + Sync {
    async fn handle(&self, req: Request<Bytes>) -> Response<Bytes>;
}

// implement Handler for async fn that takes Request<Bytes> and returns Response<Bytes>

#[async_trait::async_trait]
impl<F, Fut, R> Handler for F
where
    F: Send + Sync + Fn(Request<Bytes>) -> Fut,
    Fut: Future<Output = R> + Send + Sync + 'static,
    R: IntoResponse + Send + Sync + 'static,
{
    async fn handle(&self, req: Request<Bytes>) -> Response<Bytes> {
        self(req).await.into_response()
    }
}

#[async_trait::async_trait]
impl Handler for Router {
    async fn handle(&self, req: Request<Bytes>) -> Response<Bytes> {
        let handler = self
            .handlers
            .get(&(req.method().clone(), req.uri().path().to_string()));

        match handler {
            Some(handler) => handler.handle(req).await,
            None => Response::builder()
                .status(404)
                .body(Bytes::from("Not Found"))
                .expect("failed to build response"),
        }
    }
}

#[derive(Clone)]
pub struct Router {
    pub(crate) handlers: Arc<HashMap<(Method, String), Box<dyn Handler>>>,
}

impl Router {
    pub fn builder() -> RouterBuilder {
        RouterBuilder::default()
    }
}

#[derive(Default)]
pub struct RouterBuilder {
    pub(crate) handlers: HashMap<(Method, String), Box<dyn Handler>>,
}

impl Debug for RouterBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, _) in self.handlers.iter() {
            writeln!(f, "{:?}", key)?;
        }
        Ok(())
    }
}

impl RouterBuilder {
    pub fn add<H>(mut self, method: Method, path: &str, handler: H) -> Self
    where
        H: Handler + 'static,
    {
        self.handlers
            .insert((method, path.to_string()), Box::new(handler));
        self
    }

    pub fn get(self, path: &str, handler: impl Handler + 'static) -> Self {
        self.add(Method::GET, path, handler)
    }

    pub fn post(self, path: &str, handler: impl Handler + 'static) -> Self {
        self.add(Method::POST, path, handler)
    }

    pub fn merge(mut self, other: impl Into<RouterBuilder>) -> Self {
        self.handlers.extend(other.into().handlers);
        self
    }

    pub fn build(self) -> Router {
        Router {
            handlers: Arc::new(self.handlers),
        }
    }
}

impl From<RouterBuilder> for Router {
    fn from(builder: RouterBuilder) -> Self {
        builder.build()
    }
}

pub trait IntoResponse {
    fn into_response(self) -> Response<Bytes>;
}

impl IntoResponse for &'static str {
    fn into_response(self) -> Response<Bytes> {
        Response::builder()
            .status(200)
            .body(Bytes::from(self))
            .expect("failed to build response")
    }
}

impl IntoResponse for String {
    fn into_response(self) -> Response<Bytes> {
        Response::builder()
            .status(200)
            .body(Bytes::from(self))
            .expect("failed to build response")
    }
}

impl IntoResponse for Response<Bytes> {
    fn into_response(self) -> Response<Bytes> {
        self
    }
}
