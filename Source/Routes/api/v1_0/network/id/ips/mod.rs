
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.05.08                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


pub mod group;


use actix_web::{http::header::ContentType, HttpResponse, web};
use sqlx::postgres::PgPool;


use crate::Query::{query_to_response, Queries::IP::SELECT_IPs_by_Network_id};


// `/api/v1.0/networks/id/{id}/ips`
pub async fn index() -> HttpResponse
{
	let body = r#"
	{	
		"/api/v1.0/network/id/{network_id}/ips/all": "List all IPs based on network id",
		"/api/v1.0/network/id/{network_id}/ips/group": "Queries for IPs based on group and network id"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/networks/id/{id}/ips/all`
pub async fn all(pool: web::Data<(PgPool)>, path: web::Path<(i32)>) -> HttpResponse
{
	let (id) = path.into_inner();
	let query_response = SELECT_IPs_by_Network_id(pool.as_ref(), id).await;
	return query_to_response(query_response);
}
