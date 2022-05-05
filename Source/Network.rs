
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
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Network
{
	pub label: String,
	pub gateway: String,
	pub netmask: String
}


impl Network
{
	pub fn new(label: String, gateway: String, netmask: String) -> Network
	{
		return Network{label: label, gateway: gateway, netmask: netmask};
	}


	pub fn to_string(self) -> Result<String, serde_json::Error>
	{
		return serde_json::to_string(&self);
	}
}
