use gotham::hyper;

use super::middleware;

#[derive(serde::Deserialize, gotham_derive::StateData, gotham_derive::StaticResponseExtender)]
pub struct IdExtractor {
    id: String,
}

pub fn game_connection(
    mut state: gotham::state::State,
) -> (gotham::state::State, hyper::Response<hyper::Body>) {
    use gotham::handler::IntoResponse;
    use gotham::state::FromState;

    let id = IdExtractor::take_from(&mut state).id;
    let game = middleware::Game::borrow_mut_from(&mut state);

    let response = game.get(&id).map_or_else(
        || {
            hyper::Response::builder()
                .status(hyper::StatusCode::NOT_FOUND)
                .body(hyper::Body::empty())
                .unwrap()
        },
        |r| r.into_response(&state),
    );
    (state, response)
}
