
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


use actix_web::{HttpResponse, http::header::ContentType, web};
use sqlx::postgres::PgPool;

use crate::Queries::{query_to_json, SELECT_Groups};


// `/api/v1.0/group`
pub async fn index() -> HttpResponse
{
	// SELECT_Network();
	let body = r#"
	{
		"/api/v1.0/groups": "Get all available groups",
		"/api/v1.0/group/id/{id}": "Get a group by its ID",
		"/api/v1.0/group/label/{label}": "Get a group by its label"
	}
	"#;

	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/groups
pub async fn groups(pool: web::Data<(PgPool)>) -> HttpResponse
{
	let query_response = SELECT_Groups(pool.as_ref()).await;
	let body = query_to_json(query_response);
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}
