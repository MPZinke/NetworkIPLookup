
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


use crate::DBTables::Group::Group;
use crate::Query::QueryError::{NewNotFoundError, QueryError};


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


// —————————————————————————————————————————————————— GROUP::BY IP —————————————————————————————————————————————————— //

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
