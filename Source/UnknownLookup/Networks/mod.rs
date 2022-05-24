
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.05.21                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


pub mod Netgear;


use crate::DBTables::Device::Device;
use crate::Query::QueryError;
use crate::SearchType::{DeviceAttributeSearch, NetworkSearch};
use crate::UnknownLookup;


// FROM: https://stackoverflow.com/a/49287579
pub trait NetworkInterface
{
	const ATTACHED_DEVICES_PATH: &'static str;
	fn build_device_expression(device: &DeviceAttributeSearch) -> String;
	fn convert_section_to_device(network: &NetworkSearch, section: &String) -> Device;
}


pub async fn lookup_device(device: &DeviceAttributeSearch, network: &NetworkSearch) -> Result<Device, QueryError>
{
	match(network.network().label.as_ref())
	{
		"Home" => return UnknownLookup::lookup_device_on_network::<Netgear::Netgear>(device, network).await,
		_ => UnknownLookup::lookup_device_on_network::<Netgear::Netgear>(device, network).await
	}
}
