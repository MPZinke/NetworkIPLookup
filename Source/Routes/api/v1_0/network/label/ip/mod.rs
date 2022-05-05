
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


// pub mod address;
// pub mod group;
// pub mod label;


use actix_web::{http::header::ContentType, HttpResponse, web};


// `/api/v1.0/networks/label/{label}/ip`
pub async fn index(path: web::Path<(String)>) -> HttpResponse
{
	let (label) = path.into_inner();
	let body = format!("/api/v1.0/networks/label/{}/ip", label);
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}