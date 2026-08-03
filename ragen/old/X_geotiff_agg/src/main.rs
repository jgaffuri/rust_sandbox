use anyhow::Result;
use geotiff_agg::aggregate_geotiff_by_region;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        eprintln!(
            "Usage: {} <gpkg_path> <region_id_attr> <tiff_path> <output_csv>",
            args[0]
        );
        std::process::exit(1);
    }
    aggregate_geotiff_by_region(&args[1], &args[2], &args[3], &args[4])
}
