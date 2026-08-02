use std::collections::HashMap;


pub struct Feature {
    pub geometry: geos::Geometry,
    pub fields: HashMap<String, gdal::vector::FieldValue>,
}


