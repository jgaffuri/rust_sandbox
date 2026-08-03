use crate::ragen::base::Feature;
use anyhow::{Result, anyhow};
use gdal::spatial_ref::SpatialRef;
use gdal::vector::{Feature as GDALFeature, FieldValue, LayerAccess, LayerOptions, OGRFieldType};
use gdal::{Dataset, DriverManager};
use gdal_sys::OGRwkbGeometryType;
use geos::Geom;
use geos::GeometryTypes;
use std::collections::HashMap;

pub struct LoadFeaturesOptions<'a> {
    pub layer_name: Option<&'a str>,
    pub bbox: Option<&'a [f64; 4]>,
    pub attribute_filter: Option<&'a str>,
}

pub fn load_features(path: &str, options: LoadFeaturesOptions<'_>) -> Result<Vec<Feature>> {
    let dataset = Dataset::open(path)?;

    let mut layer = match options.layer_name {
        Some(name) => dataset.layer_by_name(name)?,
        None => dataset.layer(0)?,
    };

    if let Some(bbox) = options.bbox {
        layer.set_spatial_filter_rect(bbox[0], bbox[1], bbox[2], bbox[3]);
    }

    if let Some(filter) = options.attribute_filter {
        layer.set_attribute_filter(filter)?;
    }

    layer
        .features()
        .map(|feature| {
            let g = feature
                .geometry()
                .ok_or_else(|| anyhow!("feature has no geometry"))?;

            let g: Vec<u8> = g.wkb()?;
            let g = geos::Geometry::new_from_wkb(&g)?;

            let mut fields: HashMap<String, FieldValue> = HashMap::new();
            for fff in feature.fields() {
                let key = fff.0;
                let value = fff.1.unwrap();
                fields.insert(key, value);
                //println!("{}    {:?}", key, value)
            }

            Ok(Feature {
                geometry: g,
                attributes: fields,
            })
        })
        .collect()
}

pub fn save_features(fs: &Vec<Feature>, path: &str, epsg: Option<u32>, layer_name: Option<&str>) -> Result<()> {
    if fs.is_empty() {
        return Err(anyhow!("No features to save"));
    }
    let f0 = &fs[0];

    let geom_type = ogr_geometry_type_of(&f0.geometry);
    let srs_projected = SpatialRef::from_epsg(epsg.unwrap_or(4326))?;

    //
    let driver = DriverManager::get_driver_by_name("GPKG")?;
    let mut out_dataset = driver.create_vector_only(path)?;

    let out_layer = out_dataset.create_layer(LayerOptions {
        name: layer_name.unwrap_or("layer"),
        srs: Some(&srs_projected),
        ty: geom_type,
        ..Default::default()
    })?;

    // Recreate the attribute schema on the output layer.
    let schema = feature_to_gpkg_schema(&f0);
    for (name, field_type) in &schema {
        out_layer.create_defn_fields(&[(name.as_str(), *field_type)])?;
    }

    let field_indices: HashMap<String, usize> = out_layer
        .defn()
        .fields()
        .enumerate()
        .map(|(i, field)| (field.name(), i))
        .collect();

    for record in fs {
        let mut feature = GDALFeature::new(out_layer.defn())?;

        //let ggg = record.geometry.to_gdal()?;
        //let ogr_geom = gdal::vector::Geometry::try_from(record.geometry)
        //    .expect("could not convert geo_types geometry back to OGR");

        let wkb = record.geometry.to_wkb()?;
        let g = gdal::vector::Geometry::from_wkb(&wkb)?;
        feature.set_geometry(g)?;

        for (name, value) in &record.attributes {
            if let Some(&index) = field_indices.get(name) {
                feature.set_field(index, value)?;
            }
        }
        feature.create(&out_layer)?;
    }

    Ok(())
}

/*pub struct GPKGMetadata {
    //pub srs: Option<gdal::spatial_ref::SpatialRef>,
    pub geom_field_type: u32,
    pub field_defs: Vec<(String, gdal::vector::OGRFieldType::Type)>,
}
*/

/*pub fn get_metadata(path: &str) -> GPKGMetadata {
    let dataset: Dataset = Dataset::open(path).unwrap();
    let layer: gdal::vector::Layer<'_> = dataset.layer(0).unwrap();
    let geom_field_type = layer
        .defn()
        .geom_fields()
        .next()
        .map(|f| f.field_type())
        .unwrap_or(gdal_sys::OGRwkbGeometryType::wkbUnknown);
    let field_defs: Vec<(String, gdal::vector::OGRFieldType::Type)> = layer
        .defn()
        .fields()
        .map(|f| (f.name(), f.field_type()))
        .collect();
    GPKGMetadata {
        geom_field_type: geom_field_type,
        field_defs: field_defs,
    }
}
*/

fn feature_to_gpkg_schema(feature: &Feature) -> Vec<(String, gdal::vector::OGRFieldType::Type)> {
    let field_defs = feature
        .attributes
        .iter()
        .map(|(name, value)| (name.clone(), ogr_field_type_of(value)))
        .collect();

    field_defs
}

fn ogr_geometry_type_of(geom: &geos::Geometry) -> OGRwkbGeometryType::Type {
    match geom
        .geometry_type()
        .expect("could not get geos geometry type")
    {
        GeometryTypes::Point => OGRwkbGeometryType::wkbPoint,
        GeometryTypes::LineString | GeometryTypes::LinearRing => OGRwkbGeometryType::wkbLineString,
        GeometryTypes::Polygon => OGRwkbGeometryType::wkbPolygon,
        GeometryTypes::MultiPoint => OGRwkbGeometryType::wkbMultiPoint,
        GeometryTypes::MultiLineString => OGRwkbGeometryType::wkbMultiLineString,
        GeometryTypes::MultiPolygon => OGRwkbGeometryType::wkbMultiPolygon,
        GeometryTypes::GeometryCollection => OGRwkbGeometryType::wkbGeometryCollection,
    }
}

fn ogr_field_type_of(value: &FieldValue) -> OGRFieldType::Type {
    match value {
        FieldValue::IntegerValue(_) => OGRFieldType::OFTInteger,
        FieldValue::IntegerListValue(_) => OGRFieldType::OFTIntegerList,
        FieldValue::Integer64Value(_) => OGRFieldType::OFTInteger64,
        FieldValue::Integer64ListValue(_) => OGRFieldType::OFTInteger64List,
        FieldValue::RealValue(_) => OGRFieldType::OFTReal,
        FieldValue::RealListValue(_) => OGRFieldType::OFTRealList,
        FieldValue::StringValue(_) => OGRFieldType::OFTString,
        FieldValue::StringListValue(_) => OGRFieldType::OFTStringList,
        FieldValue::DateValue(_) => OGRFieldType::OFTDate,
        FieldValue::DateTimeValue(_) => OGRFieldType::OFTDateTime,
    }
}
