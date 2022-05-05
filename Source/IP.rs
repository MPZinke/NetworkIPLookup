

use postgres::Row;
use serde_json;
use serde::{Deserialize, Serialize};


use crate::Network::Network;
use crate::QueryError::QueryError;


#[derive(Serialize, Deserialize)]
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
	pub fn new(groups: Vec<String>, row: &Row) -> IP
	{
		return IP{address: row.get("address"), label: row.get("label"), is_reservation: row.get("is_reservation"),
		  is_static: row.get("is_static"), mac: row.get("mac"), groups: groups,
		  Network: Network::new(row.get("Network.label"),
		  row.get("Network.gateway"), row.get("Network.netmask"))};
	}


	pub fn to_string(self) -> Result<String, serde_json::Error>
	{
		return serde_json::to_string(&self);
	}
}
