
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


pub mod ip;
pub mod ips;


use actix_web::{HttpResponse, http::header::ContentType, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sqlx::postgres::PgPool;


use crate::Query::{query_to_response, Queries::Network::SELECT_Network_by_id};


// `/api/v1.0/network/id`
pub async fn index() -> HttpResponse
{
	let body = r#"
	{
		"/api/v1.0/network/id/{network_id}": "Get a network by id",
		"/api/v1.0/network/id/{network_id}/ip": "Queries for IP based on network id"
		"/api/v1.0/network/id/{network_id}/ips": "Queries for IPs based one network id"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/network/id/{network_id}`
pub async fn id(auth: BearerAuth, path: web::Path<(i32)>, pool: web::Data<(PgPool)>) -> HttpResponse
{
	if(env!("NETWORKIPLOOKUP_BEARERTOKEN") != auth.token())
	{
		return HttpResponse::Unauthorized().insert_header(ContentType::json()).body("{\"error\": \"Unauthorized\"}");
	}

	let id = path.into_inner();
	let query_response = SELECT_Network_by_id(pool.as_ref(), id).await;
	return query_to_response(query_response);
}
