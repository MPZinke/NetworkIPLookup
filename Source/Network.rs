
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


use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
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
}
