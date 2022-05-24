
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
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sqlx::postgres::PgPool;


use crate::DBTables::Group::Group;
use crate::Query::{query_to_response, Group::SELECT_Group_by_label};
use crate::Query::QueryError;


// `/api/v1.0/group/label`
pub async fn index() -> HttpResponse
{
	let body: &str = r#"
	{
		"/api/v1.0/group/label/{group_label}": "Get a group by label"
	}
	"#;

	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/group/label/{group_label}`
pub async fn label(auth: BearerAuth, path: web::Path<String>, pool: web::Data<PgPool>) -> HttpResponse
{
	if(env!("NETWORKLOOKUP_BEARERTOKEN") != auth.token())
	{
		return HttpResponse::Unauthorized().insert_header(ContentType::json()).body(r#"{"error": "Unauthorized"}"#);
	}

	let label = path.into_inner();
	let query_response: Result<Group, QueryError> = SELECT_Group_by_label(pool.as_ref(), &label).await;
	return query_to_response(query_response);
}
