
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


use actix_web::{http::header::ContentType, HttpResponse, web};
use sqlx::postgres::PgPool;

use crate::Queries::{query_to_json, SELECT_IP_by_Network_label_AND_IP_id};


// `/api/v1.0/network/label/{label}/ip/id`
pub async fn index(pool: web::Data<(PgPool)>) -> HttpResponse
{
	let body = r#"
	{
		"/api/v1.0/network/label/{label}/ip/id/{id}": "Get an IP by IP id and network label"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/network/label/{label}/ip/id/{id}`
pub async fn value(pool: web::Data<(PgPool)>, path: web::Path<(String, i32)>) -> HttpResponse
{
	let (Network_label, IP_id) = path.into_inner();
	let query_response = SELECT_IP_by_Network_label_AND_IP_id(pool.as_ref(), Network_label, IP_id).await;
	let body = query_to_json(query_response);
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}
