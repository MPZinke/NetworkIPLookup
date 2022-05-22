
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
use crate::Query::Queries::{Network::SELECT_Network_by_id, Device::SELECT_Device_by_Network_id_AND_Device_address};
use crate::Query::QueryError::QueryError as Error;
use crate::UnknownLookup::{lookup_device_on_network, SearchType::{DeviceAttributeSearch, NetworkSearch}};


// `/api/v1.0/network/id/{network_id}/device/address`
pub async fn index() -> HttpResponse
{
	let body: &str = r#"
	{
		"/api/v1.0/network/id/{network_id}/device/address/{device_address}": "Get a device by device address and network id"
	}
	"#;
	return HttpResponse::Ok().insert_header(ContentType::json()).body(body);
}


// `/api/v1.0/network/id/{network_id}/device/address/{device_address}`
pub async fn address(auth: BearerAuth, path: web::Path<(i32, String)>, pool: web::Data<PgPool>) -> HttpResponse
{
	if(env!("NETWORKLOOKUP_BEARERTOKEN") != auth.token())
	{
		return HttpResponse::Unauthorized().insert_header(ContentType::json()).body(r#"{"error": "Unauthorized"}"#);
	}

	let (Network_id, Device_address) = path.into_inner();
	let query_response: Result<Device, Error> = SELECT_Device_by_Network_id_AND_Device_address(pool.as_ref(),
	  Network_id, &Device_address).await;

	// If not found in DB, try to find Device address by scanning network
	if(query_NotFound(&query_response))
	{
		// Check and make sure Network exists
		let network_result: Result<Network, Error> = SELECT_Network_by_id(pool.as_ref(), Network_id).await;
		let network: NetworkSearch = match(network_result)
		{
			Ok(network) => NetworkSearch::id(network),
			// Allow both NotFound & DB Errors to reach top level. If DB goes wrong, it needs to be visible.
			Err(_) => return query_to_response(network_result)
		};

		let address_attribute = DeviceAttributeSearch::address(Device_address.clone());
		let Device_lookup_result: Result<Device, Error> = lookup_device_on_network(&address_attribute, &network).await;
		return query_to_response(Device_lookup_result);
	}

	return query_to_response(query_response);
}
