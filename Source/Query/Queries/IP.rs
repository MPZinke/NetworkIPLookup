
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.05.09                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


use sqlx::{query, PgPool, postgres::PgRow, Row};


use crate::DBTables::IP::IP;
use crate::Query::QueryError::{NewNotFoundError, QueryError};
use crate::Query::Queries::Group::{SELECT_Groups_by_IP_address, SELECT_Groups_by_IP_id, SELECT_Groups_by_IP_label};


pub async fn SELECT_IP_by_Network_id_AND_IP_address(pool: &PgPool, Network_id: i32, IP_address: &String)
  -> Result<IP, QueryError>
{
	let query_str: &str = r#"
	  SELECT "IP"."address", "IP"."label", "IP"."is_reservation", "IP"."is_static",
	    "IP"."mac", "Network"."label" AS "Network.label", "Network"."gateway" AS "Network.gateway",
	    "Network"."netmask" AS "Network.netmask"
	  FROM "IP"
	  JOIN "Network" ON "IP"."Network.id" = "Network"."id"
	  WHERE "Network"."id" = $1
	    AND "IP"."address" = $2;
	"#;
	let result: Vec<PgRow> = query(query_str)
	  .bind(Network_id)
	  .bind(IP_address.clone())
	  .fetch_all(pool).await?;

	if(result.len() < 1)
	{
		let message: String = format!("No results found for `Network`.`id`: '{}', `IP`.`address`: '{}'", Network_id,
		  IP_address);
		return Err(NewNotFoundError(message));
	}

	let groups: Vec<String> = SELECT_Groups_by_IP_address(pool, &IP_address).await?;
	return Ok(IP::new(groups, &result[0]));
}


pub async fn SELECT_IP_by_Network_label_AND_IP_address(pool: &PgPool, Network_label: &String, IP_address: &String)
  -> Result<IP, QueryError>
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
	let result: Vec<PgRow> = query(query_str)
	  .bind(Network_label.clone())
	  .bind(IP_address.clone())
	  .fetch_all(pool).await?;
	if(result.len() < 1)
	{
		let message: String = format!("No results found for `Network`.`label`: '{}', `IP`.`address`: '{}'",
		  Network_label, IP_address);
		return Err(NewNotFoundError(message));
	}

	let groups: Vec<String> = SELECT_Groups_by_IP_address(pool, &IP_address).await?;
	return Ok(IP::new(groups, &result[0]));
}


pub async fn SELECT_IP_by_Network_id_AND_IP_id(pool: &PgPool, Network_id: i32, IP_id: i32)
  -> Result<IP, QueryError>
{
	let query_str: &str = r#"
	  SELECT "IP"."address", "IP"."label", "IP"."is_reservation", "IP"."is_static",
	    "IP"."mac", "Network"."label" AS "Network.label", "Network"."gateway" AS "Network.gateway",
	    "Network"."netmask" AS "Network.netmask"
	  FROM "IP"
	  JOIN "Network" ON "IP"."Network.id" = "Network"."id"
	  WHERE "Network"."id" = $1
	    AND "IP"."id" = $2;
	"#;
	let result: Vec<PgRow> = query(query_str)
	  .bind(Network_id.clone())
	  .bind(IP_id.clone())
	  .fetch_all(pool).await?;

	if(result.len() < 1)
	{
		let message: String = format!("No results found for `Network`.`id`: '{}', `IP`.`id`: '{}'", Network_id, IP_id);
		return Err(NewNotFoundError(message));
	}

	let groups: Vec<String> = SELECT_Groups_by_IP_id(pool, IP_id).await?;
	return Ok(IP::new(groups, &result[0]));
}


