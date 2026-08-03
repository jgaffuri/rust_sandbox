use geos::{Geom, Geometry, GeometryTypes};

pub fn to_multi(geom: Geometry) -> Result<Geometry, geos::Error> {
    match geom.geometry_type()? {
        GeometryTypes::Point => Geometry::create_multipoint(vec![geom]),
        GeometryTypes::LineString | GeometryTypes::LinearRing => {
            Geometry::create_multiline_string(vec![geom])
        }
        GeometryTypes::Polygon => Geometry::create_multipolygon(vec![geom]),
        // Already multi (or a collection) — nothing to do.
        GeometryTypes::MultiPoint
        | GeometryTypes::MultiLineString
        | GeometryTypes::MultiPolygon
        | GeometryTypes::GeometryCollection => Ok(geom),
    }
}
