
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


use crate::Query::{query_to_response, Queries::Network::SELECT_Network_by_label};


// `/api/v1.0/network/label`
pub async fn index() -> HttpResponse
{
	let body = r#"
	{
		"/api/v1.0/network/label/{network_label}": "Get a network by label",
		"/api/v1.0/network/label/{network_label}/ip": "Queries for IP based on network label"
		"/api/v1.0/network/label/{network_label}/ips": "Queries for IPs based one network label"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/network/label/{network_label}`
pub async fn label(auth: BearerAuth, path: web::Path<(String)>, pool: web::Data<(PgPool)>) -> HttpResponse
{
	if(env!("NETWORKIPLOOKUP_BEARERTOKEN") != auth.token())
	{
		return HttpResponse::Unauthorized().insert_header(ContentType::json()).body(r#"{"error": "Unauthorized"}"#);
	}

	let label = path.into_inner();
	let query_response = SELECT_Network_by_label(pool.as_ref(), &label).await;
	return query_to_response(query_response);
}