pub async fn SELECT_IP_by_Network_id_AND_IP_label(pool: &PgPool, Network_id: i32, IP_label: &String)
  -> Result<IP, QueryError>
{
	let query_str: &str = r#"
	  SELECT "IP"."address", "IP"."label", "IP"."is_reservation", "IP"."is_static",
	    "IP"."mac", "Network"."label" AS "Network.label", "Network"."gateway" AS "Network.gateway",
	    "Network"."netmask" AS "Network.netmask"
	  FROM "IP"
	  JOIN "Network" ON "IP"."Network.id" = "Network"."id"
	  WHERE "Network"."id" = $1
	    AND "IP"."label" = $2;
	"#;
	let result: Vec<PgRow> = query(query_str)
	  .bind(Network_id.clone())
	  .bind(IP_label.clone())
	  .fetch_all(pool).await?;

	if(result.len() < 1)
	{
		let message: String = format!("No results found for `Network`.`id`: '{}', `IP`.`label`: '{}'", Network_id,
		  IP_label);
		return Err(NewNotFoundError(message));
	}

	let groups: Vec<String> = SELECT_Groups_by_IP_label(pool, &IP_label).await?;
	return Ok(IP::new(groups, &result[0]));
}


pub async fn SELECT_IP_by_Network_label_AND_IP_id(pool: &PgPool, Network_label: &String, IP_id: i32)
  -> Result<IP, QueryError>
{
	let query_str: &str = r#"
	  SELECT "IP"."address", "IP"."label", "IP"."is_reservation", "IP"."is_static",
	    "IP"."mac", "Network"."label" AS "Network.label", "Network"."gateway" AS "Network.gateway",
	    "Network"."netmask" AS "Network.netmask"
	  FROM "IP"
	  JOIN "Network" ON "IP"."Network.id" = "Network"."id"
	  WHERE "Network"."label" = $1
	    AND "IP"."id" = $2;
	"#;
	let result: Vec<PgRow> = query(query_str)
	  .bind(Network_label.clone())
	  .bind(IP_id.clone())
	  .fetch_all(pool).await?;

	if(result.len() < 1)
	{
		let message: String = format!("No results found for `Network`.`label`: '{}', `IP`.`id`: '{}'", Network_label,
		  IP_id);
		return Err(NewNotFoundError(message));
	}

	let groups: Vec<String> = SELECT_Groups_by_IP_id(pool, IP_id).await?;
	return Ok(IP::new(groups, &result[0]));
}


pub async fn SELECT_IP_by_Network_label_AND_IP_label(pool: &PgPool, Network_label: &String, IP_label: &String)
  -> Result<IP, QueryError>
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
	let result: Vec<PgRow> = query(query_str)
	  .bind(Network_label.clone())
	  .bind(IP_label.clone())
	  .fetch_all(pool).await?;

	if(result.len() < 1)
	{
		let message: String = format!("No results found for `Network`.`label`: '{}', `IP`.`label`: '{}'", Network_label,
		  IP_label);
		return Err(NewNotFoundError(message));
	}

	let groups: Vec<String> = SELECT_Groups_by_IP_label(pool, &IP_label).await?;
	return Ok(IP::new(groups, &result[0]));
}


pub async fn SELECT_IPs_by_Network_id(pool: &PgPool, Network_id: i32) -> Result<Vec<IP>, QueryError>
{
	let query_str: &str = r#"
	  SELECT "IP"."address", "IP"."label", "IP"."is_reservation", "IP"."is_static",
	    "IP"."mac", "Network"."label" AS "Network.label", "Network"."gateway" AS "Network.gateway",
	    "Network"."netmask" AS "Network.netmask"
	  FROM "Group-IP"
	  JOIN "IP" ON "Group-IP"."IP.id" = "IP"."id"
	  JOIN "Group" ON "Group-IP"."Group.id" = "Group"."id"
	  JOIN "Network" ON "IP"."Network.id" = "Network"."id"
	  WHERE "Network"."id" = $1;
	"#;
	let result: Vec<PgRow> = query(query_str).bind(Network_id).fetch_all(pool).await?;
	let mut IPs: Vec<IP> = vec![];
	for row in result
	{
		let IP_label: String = row.get("label");
		let groups: Vec<String> = SELECT_Groups_by_IP_label(pool, &IP_label).await?;
		IPs.push(IP::new(groups, &row));
	}

	return Ok(IPs);
}


