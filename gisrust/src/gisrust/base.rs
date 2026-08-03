use std::collections::HashMap;
use geos::Geometry;
use gdal::vector::FieldValue;

/* A feature in the vector dataset: a geometry and associated attributes */
pub struct Feature {
    pub geometry: Geometry,
    pub attributes: HashMap<String, FieldValue>,
}
