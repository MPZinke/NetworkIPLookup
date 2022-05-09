
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


use actix_web::{HttpResponse, http::header::ContentType, web};
use sqlx::postgres::PgPool;


use crate::Queries::{query_to_response, SELECT_Groups};


// `/api/v1.0/group`
pub async fn index() -> HttpResponse
{
	// SELECT_Network();
	let body = r#"
	{
		"/api/v1.0/groups": "List all groups",
		"/api/v1.0/group/id": "Get a group by ID path",
		"/api/v1.0/group/label": "Queries for group based on label"
	}
	"#;

	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/group/all`
pub async fn all(pool: web::Data<(PgPool)>) -> HttpResponse
{
	let query_response = SELECT_Groups(pool.as_ref()).await;
	return query_to_response(query_response);
}
