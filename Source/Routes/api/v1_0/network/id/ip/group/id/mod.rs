
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


use actix_web::{http::header::ContentType, HttpResponse, web};
use sqlx::postgres::PgPool;

use crate::Queries::{query_to_json, SELECT_Networks, SELECT_Network_by_id, SELECT_Network_by_label};


// `/api/v1.0/network/id/{id}/ips/group/id`
pub async fn index(pool: web::Data<(PgPool)>) -> HttpResponse
{
	let body = r#"
	{
		"/api/v1.0/network/id/{id}/ips/group/id/{id}": "List all IPs based on group id and network id"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/network/id/{id}/ips/group/id/{id}`
pub async fn ips(pool: web::Data<(PgPool)>, path: web::Path<(i32)>) -> HttpResponse
{
	let (id) = path.into_inner();
	let query_response = SELECT_IPs_by_Network_id_AND_Group_id(pool.as_ref(), id).await;
	let body = query_to_json(query_response);
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}
