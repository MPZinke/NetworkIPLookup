
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


mod DBTables;
mod LookupUnknownIP;
mod Query;
mod Routes;


use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPool;


use crate::Routes::api;


#[actix_web::main]
async fn main() -> std::io::Result<()>
{
	let host: &str = "localhost";
	let user: &str = env!("USER");
	let DB_name: &str = "NetworkIPLookup";

	let connection_str: String = format!("postgres://{}@{}:5432/{}", user, host, DB_name);
	let connection_pool: PgPool = PgPool::connect(&connection_str).await
	  .expect("Failed to create Postgres DB connection pool");

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
			  .route("/api/v1.0/group/all", web::get().to(api::v1_0::group::all))
			  .route("/api/v1.0/group/id", web::get().to(api::v1_0::group::id::index))
			  .route("/api/v1.0/group/id/{group_id}", web::get().to(api::v1_0::group::id::id))
			  .route("/api/v1.0/group/label", web::get().to(api::v1_0::group::label::index))
			  .route("/api/v1.0/group/label/{group_label}", web::get().to(api::v1_0::group::label::label))

			  .route("/api/v1.0/network", web::get().to(api::v1_0::network::index))
			  .route("/api/v1.0/network/all", web::get().to(api::v1_0::network::all))

			  .route("/api/v1.0/network/id", web::get().to(api::v1_0::network::id::index))
			  .route("/api/v1.0/network/id/{network_id}", web::get().to(api::v1_0::network::id::id))
			  .route("/api/v1.0/network/id/{network_id}/ip", web::get().to(api::v1_0::network::id::ip::index))
			  .route("/api/v1.0/network/id/{network_id}/ip/address", web::get().to(api::v1_0::network::id::ip::address::index))
			  .route("/api/v1.0/network/id/{network_id}/ip/address/{ip_address}", web::get().to(api::v1_0::network::id::ip::address::address))
			  .route("/api/v1.0/network/id/{network_id}/ip/id", web::get().to(api::v1_0::network::id::ip::id::index))
			  .route("/api/v1.0/network/id/{network_id}/ip/id/{ip_id}", web::get().to(api::v1_0::network::id::ip::id::id))
			  .route("/api/v1.0/network/id/{network_id}/ip/label", web::get().to(api::v1_0::network::id::ip::label::index))
			  .route("/api/v1.0/network/id/{network_id}/ip/label/{ip_label}", web::get().to(api::v1_0::network::id::ip::label::label))
			  .route("/api/v1.0/network/id/{network_id}/ips", web::get().to(api::v1_0::network::id::ips::index))
			  .route("/api/v1.0/network/id/{network_id}/ips/all", web::get().to(api::v1_0::network::id::ips::all))
			  .route("/api/v1.0/network/id/{network_id}/ips/group", web::get().to(api::v1_0::network::id::ips::group::index))
			  .route("/api/v1.0/network/id/{network_id}/ips/group/id", web::get().to(api::v1_0::network::id::ips::group::id::index))
			  .route("/api/v1.0/network/id/{network_id}/ips/group/id/{group_id}", web::get().to(api::v1_0::network::id::ips::group::id::id))
			  .route("/api/v1.0/network/id/{network_id}/ips/group/label", web::get().to(api::v1_0::network::id::ips::group::label::index))
			  .route("/api/v1.0/network/id/{network_id}/ips/group/label/{group_label}", web::get().to(api::v1_0::network::id::ips::group::label::label))

			  .route("/api/v1.0/network/label", web::get().to(api::v1_0::network::label::index))
			  .route("/api/v1.0/network/label/{network_label}", web::get().to(api::v1_0::network::label::label))
			  .route("/api/v1.0/network/label/{network_label}/ip", web::get().to(api::v1_0::network::label::ip::index))
			  .route("/api/v1.0/network/label/{network_label}/ip/address", web::get().to(api::v1_0::network::label::ip::address::index))
			  .route("/api/v1.0/network/label/{network_label}/ip/address/{ip_address}", web::get().to(api::v1_0::network::label::ip::address::address))
			  .route("/api/v1.0/network/label/{network_label}/ip/id", web::get().to(api::v1_0::network::label::ip::id::index))
			  .route("/api/v1.0/network/label/{network_label}/ip/id/{ip_id}", web::get().to(api::v1_0::network::label::ip::id::id))
			  .route("/api/v1.0/network/label/{network_label}/ip/label", web::get().to(api::v1_0::network::label::ip::label::index))
			  .route("/api/v1.0/network/label/{network_label}/ip/label/{ip_label}", web::get().to(api::v1_0::network::label::ip::label::label))
			  .route("/api/v1.0/network/label/{network_label}/ips", web::get().to(api::v1_0::network::label::ips::index))
			  .route("/api/v1.0/network/label/{network_label}/ips/all", web::get().to(api::v1_0::network::label::ips::all))
			  .route("/api/v1.0/network/label/{network_label}/ips/group", web::get().to(api::v1_0::network::label::ips::group::index))
			  .route("/api/v1.0/network/label/{network_label}/ips/group/id", web::get().to(api::v1_0::network::label::ips::group::id::index))
			  .route("/api/v1.0/network/label/{network_label}/ips/group/id/{group_id}", web::get().to(api::v1_0::network::label::ips::group::id::id))
			  .route("/api/v1.0/network/label/{network_label}/ips/group/label", web::get().to(api::v1_0::network::label::ips::group::label::index))
			  .route("/api/v1.0/network/label/{network_label}/ips/group/label/{group_label}", web::get().to(api::v1_0::network::label::ips::group::label::label))
		}
	)
	  .bind("127.0.0.1:8081")?
	  .run()
	  .await
}
