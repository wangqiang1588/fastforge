use crate::traits::AppAnalyzer;
use crate::types::{AnalyzeConfig, AnalyzeError, AnalyzeResult};
use axmldecoder;
use serde_json::json;
use std::fs::File;
use std::io::{Cursor, Read};

use zip::ZipArchive;

pub struct AndroidApkAnalyzer;

impl AppAnalyzer for AndroidApkAnalyzer {
    fn new() -> Self {
        Self
    }

    fn name(&self) -> &str {
        return "android-apk";
    }

    fn is_supported_on_current_platform(&self) -> bool {
        return true;
    }

    fn perform_analyze(&self, config: &AnalyzeConfig) -> Result<AnalyzeResult, AnalyzeError> {
        // Extract AndroidManifest.xml from APK
        let manifest_content = self.extract_android_manifest(&config.path)?;

        // Parse the binary XML using axmldecoder
        let parsed_manifest = self.parse_android_manifest(&manifest_content)?;

        // Extract app metadata from parsed manifest
        let app_info = self.extract_app_info(&parsed_manifest)?;

        let data = json!({
            "platform": "android",
            "identifier": app_info.package_name,
            "name": app_info.app_name,
            "version": app_info.version_name,
            "buildNumber": app_info.version_code
        });

        log::info!("APK analysis completed for {}", config.path);
        Ok(AnalyzeResult::new(true, data))
    }
}

#[derive(Debug)]
struct AppInfo {
    package_name: String,
    app_name: String,
    version_name: String,
    version_code: String,
}

impl AndroidApkAnalyzer {
    /// Extract AndroidManifest.xml from APK file
    fn extract_android_manifest(&self, apk_path: &str) -> Result<Vec<u8>, AnalyzeError> {
        let file = File::open(apk_path)
            .map_err(|e| AnalyzeError::new(&format!("Failed to open APK file: {}", e)))?;

        let mut archive = ZipArchive::new(file)
            .map_err(|e| AnalyzeError::new(&format!("Failed to read APK archive: {}", e)))?;

        let mut manifest_file = archive.by_name("AndroidManifest.xml").map_err(|e| {
            AnalyzeError::new(&format!("Failed to find AndroidManifest.xml: {}", e))
        })?;

        let mut content = Vec::new();
        manifest_file.read_to_end(&mut content).map_err(|e| {
            AnalyzeError::new(&format!("Failed to read AndroidManifest.xml: {}", e))
        })?;

        Ok(content)
    }

    /// Parse binary AndroidManifest.xml using axmldecoder
    fn parse_android_manifest(
        &self,
        manifest_content: &[u8],
    ) -> Result<axmldecoder::XmlDocument, AnalyzeError> {
        let mut cursor = Cursor::new(manifest_content);
        let xml_doc = axmldecoder::parse(&mut cursor).map_err(|e| {
            AnalyzeError::new(&format!("Failed to decode AndroidManifest.xml: {:?}", e))
        })?;

        Ok(xml_doc)
    }

    /// Extract app information from parsed AndroidManifest.xml
    fn extract_app_info(
        &self,
        xml_doc: &axmldecoder::XmlDocument,
    ) -> Result<AppInfo, AnalyzeError> {
        let root_node = xml_doc
            .get_root()
            .as_ref()
            .ok_or_else(|| AnalyzeError::new("No root element found in AndroidManifest.xml"))?;

        let root_element = match root_node {
            axmldecoder::Node::Element(element) => element,
            _ => return Err(AnalyzeError::new("Root node is not an element")),
        };

        let attributes = root_element.get_attributes();

        // Extract package name
        let package_name = attributes
            .get("package")
            .unwrap_or(&"unknown".to_string())
            .clone();

        // Extract version code
        let version_code = attributes
            .get("android:versionCode")
            .unwrap_or(&"0".to_string())
            .clone();

        // Extract version name
        let version_name = attributes
            .get("android:versionName")
            .unwrap_or(&"1.0".to_string())
            .clone();

        // Extract app name from application element
        let mut app_name = package_name.clone();

        // Look for application element in children
        for child in root_element.get_children() {
            if let axmldecoder::Node::Element(child_element) = child {
                if child_element.get_tag() == "application" {
                    let app_attributes = child_element.get_attributes();
                    if let Some(label) = app_attributes.get("android:label") {
                        app_name = label.clone();
                        break;
                    }
                }
            }
        }

        Ok(AppInfo {
            package_name,
            app_name,
            version_name,
            version_code,
        })
    }
}
