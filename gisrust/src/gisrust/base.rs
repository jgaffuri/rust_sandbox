use std::collections::HashMap;
use geos::Geometry;
use gdal::vector::FieldValue;

pub struct Feature {
    pub geometry: Geometry,
    pub fields: HashMap<String, FieldValue>,
}


