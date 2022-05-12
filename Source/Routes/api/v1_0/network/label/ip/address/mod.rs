
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


use crate::Query::{query_to_response, Queries::IP::SELECT_IP_by_Network_label_AND_IP_address};
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
		return HttpResponse::Unauthorized().insert_header(ContentType::json()).body("{\"error\": \"Unauthorized\"}");
	}

	let (Network_label, IP_address) = path.into_inner();
	let query_response = SELECT_IP_by_Network_label_AND_IP_address(pool.as_ref(), &Network_label, &IP_address).await;

	// If not found in DB, try to find IP address by scanning network
	if(query_response.is_err())
	{
		match(lookup_IP_on_network(IP_address).await)
		{
			Some(value)
			=>{ 
				match(serde_json::to_string(&value))
				{
					Ok(response_body) => println!("{}", response_body),
					Err(error) => println!("{{\"error\": \"{}\"}}", error)
				}
			}
			None => println!("Nothing found")
		}
	}

	return query_to_response(query_response);
}
