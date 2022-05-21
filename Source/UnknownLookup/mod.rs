
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


pub mod Expression;


use regex::Regex;
use reqwest;


use crate::DBTables::Device::Device;
use crate::DBTables::Network::Network;
use crate::Query::QueryError::{NewNotFoundError, QueryError as Error};


async fn router_request(network: &Network) -> Result<String, reqwest::Error>
{
	//TODO: Add header
	return reqwest::get(format!("http://{}/DEV_device_dev_id.htm", &network.gateway)).await?.text().await;
}


fn filter_response_for_Devices_section(value: &Expression::Expression, response: &String) -> Option<String>
{
	let mut ip_expression: String = r"([0-9]{1,3}.){3}[0-9]{1,3}".to_string();
	let mut label_expression: String = r"[\(\)_\s*\-:#&a-zA-Z0-9]*".to_string();
	let mut mac_expression: String = r"([a-fA-F0-9]{2}:){5}[a-fA-F0-9]{2}".to_string();

	match(value)
	{
		Expression::Expression::ip(ip) => ip_expression = ip.clone(),
		Expression::Expression::label(label) => label_expression = label.clone(),
		Expression::Expression::mac(mac) => mac_expression = mac.clone()
	}

	let expression: String = format!(
	  concat!(r#"<tr><td\s*align="center"><input\s*name="check_dev"\s*type="checkbox"\s*value="{}"onclick="handle_ch"#,
	    r#"eckboxElements\(this\);"></td>\s*\s*\s*<td\s*align="center"\s*name="show_status"><span\s*class="clickMe"\"#,
	    r#"s*>[a-zA-Z]*</span></td>\s*\s*\s*<td\s*align="center"><span\s*class="clickMe"\s*>(2\.4G\s*Wireless|5G\s*W"#,
	    r#"ireless|Wired)</span></td>\s*\s*\s*<td\s*align="center"><span\s*class="clickMe"\s*\s*><table\s*width="100"#,
	    r#"%"\s*title="[\(\)_\s*\-:#&a-zA-Z0-9]+"><tr><td><img\s*width=40px\s*height=40px\s*src="[_/\.a-zA-Z0-9]+"><"#,
	    r#"/td><td\s*align="right"><span>[\(\)_\s*\-:#&a-zA-Z0-9]*<br>{}</span></td></tr></table></span></td>\s*<td\"#,
	    r#"s*align="center"><span\s*class="clickMe"\s*>{}</span></td>\s*\s*\s*<td\s*align="center"><span\s*class="cl"#,
	    r#"ickMe"\s*name="mac"\s*>{}</span></td>"#),
	  mac_expression, label_expression, ip_expression, mac_expression);

	let regex: Regex = Regex::new(&expression).unwrap();

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


//TODO: Rework to get ip, label, mac, etc.
fn convert_section_to_Device(value: &Expression::Expression, network: Network, section: &String) -> Device
{
	let label_regex: String = r"<br>[\(\)_ \-:#&a-zA-Z0-9]*</span>".to_string();
	let label_section: String = regex_and_default_to_empty_string(&label_regex, section);
	let label: String = label_section[4..label_section.len()-7].to_string();
	let mac: String = regex_and_default_to_empty_string(&r"([a-fA-F0-9]{2}:){5}[a-fA-F0-9]{2}".to_string(), section);

	return Device{address: format!("{:?}", value), label: label, is_reservation: false, is_static: false, mac: Some(mac),
	  groups: vec![], Network: network};
}


pub async fn lookup_Device_on_network(value: &Expression::Expression, network: Network) -> Result<Device, Error>
{
	let response: String = match(router_request(&network).await)
	{
		Ok(response) => response,
		Err(error) => return Err(NewNotFoundError(error.to_string()))
	};

	let section: String = match(filter_response_for_Devices_section(value, &response))
	{
		Some(section) => section,
		None
		=>
		{
			let body: String = format!("No results found for `Network`.`label`: '{}', `Device`.`{}`: '{:?}'", 
			  &network.label, value/*.describe()*/, value);
			return Err(NewNotFoundError(body));
		}
	};

	return Ok(convert_section_to_Device(value, network, &section))
}
