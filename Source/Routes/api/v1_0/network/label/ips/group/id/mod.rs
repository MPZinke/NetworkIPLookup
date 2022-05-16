
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
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sqlx::postgres::PgPool;


use crate::DBTables::IP::IP;
use crate::Query::{query_to_response, Queries::IP::SELECT_IPs_by_Network_label_AND_Group_id};
use crate::Query::QueryError::QueryError as Error;


// `/api/v1.0/network/label/{network_label}/ips/group/id`
pub async fn index() -> HttpResponse
{
	let body: &str = r#"
	{
		"/api/v1.0/network/label/{network_label}/ips/group/id/{group_id}": "List all IPs based on group id and network label"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/network/label/{network_label}/ips/group/id/{group_id}`
pub async fn id(auth: BearerAuth, path: web::Path<(String, i32)>, pool: web::Data<PgPool>) -> HttpResponse
{
	if(env!("NETWORKIPLOOKUP_BEARERTOKEN") != auth.token())
	{
		return HttpResponse::Unauthorized().insert_header(ContentType::json()).body(r#"{"error": "Unauthorized"}"#);
	}

	let (Network_label, Group_id) = path.into_inner();
	let query_response: Result<Vec<IP>, Error> = SELECT_IPs_by_Network_label_AND_Group_id(pool.as_ref(), &Network_label,
	  Group_id).await;
	return query_to_response(query_response);
}
