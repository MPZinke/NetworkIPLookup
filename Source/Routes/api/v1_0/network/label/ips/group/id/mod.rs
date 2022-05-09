
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


use crate::Queries::{query_to_response, SELECT_IPs_by_Network_label_AND_Group_id};


// `/api/v1.0/network/label/{network_label}/ips/group/id`
pub async fn index() -> HttpResponse
{
	let body = r#"
	{
		"/api/v1.0/network/label/{network_label}/ips/group/id/{group_id}": "List all IPs based on group id and network label"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/network/label/{network_label}/ips/group/id/{group_id}`
pub async fn id(pool: web::Data<(PgPool)>, path: web::Path<(String, i32)>) -> HttpResponse
{
	let (Network_label, Group_id) = path.into_inner();
	let query_response = SELECT_IPs_by_Network_label_AND_Group_id(pool.as_ref(), Network_label, Group_id).await;
	return query_to_response(query_response);
}
