use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Bill {
    pub id: i32,
    pub user: i32,
}
