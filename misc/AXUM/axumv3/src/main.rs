#![allow(dead_code)]
#![allow(unused_imports)]
use axum::{
    async_trait,
    body::StreamBody,
    extract::{FromRequest, Query, State},
    handler::Handler,
    http::{header::CONTENT_TYPE, Request, StatusCode, Uri},
    middleware::{self, Next},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Form, Json, RequestExt, Router,
};
use axum_macros::debug_handler;
use minijinja::{context, Environment};
#[cfg(test)]
use mockall::automock;
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::time;
use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;
use tracing::Level;
use tracing_subscriber::fmt;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Serialize)]
#[cfg_attr(test, derive(Deserialize, Eq, PartialEq, Debug, Copy, Clone, Default))]
pub struct Hero {
    pub id: &'static str,
    pub name: &'static str,
}
#[derive(Clone)]
struct AppState(Vec<String>);

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

#[tokio::main]
async fn main() {
    /*  let server_config = ServerConfig {
        foo: "0.0.0.0".into(),
    };*/
    let MainState = AppState(vec_of_strings!["alex", "peter", "alice"]);
    let repo = Arc::new(HeroRepo()) as DynHeroesRepository;
    let subscriber = fmt()
        .compact()
        .with_thread_ids(true)
        .with_target(false)
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    tracing::info!("start main server loop");

    let app = Router::new()
        // .fallback(fallback)
        .nest("/heroes", heroes_routes())
        .with_state(repo)
        .route("/demo-form", get(get_demo_form).post(post_demo_form))
        .route("/devices", get(hello_html))
        .route("/test1", get(test_get_form).post(test_post_form))
        .with_state(MainState)
        .route("/test", get(show_form).post(accept_form));
    //.route("/ss", post(handler_test))
    //.with_state(&MainState);
    //.route("/", post(handler));

    // Start the server. Note that for brevity, we do not add logging, graceful shutdown, etc.
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn test_get_form() -> axum::response::Html<&'static str> {
    include_str!("./pages/test4.html").into()
}

#[derive(Debug, Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct User {
    pub username: String,
}

async fn test_post_form(form: axum::extract::Form<User>) -> axum::response::Html<String> {
    let user: User = form.0;
    format!(
        r#"
        <!doctype html>
        <html>
            <head>
                <title>Username</title>
            </head>
            <body>
                <h1>username</h1>
                {:?}
            </body>
        </html>
        "#,
        &user
    )
    .into()
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Input {
    name: String,
    email: String,
}
async fn accept_form(Form(input): Form<Input>) {
    println!("aaaaaaaaaaaaaaa");
    dbg!(&input);
}
#[derive(Deserialize, Debug)]
pub struct TestInput {
    username: String,
}
#[derive(Serialize)]
struct Resp0 {
    created: bool,
    username: String,
}
/*
async fn handler_test(
    State(state): State<AppState>,
    JsonOrForm(payload): JsonOrForm<TestInput>,
) -> impl IntoResponse {
    //async fn handler_test(State(state): State<AppState>) {
    println!("state is {}", state.0); // extracting common state
    dbg!(payload);
    let resp = Resp0 {
        created: true,
        username: state.0.to_owned(),
    };
    (StatusCode::CREATED, Json(resp))
    // include_str!("./pages/test2.html").into()
}
*/
struct JsonOrForm<T>(T);

#[async_trait]
impl<S, B, T> FromRequest<S, B> for JsonOrForm<T>
where
    B: Send + 'static,
    S: Send + Sync,
    Json<T>: FromRequest<(), B>,
    Form<T>: FromRequest<(), B>,
    T: 'static,
{
    type Rejection = Response;

    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let content_type_header = req.headers().get(CONTENT_TYPE);
        let content_type = content_type_header.and_then(|value| value.to_str().ok());

        if let Some(content_type) = content_type {
            if content_type.starts_with("application/json") {
                let Json(payload) = req.extract().await.map_err(IntoResponse::into_response)?;
                return Ok(Self(payload));
            }

            if content_type.starts_with("application/x-www-form-urlencoded") {
                let Form(payload) = req.extract().await.map_err(IntoResponse::into_response)?;
                return Ok(Self(payload));
            }
        }

        Err(StatusCode::UNSUPPORTED_MEDIA_TYPE.into_response())
    }
}

pub async fn get_demo_form() -> axum::response::Html<&'static str> {
    r#"
    <!doctype html>
    <html>
        <head>
            <title>Book</title>
        </head>
        <body>
            <h1>Book</h1>
            <form method="post" action="/demo-form">
                <p>
                    <label for="title">
                        Title:
                        <br>
                        <input name="title">
                    </label>
                </p>
                <p>
                    <label for="author">
                        Author:
                        <br>
                        <input name="author">
                    </label>
                </p>
                <p>
                    <input type="submit">
                </p?
            </form>
        </body>
    </html>
    "#
    .into()
}
#[derive(Debug, Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct Book {
    pub title: String,
    pub author: String,
}
pub async fn post_demo_form(form: axum::extract::Form<Book>) -> axum::response::Html<String> {
    let book: Book = form.0;
    format!(
        r#"
        <!doctype html>
        <html>
            <head>
                <title>Book</title>
            </head>
            <body>
                <h1>Book</h1>
                {:?}
            </body>
        </html>
        "#,
        &book
    )
    .into()
}

