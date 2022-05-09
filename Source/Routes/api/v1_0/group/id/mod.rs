
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


use actix_web::{HttpResponse, http::header::ContentType, web};
use sqlx::postgres::PgPool;


use crate::Queries::{query_to_response, SELECT_Group_by_id};


// `/api/v1.0/group/id`
pub async fn index() -> HttpResponse
{
	// SELECT_Network();
	let body = r#"
	{
		"/api/v1.0/group/id/{group_id}": "Get a group by ID"
	}
	"#;

	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/group/id/{group_id}`
pub async fn id(pool: web::Data<(PgPool)>, path: web::Path<(i32)>) -> HttpResponse
{
	let (id) = path.into_inner();
	let query_response = SELECT_Group_by_id(pool.as_ref(), id).await;
	return query_to_response(query_response);
}
