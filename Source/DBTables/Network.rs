
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


use serde::Serialize;


#[derive(Clone, Debug, Serialize)]
pub struct Network
{
	pub id: i32,
	pub auth_value: Option<String>,
	pub label: String,
	pub gateway: String,
	pub netmask: String
}


impl Network
{
	pub fn new(id: i32, auth_value: Option<String>, label: String, gateway: String, netmask: String) -> Network
	{
		return Network{id: id, auth_value: auth_value, label: label, gateway: gateway, netmask: netmask};
	}
}


impl std::fmt::Display for Network
{
    fn fmt(&self, format: &mut std::fmt::Formatter) -> std::fmt::Result
    {
		return write!(format, "{}", serde_json::to_string(self).unwrap());
    }
}
