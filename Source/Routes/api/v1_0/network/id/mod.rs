
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
pub mod ips;


use actix_web::{HttpResponse, http::header::ContentType, web};
use sqlx::postgres::PgPool;


use crate::Queries::{query_to_json, SELECT_Network_by_id};


// `/api/v1.0/network/id`
pub async fn index() -> HttpResponse
{
	let body = r#"
	{
		"/api/v1.0/network/id/{id}": "Get a network by id",
		"/api/v1.0/network/id/{id}/ip": "Queries for IP based on network id"
		"/api/v1.0/network/id/{id}/ips": "Queries for IPs based one network id"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/network/id/{id}`
pub async fn id(pool: web::Data<(PgPool)>, path: web::Path<(i32)>) -> HttpResponse
{
	let (id) = path.into_inner();
	let query_response = SELECT_Network_by_id(pool.as_ref(), id).await;
	let body = query_to_json(query_response);
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}
