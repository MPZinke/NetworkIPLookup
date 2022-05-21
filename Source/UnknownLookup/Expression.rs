
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.05.21                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


pub enum Expression
{
	ip(String),
	label(String),
	mac(String)
}


impl std::fmt::Debug for Expression
{
    fn fmt(&self, format: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match(self)
        {
        	Expression::ip(value) => write!(format, "{}", value),
			Expression::label(value) => write!(format, "{}", value),
			Expression::mac(value) => write!(format, "{}", value)
        }
    }
}


impl std::fmt::Display for Expression
{
    fn fmt(&self, format: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match(self)
        {
        	Expression::ip(_) => write!(format, "ip"),
			Expression::label(_) => write!(format, "label"),
			Expression::mac(_) => write!(format, "mac")
        }
    }
}