
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


use crate::DBTables::{Device::Device, Network::Network};
use crate::UnknownLookup::SearchType::DeviceAttributeSearch;


trait NetworkMod
{
	fn convert_section_to_device(network: Network, section: &String, value: &DeviceAttributeSearch) -> Device;
	fn device_expression(value: &DeviceAttributeSearch) -> String;
}
