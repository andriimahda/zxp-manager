use std::fs;
use std::path::{Path, PathBuf};
use quick_xml::events::Event;
use quick_xml::reader::Reader;

// Constants
const CEP_EXTENSIONS_PATH: &str = "~/Library/Application Support/Adobe/CEP/extensions/";

// Data structures
#[derive(Debug, Clone)]
pub struct Plugin {
    pub name: String,
    pub version: String,
    pub size: String,
    pub path: PathBuf,
    pub plugin_type: PluginType,
}

#[derive(Debug, Clone)]
pub enum PluginType {
    Native,      // Bundle ID starts with "com.adobe."
    Installed,   // Third-party plugins
}

#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub bundle_id: String,
    pub name: String,
    pub version: String,
}

#[derive(Debug)]
pub enum PluginError {
    DirectoryNotFound,
    PermissionDenied,
    ManifestNotFound,
    InvalidManifest,
}

impl std::fmt::Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginError::DirectoryNotFound => write!(f, "Directory not found"),
            PluginError::PermissionDenied => write!(f, "Permission denied"),
            PluginError::ManifestNotFound => write!(f, "Manifest not found"),
            PluginError::InvalidManifest => write!(f, "Invalid manifest"),
        }
    }
}

impl std::error::Error for PluginError {}

impl From<std::io::Error> for PluginError {
    fn from(error: std::io::Error) -> Self {
        match error.kind() {
            std::io::ErrorKind::NotFound => PluginError::DirectoryNotFound,
            std::io::ErrorKind::PermissionDenied => PluginError::PermissionDenied,
            _ => PluginError::DirectoryNotFound,
        }
    }
}

// Data operations
pub fn scan_cep_plugins() -> Result<Vec<Plugin>, PluginError> {
    // 1. Use system-wide CEP extensions directory
    let cep_path = Path::new("/Library/Application Support/Adobe/CEP/extensions/");
    
    // 2. Check if directory exists
    if !cep_path.exists() {
        log::warn!("CEP extensions directory not found: {:?}", cep_path);
        return Ok(Vec::new());
    }
    
    // 3. Read directory contents
    let entries = fs::read_dir(&cep_path)?;
    let mut plugins = Vec::new();
    
    // 4. For each subdirectory
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        // Only process directories
        if !path.is_dir() {
            continue;
        }
        
        // Check if it's a valid plugin
        if !is_valid_plugin(&path) {
            continue;
        }
        
        // Parse manifest
        let manifest_path = path.join("CSXS").join("manifest.xml");
        match parse_manifest_xml(&manifest_path) {
            Ok(plugin_info) => {
                let plugin_type = determine_plugin_type(&plugin_info.bundle_id);
                let size = calculate_folder_size(&path);
                
                plugins.push(Plugin {
                    name: plugin_info.name,
                    version: plugin_info.version,
                    size,
                    path: path.clone(),
                    plugin_type,
                });
            }
            Err(e) => {
                log::warn!("Failed to parse manifest for {:?}: {}", path, e);
            }
        }
    }
    
    Ok(plugins)
}

pub fn parse_manifest_xml(manifest_path: &Path) -> Result<PluginInfo, PluginError> {
    let xml_content = fs::read_to_string(manifest_path)
        .map_err(|_| PluginError::ManifestNotFound)?;
    
    let mut reader = Reader::from_str(&xml_content);
    reader.config_mut().trim_text(true);
    
    let mut bundle_id = String::new();
    let mut name = String::new();
    let mut version = String::new();
    
    let mut buf = Vec::new();
    
    loop {
        match reader.read_event_into(&mut buf) {
            Err(_) => return Err(PluginError::InvalidManifest),
            Ok(Event::Eof) => break,
            
            // Look for ExtensionBundleId attribute
            Ok(Event::Start(ref e)) if e.name().as_ref() == b"ExtensionManifest" => {
                for attr in e.attributes() {
                    let attr = attr.map_err(|_| PluginError::InvalidManifest)?;
                    match attr.key.as_ref() {
                        b"ExtensionBundleId" => {
                            bundle_id = String::from_utf8_lossy(&attr.value).to_string();
                        }
                        b"ExtensionBundleName" => {
                            name = String::from_utf8_lossy(&attr.value).to_string();
                        }
                        b"ExtensionBundleVersion" => {
                            version = String::from_utf8_lossy(&attr.value).to_string();
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        buf.clear();
    }
    
    if bundle_id.is_empty() {
        return Err(PluginError::InvalidManifest);
    }
    
    // Use bundle_id as fallback name if name is empty
    if name.is_empty() {
        name = bundle_id.clone();
    }
    
    Ok(PluginInfo {
        bundle_id,
        name,
        version: if version.is_empty() { "Unknown".to_string() } else { version },
    })
}

pub fn calculate_folder_size(path: &Path) -> String {
    match calculate_folder_size_bytes(path) {
        Ok(bytes) => format_size(bytes),
        Err(e) => {
            log::warn!("Failed to calculate size for {:?}: {}", path, e);
            "Unknown".to_string()
        }
    }
}

// Helper functions
pub fn determine_plugin_type(bundle_id: &str) -> PluginType {
    if bundle_id.starts_with("com.adobe.") {
        PluginType::Native
    } else {
        PluginType::Installed
    }
}

fn is_valid_plugin(plugin_dir: &Path) -> bool {
    plugin_dir.join("CSXS").join("manifest.xml").exists()
}

fn calculate_folder_size_bytes(path: &Path) -> Result<u64, std::io::Error> {
    let mut total_size = 0;
    
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        
        if metadata.is_file() {
            total_size += metadata.len();
        } else if metadata.is_dir() {
            total_size += calculate_folder_size_bytes(&entry.path())?;
        }
    }
    
    Ok(total_size)
}

fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}