
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


pub mod id;
pub mod label;


use actix_web::{http::header::ContentType, HttpResponse};


// `/api/v1.0/network/id/{network_id}/ips/group`
pub async fn index() -> HttpResponse
{
	let body = r#"
	{
		"/api/v1.0/network/id/{network_id}/ips/group/id": "Queries for IPs based on group id and network id",
		"/api/v1.0/network/id/{network_id}/ips/group/label": "Queries for IPs based on group label and network id"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}
