
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


pub mod id;
pub mod label;


use actix_web::{http::header::ContentType, HttpResponse, web};
use sqlx::postgres::PgPool;

use crate::Queries::{query_to_json, SELECT_IPs_by_Network_id};


pub async fn index() -> HttpResponse
{
	let body = r#"
	{
		"/api/v1.0/network/id/{id}/ips": "Gets all the IPs for a network",
		"/api/v1.0/network/id/{id}/ip/id": "List endpoints for the IPs for a network",
		"/api/v1.0/network/id/{id}/ip/label": "List endpoints for the IPs for a network"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/networks/id/{id}/ips`
pub async fn ips(pool: web::Data<(PgPool)>, path: web::Path<(i32)>) -> HttpResponse
{
	let (id) = path.into_inner();
	let query_response = SELECT_IPs_by_Network_id(pool.as_ref(), id).await;
	let body = query_to_json(query_response);
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}
