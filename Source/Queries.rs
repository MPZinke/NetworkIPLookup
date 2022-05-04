
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


use postgres;
use postgres::{Client, NoTls, Row, types::ToSql};
use std;
use std::io::ErrorKind;


use crate::QueryError::QueryError;
use crate::IP::IP;


// ———————————————————————————————————————————————————— QUERIES ———————————————————————————————————————————————————— //
// ————————————————————————————————————————————————————————————————————————————————————————————————————————————————— //


pub fn query(query: &str, args: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, postgres::error::Error>
{
	let mut connection: Client = Client::connect("host=localhost user=mpzinke dbname=NetworkIPLookup", NoTls)?;
	return connection.query(query, args);
}


pub fn query_Group_by_IP_id(IP_id: i32) -> Result<Vec<String>, QueryError>
{
	let query_str: &str = concat!(
	  "SELECT \"Group\".\"label\"\n",
	  "FROM \"Group-IP\"\n",
	  "JOIN \"Group\" ON \"Group-IP\".\"Group.id\" = \"Group\".\"id\"\n",
	  "WHERE \"Group-IP\".\"IP.id\" = $1;\n"
	);

	let result: Vec<Row> = query(query_str, &[&IP_id])?;

	let mut groups: Vec<String> = vec![];
	for row in result
	{
		groups.push(row.get(0));
	}
	return Ok(groups);
}


pub fn query_IP_by_id(id: i32) -> Result<IP, QueryError>
{
	let query_str: &str = concat!(
	  "SELECT \"IP\".\"address\", \"IP\".\"label\", \"IP\".\"is_reservation\", \"IP\".\"is_static\",\n",
	  "  \"IP\".\"mac\", \"Network\".\"label\" AS \"Network.label\", \"Network\".\"gateway\" AS \"Network.gateway\",\n",
	  "  \"Network\".\"netmask\" AS \"Network.netmask\"\n",
	  "FROM \"IP\"\n",
	  "JOIN \"Network\" ON \"IP\".\"Network.id\" = \"Network\".\"id\"\n",
	  "WHERE \"IP\".\"id\" = $1;"
	);

	let groups: Vec<String> = query_Group_by_IP_id(id)?;
	let result: Vec<Row> = query(query_str, &[&id])?;
	if(result.len() < 1)
	{
		// return Err(error::Error::from(io::Error::new(ErrorKind::NotFound, format!("No results found for `IP`.`id`: {}", id))));
		return Err(QueryError::NotFound(std::io::Error::new(ErrorKind::NotFound, format!("No results found for `IP`.`id`: {}", id))));
	}

	return Ok(IP::new(groups, &result[0]));
}

