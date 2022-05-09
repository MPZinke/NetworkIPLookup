
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


use crate::Queries::{query_to_json, SELECT_IP_by_Network_id_AND_IP_label};


// `/api/v1.0/network/id/{id}/ip/label`
pub async fn index() -> HttpResponse
{
	let body = r#"
	{
		"/api/v1.0/network/id/{id}/ip/label/{label}": "Get an IP by IP label and network id"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/network/id/{id}/ip/label/{label}`
pub async fn label(pool: web::Data<(PgPool)>, path: web::Path<(i32, String)>) -> HttpResponse
{
	let (Network_id, IP_label) = path.into_inner();
	let query_response = SELECT_IP_by_Network_id_AND_IP_label(pool.as_ref(), Network_id, IP_label).await;
	let body = query_to_json(query_response);
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}
