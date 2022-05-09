
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


use crate::Network::Network;


#[derive(Debug, Deserialize, Serialize)]
pub struct IP
{
	pub address: String,
	pub label: String,
	pub is_reservation: bool,
	pub is_static: bool,
	pub mac: Option<String>,
	pub groups: Vec<String>,
	pub Network: Network
}


impl IP
{
	pub fn new(groups: Vec<String>, row: &PgRow) -> IP
	{
		return IP{address: row.get("address"), label: row.get("label"), is_reservation: row.get("is_reservation"),
		  is_static: row.get("is_static"), mac: row.get("mac"), groups: groups,
		  Network: Network::new(row.get("Network.label"),
		  row.get("Network.gateway"), row.get("Network.netmask"))};
	}
}
