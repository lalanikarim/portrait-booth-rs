use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{Html, IntoResponse},
    Extension,
};
use std::{error::Error, net::SocketAddr, path::PathBuf};

use async_graphql::{
    http::GraphiQLSource, Context, EmptyMutation, EmptySubscription, Object, Schema,
};
use axum::{
    body::{boxed, Body},
    http::{Response, StatusCode},
    routing::{get, get_service},
    Router, Server,
};
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};
use tokio::fs;
use tower_http::services::ServeDir;

#[macro_use]
extern crate dotenv_codegen;

struct EchoResponse {
    response: String,
}

#[Object]
impl EchoResponse {
    async fn response(&self) -> &str {
        &self.response
    }
}

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn ping<'a>(&self, _ctx: &Context<'a>) -> &str {
        "pong"
    }
    async fn echo<'a>(&self, _ctx: &Context<'a>, msg: String) -> EchoResponse {
        EchoResponse { response: msg }
    }
}

type PortraitBoothSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

async fn graphql_handler(
    schema: Extension<PortraitBoothSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db = Surreal::new::<Ws>(dotenv!("SURREALDB_URL")).await.unwrap();
    db.signin(Root {
        username: dotenv!("SURREALDB_USERNAME"),
        password: dotenv!("SURREALDB_PASSWORD"),
    })
    .await
    .unwrap();
    db.use_ns(dotenv!("SURREALDB_NS"))
        .use_db(dotenv!("SURREALDB_DATABASE"))
        .await
        .unwrap();

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(db)
        .finish();

    let app = Router::new()
        .route("/graphql", get(graphiql).post(graphql_handler))
        .layer(Extension(schema))
        .fallback({
            let dist_dir = "../dist".to_owned();
            get_service(ServeDir::new(dist_dir).append_index_html_on_directories(true))
                .handle_error(|error| async move {
                    println!("ServeDir Service Error!");
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("internal server error: {error}"),
                    )
                })
        })
        .fallback(get(move |req| async {
            let dist_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("dist");
            let spa_index = "./dist/index.html";
            if let Ok(index_content) = fs::read_to_string(spa_index).await {
                match ServeDir::new(dist_dir).try_call(req).await {
                    Ok(resp) => match resp.status() {
                        StatusCode::NOT_FOUND => Response::builder()
                            .status(StatusCode::OK)
                            .body(boxed(Body::from(index_content)))
                            .unwrap(),
                        _ => resp.map(boxed),
                    },
                    Err(err) => Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .body(boxed(Body::from(format!("index not found: {err:?}"))))
                        .unwrap(),
                }
            } else {
                Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(boxed(Body::from("index not found")))
                    .unwrap()
            }
        }));
    let api_addr: SocketAddr = dotenv!("API_LISTEN_ON").parse().unwrap();
    Server::bind(&api_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
