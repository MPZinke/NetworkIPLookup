
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


use actix_web::{http::header::ContentType, HttpResponse, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sqlx::postgres::PgPool;


use crate::DBTables::{Device::Device, Network::Network};
use crate::Query::{query_NotFound, query_to_response};
use crate::Query::
{
	Network::SELECT_Network_by_label,
	Device::SELECT_Device_by_Network_label_AND_Device_address
};
use crate::Query::QueryError;
use crate::SearchType::{DeviceAttributeSearch, NetworkSearch};
use crate::UnknownLookup::Networks::lookup_device;


// `/api/v1.0/network/label/{network_label}/device/address`
pub async fn index() -> HttpResponse
{
	let body: &str = r#"
	{
		"/api/v1.0/network/label/{network_label}/device/address/{device_address}": "Get a device by device address and network label"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/network/label/{network_label}/device/address/{device_address}`
pub async fn address(auth: BearerAuth, path: web::Path<(String, String)>, pool: web::Data<PgPool>) -> HttpResponse
{
	if(env!("NETWORKLOOKUP_BEARERTOKEN") != auth.token())
	{
		return HttpResponse::Unauthorized().insert_header(ContentType::json()).body(r#"{"error": "Unauthorized"}"#);
	}

	let (Network_label, Device_address) = path.into_inner();
	let device_result: Result<Device, QueryError> = SELECT_Device_by_Network_label_AND_Device_address(pool.as_ref(),
	  &Network_label, &Device_address).await;

	// If not found in DB, try to find Device address by scanning network
	if(query_NotFound(&device_result))
	{
		// Check and make sure Network exists
		let network_result: Result<Network, QueryError> = SELECT_Network_by_label(pool.as_ref(), &Network_label).await;
		let network_search: NetworkSearch = match(network_result)
		{
			Ok(network_search) => NetworkSearch::label(network_search),
			Err(_) => return query_to_response(network_result)  // show both errors: DB errors need to be visible too
		};

		if(network_search.network().auth_value.is_some())
		{
			let address_attribute: DeviceAttributeSearch = DeviceAttributeSearch::address(Device_address.clone());
			let lookup_result: Result<Device, QueryError> = lookup_device(&address_attribute, &network_search).await;
			return query_to_response(lookup_result);
		}
	}

	return query_to_response(device_result);
}
