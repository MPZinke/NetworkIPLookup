
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


use crate::DBTables::Device::Device;
use crate::Query::QueryError::{NewNotFoundError, QueryError as Error};
use crate::SearchType::{DeviceAttributeSearch, NetworkSearch};
use crate::UnknownLookup::Networks::NetworkInterface;


fn device_not_found_error(device: &DeviceAttributeSearch, network: &NetworkSearch) -> Error
{
	let network_search_attritube: String = match(network)
	{
		NetworkSearch::id(network_value) => network_value.id.to_string(),
		NetworkSearch::label(network_value) => network_value.label.clone()
	};

	let body: String = format!("No results found for `Network`.`{}`: '{:?}', `Device`.`{}`: '{:?}'", network,
	  network_search_attritube, device, device);
	return NewNotFoundError(body);
}


async fn router_request(network: &NetworkSearch) -> Result<String, reqwest::Error>
{
	//TODO: Add header
	return reqwest::get(format!("http://{}/DEV_device_dev_id.htm", &network.network().gateway)).await?.text().await;
}


fn filter_response_for_device_section(expression: String, response: &String) -> Option<String>
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


pub async fn lookup_device_on_network<N>(device: &DeviceAttributeSearch, network: &NetworkSearch)
  -> Result<Device, Error>
  where N: NetworkInterface
{
	let response: String = match(router_request(&network).await)
	{
		Ok(response) => response,
		Err(error) => return Err(NewNotFoundError(error.to_string()))
	};

	let expression: String = N::device_expression(device);

	let section: String = match(filter_response_for_device_section(expression, &response))
	{
		Some(section) => section,
		None => return Err(device_not_found_error(device, network))
	};

	return Ok(N::convert_section_to_device(device, network, &section));
}
