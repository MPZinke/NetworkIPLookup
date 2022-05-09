
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


use actix_web::{http::header::ContentType, HttpResponse};
use serde_json;
use serde::Serialize;
use sqlx::{query, PgPool, postgres::PgRow, Row};


use crate::Group::Group;
use crate::IP::IP;
use crate::Network::Network;
use crate::QueryError::{NewNotFoundError, QueryError};


pub fn query_to_response<T: Serialize>(generic_query: Result<T, QueryError>) -> HttpResponse
{
	let response_generic: T = match(generic_query)
	{
		Ok(response_generic) => response_generic,
		Err(error) =>
		{
			let response = match(error)
			{
				QueryError::Postgres(_) => HttpResponse::InternalServerError,
				QueryError::NotFound(_) => HttpResponse::NotFound
			};
			let error_message = format!("{{\"error\": \"{}\"}}", error);
			return response().insert_header(ContentType::json()).body(error_message);
		}
	};

	match(serde_json::to_string(&response_generic))
	{
		Ok(response_body) => return HttpResponse::Ok().insert_header(ContentType::json()).body(response_body),
		Err(error) => 
		{
			let error_message = format!("{{\"error\": \"{}\"}}", error);
			return HttpResponse::InternalServerError().insert_header(ContentType::json()).body(error_message);
		}
	}
}


// ———————————————————————————————————————————————————— QUERIES  ———————————————————————————————————————————————————— //
// —————————————————————————————————————————————————————————————————————————————————————————————————————————————————— //

// ———————————————————————————————————————————————— QUERIES::NETWORK ———————————————————————————————————————————————— //

pub async fn SELECT_Networks(pool: &PgPool) -> Result<Vec<Network>, QueryError>
{
	let query_str: &str = r#"
	  SELECT "label", "gateway", "netmask"
	  FROM "Network";
	"#;
	let result: Vec<PgRow> = query(query_str).fetch_all(pool).await?;

	let mut networks: Vec<Network> = vec![];
	for row in result
	{
		networks.push(Network::new(row.get("label"), row.get("gateway"), row.get("netmask")));
	}
	return Ok(networks);
}


pub async fn SELECT_Network_by_id(pool: &PgPool, id: i32) -> Result<Network, QueryError>
{
	let query_str: &str = r#"
	  SELECT "label", "gateway", "netmask"
	  FROM "Network"
	  WHERE "id" = $1;
	"#;
	let result: Vec<PgRow> = query(query_str).bind(id).fetch_all(pool).await?;
	if(result.len() < 1)
	{
		return Err(NewNotFoundError(format!("No results found for `Network`.`id`: '{}'", id)));
	}

	return Ok(Network::new(result[0].get("label"), result[0].get("gateway"), result[0].get("netmask")));
}


pub async fn SELECT_Network_by_label(pool: &PgPool, label: String) -> Result<Network, QueryError>
{
	let query_str: &str = r#"
	  SELECT "label", "gateway", "netmask"
	  FROM "Network"
	  WHERE "label" = $1;
	"#;
	let result: Vec<PgRow> = query(query_str).bind(label.clone()).fetch_all(pool).await?;
	if(result.len() < 1)
	{
		return Err(NewNotFoundError(format!("No results found for `Network`.`label`: '{}'", label)));
	}

	return Ok(Network::new(result[0].get("label"), result[0].get("gateway"), result[0].get("netmask")));
}


// ————————————————————————————————————————————————— QUERIES::GROUP ————————————————————————————————————————————————— //

pub async fn SELECT_Groups(pool: &PgPool) -> Result<Vec<Group>, QueryError>
{
	let query_str: &str = r#"
	  SELECT "id", "label"
	  FROM "Group";
	"#;
	let result: Vec<PgRow> = query(query_str).fetch_all(pool).await?;

	let mut groups: Vec<Group> = vec![];
	for row in result
	{
		groups.push(Group::new(&row));
	}
	return Ok(groups);
}


