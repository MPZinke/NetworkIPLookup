
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


pub mod group;


use actix_web::{http::header::ContentType, HttpResponse, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sqlx::postgres::PgPool;


use crate::DBTables::IP::IP;
use crate::Query::{query_to_response, Queries::IP::SELECT_IPs_by_Network_id};
use crate::Query::QueryError::QueryError as Error;


// `/api/v1.0/networks/id/{id}/ips`
pub async fn index() -> HttpResponse
{
	let body: &str = r#"
	{	
		"/api/v1.0/network/id/{network_id}/ips/all": "List all IPs based on network id",
		"/api/v1.0/network/id/{network_id}/ips/group": "Queries for IPs based on group and network id"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/networks/id/{id}/ips/all`
pub async fn all(auth: BearerAuth, path: web::Path<i32>, pool: web::Data<PgPool>) -> HttpResponse
{
	if(env!("NETWORKIPLOOKUP_BEARERTOKEN") != auth.token())
	{
		return HttpResponse::Unauthorized().insert_header(ContentType::json()).body(r#"{"error": "Unauthorized"}"#);
	}

	let id = path.into_inner();
	let query_response: Result<Vec<IP>, Error> = SELECT_IPs_by_Network_id(pool.as_ref(), id).await;
	return query_to_response(query_response);
}
