
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


use crate::DBTables::Device::Device;
use crate::Query::{query_to_response, Queries::Device::SELECT_Devices_by_Network_id_AND_Group_label};
use crate::Query::QueryError::QueryError as Error;


// `/api/v1.0/network/id/{network_id}/devices/group`
pub async fn index() -> HttpResponse
{
	let body: &str = r#"
	{
		"/api/v1.0/network/id/{network_id}/devices/group/label/{group_label}": "List all devices based on group label and network id"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/network/id/{network_id}/devices/group/label/{group_label}`
pub async fn label(auth: BearerAuth, path: web::Path<(i32, String)>, pool: web::Data<PgPool>) -> HttpResponse
{
	if(env!("NETWORKLOOKUP_BEARERTOKEN") != auth.token())
	{
		return HttpResponse::Unauthorized().insert_header(ContentType::json()).body(r#"{"error": "Unauthorized"}"#);
	}

	let (Network_id, Group_label) = path.into_inner();
	let query_response: Result<Vec<Device>, Error> = SELECT_Devices_by_Network_id_AND_Group_label(pool.as_ref(), Network_id,
	  &Group_label).await;
	return query_to_response(query_response);
}