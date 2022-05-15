
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.05.09                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


use actix_web::{http::header::ContentType, HttpResponse, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sqlx::postgres::PgPool;


use crate::Query::{query_NotFound, query_to_response};
use crate::Query::Queries::{Network::SELECT_Network_by_label, IP::SELECT_IP_by_Network_label_AND_IP_address};
use crate::Query::QueryError::QueryError as Error;
use crate::UnknownLookup::lookup_IP_on_network;


pub async fn index() -> HttpResponse
{
	let body = r#"
	{
		"/api/v1.0/network/id/{network_id}/ip/address/{ip_address}": "Get an IP by IP address and network label"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


pub async fn address(auth: BearerAuth, path: web::Path<(String, String)>, pool: web::Data<(PgPool)>) -> HttpResponse
{
	if(env!("NETWORKIPLOOKUP_BEARERTOKEN") != auth.token())
	{
		return HttpResponse::Unauthorized().insert_header(ContentType::json()).body(r#"{"error": "Unauthorized"}"#);
	}

	let (Network_label, IP_address) = path.into_inner();
	let query_response = SELECT_IP_by_Network_label_AND_IP_address(pool.as_ref(), &Network_label, &IP_address).await;

	// If not found in DB, try to find IP address by scanning network
	if(query_NotFound(&query_response))
	{
		let network_query_response = match(SELECT_Network_by_label(pool.as_ref(), &Network_label).await)
		{
			Ok(network_query_response) => network_query_response,
			Err(network_error)
			=>
			{
				match(network_error)
				{
					Error::Postgres(_) => return query_to_response(query_response),
					Error::NotFound(_)
					=>
					{
						let body = format!(r#"{{"error": "No known network '{}'"}}"#, &Network_label);
						return HttpResponse::NotFound().insert_header(ContentType::json()).body(body);
					}
				}
			}
		};

		match(lookup_IP_on_network(IP_address, network_query_response).await)
		{
			Some(ip) => return query_to_response(Ok(ip)),
			None => return query_to_response(query_response)
		}
	}

	return query_to_response(query_response);
}
