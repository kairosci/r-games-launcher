use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::api::{EpicClient, Game};
use crate::auth::AuthManager;
use crate::config::Config;
use crate::{Error, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledGame {
    pub app_name: String,
    pub app_title: String,
    pub app_version: String,
    pub install_path: PathBuf,
    pub executable: String,
}

impl InstalledGame {
    pub fn save(&self, config: &Config) -> Result<()> {
        let games_dir = Self::installed_games_dir(config)?;
        fs::create_dir_all(&games_dir)?;

        let game_file = games_dir.join(format!("{}.json", self.app_name));
        let contents = serde_json::to_string_pretty(self)?;
        fs::write(&game_file, contents)?;

        Ok(())
    }

    pub fn load(config: &Config, app_name: &str) -> Result<Self> {
        let games_dir = Self::installed_games_dir(config)?;
        let game_file = games_dir.join(format!("{}.json", app_name));

        if !game_file.exists() {
            return Err(Error::GameNotFound(app_name.to_string()));
        }

        let contents = fs::read_to_string(&game_file)?;
        Ok(serde_json::from_str(&contents)?)
    }

    pub fn list_installed(config: &Config) -> Result<Vec<Self>> {
        let games_dir = Self::installed_games_dir(config)?;

        if !games_dir.exists() {
            return Ok(vec![]);
        }

        let mut games = Vec::new();

        for entry in fs::read_dir(&games_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(contents) = fs::read_to_string(&path) {
                    if let Ok(game) = serde_json::from_str::<InstalledGame>(&contents) {
                        games.push(game);
                    }
                }
            }
        }

        Ok(games)
    }

    pub fn delete(&self, config: &Config) -> Result<()> {
        let games_dir = Self::installed_games_dir(config)?;
        let game_file = games_dir.join(format!("{}.json", self.app_name));

        if game_file.exists() {
            fs::remove_file(&game_file)?;
        }

        Ok(())
    }

    fn installed_games_dir(_config: &Config) -> Result<PathBuf> {
        let data_dir = Config::data_dir()?;
        Ok(data_dir.join("installed"))
    }
}

pub struct GameManager {
    config: Config,
    auth: AuthManager,
    client: EpicClient,
}

impl GameManager {
    pub fn new(config: Config, auth: AuthManager) -> Result<Self> {
        let client = EpicClient::new()?;
        Ok(Self {
            config,
            auth,
            client,
        })
    }

    pub async fn list_library(&self) -> Result<Vec<Game>> {
        let token = self.auth.get_token()?;
        self.client.get_games(token).await
    }

    pub fn list_installed(&self) -> Result<Vec<InstalledGame>> {
        InstalledGame::list_installed(&self.config)
    }

    pub async fn install_game(&self, app_name: &str) -> Result<()> {
        let token = self.auth.get_token()?;

        log::info!("Starting installation for game: {}", app_name);

        // Get game manifest
        let manifest_id = self.client.get_game_manifest(token, app_name).await?;
        
        log::info!("Retrieved manifest ID: {}", manifest_id);

        // Create install directory
        let install_path = self.config.install_dir.join(app_name);
        fs::create_dir_all(&install_path)?;
        
        log::info!("Created install directory: {:?}", install_path);

        // For now, create a basic installation record
        // In a full implementation, this would:
        // 1. Download the manifest file from CDN
        // 2. Parse the manifest to get chunk list and file list
        // 3. Download all chunks (with progress tracking)
        // 4. Reconstruct files from chunks
        // 5. Set proper permissions
        
        // Create a placeholder installed game entry
        let installed_game = InstalledGame {
            app_name: app_name.to_string(),
            app_title: app_name.to_string(), // Would be from manifest
            app_version: "1.0.0".to_string(), // Would be from manifest
            install_path: install_path.clone(),
            executable: format!("{}.exe", app_name), // Would be from manifest
        };
        
        installed_game.save(&self.config)?;
        
        log::info!("Game installation record created for: {}", app_name);
        
        println!();
        println!("Note: Full file download is not yet implemented.");
        println!("This creates a placeholder installation record.");
        println!();
        println!("To complete the implementation, the following steps are needed:");
        println!("  1. Download manifest from CDN using manifest ID");
        println!("  2. Parse manifest to extract chunk and file information");
        println!("  3. Download game chunks with progress tracking");
        println!("  4. Reconstruct files from downloaded chunks");
        println!("  5. Verify file integrity using checksums");
        
        Ok(())
    }

    pub fn launch_game(&self, app_name: &str) -> Result<()> {
        let game = InstalledGame::load(&self.config, app_name)?;

        let executable_path = game.install_path.join(&game.executable);

        if !executable_path.exists() {
            return Err(Error::Other(format!(
                "Executable not found: {:?}",
                executable_path
            )));
        }

        log::info!("Launching game: {} ({})", game.app_title, game.app_name);

        Command::new(&executable_path)
            .current_dir(&game.install_path)
            .spawn()
            .map_err(|e| Error::Other(format!("Failed to launch game: {}", e)))?;

        Ok(())
    }

    pub fn uninstall_game(&self, app_name: &str) -> Result<()> {
        let game = InstalledGame::load(&self.config, app_name)?;

        // Remove game files
        if game.install_path.exists() {
            fs::remove_dir_all(&game.install_path)?;
        }

        // Remove metadata
        game.delete(&self.config)?;

        log::info!("Uninstalled game: {} ({})", game.app_title, game.app_name);

        Ok(())
    }
}