pub async fn SELECT_IPs_by_Network_label(pool: &PgPool, Network_label: &String) -> Result<Vec<IP>, QueryError>
{
	let query_str: &str = r#"
	  SELECT "IP"."address", "IP"."label", "IP"."is_reservation", "IP"."is_static",
	    "IP"."mac", "Network"."label" AS "Network.label", "Network"."gateway" AS "Network.gateway",
	    "Network"."netmask" AS "Network.netmask"
	  FROM "Group-IP"
	  JOIN "IP" ON "Group-IP"."IP.id" = "IP"."id"
	  JOIN "Group" ON "Group-IP"."Group.id" = "Group"."id"
	  JOIN "Network" ON "IP"."Network.id" = "Network"."id"
	  WHERE "Network"."label" = $1;
	"#;
	let result: Vec<PgRow> = query(query_str).bind(Network_label.clone()).fetch_all(pool).await?;
	let mut IPs: Vec<IP> = vec![];
	for row in result
	{
		let IP_label: String = row.get("label");
		let groups: Vec<String> = SELECT_Groups_by_IP_label(pool, &IP_label).await?;
		IPs.push(IP::new(groups, &row));
	}

	return Ok(IPs);
}


pub async fn SELECT_IPs_by_Network_id_AND_Group_id(pool: &PgPool, Network_id: i32, Group_id: i32)
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
	  WHERE "Network"."id" = $1
	    AND "Group"."id" = $2;
	"#;
	let result: Vec<PgRow> = query(query_str)
	  .bind(Network_id.clone())
	  .bind(Group_id.clone())
	  .fetch_all(pool).await?;

	let mut IPs: Vec<IP> = vec![];
	for row in result
	{
		let IP_label: String = row.get("label");
		let groups: Vec<String> = SELECT_Groups_by_IP_label(pool, &IP_label).await?;
		IPs.push(IP::new(groups, &row));
	}

	return Ok(IPs);
}


pub async fn SELECT_IPs_by_Network_id_AND_Group_label(pool: &PgPool, Network_id: i32, Group_label: &String)
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
	  WHERE "Network"."id" = $1
	    AND "Group"."label" = $2;
	"#;
	let result: Vec<PgRow> = query(query_str)
	  .bind(Network_id.clone())
	  .bind(Group_label.clone())
	  .fetch_all(pool).await?;

	let mut IPs: Vec<IP> = vec![];
	for row in result
	{
		let IP_label: String = row.get("label");
		let groups: Vec<String> = SELECT_Groups_by_IP_label(pool, &IP_label).await?;
		IPs.push(IP::new(groups, &row));
	}

	return Ok(IPs);
}


pub async fn SELECT_IPs_by_Network_label_AND_Group_id(pool: &PgPool, Network_label: &String, Group_id: i32)
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
	    AND "Group"."id" = $2;
	"#;
	let result: Vec<PgRow> = query(query_str)
	  .bind(Network_label.clone())
	  .bind(Group_id.clone())
	  .fetch_all(pool).await?;

	let mut IPs: Vec<IP> = vec![];
	for row in result
	{
		let IP_label: String = row.get("label");
		let groups: Vec<String> = SELECT_Groups_by_IP_label(pool, &IP_label).await?;
		IPs.push(IP::new(groups, &row));
	}

	return Ok(IPs);
}


pub async fn SELECT_IPs_by_Network_label_AND_Group_label(pool: &PgPool, Network_label: &String, Group_label: &String)
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
	let result: Vec<PgRow> = query(query_str)
	  .bind(Network_label.clone())
	  .bind(Group_label.clone())
	  .fetch_all(pool).await?;

	let mut IPs: Vec<IP> = vec![];
	for row in result
	{
		let IP_label: String = row.get("label");
		let groups: Vec<String> = SELECT_Groups_by_IP_label(pool, &IP_label).await?;
		IPs.push(IP::new(groups, &row));
	}

	return Ok(IPs);
}
