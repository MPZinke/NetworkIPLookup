
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


use crate::Query::{query_to_response, Queries::IP::SELECT_IP_by_Network_label_AND_IP_id};


// `/api/v1.0/network/label/{network_label}/ip/id`
pub async fn index() -> HttpResponse
{
	let body = r#"
	{
		"/api/v1.0/network/label/{network_label}/ip/id/{ip_id}": "Get an IP by IP id and network label"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/network/label/{network_label}/ip/id/{ip_id}`
pub async fn id(pool: web::Data<(PgPool)>, path: web::Path<(String, i32)>) -> HttpResponse
{
	let (Network_label, IP_id) = path.into_inner();
	let query_response = SELECT_IP_by_Network_label_AND_IP_id(pool.as_ref(), Network_label, IP_id).await;
	return query_to_response(query_response);
}
