
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


pub mod device;
pub mod devices;


use actix_web::{HttpResponse, http::header::ContentType, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sqlx::postgres::PgPool;


use crate::DBTables::Network::Network;
use crate::LookupError::LookupError;
use crate::Query::{query_to_response, Network::SELECT_Network_by_label};


// `/api/v1.0/network/label`
pub async fn index() -> HttpResponse
{
	let body: &str = r#"
	{
		"/api/v1.0/network/label/{network_label}": "Get a network by label",
		"/api/v1.0/network/label/{network_label}/device": "Queries for device based on network label",
		"/api/v1.0/network/label/{network_label}/devices": "Queries for devices based one network label"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/network/label/{network_label}`
pub async fn label(auth: BearerAuth, path: web::Path<String>, pool: web::Data<PgPool>) -> HttpResponse
{
	if(env!("NETWORKLOOKUP_BEARERTOKEN") != auth.token())
	{
		return HttpResponse::Unauthorized().insert_header(ContentType::json()).body(r#"{"error": "Unauthorized"}"#);
	}

	let label = path.into_inner();
	let query_response: Result<Network, LookupError> = SELECT_Network_by_label(pool.as_ref(), &label).await;
	return query_to_response(query_response);
}
