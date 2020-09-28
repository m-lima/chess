use gotham::hyper;

const WEBSOCKET: &str = "websocker";

pub fn requested(headers: &hyper::HeaderMap) -> bool {
    headers.get(hyper::header::UPGRADE) == Some(&hyper::header::HeaderValue::from_static(WEBSOCKET))
}

pub fn accept(
    headers: &hyper::HeaderMap,
    body: hyper::Body,
) -> Result<
    (
        hyper::Response<hyper::Body>,
        impl Future<Output = Result<WebSocketStream<Upgraded>, hyper::Error>>,
    ),
    (),
> {
    let res = response(headers)?;
    let ws = body.on_upgrade().and_then(|upgraded| {
        WebSocketStream::from_raw_socket(upgraded, Role::Server, None).map(Ok)
    });

    Ok((res, ws))
}
