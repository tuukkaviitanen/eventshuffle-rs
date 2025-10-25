use poem::{EndpointExt, Error, Result, Route, Server, listener::TcpListener, web::Data};
use poem_openapi::{ApiResponse, Object, OpenApi, OpenApiService, payload::Json};

struct Api;

#[derive(ApiResponse)]
enum GetEventsResponse {
    #[oai(status = "200")]
    Success(Json<Vec<Event>>),
    #[oai(status = "500")]
    InternalError,
}

#[derive(ApiResponse)]
#[oai(bad_request_handler = "bad_request_handler")]
enum PostEventResponse {
    #[oai(status = "201")]
    Success(Json<Vec<Event>>),
    #[oai(status = "400")]
    BadRequest(Json<ErrorBody>),
    #[oai(status = "500")]
    InternalError,
}

fn bad_request_handler(e: Error) -> PostEventResponse {
    PostEventResponse::BadRequest(Json(ErrorBody {
        error: e.to_string(),
    }))
}

#[OpenApi]
impl Api {
    /// Get list of events
    #[oai(path = "/events", method = "get")]
    async fn get_events(&self, conn: Data<&sqlx::postgres::PgPool>) -> Result<GetEventsResponse> {
        let events = sqlx::query_as!(Event, "SELECT id, name FROM event")
            .fetch_all(conn.0)
            .await
            .map_err(|err| {
                println!("Failed to fetch events {err}");
                GetEventsResponse::InternalError
            })?;

        Ok(GetEventsResponse::Success(Json(events)))
    }

    #[oai(path = "/events", method = "post")]
    async fn create_event(
        &self,
        body: Json<CreateEventBody>,
        conn: Data<&sqlx::postgres::PgPool>,
    ) -> Result<PostEventResponse> {
        let event = sqlx::query_as!(
            Event,
            "INSERT INTO event (name) VALUES ($1) RETURNING id, name",
            body.name
        )
        .fetch_one(conn.0)
        .await
        .map_err(|err| {
            println!("Failed to create event {err}");
            PostEventResponse::InternalError
        })?;

        Ok(PostEventResponse::Success(Json(vec![event])))
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    const URL_PREFIX: &str = "/api/v1";
    let api_service =
        OpenApiService::new(Api, "Eventshuffle", "development").url_prefix(URL_PREFIX);
    let ui = api_service.swagger_ui();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Connecting to database at: {}", database_url);
    let db_pool = sqlx::postgres::PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("Running migrations to the database");
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to run database migrations");

    const PORT: &str = "8080";

    println!("Listening on port {PORT}");

    let app = Route::new()
        .nest(URL_PREFIX, api_service)
        .nest("/", ui)
        .data(db_pool);
    Server::new(TcpListener::bind(format!("0.0.0.0:{PORT}")))
        .run(app)
        .await
}

#[derive(sqlx::FromRow, Debug, serde::Serialize, Object)]
struct Event {
    id: String,
    name: String,
}

#[derive(sqlx::FromRow, Debug, serde::Serialize, Object)]
struct EventDate {
    id: String,
    name: String,
    event_id: String,
}

#[derive(Debug, serde::Serialize, Object)]
struct ErrorBody {
    error: String,
}

#[derive(Object)]
struct CreateEventBody {
    name: String,
}
