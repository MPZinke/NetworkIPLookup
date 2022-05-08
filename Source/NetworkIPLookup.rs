
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
			  .route("/api/v1.0/groups", web::get().to(api::v1_0::group::all))
			  .route("/api/v1.0/group/id", web::get().to(api::v1_0::group::id::index))
			  .route("/api/v1.0/group/id/{id}", web::get().to(api::v1_0::group::id::id))
			  .route("/api/v1.0/group/label", web::get().to(api::v1_0::group::label::index))
			  .route("/api/v1.0/group/label/{label}", web::get().to(api::v1_0::group::label::label))

			  .route("/api/v1.0/network", web::get().to(api::v1_0::network::index))
			  .route("/api/v1.0/networks", web::get().to(api::v1_0::network::all))

			  .route("/api/v1.0/network/id", web::get().to(api::v1_0::network::id::index))
			  .route("/api/v1.0/network/id/{id}", web::get().to(api::v1_0::network::id::id))
			  .route("/api/v1.0/network/id/{id}/ip", web::get().to(api::v1_0::network::id::ip::index))
			  .route("/api/v1.0/network/id/{id}/ip/id", web::get().to(api::v1_0::network::id::ip::id::index))
			  .route("/api/v1.0/network/id/{id}/ip/id/{id}", web::get().to(api::v1_0::network::id::ip::id::id))
			  .route("/api/v1.0/network/id/{id}/ip/label", web::get().to(api::v1_0::network::id::ip::label::index))
			  .route("/api/v1.0/network/id/{id}/ip/label/{label}", web::get().to(api::v1_0::network::id::ip::label::label))
			  .route("/api/v1.0/network/id/{id}/ips", web::get().to(api::v1_0::network::id::ips::all))
			  .route("/api/v1.0/network/id/{id}/ips/*", web::get().to(api::v1_0::network::id::ips::all))
			  .route("/api/v1.0/network/id/{id}/ips/group", web::get().to(api::v1_0::network::id::ips::group::index))
			  .route("/api/v1.0/network/id/{id}/ips/group/id", web::get().to(api::v1_0::network::id::ips::group::id::index))
			  .route("/api/v1.0/network/id/{id}/ips/group/id/{id}", web::get().to(api::v1_0::network::id::ips::group::id::id))
			  .route("/api/v1.0/network/id/{id}/ips/group/label", web::get().to(api::v1_0::network::id::ips::group::label::index))
			  .route("/api/v1.0/network/id/{id}/ips/group/label/{label}", web::get().to(api::v1_0::network::id::ips::group::label::label))

			  .route("/api/v1.0/network/label", web::get().to(api::v1_0::network::label::index))
			  .route("/api/v1.0/network/label/{label}", web::get().to(api::v1_0::network::label::label))
			  .route("/api/v1.0/network/label/{label}/ip", web::get().to(api::v1_0::network::label::ip))
			  .route("/api/v1.0/network/label/{label}/ip/id", web::get().to(api::v1_0::network::label::ip::id::index))
			  .route("/api/v1.0/network/label/{label}/ip/id/{id}", web::get().to(api::v1_0::network::label::ip::id::id))
			  .route("/api/v1.0/network/label/{label}/ip/label", web::get().to(api::v1_0::network::label::ip::label::index))
			  .route("/api/v1.0/network/label/{label}/ip/label/{label}", web::get().to(api::v1_0::network::label::ip::label::label))
			  .route("/api/v1.0/network/label/{label}/ips", web::get().to(api::v1_0::network::label::ips::index))
			  .route("/api/v1.0/network/label/{label}/ips/*", web::get().to(api::v1_0::network::label::ips::all))
			  .route("/api/v1.0/network/label/{label}/ips/group", web::get().to(api::v1_0::network::label::ips::group::index))
			  .route("/api/v1.0/network/label/{label}/ips/group/id", web::get().to(api::v1_0::network::label::ips::group::id::index))
			  .route("/api/v1.0/network/label/{label}/ips/group/id/{id}", web::get().to(api::v1_0::network::label::ips::group::id::id))
			  .route("/api/v1.0/network/label/{label}/ips/group/label", web::get().to(api::v1_0::network::label::ips::group::label::index))
			  .route("/api/v1.0/network/label/{label}/ips/group/label/{label}", web::get().to(api::v1_0::network::label::ips::group::label::label))
		}
	)
	  .bind("127.0.0.1:8080")?
	  .run()
	  .await
}
