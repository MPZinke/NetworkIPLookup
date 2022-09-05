
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


use sqlx::{postgres::PgRow, Row};
use serde::Serialize;


use crate::DBTables::{Device::Device, Group::Group};


#[derive(Debug, Serialize)]
pub struct Service
{
	pub id: i32,
	pub label: String,
	pub port: i16,
	pub device: Device
}


impl Service
{
	pub fn new(groups: Vec<Group>, row: &PgRow) -> Service
	{
		return Service{id: row.get("Service.id"), label: row.get("Service.label"), port: row.get("port"),
		  device: Device::new(groups, row)};
	}
}
