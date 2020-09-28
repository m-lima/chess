use gotham::hyper;

use super::middleware;
use super::websocket;

#[derive(serde::Deserialize, gotham_derive::StateData, gotham_derive::StaticResponseExtender)]
pub struct IdExtractor {
    id: u32,
}

pub fn game_connection(
    mut state: gotham::state::State,
) -> (gotham::state::State, hyper::Response<hyper::Body>) {
    use gotham::state::FromState;

    let id = IdExtractor::take_from(&mut state).id;
    let game = middleware::Game::borrow_mut_from(&mut state);

    if let Some(game) = game.get(id) {
        let headers = hyper::HeaderMap::take_from(&mut state);
        let body = hyper::Body::take_from(&mut state);
        if let Ok((response, ws)) = websocket::accept(&headers, body) {
            let req_id = request_id(&state).to_owned();
            let ws = ws
                .map_err(|err| eprintln!("websocket init error: {}", err))
                .and_then(move |ws| connected(req_id, ws));

            tokio::spawn(ws);

            (state, response)
        } else {
            (state, error_response(hyper::StatusCode::BAD_REQUEST))
        }
    } else {
        (state, error_response(hyper::StatusCode::NOT_FOUND))
    }
}

fn error_response(status: hyper::StatusCode) -> hyper::Response<hyper::Body> {
    hyper::Response::builder()
        .status(status)
        .body(hyper::Body::empty())
        .unwrap()
}
