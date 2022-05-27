
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.05.07                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


use actix_web::{http::header::ContentType, HttpResponse, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sqlx::postgres::PgPool;


use crate::DBTables::Device::Device;
use crate::LookupError::LookupError;
use crate::Query::{query_to_response, Device::SELECT_Device_by_Network_label_AND_Device_id};


// `/api/v1.0/network/label/{network_label}/device/id`
pub async fn index() -> HttpResponse
{
	let body: &str = r#"
	{
		"/api/v1.0/network/label/{network_label}/device/id/{device_id}": "Get a device by device id and network label"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/network/label/{network_label}/device/id/{device_id}`
pub async fn id(auth: BearerAuth, path: web::Path<(String, i32)>, pool: web::Data<PgPool>) -> HttpResponse
{
	if(env!("NETWORKLOOKUP_BEARERTOKEN") != auth.token())
	{
		return HttpResponse::Unauthorized().insert_header(ContentType::json()).body(r#"{"error": "Unauthorized"}"#);
	}

	let (Network_label, Device_id) = path.into_inner();
	let query_response: Result<Device, LookupError> = SELECT_Device_by_Network_label_AND_Device_id(pool.as_ref(),
	  &Network_label, Device_id).await;
	return query_to_response(query_response);
}
