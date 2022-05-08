
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

use crate::Queries::{query_to_json, SELECT_Networks, SELECT_Network_by_id, SELECT_Network_by_label};


// `/api/v1.0/network/id/{id}/ip/label`
pub async fn index(pool: web::Data<(PgPool)>) -> HttpResponse
{
	let body = r#"
	{
		"/api/v1.0/network/id/{id}/ip/label/{label}": "Get and IP based on label for a Network id"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/network/Network_id/{id}/ip/label/{label}`
// pub async fn value(pool: web::Data<(PgPool)>, path: web::Path<(i32, label)>) -> HttpResponse
// {
// 	let (Network_id, IP_label) = path.into_inner();
// 	let query_response = SELECT_Network_by_Network_id_AND_IP_label(pool.as_ref(), Network_id, IP_label).await;
// 	let body = query_to_json(query_response);
// 	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
// }