use anyhow::Result;
use geos::{Geom, Geometry, GeometryTypes};
use wkt::{ToWkt, TryFromWkt};
use geo::{Euclidean, Distance, Point, MinimumRotatedRect};



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



pub fn elongation_measure(geometry: &geo_types::Geometry<f64>) -> Result<f64> {
    let min_rot_rect = geometry.minimum_rotated_rect();
    if let Some(min_rot_rect) = min_rot_rect {
        let coords = min_rot_rect.exterior().coords().collect::<Vec<_>>();
        if coords.len() >= 4 {
            let p0: Point = Point::new(coords[0].x, coords[0].y);
            let p1: Point = Point::new(coords[1].x, coords[1].y);
            let p2: Point = Point::new(coords[2].x, coords[2].y);
            let width = Euclidean.distance(p0, p1);
            let height = Euclidean.distance(p1, p2);
            if width > 0.0 && height > 0.0 {
                return Ok(width.min(height) / width.max(height));
            }
        }
    }
    Ok(0.0)
}
