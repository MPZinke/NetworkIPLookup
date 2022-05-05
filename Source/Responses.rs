
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


use crate::IP::IP;
use crate::QueryError::QueryError;


pub fn IP_query_to_response_JSON(IP_query: Result<IP, QueryError>) -> String
{
	let response_IP: IP = match(IP_query)
	{
		Ok(response_IP) => response_IP,
		Err(error) => return format!("{{\"error\": \"{}\"}}", error)
	};

	match(response_IP.to_string())
	{
		Ok(response_body) => return response_body,
		Err(error) => return format!("{{\"error\": \"{}\"}}", error)
	}
}


pub fn IPs_query_to_response_JSON(IPs_query: Result<Vec<IP>, QueryError>) -> String
{
	let response_IPs: Vec<IP> = match(IPs_query)
	{
		Ok(response_IPs) => response_IPs,
		Err(error) => return format!("{{\"error\": \"{}\"}}", error)
	};

	match(serde_json::to_string(&response_IPs))
	{
		Ok(response_body) => return response_body,
		Err(error) => return format!("{{\"error\": \"{}\"}}", error)
	}
}