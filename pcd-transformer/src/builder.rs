use std::sync::Arc;

use projection_transform::{crs::EpsgCode, vshift::Jgd2011ToWgs84};

use crate::transform::{projection::ProjectionTransform, SerialTransform, Transform};

pub trait TransformBuilder {
    fn build(&self) -> Box<dyn Transform>;
}

pub struct PointCloudTransformBuilder {
    input_epsg: EpsgCode,
    output_epsg: EpsgCode,
    jgd2wgs: Arc<Jgd2011ToWgs84>,
}

impl TransformBuilder for PointCloudTransformBuilder {
    fn build(&self) -> Box<dyn Transform> {
        let mut transformers = SerialTransform::default();

        // Use new_auto to automatically select the best transformation strategy
        let projection_transform = ProjectionTransform::new_auto(self.input_epsg, self.output_epsg)
            .expect("Failed to create projection transformer");

        transformers.push(Box::new(projection_transform));

        Box::new(transformers)
    }
}

impl PointCloudTransformBuilder {
    /// Create a new builder with automatic transformation strategy selection
    ///
    /// # Arguments
    /// * `input_epsg` - Input coordinate system EPSG code
    /// * `output_epsg` - Output coordinate system EPSG code
    pub fn new(input_epsg: EpsgCode, output_epsg: EpsgCode) -> Self {
        Self {
            input_epsg,
            output_epsg,
            jgd2wgs: Jgd2011ToWgs84::default().into(),
        }
    }

    /// Legacy constructor for Japan-specific transformations (deprecated)
    ///
    /// This method is kept for backward compatibility but will be removed in future versions.
    /// Use `new(input_epsg, output_epsg)` instead.
    #[deprecated(since = "0.1.0", note = "Use new(input_epsg, output_epsg) instead")]
    pub fn new_legacy(output_epsg: EpsgCode) -> Self {
        // Assume JGD2011 input for legacy compatibility
        Self {
            input_epsg: 6677, // Default to JGD2011 Zone IX
            output_epsg,
            jgd2wgs: Jgd2011ToWgs84::default().into(),
        }
    }
}
