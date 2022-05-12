
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


async fn router_request() -> Result<String, reqwest::Error>
{
	return reqwest::get(format!("http://{}/Local.html", env!("NETWORKIPLOOKUP_ROUTER_DOMAIN"))).await?.text().await;
}


pub fn response_contains_IP_address(address: &String, response: &String) -> bool
{
	// Don't assume the regex is properly formatted because user can put garbage into the address param.
	let expression = format!(r#"<td align="center"><span class="clickMe" >{}</span></td>"#, address);
	let regex = match(Regex::new(&expression))
	{
		Ok(regex) => regex,
		Err(_) => return false
	};

	return regex.is_match(&response);
}


pub fn scrape_IP_data_from_response(address: &String, response: &String) -> Option<String>
{
	let mac_regex = r"([a-fA-F0-9]{2}:){5}[a-fA-F0-9]{2}";
	let expression = format!(r#"<tr><td align="center"><input name="check_dev" type="checkbox" value="{}"onclick="handle_checkboxElements\(this\);"></td>
		<td align="center" name="show_status"><span class="clickMe" >[a-zA-Z]*</span></td>
		<td align="center"><span class="clickMe" >(2\.4G Wireless|5G Wireless|Wired)</span></td>
		<td align="center"><span class="clickMe"  ><table width="100%" title="[\(\)_ \-:#&a-zA-Z0-9]+"><tr><td><img width=40px height=40px src="[_/\.a-zA-Z0-9]+"></td><td align="right"><span>[\(\)_ \-:#&a-zA-Z0-9]*<br>[\(\)_ \-:#&a-zA-Z0-9]*</span></td></tr></table></span></td>
		<td align="center"><span class="clickMe" >{}</span></td>
		<td align="center"><span class="clickMe" name="mac" >{}</span></td>"#, mac_regex, address, mac_regex);

	let regex = match(Regex::new(&expression))
	{
		Ok(regex) => regex,
		Err(_) => return None
	};

	match(regex.find(&response))
	{
		Some(string) => return Some(string.as_str().to_string()),
		None => return None
	}
}


pub async fn lookup_IP_on_network(address: String) -> Option<IP>
{
	match router_request().await
	{
		Err(_) => return None,
		Ok(value)
		=>
		{
			if(!response_contains_IP_address(&address, &value))
			{
				println!("None");
				return None;
			}

			let body = scrape_IP_data_from_response(&address, &value);
			if(body.is_none())
			{
				return None;
			}

			println!("Is match {}", body.unwrap());
			return Some(IP{address: address, label: "TBD".to_string(), is_reservation: false, is_static: false, mac: None,
			  groups: vec![], Network: Network::new("TBD".to_string(), "TBD".to_string(), "TBD".to_string())});
		}
	}
}
