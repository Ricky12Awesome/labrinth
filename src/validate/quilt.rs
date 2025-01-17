use crate::validate::{
    SupportedGameVersions, ValidationError, ValidationResult,
};
use std::io::Cursor;
use time::OffsetDateTime;
use zip::ZipArchive;

pub struct QuiltValidator;

impl super::Validator for QuiltValidator {
    fn get_file_extensions(&self) -> &[&str] {
        &["jar", "zip"]
    }

    fn get_project_types(&self) -> &[&str] {
        &["mod"]
    }

    fn get_supported_loaders(&self) -> &[&str] {
        &["fabric"]
    }

    fn get_supported_game_versions(&self) -> SupportedGameVersions {
        // Time since release of 18w49a, the first fabric version
        SupportedGameVersions::PastDate(OffsetDateTime::from_unix_timestamp(
            1646070100,
        ))
    }

    fn validate(
        &self,
        archive: &mut ZipArchive<Cursor<bytes::Bytes>>,
    ) -> Result<ValidationResult, ValidationError> {
        archive.by_name("quilt.mod.json").map_err(|_| {
            ValidationError::InvalidInput(
                "No quilt.mod.json present for Quilt file.".into(),
            )
        })?;

        if !archive.file_names().any(|name| {
            name.ends_with("refmap.json") || name.ends_with(".class")
        }) {
            return Ok(ValidationResult::Warning(
                "Quilt mod file is a source file!",
            ));
        }

        Ok(ValidationResult::Pass)
    }
}
