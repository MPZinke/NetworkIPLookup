
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


use crate::IP::IP;
use crate::QueryError::{NewNotFoundError, QueryError};


// ———————————————————————————————————————————————————— QUERIES  ———————————————————————————————————————————————————— //
// —————————————————————————————————————————————————————————————————————————————————————————————————————————————————— //

pub fn query(query: &str, args: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, postgres::error::Error>
{
	let mut connection: Client = Client::connect("host=localhost user=mpzinke dbname=NetworkIPLookup", NoTls)?;
	return connection.query(query, args);
}


// ————————————————————————————————————————————————— QUERIES::GROUP ————————————————————————————————————————————————— //

pub fn SELECT_Group_by_IP_id(IP_id: i32) -> Result<Vec<String>, QueryError>
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


pub fn SELECT_Group_by_IP_address(IP_address: String) -> Result<Vec<String>, QueryError>
{
	let query_str: &str = r#"
	  SELECT "Group"."label"
	  FROM "Group-IP"
	  JOIN "IP" ON "Group-IP"."IP.id" = "IP"."id"
	  JOIN "Group" ON "Group-IP"."Group.id" = "Group"."id"
	  WHERE "IP"."address" = $1;
	  "#;

	let result: Vec<Row> = query(query_str, &[&IP_address])?;

	let mut groups: Vec<String> = vec![];
	for row in result
	{
		groups.push(row.get(0));
	}
	return Ok(groups);
}


pub fn SELECT_Group_by_IP_label(IP_label: String) -> Result<Vec<String>, QueryError>
{
	let query_str: &str = r#"
	  SELECT "Group"."label"
	  FROM "Group-IP"
	  JOIN "IP" ON "Group-IP"."IP.id" = "IP"."id"
	  JOIN "Group" ON "Group-IP"."Group.id" = "Group"."id"
	  WHERE "IP"."label" = $1;
	  "#;

	let result: Vec<Row> = query(query_str, &[&IP_label])?;

	let mut groups: Vec<String> = vec![];
	for row in result
	{
		groups.push(row.get(0));
	}
	return Ok(groups);
}


// —————————————————————————————————————————————————— QUERIES::IP  —————————————————————————————————————————————————— //

pub fn SELECT_IP_by_Network_label_AND_IP_address(Network_label: String, IP_address: String) -> Result<IP, QueryError>
{
	let query_str: &str = r#"
	  SELECT "IP"."address", "IP"."label", "IP"."is_reservation", "IP"."is_static",
	    "IP"."mac", "Network"."label" AS "Network.label", "Network"."gateway" AS "Network.gateway",
	    "Network"."netmask" AS "Network.netmask"
	  FROM "IP"
	  JOIN "Network" ON "IP"."Network.id" = "Network"."id"
	  WHERE "Network"."label" = $1
	    AND "IP"."address" = $2;
	  "#;

	let result: Vec<Row> = query(query_str, &[&Network_label, &IP_address])?;
	let groups: Vec<String> = SELECT_Group_by_IP_address(IP_address.clone())?;
	if(result.len() < 1)
	{
		let message = format!("No results found for `Network`.`label`: {}, `IP`.`address`: {}", Network_label,
		  IP_address);
		return Err(NewNotFoundError(message));
	}

	return Ok(IP::new(groups, &result[0]));
}


pub fn SELECT_IP_by_Network_label_AND_IP_label(Network_label: String, IP_label: String) -> Result<IP, QueryError>
{
	let query_str: &str = r#"
	  SELECT "IP"."address", "IP"."label", "IP"."is_reservation", "IP"."is_static",
	    "IP"."mac", "Network"."label" AS "Network.label", "Network"."gateway" AS "Network.gateway",
	    "Network"."netmask" AS "Network.netmask"
	  FROM "IP"
	  JOIN "Network" ON "IP"."Network.id" = "Network"."id"
	  WHERE "Network"."label" = $1
	    AND "IP"."label" = $2;
	  "#;

	let result: Vec<Row> = query(query_str, &[&Network_label, &IP_label])?;
	let groups: Vec<String> = SELECT_Group_by_IP_label(IP_label.clone())?;
	if(result.len() < 1)
	{
		let message = format!("No results found for `Network`.`label`: {}, `IP`.`label`: {}", Network_label, IP_label);
		return Err(NewNotFoundError(message));
	}

	return Ok(IP::new(groups, &result[0]));
}


pub fn SELECT_IPs_by_Network_label_AND_Group_label(Network_label: String, Group_label: String)
 -> Result<Vec<IP>, QueryError>
{
	let query_str: &str = r#"
	  SELECT "IP"."address", "IP"."label", "IP"."is_reservation", "IP"."is_static",
	    "IP"."mac", "Network"."label" AS "Network.label", "Network"."gateway" AS "Network.gateway",
	    "Network"."netmask" AS "Network.netmask"
	  FROM "Group-IP"
	  JOIN "IP" ON "Group-IP"."IP.id" = "IP"."id"
	  JOIN "Group" ON "Group-IP"."Group.id" = "Group"."id"
	  JOIN "Network" ON "IP"."Network.id" = "Network"."id"
	  WHERE "Network"."label" = $1
	    AND "Group"."label" = $2;
	"#;

	let result: Vec<Row> = query(query_str, &[&Network_label, &Group_label])?;
	let mut IPs: Vec<IP> = vec![];
	for row in result
	{
		let IP_label: String = row.get("label");
		let groups: Vec<String> = SELECT_Group_by_IP_label(IP_label)?;
		IPs.push(IP::new(groups, &row));
	}

	return Ok(IPs);
}



// pub fn SELECT_IP_by_Network_label_AND_IP_label(/* Network_id: i32, */ address: String) -> Result<IP, QueryError>
// {
// 	let query_str: &str = concat!(
// 	  ""
// 	);
// }
