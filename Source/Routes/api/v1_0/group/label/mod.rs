
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


use crate::Query::{query_to_response, Queries::Group::SELECT_Group_by_label};


// `/api/v1.0/group/label`
pub async fn index() -> HttpResponse
{
	let body = r#"
	{
		"/api/v1.0/group/label/{group_label}": "Get a group by label"
	}
	"#;

	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/group/label/{group_label}`
pub async fn label(auth: BearerAuth, path: web::Path<(String)>, pool: web::Data<(PgPool)>) -> HttpResponse
{
	if(env!("NETWORKIPLOOKUP_BEARERTOKEN") != auth.token())
	{
		return HttpResponse::Unauthorized().insert_header(ContentType::json()).body("{\"error\": \"Unauthorized\"}");
	}

	let label = path.into_inner();
	let query_response = SELECT_Group_by_label(pool.as_ref(), &label).await;
	return query_to_response(query_response);
}
