use diesel::{prelude::*};

use crate::{
    errors::*,
    models::{groups::*},
    schema::{groups},
};

pub fn check_group_id(
    id: i32,
    c: &diesel::PgConnection
) -> Result<(), Error> {
    let ret = groups::table
        .filter(groups::id.eq(id))
        .execute(c);
    match ret {
        Ok(o) => match o {
                1 => Ok(()),
                0 => Err(Error::NotFound(format!("Group id:{id} is not found").to_string())),
                _ => Err(Error::BadRequest("???".to_string())),
            },
        Err(e) => Err(Error::Internal(e.to_string())),
    }
}

pub fn create(
    group: &NewGroup,
    c: &diesel::PgConnection
) -> Result<Group, Error> {
    let ret = diesel::insert_into(groups::table)
        .values(group)
        .get_result(c);
    match ret {
        Ok(o) => Ok(o),
        Err(e) => Err(Error::Internal(e.to_string())),
    }
}

pub fn destroy(
    id: i32,
    c: &diesel::PgConnection
) -> Result<(), Error> {
    let ret = diesel::delete(
        groups::table.filter(groups::id.eq(id)))
        .execute(c);
    match ret {
        Ok(o) => {
            match o {
                1 => Ok(()),
                0 => Err(Error::NotFound(format!("Group id:{id} is not found"))),
                _ => Err(Error::BadRequest("???".to_string())),
            }
        }
        Err(e) => Err(Error::Internal(e.to_string())),
    }
}