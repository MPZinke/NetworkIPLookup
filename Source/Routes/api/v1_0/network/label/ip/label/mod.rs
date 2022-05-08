
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

use crate::Queries::{query_to_json, SELECT_IP_by_Network_label_AND_IP_label};


// `/api/v1.0/network/label/{label}/ip/label`
pub async fn index(pool: web::Data<(PgPool)>) -> HttpResponse
{
	let body = r#"
	{
		"/api/v1.0/network/label/{label}/ip/label/{label}": "Get an IP by IP label and network label"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/network/label/{label}/ip/label/{label}`
pub async fn value(pool: web::Data<(PgPool)>, path: web::Path<(String, String)>) -> HttpResponse
{
	let (Network_label, IP_label) = path.into_inner();
	let query_response = SELECT_IP_by_Network_label_AND_IP_label(pool.as_ref(), Network_label, IP_label).await;
	let body = query_to_json(query_response);
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}
