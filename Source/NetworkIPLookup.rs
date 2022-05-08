
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.04.29                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


#![allow(non_snake_case)]
#![allow(unused_parens)]
#![allow(non_camel_case_types)]


use actix_web::{web, App, HttpServer};


mod IP;
mod Network;
mod Queries;
mod QueryError;
mod Routes;


use sqlx::postgres::PgPool;


use crate::Routes::api;


#[actix_web::main]
async fn main() -> std::io::Result<()>
{
	let host = "localhost";
	let user = env!("USER");
	let DB_name = "NetworkIPLookup";

	let connection_str = format!("postgres://{}@{}:5432/{}", user, host, DB_name);
	let connection_pool = PgPool::connect(&connection_str).await.expect("Failed to create Postgres DB connection pool");

	HttpServer::new
	(
		move ||
		{
			App::new().app_data(web::Data::new(connection_pool.clone()))
			  .route("/", web::get().to(Routes::index))
			  .route("/api", web::get().to(api::index))
			  .route("/api/v1.0", web::get().to(api::v1_0::index))
			  // —— GROUP —— //
			  .route("/api/v1.0/group", web::get().to(api::v1_0::group::index))
			  .route("/api/v1.0/groups", web::get().to(api::v1_0::group::groups))
			  // —— NETWORK —— //
			  .route("/api/v1.0/network", web::get().to(api::v1_0::network::index))
			  .route("/api/v1.0/networks", web::get().to(api::v1_0::network::networks))
			  // —— NETWORK::ID —— //
			  .route("/api/v1.0/network/id", web::get().to(api::v1_0::network::id::index))
			  .route("/api/v1.0/network/id/{id}", web::get().to(api::v1_0::network::id::value))
			  .route("/api/v1.0/network/id/{id}/ip", web::get().to(api::v1_0::network::id::ip::index))
			  .route("/api/v1.0/network/id/{id}/ips", web::get().to(api::v1_0::network::id::ip::ips))
			  // —— NETWORK::LABEL —— //
			  .route("/api/v1.0/network/label", web::get().to(api::v1_0::network::label::index))
			  .route("/api/v1.0/network/label/{label}", web::get().to(api::v1_0::network::label::value))
			  .route("/api/v1.0/network/label/{label}/ip", web::get().to(api::v1_0::network::label::ip::index))
			  .route("/api/v1.0/network/label/{label}/ips", web::get().to(api::v1_0::network::label::ip::ips))
		}
	)
	  .bind("127.0.0.1:8080")?
	  .run()
	  .await
}
