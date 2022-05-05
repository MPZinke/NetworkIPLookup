
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.05.05                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


pub mod network;


use actix_web::{HttpResponse, http::header::ContentType};


// `/api/v1.0`
pub async fn index() -> HttpResponse
{
	// list options: ['/api/v1.0/network']
	let body = r#"{"/api/v1.0/network": "Query all networks"}"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);

}