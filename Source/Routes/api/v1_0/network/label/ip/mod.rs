
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

use crate::Queries::{query_to_json, SELECT_IPs_by_Network_label};


// `/api/v1.0/networks/label/{label}/ip`
pub async fn index(path: web::Path<(String)>) -> HttpResponse
{
	let (label) = path.into_inner();
	let body = format!("/api/v1.0/networks/label/{}/ip", label);
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/networks/label/{label}/ips`
pub async fn ips(pool: web::Data<(PgPool)>, path: web::Path<(String)>) -> HttpResponse
{
	let (label) = path.into_inner();
	let query_response = SELECT_IPs_by_Network_label(pool.as_ref(), label).await;
	let body = query_to_json(query_response);
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}
