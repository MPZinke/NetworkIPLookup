

use postgres::Row;


pub struct IP
{
	pub address: String,
	pub label: String,
	pub is_reservation: bool,
	pub is_static: bool,
	pub mac: Option<String>,
	pub Groups: Vec<String>,
	pub Network_label: String,
	pub Network_gateway: String,
	pub Network_netmask: String
}


impl IP
{
	pub fn new(groups: Vec<String>, row: &Row) -> IP
	{
		return IP{address: row.get("address"), label: row.get("label"), is_reservation: row.get("is_reservation"),
		  is_static: row.get("is_static"), mac: row.get("mac"), Groups: groups, Network_label: row.get("Network.label"),
		  Network_gateway: row.get("Network.gateway"), Network_netmask: row.get("Network.netmask")};
	}


	pub fn to_string(self) -> String
	{
		let string: String = format!("{{\"address\": \"{}\"}}", self.address);
		return string;
	}
}