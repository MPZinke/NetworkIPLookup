
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


pub mod address;
pub mod id;
pub mod label;


use actix_web::{http::header::ContentType, HttpResponse};


// `/api/v1.0/network/id/{id}/ip`
pub async fn index() -> HttpResponse
{
	let body = r#"
	{
		"/api/v1.0/network/id/{id}/ip/address": "Queries for IP based on IP address and network id",
		"/api/v1.0/network/id/{id}/ip/id": "Queries for IP based on IP id and network id",
		"/api/v1.0/network/id/{id}/ip/label": "Queries for IP based on IP label and network id",
		"/api/v1.0/network/id/{id}/ips/group": "Queries for IPs based on group and network id"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}
