
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.05.09                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


use actix_web::{http::header::ContentType, HttpResponse, web};
use sqlx::postgres::PgPool;


use crate::Query::{query_to_response, Queries::IP::SELECT_IP_by_Network_id_AND_IP_address};


pub async fn index() -> HttpResponse
{
	let body = r#"
	{
		"/api/v1.0/network/id/{network_id}/ip/address/{ip_address}": "Get an IP by IP address and network label"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


pub async fn address(pool: web::Data<(PgPool)>, path: web::Path<(i32, String)>) -> HttpResponse
{
	let (Network_id, IP_address) = path.into_inner();
	let query_response = SELECT_IP_by_Network_id_AND_IP_address(pool.as_ref(), Network_id, IP_address).await;
	return query_to_response(query_response);
}
