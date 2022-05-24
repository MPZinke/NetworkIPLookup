
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.05.11                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


pub mod Networks;


use regex::Regex;
use reqwest;
use reqwest::header::AUTHORIZATION;


use crate::DBTables::Device::Device;
use crate::Query::{NewNotFoundError, QueryError};
use crate::SearchType::{DeviceAttributeSearch, NetworkSearch};
use crate::UnknownLookup::Networks::NetworkInterface;


fn device_not_found_error(device: &DeviceAttributeSearch, network: &NetworkSearch) -> QueryError
{
	let network_search_attritube: String = match(network)
	{
		NetworkSearch::id(network_value) => network_value.id.to_string(),
		NetworkSearch::label(network_value) => network_value.label.clone()
	};

	let body: String = format!("No results found for `Network`.`{}`: '{}', `Device`.`{}`: '{:?}'", network,
	  network_search_attritube, device, device);
	return NewNotFoundError(body);
}


// SUMMARY: Looks up a device on the network for a given NetworkInterface <N> (eg Netgear's Web Interface).
// PARAMS:  Takes the device to search for, the network to search on, and the interface ( <N> ) to search with.
// DETAILS: Makes a request to the router's page. If request was successful, the data from the page is sent to <N>'s
//          devices expression builder. 
pub async fn lookup_device_on_network<N>(device: &DeviceAttributeSearch, network: &NetworkSearch)
  -> Result<Device, QueryError>
  where N: NetworkInterface
{
	let response: String = match(router_request(N::ATTACHED_DEVICES_PATH, &network).await)
	{
		Ok(response) => response,
		Err(error) => return Err(NewNotFoundError(error.to_string()))
	};

	let expression: String = N::build_device_expression(device);

	let section: String = match(regex_response_for_device_section(expression, &response))
	{
		Some(section) => section,
		None => return Err(device_not_found_error(device, network))
	};

	return Ok(N::convert_section_to_device(network, &section));
}


fn regex_and_default_to_empty_string(expression: &String, section: &String) -> String
{
	let empty_string: String = "".to_string();
	match(Regex::new(&expression))
	{
		Err(_) => return empty_string,
		Ok(regex)
		=>
		{
			match(regex.find(&section))
			{
				Some(match_value) => return match_value.as_str().to_string(),
				None => return empty_string
			}
		}
	}
}


fn regex_response_for_device_section(expression: String, response: &String) -> Option<String>
{
	let regex: Regex = match(Regex::new(&expression))
	{
		Ok(regex) => regex,
		Err(_) => return None
	};

	match(regex.find(&response))
	{
		None => return None,
		Some(match_value) => return Some(match_value.as_str().to_string())
	}
}


async fn router_request(attached_devices_path: &str, network: &NetworkSearch) -> Result<String, RequestError>
{
	let mut headers = reqwest::header::HeaderMap::new();
	let auth_value = network.network().auth_value.as_ref().unwrap().as_ref();
	let header_value = reqwest::header::HeaderValue::from_str(auth_value)?;
	headers.insert(AUTHORIZATION, header_value);

	let client = reqwest::Client::builder().default_headers(headers).build()?;
	return Ok(client.get(format!("http://{}/{}", &network.network().gateway, attached_devices_path)).send().await?
	  .text().await?);
}


// ——————————————————————————————————————————————————— ERROR ENUM ——————————————————————————————————————————————————— //

// FROM: https://fettblog.eu/rust-enums-wrapping-errors/
//  AND: https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/wrap_error.html

#[derive(Debug)]
pub enum RequestError
{
    InvalidHeader(reqwest::header::InvalidHeaderValue),
    Reqwest(reqwest::Error),
    Generic(std::io::Error),
}


impl std::fmt::Display for RequestError
{
    fn fmt(&self, format: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match(self)
        {
            RequestError::InvalidHeader(error) => write!(format, "{}", error),
            RequestError::Reqwest(error) => write!(format, "{}", error),
            RequestError::Generic(error) => write!(format, "{}", error),
        }
    }
}


impl From<reqwest::header::InvalidHeaderValue> for RequestError
{
    fn from(err: reqwest::header::InvalidHeaderValue) -> Self
    {
        RequestError::InvalidHeader(err)
    }
}


impl From<reqwest::Error> for RequestError
{
    fn from(err: reqwest::Error) -> Self
    {
        RequestError::Reqwest(err)
    }
}


impl From<std::io::Error> for RequestError
{
    fn from(err: std::io::Error) -> Self
    {
        RequestError::Generic(err)
    }
}
