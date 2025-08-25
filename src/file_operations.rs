use std::fs;
use std::path::{Path, PathBuf};
use std::io::Read;
use rfd::FileDialog;
use zip::ZipArchive;
use crate::data_operations::parse_manifest_xml;

#[derive(Debug)]
pub enum FileOperationError {
    DialogCancelled,
    InvalidExtension,
    FileNotFound,
    PermissionDenied,
    InvalidZip,
    ExtractError,
}

impl std::fmt::Display for FileOperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileOperationError::DialogCancelled => write!(f, "File dialog was cancelled"),
            FileOperationError::InvalidExtension => write!(f, "File must have .zxp extension"),
            FileOperationError::FileNotFound => write!(f, "File not found"),
            FileOperationError::PermissionDenied => write!(f, "Permission denied"),
            FileOperationError::InvalidZip => write!(f, "Invalid or corrupt ZXP file"),
            FileOperationError::ExtractError => write!(f, "Failed to extract ZXP file"),
        }
    }
}

impl std::error::Error for FileOperationError {}

// File operations
pub fn select_zxp_file() -> Result<PathBuf, FileOperationError> {
    // Opens native file picker dialog
    // Filters for .zxp files only
    // Returns selected file path or error if cancelled/invalid
    
    let file_path = FileDialog::new()
        .add_filter("ZXP Files", &["zxp"])
        .set_title("Select ZXP Plugin File")
        .pick_file()
        .ok_or(FileOperationError::DialogCancelled)?;
    
    // Validate extension (double-check)
    if !is_valid_zxp_extension(&file_path) {
        return Err(FileOperationError::InvalidExtension);
    }
    
    log::info!("Selected ZXP file: {:?}", file_path);
    Ok(file_path)
}

pub fn install_zxp(zxp_path: &Path) -> Result<PathBuf, FileOperationError> {
    // 1. Validate ZXP file exists and has correct extension
    // 2. Open ZXP (ZIP) file for reading  
    // 3. Parse manifest.xml from ZIP to get Extension ID
    // 4. Create target directory: /Library/.../extensions/{extension_id}/
    // 5. Extract all ZIP contents to target directory
    // 6. OS handles permission prompts if needed
    
    if !zxp_path.exists() {
        return Err(FileOperationError::FileNotFound);
    }
    
    if !is_valid_zxp_extension(zxp_path) {
        return Err(FileOperationError::InvalidExtension);
    }
    
    log::info!("Installing ZXP file: {:?}", zxp_path);
    
    // Open ZIP archive
    let file = fs::File::open(zxp_path)
        .map_err(|_| FileOperationError::FileNotFound)?;
    
    let mut archive = ZipArchive::new(file)
        .map_err(|_| FileOperationError::InvalidZip)?;
    
    // Parse manifest.xml from ZIP to get Extension ID
    let extension_id = extract_extension_id_from_zip(&mut archive)?;
    
    // Create target directory
    let cep_path = Path::new("/Library/Application Support/Adobe/CEP/extensions/");
    let target_dir = cep_path.join(&extension_id);
    
    log::info!("Installing to directory: {:?}", target_dir);
    
    // Create target directory if it doesn't exist
    fs::create_dir_all(&target_dir)
        .map_err(|e| match e.kind() {
            std::io::ErrorKind::PermissionDenied => FileOperationError::PermissionDenied,
            _ => FileOperationError::ExtractError,
        })?;
    
    // Extract all files from ZIP to target directory
    archive.extract(&target_dir)
        .map_err(|_| FileOperationError::ExtractError)?;
    
    log::info!("ZXP installation completed for: {}", extension_id);
    Ok(target_dir)
}

pub fn remove_plugin(plugin_path: &Path) -> Result<(), FileOperationError> {
    // 1. Validate plugin directory exists
    // 2. Check if we have permission to delete
    // 3. Remove entire plugin directory and contents
    // 4. Handle any permission errors gracefully
    
    if !plugin_path.exists() {
        return Err(FileOperationError::FileNotFound);
    }
    
    if !plugin_path.is_dir() {
        return Err(FileOperationError::InvalidExtension);
    }
    
    log::info!("Removing plugin: {:?}", plugin_path);
    
    fs::remove_dir_all(plugin_path)
        .map_err(|e| match e.kind() {
            std::io::ErrorKind::PermissionDenied => FileOperationError::PermissionDenied,
            _ => FileOperationError::ExtractError,
        })?;
    
    log::info!("Plugin removal completed");
    Ok(())
}

// Helper functions
fn is_valid_zxp_extension(file_path: &Path) -> bool {
    // Validates file has .zxp extension (case insensitive)
    file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase() == "zxp")
        .unwrap_or(false)
}

fn extract_extension_id_from_zip(archive: &mut ZipArchive<fs::File>) -> Result<String, FileOperationError> {
    // Find and read CSXS/manifest.xml from ZIP
    let manifest_file = archive
        .by_name("CSXS/manifest.xml")
        .map_err(|_| FileOperationError::InvalidZip)?;
    
    // Read manifest content
    let mut content = String::new();
    let mut reader = manifest_file;
    reader.read_to_string(&mut content)
        .map_err(|_| FileOperationError::InvalidZip)?;
    
    // Parse manifest XML to get Extension ID
    // Create temporary file for parsing (parse_manifest_xml expects Path)
    let temp_dir = std::env::temp_dir();
    let temp_manifest = temp_dir.join("temp_manifest.xml");
    
    fs::write(&temp_manifest, content)
        .map_err(|_| FileOperationError::ExtractError)?;
    
    let plugin_info = parse_manifest_xml(&temp_manifest)
        .map_err(|_| FileOperationError::InvalidZip)?;
    
    // Clean up temp file
    let _ = fs::remove_file(&temp_manifest);
    
    // Extract the main extension ID (before ".panel" if present)
    let extension_id = plugin_info.bundle_id
        .split(".panel")
        .next()
        .unwrap_or(&plugin_info.bundle_id)
        .to_string();
    
    Ok(extension_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_zxp_extension_validation() {
        assert!(is_valid_zxp_extension(&PathBuf::from("test.zxp")));
        assert!(is_valid_zxp_extension(&PathBuf::from("test.ZXP")));
        assert!(!is_valid_zxp_extension(&PathBuf::from("test.zip")));
        assert!(!is_valid_zxp_extension(&PathBuf::from("test")));
    }
}