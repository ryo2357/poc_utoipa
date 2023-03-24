use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use axum::{routing, Router, Server};
use hyper::Error;

mod todo;
use todo::Store;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let store = Arc::new(Store::default());
    let app = Router::new()
        // .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .route(
            "/todo",
            routing::get(todo::list_todos).post(todo::create_todo),
        )
        .route("/todo/search", routing::get(todo::search_todos))
        .route(
            "/todo/:id",
            routing::put(todo::mark_done).delete(todo::delete_todo),
        )
        .with_state(store);

    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
    println!("サーバー起動前");
    Server::bind(&address)
        .serve(app.into_make_service())
        .await?;

    println!("サーバー起動中は到達しない");
    Ok(())
}
