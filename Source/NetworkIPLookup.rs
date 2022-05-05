
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.04.29                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


#![allow(non_snake_case)]
#![allow(unused_parens)]
#![allow(non_camel_case_types)]


mod IP;
mod Network;
mod Queries;
mod QueryError;
mod Responses;


use crate::Queries::
{
	SELECT_Networks,
	SELECT_Groups,
	SELECT_IP_by_Network_label_AND_IP_address,
	SELECT_IP_by_Network_label_AND_IP_label,
	SELECT_IPs_by_Network_label,
	SELECT_IPs_by_Network_label_AND_Group_label
};
use crate::Responses::generic_query_to_response_JSON;


fn query_test() -> ()
{
	{
		let query_response = SELECT_Networks();
		let response_body = generic_query_to_response_JSON(query_response);
		println!("{}", response_body);
	}

	{
		let query_response = SELECT_Groups();
		let response_body = generic_query_to_response_JSON(query_response);
		println!("{}", response_body);
	}

	{
		let Network_label: String = "Home".to_string();
		let IP_address: String = "192.168.1.21".to_string();
		let query_response = SELECT_IP_by_Network_label_AND_IP_address(Network_label, IP_address);
		let response_body = generic_query_to_response_JSON(query_response);
		println!("{}", response_body);
	}

	{
		let Network_label: String = "Home".to_string();
		let IP_label: String = "Bedroom-Curtain".to_string();
		let query_response = SELECT_IP_by_Network_label_AND_IP_label(Network_label, IP_label);
		let response_body = generic_query_to_response_JSON(query_response);
		println!("{}", response_body);
	}

	{
		let Network_label: String = "Home".to_string();
		let query_response = SELECT_IPs_by_Network_label(Network_label);
		let responses_body = generic_query_to_response_JSON(query_response);
		println!("{}", responses_body);
	}

	{
		let Network_label: String = "Home".to_string();
		let Group_label: String = "Livingroom".to_string();
		let query_response = SELECT_IPs_by_Network_label_AND_Group_label(Network_label, Group_label);
		let responses_body = generic_query_to_response_JSON(query_response);
		println!("{}", responses_body);
	}
}



fn main()
{
	query_test();
}
