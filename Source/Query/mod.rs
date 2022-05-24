
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


pub mod Device;
pub mod Group;
pub mod Network;


use actix_web::{http::header::ContentType, HttpResponse, HttpResponseBuilder};
use serde_json;
use serde::Serialize;


pub fn query_NotFound<T: Serialize>(generic_query: &Result<T, QueryError>) -> bool
{
	match(generic_query)
	{
		Ok(_) => return false,
		Err(error)
		=>
		{
			match(error)
			{
				QueryError::Postgres(_) => return false,
				QueryError::NotFound(_) => return true
			}
		}
	}
}


pub fn query_to_response<T: Serialize>(generic_query: Result<T, QueryError>) -> HttpResponse
{
	let response_generic: T = match(generic_query)
	{
		Ok(response_generic) => response_generic,
		Err(error) =>
		{
			let response: fn() -> HttpResponseBuilder = match(error)
			{
				QueryError::Postgres(_) => HttpResponse::InternalServerError,
				QueryError::NotFound(_) => HttpResponse::NotFound
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


// ——————————————————————————————————————————————————— ERROR ENUM ——————————————————————————————————————————————————— //

// FROM: https://fettblog.eu/rust-enums-wrapping-errors/
//  AND: https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/wrap_error.html

#[derive(Debug)]
pub enum QueryError
{
    Postgres(sqlx::error::Error),
    NotFound(std::io::Error),
}


impl std::fmt::Display for QueryError
{
    fn fmt(&self, format: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match(self)
        {
            QueryError::Postgres(error) => write!(format, "{}", error),
            QueryError::NotFound(error) => write!(format, "{}", error),
        }
    }
}


impl From<sqlx::error::Error> for QueryError
{
    fn from(err: sqlx::error::Error) -> Self
    {
        QueryError::Postgres(err)
    }
}


impl From<std::io::Error> for QueryError
{
    fn from(err: std::io::Error) -> Self
    {
        QueryError::NotFound(err)
    }
}


// ———————————————————————————————————————————————— HELPER FUNCTIONS ———————————————————————————————————————————————— //

pub fn NewNotFoundError(message: String) -> QueryError
{
	return QueryError::NotFound(std::io::Error::new(std::io::ErrorKind::NotFound, message));
}

