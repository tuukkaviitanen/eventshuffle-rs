use poem::{listener::TcpListener, EndpointExt, Route, Server};
use poem_openapi::{OpenApi, OpenApiService, payload::PlainText};

struct Api;

#[OpenApi]
impl Api {
    /// Hello world
    #[oai(path = "/", method = "get")]
    async fn index(&self) -> PlainText<&'static str> {
        PlainText("Hello World")
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
