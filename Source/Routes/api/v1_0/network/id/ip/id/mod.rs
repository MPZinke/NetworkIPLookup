
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


use actix_web::{http::header::ContentType, HttpResponse, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sqlx::postgres::PgPool;


use crate::Query::{query_to_response, Queries::IP::SELECT_IP_by_Network_id_AND_IP_id};


// `/api/v1.0/network/id/{network_id}/ip/id`
pub async fn index() -> HttpResponse
{
	let body = r#"
	{
		"/api/v1.0/network/id/{network_id}/ip/id/{ip_id}": "Get an IP by IP id and network id"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/network/id/{network_id}/ip/id/{ip_id}`
pub async fn id(auth: BearerAuth, path: web::Path<(i32, i32)>, pool: web::Data<(PgPool)>) -> HttpResponse
{
	if(env!("NETWORKIPLOOKUP_BEARERTOKEN") != auth.token())
	{
		return HttpResponse::Unauthorized().insert_header(ContentType::json()).body("{\"error\": \"Unauthorized\"}");
	}

	let (Network_id, IP_id) = path.into_inner();
	let query_response = SELECT_IP_by_Network_id_AND_IP_id(pool.as_ref(), Network_id, IP_id).await;
	return query_to_response(query_response);
}
