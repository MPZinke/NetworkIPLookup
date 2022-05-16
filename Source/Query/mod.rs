
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.05.09                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


pub mod Queries;
pub mod QueryError;


use actix_web::{http::header::ContentType, HttpResponse, HttpResponseBuilder};
use serde_json;
use serde::Serialize;


use crate::Query::QueryError::QueryError as Error;


pub fn query_NotFound<T: Serialize>(generic_query: &Result<T, Error>) -> bool
{
	match(generic_query)
	{
		Ok(_) => return false,
		Err(error)
		=>
		{
			match(error)
			{
				Error::Postgres(_) => return false,
				Error::NotFound(_) => return true
			}
		}
	}
}


pub fn query_to_response<T: Serialize>(generic_query: Result<T, Error>) -> HttpResponse
{
	let response_generic: T = match(generic_query)
	{
		Ok(response_generic) => response_generic,
		Err(error) =>
		{
			let response: fn() -> HttpResponseBuilder = match(error)
			{
				Error::Postgres(_) => HttpResponse::InternalServerError,
				Error::NotFound(_) => HttpResponse::NotFound
			};
			let error_message: String = format!(r#"{{"error": "{}"}}"#, error);
			return response().insert_header(ContentType::json()).body(error_message);
		}
	};

	match(serde_json::to_string(&response_generic))
	{
		Ok(response_body) => return HttpResponse::Ok().insert_header(ContentType::json()).body(response_body),
		Err(error) => 
		{
			let error_message: String = format!(r#"{{"error": "{}"}}"#, error);
			return HttpResponse::InternalServerError().insert_header(ContentType::json()).body(error_message);
		}
	}
}
