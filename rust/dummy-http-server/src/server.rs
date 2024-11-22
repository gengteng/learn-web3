use crate::codec;
use crate::handler::{Handler, Router};
use futures::{SinkExt, StreamExt};
use std::future::Future;
use std::net::SocketAddr;
use std::pin::pin;
use std::time::Duration;
use tokio::time::timeout;

#[derive(Default)]
pub struct HttpServer {
    closers: Vec<tokio::sync::oneshot::Sender<()>>,
    handles: Vec<tokio::task::JoinHandle<()>>,
}

impl HttpServer {
    pub async fn serve<F>(
        mut self,
        addr: SocketAddr,
        router: impl Into<Router>,
        shutdown_signal: F,
    ) -> anyhow::Result<()>
    where
        F: Future<Output = ()>,
    {
        let router = router.into();
        let tcp_listener = tokio::net::TcpListener::bind(&addr).await?;

        let mut shutdown_signal = pin!(shutdown_signal);

        loop {
            tokio::select! {
                _ = &mut shutdown_signal => {
                    break;
                }
                result = tcp_listener.accept() => {
                    match result {
                        Ok((stream, remote_addr)) => {
                            tracing::info!("accepted connection from: {}", remote_addr);
                            let (closer, close_signal) = tokio::sync::oneshot::channel::<()>();
                            let handle = tokio::spawn(handle_stream(stream, router.clone(), close_signal));
                            self.closers.push(closer);
                            self.handles.push(handle);
                        }
                        Err(e) => {
                            tracing::error!("failed to accept connection: {}", e);
                            break;
                        }
                    }
                }
            }
        }

        self.closers.clear();
        tracing::info!("Shutting down server...");
        let _ = timeout(
            Duration::from_secs(10),
            futures::future::join_all(self.handles),
        )
        .await?;
        tracing::info!("All connections closed, server shut down, bye!");
        Ok(())
    }
}

#[tracing::instrument(skip(stream, router, close_signal))]
async fn handle_stream(
    stream: tokio::net::TcpStream,
    router: Router,
    close_signal: tokio::sync::oneshot::Receiver<()>,
) {
    let (read_half, write_half) = tokio::io::split(stream);

    let mut framed_read = tokio_util::codec::FramedRead::new(read_half, codec::HttpRequestCodec);
    let mut framed_write =
        tokio_util::codec::FramedWrite::new(write_half, codec::HttpResponseCodec);

    let mut close_signal = pin!(close_signal);
    loop {
        tokio::select! {
            _ = &mut close_signal => {
                break;
            }
            req = framed_read.next() => {
                tracing::debug!("received request: {:?}", req);
                let req = match req {
                    Some(Ok(req)) => req,
                    Some(Err(e)) => {
                        tracing::error!("failed to read request: {}", e);
                        break;
                    }
                    None => {
                        break;
                    }
                };

                let res = router.handle(req).await;
                match framed_write.send(res.clone()).await {
                    Ok(_) => {
                        tracing::debug!("sent response: {:?}", res);
                    }
                    Err(e) => {
                        tracing::error!("failed to write response [{res:?}]: {}", e);
                        break;
                    }
                }
            }
        }
    }
}