/*
#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    foo11: String,
}
async fn handler(JsonOrForm(payload): JsonOrForm<Payload>) {
    dbg!(payload);
}
*/
// handle no route
pub async fn fallback(uri: axum::http::Uri) -> (StatusCode, String) {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route {}", uri),
    )
}
async fn test_html() -> axum::response::Html<&'static str> {
    include_str!("./pages/test2.html").into()
}

async fn hello_html(State(state): State<AppState>) -> axum::response::Html<String> {
    //include_str!("./pages/test1.html").into()
    let mut env = Environment::new();
    env.add_template("hello.txt", include_str!("hello.txt"))
        .unwrap();
    let tmpl = env.get_template("hello.txt").unwrap();
    tmpl.render(context!(names => state.0 )).unwrap().into()
}
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    println!("signal shutdown");
}
#[cfg_attr(test, automock)]
#[async_trait]
trait HeroesRepositoryTrait {
    async fn get_by_name(&self, name: &str) -> Result<Vec<Hero>, DataBaseError>;
}

fn heroes_routes() -> Router<DynHeroesRepository> {
    Router::new().route("/", get(get_heroes))
}

#[debug_handler]
async fn get_heroes(
    State(repo): State<DynHeroesRepository>,
    filter: Query<GetHeroFilter>,
) -> impl IntoResponse {
    println!("GOT {:?}", filter.name.to_owned());
    let mut name_filter = filter.name.to_owned().unwrap_or("%".to_string());
    if !name_filter.ends_with("%") {
        name_filter.push('%');
    }
    let res = repo.get_by_name(name_filter.as_str()).await;
    match res {
        Ok(heroes) => {
            tracing::info!("valid hero found!");
            Json(heroes).into_response()
        }
        Err(DataBaseError::NotFound) => {
            tracing::error!("hero not found!");
            StatusCode::NOT_FOUND.into_response()
        }
        Err(_) => {
            tracing::error!("some tecnical error!");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

type HeroesRepositoryState = Arc<HeroRepo>;
type DynHeroesRepository = Arc<dyn HeroesRepositoryTrait + Send + Sync>;

#[derive(Deserialize)]
pub struct GetHeroFilter {
    name: Option<String>,
}
struct HeroRepo();

#[async_trait]
impl HeroesRepositoryTrait for HeroRepo {
    async fn get_by_name(&self, name: &str) -> Result<Vec<Hero>, DataBaseError> {
        const HEROES: [Hero; 2] = [
            Hero {
                id: "1",
                name: "wonder woman",
            },
            Hero {
                id: "2",
                name: "wounder man",
            },
        ];
        let found_heroes = HEROES
            .into_iter()
            .filter(|hero: &Hero| {
                if let Some(stripped_name) = name.strip_suffix('%') {
                    // filter
                    hero.name.starts_with(stripped_name)
                } else {
                    hero.name == name
                }
            })
            .collect::<Vec<Hero>>();
        if found_heroes.is_empty() {
            Err(DataBaseError::NotFound)
        } else {
            println!("your name :{}", name);
            Ok(found_heroes)
        }
    }
}

#[derive(Serialize)]
enum DataBaseError {
    NotFound,
    InternalError,
}

async fn show_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/test" method="post">
                #<form action="/devices" method="post">
                    <label for="name">
                        Enter your name:
                        <input type="text" name="name">
                    </label>
                    <label>
                        Enter your email:
                        <input type="text" name="email">
                    </label>
                    <input type="submit" value="Subscribe!">
                </form>
            </body>
        </html>
        "#,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use mockall::predicate::*;
    use rstest::rstest;
    use serde_json::{json, Value};
    use tower::ServiceExt;

    /// Helper function to create a GET request for a given URI.
    fn send_get_request(uri: &str) -> Request<Body> {
        Request::builder()
            .uri(uri)
            .method("GET")
            .body(Body::empty())
            .unwrap()
    }
    #[rstest]
    #[case("/?name=Wonder", "Wonder%")] // Verify that % is appended to the filter
    #[case("/?name=Wonder%", "Wonder%")] // Verify that % is not appended to the filter if it already ends with %
    #[case("/", "%")] // Verify that % is used as the default filter
    #[tokio::test]
    async fn get_by_name_success(#[case] uri: &'static str, #[case] expected_filter: &'static str) {
        // Create a vector of dummy heroes to return from the mock repository.
        let dummy_heroes = vec![Default::default()];

        // Create a mock repository and set the expectations for the get_by_name method.
        // Note that we are filtering the expectation by the expected filter string.
        let mut repo_mock = MockHeroesRepositoryTrait::new();
        let result = Ok(dummy_heroes.clone());
        repo_mock
            .expect_get_by_name()
            .with(eq(expected_filter))
            .return_once(move |_| result);

        // Create mock repository
        let repo = Arc::new(repo_mock) as DynHeroesRepository;

        // Create the app with the mock repository as state.
        let app = heroes_routes().with_state(repo);

        // Call the app with a GET request to the get_heroes endpoint.
        let response = app.oneshot(send_get_request(uri)).await.unwrap();

        // Check the response status code.
        assert_eq!(response.status(), StatusCode::OK);

        // Check the response body.
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!(&dummy_heroes));
    }
}
