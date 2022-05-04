
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.05.04                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


use postgres;
use std;


// ——————————————————————————————————————————————————— ERROR ENUM ——————————————————————————————————————————————————— //

#[derive(Debug)]
pub enum QueryError
{
    Postgres(postgres::error::Error),
    NotFound(std::io::Error),
}


impl std::fmt::Display for QueryError
{
    fn fmt(&self, format: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match(self)
        {
            QueryError::Postgres(error) =>
                write!(format, "{}", error),
            QueryError::NotFound(error) =>
                write!(format, "{}", error),
        }
    }
}


impl From<postgres::error::Error> for QueryError
{
    fn from(err: postgres::error::Error) -> Self
    {
        QueryError::Postgres(err)
    }
}

impl From<std::io::Error> for QueryError
{
    fn from(err: std::io::Error) -> Self
    {
        QueryError::NotFound(err)
    }
}