//TODO: Convert to object
pub async fn SELECT_Group_by_id(pool: &PgPool, id: i32) -> Result<Group, QueryError>
{
	let query_str: &str = r#"
	  SELECT "id", "label"
	  FROM "Group"
	  WHERE "id" = $1;
	"#;
	let result: Vec<PgRow> = query(query_str).bind(id).fetch_all(pool).await?;

	if(result.len() < 1)
	{
		return Err(NewNotFoundError(format!("No results found for `Group`.`id`: '{}'", id)));
	}
	return Ok(Group::new(&result[0]));
}


pub async fn SELECT_Group_by_label(pool: &PgPool, label: String) -> Result<Group, QueryError>
{
	let query_str: &str = r#"
	  SELECT "id", "label"
	  FROM "Group"
	  WHERE "label" = $1;
	"#;
	let result: Vec<PgRow> = query(query_str).bind(label.clone()).fetch_all(pool).await?;

	if(result.len() < 1)
	{
		return Err(NewNotFoundError(format!("No results found for `Group`.`label`: '{}'", label)));
	}
	return Ok(Group::new(&result[0]));
}


// ————————————————————————————————————————————— QUERIES::GROUP::FOR IP ————————————————————————————————————————————— //

pub async fn SELECT_Groups_by_IP_id(pool: &PgPool, IP_id: i32) -> Result<Vec<String>, QueryError>
{
	let query_str: &str = concat!(
	  "SELECT \"Group\".\"label\"\n",
	  "FROM \"Group-IP\"\n",
	  "JOIN \"Group\" ON \"Group-IP\".\"Group.id\" = \"Group\".\"id\"\n",
	  "WHERE \"Group-IP\".\"IP.id\" = $1;\n"
	);

	let result: Vec<PgRow> = query(query_str).bind(IP_id).fetch_all(pool).await?;

	let mut groups: Vec<String> = vec![];
	for row in result
	{
		groups.push(row.get(0));
	}
	return Ok(groups);
}


pub async fn SELECT_Groups_by_IP_address(pool: &PgPool, IP_address: String) -> Result<Vec<String>, QueryError>
{
	let query_str: &str = r#"
	  SELECT "Group"."label"
	  FROM "Group-IP"
	  JOIN "IP" ON "Group-IP"."IP.id" = "IP"."id"
	  JOIN "Group" ON "Group-IP"."Group.id" = "Group"."id"
	  WHERE "IP"."address" = $1;
	"#;
	let result: Vec<PgRow> = query(query_str).bind(IP_address).fetch_all(pool).await?;

	let mut groups: Vec<String> = vec![];
	for row in result
	{
		groups.push(row.get(0));
	}
	return Ok(groups);
}


pub async fn SELECT_Groups_by_IP_label(pool: &PgPool, IP_label: String) -> Result<Vec<String>, QueryError>
{
	let query_str: &str = r#"
	  SELECT "Group"."label"
	  FROM "Group-IP"
	  JOIN "IP" ON "Group-IP"."IP.id" = "IP"."id"
	  JOIN "Group" ON "Group-IP"."Group.id" = "Group"."id"
	  WHERE "IP"."label" = $1;
	"#;
	let result: Vec<PgRow> = query(query_str).bind(IP_label).fetch_all(pool).await?;

	let mut groups: Vec<String> = vec![];
	for row in result
	{
		groups.push(row.get(0));
	}
	return Ok(groups);
}


// —————————————————————————————————————————————————— QUERIES::IP  —————————————————————————————————————————————————— //

pub async fn SELECT_IP_by_Network_id_AND_IP_address(pool: &PgPool, Network_id: i32, IP_address: String)
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
	  .bind(Network_id.clone())
	  .bind(IP_address.clone())
	  .fetch_all(pool).await?;

	if(result.len() < 1)
	{
		let message = format!("No results found for `Network`.`label`: '{}', `IP`.`address`: '{}'", Network_id,
		  IP_address);
		return Err(NewNotFoundError(message));
	}

	let groups: Vec<String> = SELECT_Groups_by_IP_address(pool, IP_address).await?;
	return Ok(IP::new(groups, &result[0]));
}


