use crate::persistence::{db, db_structure};
use crate::persistence::db_structure::{Datatype, Origin};

struct Core {
    id: i64,
    datatype: Datatype,
    origin: Origin,
    
}
