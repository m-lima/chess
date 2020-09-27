use gotham::hyper;

#[derive(Clone, gotham_derive::NewMiddleware)]
pub struct Cors(hyper::header::HeaderValue);

impl Cors {
    pub fn new() -> Self {
        Self(hyper::header::HeaderValue::from_static(
            "http://localhost:3030/",
        ))
    }
}

impl gotham::middleware::Middleware for Cors {
    fn call<C>(
        self,
        state: gotham::state::State,
        chain: C,
    ) -> std::pin::Pin<Box<gotham::handler::HandlerFuture>>
    where
        C: FnOnce(gotham::state::State) -> std::pin::Pin<Box<gotham::handler::HandlerFuture>>
            + Send
            + 'static,
    {
        Box::pin(async {
            // Allowed because this is third-party code being flagged
            #[allow(clippy::used_underscore_binding)]
            chain(state).await.map(|(state, mut response)| {
                let header = response.headers_mut();
                header.insert(hyper::header::ACCESS_CONTROL_ALLOW_ORIGIN, self.0);
                (state, response)
            })
        })
    }
}

#[derive(Clone, gotham_derive::NewMiddleware)]
pub struct Log;

impl gotham::middleware::Middleware for Log {
    fn call<C>(
        self,
        state: gotham::state::State,
        chain: C,
    ) -> std::pin::Pin<Box<gotham::handler::HandlerFuture>>
    where
        C: FnOnce(gotham::state::State) -> std::pin::Pin<Box<gotham::handler::HandlerFuture>>
            + Send
            + 'static,
    {
        Box::pin(async {
            // Allowed because this is third-party code being flagged
            #[allow(clippy::used_underscore_binding)]
            chain(state).await.map(|(state, response)| {
                {
                    use gotham::state::FromState;

                    let ip = hyper::HeaderMap::borrow_from(&state)
                        .get(hyper::header::HeaderName::from_static("x-forwarded-for"))
                        .and_then(|fwd| fwd.to_str().ok())
                        .map_or_else(
                            || {
                                gotham::state::client_addr(&state).map_or_else(
                                    || String::from("??"),
                                    |addr| addr.ip().to_string(),
                                )
                            },
                            |fwd| format!("{} [p]", fwd),
                        );

                    // Request info
                    let path = hyper::Uri::borrow_from(&state);
                    let method = hyper::Method::borrow_from(&state);
                    let length = hyper::HeaderMap::borrow_from(&state)
                        .get(hyper::header::CONTENT_LENGTH)
                        .and_then(|len| len.to_str().ok())
                        .unwrap_or("");

                    // Response info
                    let status = response.status().as_u16();

                    // Log out
                    log::info!("{} {} - {} {} {}", status, ip, method, path, length);
                }

                (state, response)
            })
        })
    }
}

#[derive(Clone, gotham_derive::StateData, gotham_derive::NewMiddleware)]
pub struct Game {
    games: std::sync::Arc<std::sync::Mutex<std::collections::HashMap<String, String>>>,
}

impl Game {
    pub fn new() -> Self {
        let mut map = std::collections::HashMap::new();
        map.insert(String::from("123"), String::from("a game is running"));
        Self {
            games: std::sync::Arc::new(std::sync::Mutex::new(map)),
        }
    }

    pub fn get(&self, id: &str) -> Option<String> {
        let games = self.games.lock().unwrap();
        games.get(id).map(std::clone::Clone::clone)
    }
}

impl gotham::middleware::Middleware for Game {
    fn call<Chain>(
        self,
        mut state: gotham::state::State,
        chain: Chain,
    ) -> std::pin::Pin<Box<gotham::handler::HandlerFuture>>
    where
        Chain: FnOnce(gotham::state::State) -> std::pin::Pin<Box<gotham::handler::HandlerFuture>>,
    {
        state.put(self);
        chain(state)
    }
}
