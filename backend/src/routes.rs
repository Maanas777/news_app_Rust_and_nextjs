// src/routes.rs
use warp::Filter;
use super::handler;


pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_news()
}

fn get_news() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end()  // This will match the root path "/"
        .and(warp::get())
        .and_then(handler::handle_news)
}
