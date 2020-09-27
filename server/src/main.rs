#![deny(warnings, clippy::pedantic, clippy::all)]
#![warn(rust_2018_idioms)]

mod handler;
mod middleware;

fn routes() -> gotham::router::Router {
    use gotham::pipeline;
    use gotham::router::builder;

    let pipeline = pipeline::new_pipeline()
        .add(middleware::Log)
        .add(middleware::Game::new())
        .add(middleware::Cors::new())
        .build();

    let (chain, pipelines) = pipeline::single::single_pipeline(pipeline);

    builder::build_router(chain, pipelines, |route| {
        use gotham::router::builder::{DefineSingleRoute, DrawRoutes};
        route
            .get("/:id")
            .with_path_extractor::<handler::IdExtractor>()
            .to(handler::game_connection);
    })
}

fn init_logger() {
    let config = simplelog::ConfigBuilder::new()
        .set_time_format_str("%Y-%m-%dT%H:%M:%SZ")
        .build();

    simplelog::TermLogger::init(
        simplelog::LevelFilter::Info,
        config,
        simplelog::TerminalMode::Mixed,
    )
    .expect("Could not initialize logger");
}

fn main() {
    init_logger();

    gotham::start_with_num_threads(format!("0.0.0.0:{}", 3030), routes(), 1);
}
