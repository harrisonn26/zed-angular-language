use std::{collections::HashSet, env, path::PathBuf};
use zed_extension_api::{self as zed, serde_json, Result};

struct AngularExtension {
    installed: HashSet<String>,
}

const PACKAGE_NAME: &str = "@angular/language-server";

fn get_package_path(package_name: &str) -> Result<PathBuf> {
    let path = env::current_dir()
        .map_err(|e| e.to_string())?
        .join("node_modules")
        .join(package_name);
    Ok(path)
}

impl AngularExtension {
    fn install_package_if_needed(
        &mut self,
        id: &zed::LanguageServerId,
        package_name: &str,
    ) -> Result<()> {
        let installed_version = zed::npm_package_installed_version(package_name)?;

        // If package is already installed in this session, skip reinstallation
        if installed_version.is_some() && self.installed.contains(package_name) {
            return Ok(());
        }

        zed::set_language_server_installation_status(
            id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let latest_version = zed::npm_package_latest_version(package_name)?;

        if installed_version.as_ref() != Some(&latest_version) {
            println!("Installing {package_name}@{latest_version}...");

            zed::set_language_server_installation_status(
                id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            if let Err(error) = zed::npm_install_package(package_name, &latest_version) {
                // If installation failed but we have an existing version, continue with that
                if installed_version.is_none() {
                    return Err(error);
                }
            }
        } else {
            println!("Found {package_name}@{latest_version} installed");
        }

        self.installed.insert(package_name.into());
        Ok(())
    }
}

impl zed::Extension for AngularExtension {
    fn new() -> Self {
        Self {
            installed: HashSet::new(),
        }
    }

    fn language_server_command(
        &mut self,
        id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        self.install_package_if_needed(id, PACKAGE_NAME)?;

        let server_path = get_package_path(PACKAGE_NAME)?
            .join("bin")
            .join("ngserver")
            .to_string_lossy()
            .to_string();

        let ts_probe_path = env::current_dir()
            .map_err(|e| e.to_string())?
            .join("node_modules")
            .join("typescript")
            .join("lib")
            .to_string_lossy()
            .to_string();

        let ng_probe_path = get_package_path(PACKAGE_NAME)?
            .join("bin")
            .to_string_lossy()
            .to_string();

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![
                server_path,
                "--stdio".to_string(),
                "--tsProbeLocations".to_string(),
                ts_probe_path,
                "--ngProbeLocations".to_string(),
                ng_probe_path,
            ],
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        Ok(Some(serde_json::json!({
            "typescript": {
                "tsdk": env::current_dir()
                    .map(|p| p.join("node_modules").join("typescript").join("lib"))
                    .ok()
            },
            "angular": {
                "enable": true,
                "enableExperimentalDiagnostics": false
            }
        })))
    }
}

zed::register_extension!(AngularExtension);
