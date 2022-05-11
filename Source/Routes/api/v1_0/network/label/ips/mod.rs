
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


use crate::Query::{query_to_response, Queries::IP::SELECT_IPs_by_Network_label};


// `/api/v1.0/networks/label/{label}/ips`
pub async fn index() -> HttpResponse
{
	let body = r#"
	{	
		"/api/v1.0/network/label/{network_label}/ips/all": "List all IPs based on network label",
		"/api/v1.0/network/label/{network_label}/ips/group": "Queries for IPs based on group and network label"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/networks/label/{label}/ips/all`
pub async fn all(auth: BearerAuth, path: web::Path<(String)>, pool: web::Data<(PgPool)>) -> HttpResponse
{
	if(env!("NETWORKIPLOOKUP_BEARERTOKEN") != auth.token())
	{
		return HttpResponse::Unauthorized().insert_header(ContentType::json()).body("{\"error\": \"Unauthorized\"}");
	}

	let label = path.into_inner();
	let query_response = SELECT_IPs_by_Network_label(pool.as_ref(), label).await;
	return query_to_response(query_response);
}
