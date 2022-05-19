
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.05.05                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


use sqlx::{postgres::PgRow, Row};
use serde::{Deserialize, Serialize};


use crate::DBTables::Network::Network;


#[derive(Debug, Deserialize, Serialize)]
pub struct Device
{
	pub address: String,
	pub label: String,
	pub is_reservation: bool,
	pub is_static: bool,
	pub mac: Option<String>,
	pub groups: Vec<String>,
	pub Network: Network
}


impl Device
{
	pub fn new(groups: Vec<String>, row: &PgRow) -> Device
	{
		return Device{address: row.get("address"), label: row.get("label"), is_reservation: row.get("is_reservation"),
		  is_static: row.get("is_static"), mac: row.get("mac"), groups: groups,
		  Network: Network::new(row.get("Network.label"),
		  row.get("Network.gateway"), row.get("Network.netmask"))};
	}
}
