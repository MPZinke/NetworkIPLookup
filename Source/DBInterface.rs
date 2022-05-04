
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.05.02                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


#![allow(non_snake_case)]
#![allow(unused_parens)]


use postgres::{Client, Error, NoTls, Row, types::ToSql};


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
	pub fn new(address: String, label: String, is_reservation: bool, is_static: bool, mac: Option<String>,
	  Groups: Vec<String>, Network_label: String, Network_gateway: String, Network_netmask: String) -> IP
	{
		return IP{address: address, label: label, is_reservation: is_reservation, is_static: is_static, mac: mac,
		  Groups: Groups, Network_label: Network_label, Network_gateway: Network_gateway,
		  Network_netmask: Network_netmask};
	}


	pub fn to_string(self) -> String
	{
		let string: String = format!("{{\"address\": \"{}\"}}", self.address);
		return string;
	}
}


pub fn query(query: &str, args: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, Error>
{
	match(Client::connect("host=localhost user=mpzinke dbname=NetworkIPLookup", NoTls))
	{
		Err(error) => return Err(error),
		Ok(mut connection) =>
		{
			return connection.query(query, args);
		}
	}
}


pub fn query_IP_id(id: i32) -> Option<IP>
{


	let query_str: &str = concat!(
	  "SELECT \"IP\".\"address\", \"IP\".\"label\", \"IP\".\"is_reservation\", \"IP\".\"is_static\", ",
	  "  \"IP\".\"mac\", \"Network\".\"label\", \"Network\".\"gateway\", \"Network\".\"netmask\" ",
	  "FROM \"IP\" ",
	  "JOIN \"Network\" ON \"IP\".\"Network.id\" = \"Network\".\"id\" ",
	  "WHERE \"IP\".\"id\" = $1");

	match(query(query_str, &[&id]))
	{
		Err(error) =>
		{
			println!("{}", error);
			return None;
		},
		Ok(result) =>
		{
			let groups: Vec<String> = vec![];
			if(result.len() < 1)
			{
				return None;
			}

			return Some(IP{address: result[0].get(0), label: result[0].get(1), is_reservation: result[0].get(2),
			  is_static: result[0].get(3), mac: result[0].get(4), Groups: groups, Network_label: result[0].get(5),
			  Network_gateway: result[0].get(6), Network_netmask: result[0].get(7)});
		}
	}
}

