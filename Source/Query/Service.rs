
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.09.04                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


use sqlx::{query, PgPool, postgres::PgRow, Row};


use crate::DBTables::{Service::Service, Group::Group};
use crate::LookupError::{LookupError, NewNotFoundError};
use crate::Query::Group::{SELECT_Groups_by_Device_id, SELECT_Groups_by_Device_label};


pub async fn SELECT_Service_by_Network_id_AND_Service_label(pool: &PgPool, Network_id: i32, Service_label: &String)
  -> Result<Vec<Service>, LookupError>
{
	let query_str: &str = r#"
	  SELECT "Service"."id" AS "Service.id", "Service"."label" AS "Service.label", "Service"."port",
	    "Device"."id" AS "Device.id", "Device"."address", "Device"."label" AS "Device.label", "Device"."is_reservation",
	      "Device"."is_static", "Device"."mac",
	    "Network"."id" AS "Network.id", "Network"."auth_value", "Network"."label" AS "Network.label",
	      "Network"."gateway", "Network"."netmask"
	  FROM "Service"
	  JOIN "Device" ON "Service"."Device.id" = "Device"."id"
	  JOIN "Network" ON "Device"."Network.id" = "Network"."id"
	  WHERE "Network"."id" = $1
	    AND "Service"."label" = $2;
	"#;
	let result: Vec<PgRow> = query(query_str)
	  .bind(Network_id)
	  .bind(Service_label.clone())
	  .fetch_all(pool).await?;

	let mut services: Vec<Service> = vec![];
	for row in result
	{
		let groups: Vec<Group> = SELECT_Groups_by_Device_id(pool, row.get("Device.id")).await?;
		services.push(Service::new(groups, &row));
	}

	return Ok(services);
}


pub async fn SELECT_Device_by_Network_label_AND_Service_label(pool: &PgPool, Network_label: &String,
  Service_label: &String) -> Result<Vec<Service>, LookupError>
{
	let query_str: &str = r#"
	  SELECT "Service"."id" AS "Service.id", "Service"."label" AS "Service.label", "Service"."port",
	    "Device"."id" AS "Device.id", "Device"."address", "Device"."label" AS "Device.label", "Device"."is_reservation",
	      "Device"."is_static", "Device"."mac",
	    "Network"."id" AS "Network.id", "Network"."auth_value", "Network"."label" AS "Network.label",
	      "Network"."gateway", "Network"."netmask"
	  FROM "Service"
	  JOIN "Device" ON "Service"."Device.id" = "Device"."id"
	  JOIN "Network" ON "Device"."Network.id" = "Network"."id"
	  WHERE "Network"."label" = $1
	    AND "Service"."label" = $2;
	"#;
	let result: Vec<PgRow> = query(query_str)
	  .bind(Network_label.clone())
	  .bind(Service_label.clone())
	  .fetch_all(pool).await?;

	let mut services: Vec<Service> = vec![];
	for row in result
	{
		let groups: Vec<Group> = SELECT_Groups_by_Device_id(pool, row.get("Device.id")).await?;
		services.push(Service::new(groups, &row));
	}

	return Ok(services);
}
