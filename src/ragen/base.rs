use std::collections::HashMap;
use geos::Geometry;
use gdal::vector::FieldValue;

/* A feature in the vector dataset: a geometry and associated attributes */
pub struct Feature {
    geometry: Geometry,
    pub attributes: HashMap<String, FieldValue>,
}

impl Feature {
    pub fn new() -> Self {
        Feature {
            geometry: Geometry::create_empty_point().unwrap(),
            attributes: HashMap::new(),
        }
    }

    pub fn get_geometry(&self) -> &Geometry {
        &self.geometry
    }
    pub fn set_geometry(&mut self, geometry: Geometry) {
        self.geometry = geometry;
    }

}

