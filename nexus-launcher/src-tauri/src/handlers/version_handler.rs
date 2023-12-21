use piston_lib::data_structures::game::metadata::piston_version_manifest::PistonMetadata;
use piston_lib::processes::launcher::installation::versioning::generate_versions_metadata;
use crate::config::get_cache_path;

struct VersionHandler {
    versions: PistonMetadata,
}

impl VersionHandler {
    fn new() -> VersionHandler {
        let metadata_path = get_cache_path().join("version_metadata.json");

        if metadata_path.exists() {
            std::thread::spawn(move || generate_versions_metadata());
        }
        else {
            generate_versions_metadata();
        }
        let data = std::fs::read_to_string(metadata_path).unwrap();
        return VersionHandler {
            versions: serde_json::from_str::<PistonMetadata>(data.as_str()).unwrap()
        };
    }
}