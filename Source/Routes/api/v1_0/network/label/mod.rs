
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


use crate::Queries::{query_to_response, SELECT_Network_by_label};


// `/api/v1.0/network/label`
pub async fn index() -> HttpResponse
{
	let body = r#"
	{
		"/api/v1.0/network/label/{network_label}": "Get a network by label",
		"/api/v1.0/network/label/{network_label}/ip": "Queries for IP based on network label"
		"/api/v1.0/network/label/{network_label}/ips": "Queries for IPs based one network label"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/network/label/{network_label}`
pub async fn label(pool: web::Data<(PgPool)>, path: web::Path<(String)>) -> HttpResponse
{
	let (label) = path.into_inner();
	let query_response = SELECT_Network_by_label(pool.as_ref(), label).await;
	return query_to_response(query_response);
}
