
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


use regex::Regex;
use reqwest;


use crate::DBTables::IP::IP;
use crate::DBTables::Network::Network;
use crate::Query::QueryError::{NewNotFoundError, QueryError as Error};


async fn router_request() -> Result<String, reqwest::Error>
{
	return reqwest::get(format!("http://{}/Local.html", env!("NETWORKIPLOOKUP_ROUTER_DOMAIN"))).await?.text().await;
}


fn filter_response_for_IPs_section(address: &String, response: &String) -> Option<String>
{
	let expression: String = format!(
	  concat!(r#"<tr><td\s*align="center"><input\s*name="check_dev"\s*type="checkbox"\s*value="([a-fA-F0-9]{{2}}:){{"#,
	    r#"5}}[a-fA-F0-9]{{2}}"onclick="handle_checkboxElements\(this\);"></td>\s*\s*\s*<td\s*align="center"\s*name="#,
	    r#""show_status"><span\s*class="clickMe"\s*>[a-zA-Z]*</span></td>\s*\s*\s*<td\s*align="center"><span\s*class"#,
	    r#"="clickMe"\s*>(2\.4G\s*Wireless|5G\s*Wireless|Wired)</span></td>\s*\s*\s*<td\s*align="center"><span\s*cla"#,
	    r#"ss="clickMe"\s*\s*><table\s*width="100%"\s*title="[\(\)_\s*\-:#&a-zA-Z0-9]+"><tr><td><img\s*width=40px\s*"#,
	    r#"height=40px\s*src="[_/\.a-zA-Z0-9]+"></td><td\s*align="right"><span>[\(\)_\s*\-:#&a-zA-Z0-9]*<br>[\(\)_\s"#,
	    r#"*\-:#&a-zA-Z0-9]*</span></td></tr></table></span></td>\s*\s*\s*<td\s*align="center"><span\s*class="clickM"#,
	    r#"e"\s*>{}</span></td>\s*\s*\s*<td\s*align="center"><span\s*class="clickMe"\s*name="mac"\s*>([a-fA-F0-9]{{2"#,
	    r#"}}:){{5}}[a-fA-F0-9]{{2}}</span></td>"#),
	  address);

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


fn convert_section_to_IP(address: String, network: Network, section: &String) -> IP
{
	let label_regex: String = r"<br>[\(\)_ \-:#&a-zA-Z0-9]*</span>".to_string();
	let label_section: String = regex_and_default_to_empty_string(&label_regex, section);
	let label: String = label_section[4..label_section.len()-7].to_string();
	let mac: String = regex_and_default_to_empty_string(&r"([a-fA-F0-9]{2}:){5}[a-fA-F0-9]{2}".to_string(), section);

	return IP{address: address, label: label, is_reservation: false, is_static: false, mac: Some(mac), groups: vec![],
	  Network: network};
}


pub async fn lookup_IP_on_network(address: String, network: Network) -> Result<IP, Error>
{
	let response: String = match(router_request().await)
	{
		Ok(response) => response,
		Err(error) => return Err(NewNotFoundError(error.to_string()))
	};

	let section: String = match(filter_response_for_IPs_section(&address, &response))
	{
		Some(section) => section,
		None
		=>
		{
			let body: String = format!("No results found for `Network`.`label`: '{}', `IP`.`address`: '{}'", 
			  &network.label, &address);
			return Err(NewNotFoundError(body));
		}
	};

	return Ok(convert_section_to_IP(address, network, &section))
}
