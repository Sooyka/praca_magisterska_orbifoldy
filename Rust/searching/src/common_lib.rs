use serde::{Deserialize, Serialize};

// #[derive(Debug, PartialEq, Serialize, Deserialize)]
// pub struct TwoDimentionalManifold {
//     pub handles: i64,
//     pub cross_caps: i64,
//     pub boundry_components: i64,
// }

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "tag", content = "content")]
pub enum TwoDimentionalManifold {
    Disk,
    Sphere,
    Genus(i64),
    General {
        handles: i64,
        cross_caps: i64,
        boundry_components: i64,
    },
}
