
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
use crate::Query::QueryError::QueryError as Error;
use crate::SearchType::{DeviceAttributeSearch, NetworkSearch};
use crate::UnknownLookup;


// FROM: https://stackoverflow.com/a/49287579
pub trait NetworkInterface
{
	fn convert_section_to_device(device: &DeviceAttributeSearch, network: &NetworkSearch, section: &String) -> Device;
	fn device_expression(device: &DeviceAttributeSearch) -> String;
}


pub async fn lookup_device(device: &DeviceAttributeSearch, network: &NetworkSearch) -> Result<Device, Error>
{
	match(network.network().label.as_ref())
	{
		"Home" => return UnknownLookup::lookup_device_on_network::<Netgear::Netgear>(device, network).await,
		_ => UnknownLookup::lookup_device_on_network::<Netgear::Netgear>(device, network).await
	}
}
