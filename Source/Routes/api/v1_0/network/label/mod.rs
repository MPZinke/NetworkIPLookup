
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

use crate::Queries::{query_to_json, SELECT_Network_by_label};


// `/api/v1.0/network/label`
pub async fn index() -> HttpResponse
{
	let body = r#"
	{
		"/api/v1.0/network/label/{label}": "Gets all the IPs for a network",
		"/api/v1.0/network/label/{label}/ip": "List endpoints for the IPs for a network",
		"/api/v1.0/network/label/{label}/group": "List endpoints for the IPs for a network"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/network/label/{label}`
pub async fn value(pool: web::Data<(PgPool)>, path: web::Path<(String)>) -> HttpResponse
{
	let (label) = path.into_inner();
	let query_response = SELECT_Network_by_label(pool.as_ref(), label).await;
	let body = query_to_json(query_response);
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}
