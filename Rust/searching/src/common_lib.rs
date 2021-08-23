use serde::{Deserialize, Serialize};


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TwoDimentionalManifold {
    pub handles: i64,
    pub cross_caps: i64,
    pub boundry_components: i64,
}
