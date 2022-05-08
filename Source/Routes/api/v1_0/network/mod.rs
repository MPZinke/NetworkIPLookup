
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.05.05                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


pub mod id;
pub mod label;


use actix_web::{HttpResponse, http::header::ContentType, web};
use sqlx::postgres::PgPool;

use crate::Queries::{query_to_json, SELECT_Networks, SELECT_Network_by_id, SELECT_Network_by_label};


// `/api/v1.0/network`
pub async fn index() -> HttpResponse
{
	// SELECT_Network();
	let body = r#"
	{
		"/api/v1.0/networks": "Get all available networks",
		"/api/v1.0/network/id/{id}": "Get a network by its ID",
		"/api/v1.0/network/label/{label}": "Get a network by its label"
	}
	"#;

	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/networks
pub async fn networks(pool: web::Data<(PgPool)>) -> HttpResponse
{
	let query_response = SELECT_Networks(pool.as_ref()).await;
	let body = query_to_json(query_response);
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}
