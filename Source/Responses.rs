
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.05.04                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


use serde_json;
use serde::Serialize;


use crate::QueryError::QueryError;


pub fn generic_query_to_response_JSON<T: Serialize>(generic_query: Result<T, QueryError>) -> String
{
	let response_generic: T = match(generic_query)
	{
		Ok(response_generic) => response_generic,
		Err(error) => return format!("{{\"error\": \"{}\"}}", error)
	};

	match(serde_json::to_string(&response_generic))
	{
		Ok(response_body) => return response_body,
		Err(error) => return format!("{{\"error\": \"{}\"}}", error)
	}
}
