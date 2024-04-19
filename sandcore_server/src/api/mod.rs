mod entrance;

use axum::Router;

pub fn routes() -> Router {
	Router::new().nest("/api", entrance::routes())
}