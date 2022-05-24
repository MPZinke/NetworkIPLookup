
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


use crate::DBTables::{Device::Device, Network::Network};
use crate::Query::{query_NotFound, query_to_response};
use crate::Query::{Network::SELECT_Network_by_label, Device::SELECT_Device_by_Network_label_AND_Device_label};
use crate::Query::QueryError;
use crate::SearchType::{DeviceAttributeSearch, NetworkSearch};
use crate::UnknownLookup::Networks::lookup_device;


// `/api/v1.0/network/label/{network_label}/device/label`
pub async fn index() -> HttpResponse
{
	let body: &str = r#"
	{
		"/api/v1.0/network/label/{network_label}/device/label/{device_label}": "Get a device by device label and network label"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/network/id/{network_id}/device/label/{device_label}`
pub async fn label(auth: BearerAuth, path: web::Path<(String, String)>, pool: web::Data<PgPool>) -> HttpResponse
{
	if(env!("NETWORKLOOKUP_BEARERTOKEN") != auth.token())
	{
		return HttpResponse::Unauthorized().insert_header(ContentType::json()).body(r#"{"error": "Unauthorized"}"#);
	}

	let (Network_label, Device_label) = path.into_inner();
	let device_result: Result<Device, QueryError> = SELECT_Device_by_Network_label_AND_Device_label(pool.as_ref(),
	  &Network_label, &Device_label).await;

	// If not found in DB, try to find Device label by scanning network
	if(query_NotFound(&device_result))
	{
		// Check and make sure Network exists
		let network_result: Result<Network, QueryError> = SELECT_Network_by_label(pool.as_ref(), &Network_label).await;
		let network_search: NetworkSearch = match(network_result)
		{
			Ok(network_search) => NetworkSearch::label(network_search),
			Err(_) => return query_to_response(network_result)  // åœ
		};

		if(network_search.network().auth_value.is_some())
		{
			let label_attribute: DeviceAttributeSearch = DeviceAttributeSearch::label(Device_label.clone());
			let lookup_result: Result<Device, QueryError> = lookup_device(&label_attribute, &network_search).await;
			return query_to_response(lookup_result);
		}
	}

	return query_to_response(device_result);
}
