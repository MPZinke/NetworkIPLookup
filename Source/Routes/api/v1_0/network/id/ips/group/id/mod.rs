
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


use crate::Queries::{query_to_response, SELECT_IPs_by_Network_id_AND_Group_id};


// `/api/v1.0/network/id/{network_id}/ips/group/id`
pub async fn index() -> HttpResponse
{
	let body = r#"
	{
		"/api/v1.0/network/id/{network_id}/ips/group/id/{group_id}": "List all IPs based on group id and network id"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/network/id/{network_id}/ips/group/id/{group_id}`
pub async fn id(pool: web::Data<(PgPool)>, path: web::Path<(i32, i32)>) -> HttpResponse
{
	let (Network_id, Group_id) = path.into_inner();
	let query_response = SELECT_IPs_by_Network_id_AND_Group_id(pool.as_ref(), Network_id, Group_id).await;
	return query_to_response(query_response);
}
