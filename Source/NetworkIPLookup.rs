
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


mod DBInterface;


fn main()
{
	match DBInterface::query_IP_id(85)
	{
		Some(ip) =>
		{
			println!("It worked");
			println!("{}", ip.to_string());
		},
		None =>
		{
			println!("It broke");
		}
	}
}