pub async fn SELECT_IP_by_Network_label_AND_IP_address(pool: &PgPool, Network_label: String, IP_address: String)
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
		let message = format!("No results found for `Network`.`label`: '{}', `IP`.`address`: '{}'", Network_label,
		  IP_address);
		return Err(NewNotFoundError(message));
	}

	let groups: Vec<String> = SELECT_Groups_by_IP_address(pool, IP_address.clone()).await?;
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
		let message = format!("No results found for `Network`.`id`: '{}', `IP`.`id`: '{}'", Network_id, IP_id);
		return Err(NewNotFoundError(message));
	}

	let groups: Vec<String> = SELECT_Groups_by_IP_id(pool, IP_id).await?;
	return Ok(IP::new(groups, &result[0]));
}


pub async fn SELECT_IP_by_Network_id_AND_IP_label(pool: &PgPool, Network_id: i32, IP_label: String)
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
		let message = format!("No results found for `Network`.`id`: '{}', `IP`.`label`: '{}'", Network_id, IP_label);
		return Err(NewNotFoundError(message));
	}

	let groups: Vec<String> = SELECT_Groups_by_IP_label(pool, IP_label.clone()).await?;
	return Ok(IP::new(groups, &result[0]));
}


pub async fn SELECT_IP_by_Network_label_AND_IP_id(pool: &PgPool, Network_label: String, IP_id: i32)
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
		let message = format!("No results found for `Network`.`label`: '{}', `IP`.`id`: '{}'", Network_label, IP_id);
		return Err(NewNotFoundError(message));
	}

	let groups: Vec<String> = SELECT_Groups_by_IP_id(pool, IP_id).await?;
	return Ok(IP::new(groups, &result[0]));
}


pub async fn SELECT_IP_by_Network_label_AND_IP_label(pool: &PgPool, Network_label: String, IP_label: String)
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
		let message = format!("No results found for `Network`.`label`: '{}', `IP`.`label`: '{}'", Network_label,
		  IP_label);
		return Err(NewNotFoundError(message));
	}

	let groups: Vec<String> = SELECT_Groups_by_IP_label(pool, IP_label.clone()).await?;
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
		let groups: Vec<String> = SELECT_Groups_by_IP_label(pool, IP_label).await?;
		IPs.push(IP::new(groups, &row));
	}

	return Ok(IPs);
}


pub async fn SELECT_IPs_by_Network_label(pool: &PgPool, Network_label: String) -> Result<Vec<IP>, QueryError>
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
		let groups: Vec<String> = SELECT_Groups_by_IP_label(pool, IP_label).await?;
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
		let groups: Vec<String> = SELECT_Groups_by_IP_label(pool, IP_label).await?;
		IPs.push(IP::new(groups, &row));
	}

	return Ok(IPs);
}


pub async fn SELECT_IPs_by_Network_id_AND_Group_label(pool: &PgPool, Network_id: i32, Group_label: String)
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
		let groups: Vec<String> = SELECT_Groups_by_IP_label(pool, IP_label).await?;
		IPs.push(IP::new(groups, &row));
	}

	return Ok(IPs);
}


pub async fn SELECT_IPs_by_Network_label_AND_Group_id(pool: &PgPool, Network_label: String, Group_id: i32)
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
		let groups: Vec<String> = SELECT_Groups_by_IP_label(pool, IP_label).await?;
		IPs.push(IP::new(groups, &row));
	}

	return Ok(IPs);
}


pub async fn SELECT_IPs_by_Network_label_AND_Group_label(pool: &PgPool, Network_label: String, Group_label: String)
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
		let groups: Vec<String> = SELECT_Groups_by_IP_label(pool, IP_label).await?;
		IPs.push(IP::new(groups, &row));
	}

	return Ok(IPs);
}
