
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


use crate::Query::{query_to_response, Queries::IP::SELECT_IP_by_Network_label_AND_IP_label};


// `/api/v1.0/network/label/{network_label}/ip/label`
pub async fn index() -> HttpResponse
{
	let body = r#"
	{
		"/api/v1.0/network/label/{network_label}/ip/label/{ip_label}": "Get an IP by IP label and network label"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/network/label/{network_label}/ip/label/{ip_label}`
pub async fn label(auth: BearerAuth, path: web::Path<(String, String)>, pool: web::Data<(PgPool)>) -> HttpResponse
{
	if(env!("NETWORKIPLOOKUP_BEARERTOKEN") != auth.token())
	{
		return HttpResponse::Unauthorized().insert_header(ContentType::json()).body(r#"{"error": "Unauthorized"}"#);
	}

	let (Network_label, IP_label) = path.into_inner();
	let query_response = SELECT_IP_by_Network_label_AND_IP_label(pool.as_ref(), &Network_label, &IP_label).await;
	return query_to_response(query_response);
}
