use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::{Uuid};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Cx<T> {
    create_at: usize,
    expire_at: usize,
    from_key: T,
    is_complete: bool,
    is_ephemeral: bool,
    is_expire: bool,
    is_sign: bool,
    owner: Uuid,
    sign_at: usize,
    sign_from: Uuid,
    sign_to: Uuid,
    to_key: T,
    uuid: Uuid,
}

impl<T> Cx<T> {

}