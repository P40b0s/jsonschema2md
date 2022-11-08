use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Versioning
{
    pub major : u64,
    pub minor : u64,
    pub path : u64
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Version
{
    pub ver : Versioning,
    pub modifed : String
}
