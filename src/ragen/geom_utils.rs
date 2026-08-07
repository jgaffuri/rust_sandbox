use anyhow::Result;
use geos::{Geom, Geometry, GeometryTypes};
use wkt::{ToWkt, TryFromWkt};



pub fn geos_to_geo(geom: &geos::Geometry) -> Result<geo_types::Geometry<f64>> {
    let wkt_str = geom.to_wkt()?;
    let geo_geom = geo_types::Geometry::<f64>::try_from_wkt_str(&wkt_str).map_err(|e| anyhow::anyhow!("{}", e))?;
    Ok(geo_geom)
}

pub fn geo_to_geos(geom: &geo_types::Geometry<f64>) -> Result<geos::Geometry> {
    let geos_geom = geos::Geometry::new_from_wkt(&geom.to_wkt().to_string())?;
    Ok(geos_geom)
}




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
