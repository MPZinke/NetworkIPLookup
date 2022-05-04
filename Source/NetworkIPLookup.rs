
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


mod Queries;
mod QueryError;
mod IP;


fn main()
{
	match Queries::query_IP_by_id(19)
	{
		Ok(ip) =>
		{
			println!("It worked");
			println!("{}", ip.to_string());
		},
		Err(error) =>
		{
			println!("It broke");
			println!("{}", error);
		}
	}
}
