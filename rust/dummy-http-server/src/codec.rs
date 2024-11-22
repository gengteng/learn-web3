use bytes::{Bytes, BytesMut};
use http::{Request, Response};
use httparse::Status;
use tokio_util::codec::{Decoder, Encoder};

pub struct HttpRequestCodec;

impl Decoder for HttpRequestCodec {
    type Item = Request<Bytes>;
    type Error = anyhow::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut req = httparse::Request::new(&mut headers);
        let status = req
            .parse(src)
            .map_err(|e| anyhow::anyhow!("failed to parse request: {}", e))?;

        let Status::Complete(end) = status else {
            return Ok(None);
        };

        let method = req
            .method
            .ok_or_else(|| anyhow::anyhow!("missing method"))?;
        let uri = req.path.ok_or_else(|| anyhow::anyhow!("missing uri"))?;

        let mut builder = Request::builder().method(method).uri(uri);

        for header in req.headers.iter() {
            builder = builder.header(header.name, header.value);
        }

        // determine if the body is present
        // check Content-Length header
        if let Some(len) = req.headers.iter().find(|h| h.name == "Content-Length") {
            let len = std::str::from_utf8(len.value)
                .map_err(|e| anyhow::anyhow!("failed to parse Content-Length: {}", e))?;
            let len = len
                .parse::<usize>()
                .map_err(|e| anyhow::anyhow!("failed to parse Content-Length: {}", e))?;
            if src.len() < end + len {
                return Ok(None);
            }
            let request_src = src.split_to(end + len).freeze();
            let body = request_src.slice(end..end + len);
            let req = builder.body(body)?;
            return Ok(Some(req));
        }

        // remove the parsed part from src and get the body
        let _ = src.split_to(end);
        let req = builder.body(Bytes::new())?;

        Ok(Some(req))
    }
}

pub struct HttpResponseCodec;

impl Encoder<Response<Bytes>> for HttpResponseCodec {
    type Error = anyhow::Error;

    fn encode(&mut self, item: Response<Bytes>, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let mut buf = format!("HTTP/1.1 {}\r\n", item.status().as_str()).into_bytes();
        for (name, value) in item.headers() {
            buf.extend_from_slice(name.as_str().as_bytes());
            buf.extend_from_slice(b": ");
            buf.extend_from_slice(value.as_bytes());
            buf.extend_from_slice(b"\r\n");
        }

        // if there is a body and not content-length header, add it
        if !item.body().is_empty() {
            if !item.headers().contains_key("Content-Length") {
                buf.extend_from_slice(b"Content-Length: ");
                buf.extend_from_slice(item.body().len().to_string().as_bytes());
                buf.extend_from_slice(b"\r\n");
            }
        }

        buf.extend_from_slice(b"\r\n");
        buf.extend_from_slice(item.body());
        dst.extend_from_slice(&buf);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::BytesMut;
    use http::StatusCode;

    #[test]
    fn test_http_request() {
        let mut codec = HttpRequestCodec;
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"GET / HTTP/1.1\r\nHost: example.com\r\n\r\n");
        let req = codec.decode(&mut buf).unwrap().unwrap();
        assert_eq!(req.method(), "GET");
        assert_eq!(req.uri(), "/");
        assert_eq!(req.headers().get("Host").unwrap(), "example.com");
        assert!(req.body().is_empty());
    }

    #[test]
    fn test_http_request_with_body() {
        let mut codec = HttpRequestCodec;
        let mut buf = BytesMut::new();
        buf.extend_from_slice(
            b"POST / HTTP/1.1\r\nHost: example.com\r\nContent-Length: 5\r\n\r\nhello",
        );
        let req = codec.decode(&mut buf).unwrap().unwrap();
        assert_eq!(req.method(), "POST");
        assert_eq!(req.uri(), "/");
        assert_eq!(req.headers().get("Host").unwrap(), "example.com");
        assert_eq!(req.body().as_ref(), b"hello");
    }

    #[test]
    fn test_http_response() {
        let mut codec = HttpResponseCodec;
        let mut buf = BytesMut::new();
        let res = Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "text/plain")
            .body(Bytes::from_static(b"hello"))
            .unwrap();
        codec.encode(res, &mut buf).unwrap();
        let buf = buf.freeze();
        let buf = buf.as_ref();
        let encoded = std::str::from_utf8(buf).unwrap();
        assert!(encoded.eq_ignore_ascii_case(
            "HTTP/1.1 200\r\nContent-Type: text/plain\r\nContent-Length: 5\r\n\r\nhello"
        ));
    }
}
