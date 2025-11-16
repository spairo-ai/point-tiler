use std::error::Error;

use pcd_core::pointcloud::point::PointCloud;

pub mod csv;
pub mod las;

pub trait ParserProvider {
    fn get_parser(&self) -> Box<dyn Parser>;
}

pub trait Parser {
    fn parse(&self) -> Result<PointCloud, Box<dyn Error>>;
}

#[derive(Debug, Clone, Copy)]
pub enum Extension {
    Las,
    Laz,
    Csv,
    Txt,
}

pub fn get_extension(extension: &str) -> crate::error::Result<Extension> {
    match extension {
        "las" => Ok(Extension::Las),
        "laz" => Ok(Extension::Laz),
        "csv" => Ok(Extension::Csv),
        "txt" => Ok(Extension::Txt),
        _ => Err(crate::error::ParseError::UnsupportedExtension {
            extension: extension.to_string(),
        }),
    }
}
