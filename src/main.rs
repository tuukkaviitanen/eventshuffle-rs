use poem::{listener::TcpListener, web::Data, EndpointExt, Route, Server, Result};
use poem_openapi::{payload::Json, ApiResponse, Object, OpenApi, OpenApiService};


struct Api;

#[derive(ApiResponse)]
enum GetEventsResponse {
    #[oai(status = "200")]
    Success(Json<Vec<Event>>),
    #[oai(status = "400")]
    BadRequest,
    #[oai(status = "500")]
    InternalError
}

#[OpenApi]
impl Api {
    /// Get list of events
    #[oai(path = "/", method = "get")]
    async fn get_events(&self, conn: Data<&sqlx::postgres::PgPool>) -> Result<GetEventsResponse> {
        let events = sqlx::query_as!(Event,
            "SELECT id, name FROM event"
        )
        .fetch_all(conn.0)
        .await
        .map_err(|_| GetEventsResponse::InternalError)?;

        Ok(GetEventsResponse::Success(Json(events)))
    }

}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api_service = OpenApiService::new(Api, "eventshuffle-rs", "development").url_prefix("/api");
    let ui = api_service.swagger_ui();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Connecting to database at: {}", database_url);
    let db_pool = sqlx::postgres::PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("Running migrations to the database");
    sqlx::migrate!("./migrations").run(&db_pool).await.expect("Failed to run database migrations");

    const PORT: &str = "8080";

    println!("Listening on port {PORT}");

    let app = Route::new().nest("/api", api_service).nest("/", ui).data(db_pool);
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
    event_id: String
}
