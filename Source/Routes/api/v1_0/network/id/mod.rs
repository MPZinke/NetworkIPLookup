
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.05.07                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


pub mod ip;


use actix_web::{HttpResponse, http::header::ContentType, web};
use sqlx::postgres::PgPool;

use crate::Queries::{query_to_json, SELECT_Network_by_id};


// `/api/v1.0/network/id`
pub async fn index() -> HttpResponse
{
	let body = r#"
	{
		"/api/v1.0/network/id/{id}": "Gets all the IPs for a network",
		"/api/v1.0/network/id/{id}/ip": "List endpoints for the IPs for a network",
		"/api/v1.0/network/id/{id}/group": "List endpoints for the IPs for a network"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/network/id/{id}`
pub async fn value(pool: web::Data<(PgPool)>, path: web::Path<(i32)>) -> HttpResponse
{
	let (id) = path.into_inner();
	let query_response = SELECT_Network_by_id(pool.as_ref(), id).await;
	let body = query_to_json(query_response);
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}