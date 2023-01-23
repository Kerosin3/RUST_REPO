#![allow(dead_code)]
#![allow(unused_imports)]
use axum::{
    async_trait,
    extract::{Query, State},
    http::StatusCode,
    middleware::{self, Next},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use axum_macros::debug_handler;
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
#[tokio::main]
async fn main() {
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
        .nest("/heroes", heroes_routes())
        .with_state(repo);

    // Start the server. Note that for brevity, we do not add logging, graceful shutdown, etc.
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
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
            Ok(found_heroes)
        }
    }
}

#[derive(Serialize)]
enum DataBaseError {
    NotFound,
    InternalError,
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
