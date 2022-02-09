#![warn(rust_2018_idioms)]

use std::{convert::Infallible, env, net::SocketAddr};

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_warp::{graphql, BadRequest, Response};
use tonic::transport::Channel;
use warp::{http::Response as HttpResponse, Filter, Rejection};

use pb::{
    greeter_client::GreeterClient,
    greeter_graphql::{build_graphql_schema, GreeterSchema},
};

mod pb {
    include!("../generated/helloworld.rs");
}

mod num {
    include!("../generated/num.rs");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = ([0, 0, 0, 0], 4000).into();
    println!(
        "{} listening on http://localhost:4000",
        env!("CARGO_BIN_NAME")
    );

    let grpc_client = GreeterClient::connect("http://localhost:4001").await?;

    let schema = build_graphql_schema::<Channel>().data(grpc_client).finish();
    print!("{}", schema.sdl());

    let graphql_post = graphql(schema).and_then(
        move |(schema, request): (GreeterSchema<_>, async_graphql::Request)| async move {
            let response = schema.execute(request).await;
            Ok::<_, Infallible>(Response::from(response))
        },
    );

    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/")))
    });

    let routes = graphql_playground
        .or(graphql_post)
        .recover(|err: Rejection| async move {
            if let Some(BadRequest(err)) = err.find() {
                return Ok::<_, Infallible>(warp::reply::with_status(
                    err.to_string(),
                    warp::http::StatusCode::BAD_REQUEST,
                ));
            }

            Ok(warp::reply::with_status(
                "INTERNAL_SERVER_ERROR".to_string(),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ))
        });

    warp::serve(routes).run(addr).await;

    Ok(())
}
