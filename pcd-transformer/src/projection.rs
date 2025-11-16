use pcd_core::pointcloud::point::Point;
use projection_transform::{
    crs::*, jprect::JPRZone, proj_transform::ProjTransform, vshift::Jgd2011ToWgs84,
};
use std::sync::Mutex;
use std::collections::HashMap;

// Thread-local cache for PROJ transformers to avoid recreating them
thread_local! {
    static PROJ_CACHE: Mutex<HashMap<(EpsgCode, EpsgCode), ProjTransform>> = Mutex::new(HashMap::new());
}

/// Transform a single point from input CRS to output CRS
///
/// This function automatically selects the best transformation strategy:
/// - For Japan-specific transformations: Uses optimized JGD2011 -> WGS84 path
/// - For all other transformations: Uses PROJ library
pub fn transform_point(
    point: Point,
    input_epsg: EpsgCode,
    output_epsg: EpsgCode,
    jgd2wgs: &Jgd2011ToWgs84,
) -> crate::error::Result<Point> {
    // Check if this is a Japan-specific transformation that can use optimized path
    if is_japan_crs(input_epsg) && output_epsg == EPSG_WGS84_GEOGRAPHIC_3D {
        transform_from_jgd2011(point, Some(input_epsg), Some(output_epsg), jgd2wgs)
    } else {
        // Use PROJ for all other transformations
        transform_with_proj(point, input_epsg, output_epsg)
    }
}

/// Transform point using PROJ library
fn transform_with_proj(point: Point, input_epsg: EpsgCode, output_epsg: EpsgCode) -> crate::error::Result<Point> {
    // Get or create transformer from cache
    let (x, y, z) = PROJ_CACHE.with(|cache| -> crate::error::Result<(f64, f64, f64)> {
        let mut cache = cache.lock().unwrap();

        // Check if transformer exists in cache
        if !cache.contains_key(&(input_epsg, output_epsg)) {
            let transformer = ProjTransform::new(input_epsg, output_epsg)
                .map_err(|e| crate::error::TransformError::ProjTransformCreationFailed {
                    from: input_epsg,
                    to: output_epsg,
                    reason: e.to_string(),
                })?;
            cache.insert((input_epsg, output_epsg), transformer);
        }

        let transformer = cache.get(&(input_epsg, output_epsg)).unwrap();
        transformer
            .transform(point.x, point.y, point.z)
            .map_err(|e| crate::error::TransformError::ProjTransformFailed {
                reason: format!("Failed to transform from EPSG:{} to EPSG:{}: {}", input_epsg, output_epsg, e),
            })
    })?;

    Ok(Point {
        x,
        y,
        z,
        color: point.color,
        attributes: point.attributes,
    })
}

fn rectangular_to_lnglat(x: f64, y: f64, height: f64, input_epsg: EpsgCode) -> (f64, f64, f64) {
    let zone = JPRZone::from_epsg(input_epsg).unwrap();
    let proj = zone.projection();
    let (lng, lat, height) = proj.project_inverse(x, y, height).unwrap();
    (lng, lat, height)
}

fn transform_from_jgd2011(
    point: Point,
    rectangular: Option<EpsgCode>,
    output_epsg: Option<EpsgCode>,
    jgd2wgs: &Jgd2011ToWgs84,
) -> crate::error::Result<Point> {
    let output_epsg = output_epsg.ok_or_else(|| crate::error::TransformError::TransformationFailed {
        reason: "Output EPSG code is required".to_string(),
    })?;

    match output_epsg {
        EPSG_WGS84_GEOGRAPHIC_3D => {
            let x = point.x;
            let y = point.y;
            let z = point.z;

            let (lng, lat, height) = if let Some(input_epsg) = rectangular {
                rectangular_to_lnglat(x, y, z, input_epsg)
            } else {
                (x, y, z)
            };

            let (lng, lat, height) = jgd2wgs.convert(lng, lat, height);

            Ok(Point {
                x: lng,
                y: lat,
                z: height,
                color: point.color.clone(),
                attributes: point.attributes.clone(),
            })
        }
        EPSG_JGD2011_GEOGRAPHIC_3D => {
            let x = point.x;
            let y = point.y;
            let z = point.z;

            let (lng, lat, height) = if let Some(input_epsg) = rectangular {
                rectangular_to_lnglat(x, y, z, input_epsg)
            } else {
                (x, y, z)
            };

            Ok(Point {
                x: lng,
                y: lat,
                z: height,
                color: point.color.clone(),
                attributes: point.attributes.clone(),
            })
        }
        _ => {
            Err(crate::error::TransformError::UnsupportedOutputCrs { epsg: output_epsg })
        }
    }
}
