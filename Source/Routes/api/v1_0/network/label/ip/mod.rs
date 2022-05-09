
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


pub mod id;
pub mod label;


use actix_web::{http::header::ContentType, HttpResponse};


// `/api/v1.0/networks/label/{label}/ip`
pub async fn index() -> HttpResponse
{
	let body = r#"
	{	
		"/api/v1.0/network/label/{label}/ip/id": "Queries for IP based on IP id and network label"
		"/api/v1.0/network/label/{label}/ip/label": "Queries for IP based on IP label and network label"
		"/api/v1.0/network/label/{label}/ips/group": "Queries for IPs based on group and network label"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}
