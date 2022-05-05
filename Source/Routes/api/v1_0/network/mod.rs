
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


// pub mod id;
pub mod label;


use actix_web::{HttpResponse, http::header::ContentType};


// `/api/v1.0/network`
pub async fn index() -> HttpResponse
{
	// SELECT_Network();
	let body = r#"
	{
		"/api/v1.0/network/id/{id}": "Get a network by its ID",
		"/api/v1.0/network/label/{label}": "Get a network by its ID"
	}
	"#;

	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}