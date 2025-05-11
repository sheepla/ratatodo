use crate::models::models::TodoData;
use dirs;
use eyre::OptionExt;
use std::{fs::File, path::PathBuf};

const CACHE_FILE_NAME: &str = "ratatodo_cache.json";

#[derive(Debug, thiserror::Error)]
pub enum CacheError {
    #[error("Failed to get cache dir suitable for each OS")]
    CacheDir,

    #[error("Failed to create the file: {0}")]
    CreateFile(std::io::Error, PathBuf),

    #[error("Failed to open the file: {0}")]
    OpenFile(std::io::Error, PathBuf),

    #[error("Failed to serialize TodoData as JSON")]
    JsonSerialize(serde_json::Error),

    #[error("Failed to deserialize TodoData as JSON")]
    JsonDeserialize(serde_json::Error),
}

pub fn export_todo_data(data: &TodoData) -> Result<(), CacheError> {
    let mut cache_file_path = dirs::cache_dir().ok_or_else(|| CacheError::CacheDir)?;
    cache_file_path.push(CACHE_FILE_NAME);

    #[cfg(debug_assertions)]
    dbg!(&cache_file_path);

    let file = File::create(&cache_file_path)
        .map_err(|err| CacheError::CreateFile(err, cache_file_path.clone()))?;
    serde_json::to_writer(file, data).map_err(|err| CacheError::JsonSerialize(err))?;

    Ok(())
}

pub fn import_todo_data() -> Result<TodoData, CacheError> {
    let mut cache_file_path = dirs::cache_dir().ok_or_else(|| CacheError::CacheDir)?;
    cache_file_path.push(CACHE_FILE_NAME);

    #[cfg(debug_assertions)]
    dbg!(&cache_file_path);

    if !cache_file_path.exists() {
        return Ok(TodoData::default());
    }
    let file = File::open(&cache_file_path)
        .map_err(|err| CacheError::OpenFile(err, cache_file_path.clone()))?;
    let data = serde_json::from_reader(file).map_err(|err| CacheError::JsonDeserialize(err))?;

    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::models::{TodoData, TodoEntry, TodoEntryState};
    #[test]
    fn test_export_todo_data() -> eyre::Result<()> {
        let test_data = TodoData::default();
        export_todo_data(&test_data)?;

        let mut cache_file_path = dirs::cache_dir().expect("failed to get cache dir");
        cache_file_path.push(CACHE_FILE_NAME);

        dbg!(&cache_file_path);
        assert!(cache_file_path.exists());

        Ok(())
    }

    #[test]
    fn test_import_todo_data() -> eyre::Result<()> {
        let test_data = TodoData {
            entries: vec![
                TodoEntry {
                    title: "Learn Rust".to_string(),
                    state: TodoEntryState::InComplete,
                },
                TodoEntry {
                    title: "Reply to mail".to_string(),
                    state: TodoEntryState::Completed,
                },
                TodoEntry {
                    title: "Cleanup my room".to_string(),
                    state: TodoEntryState::InComplete,
                },
                TodoEntry {
                    title: "Go shopping for dinner".to_string(),
                    state: TodoEntryState::InComplete,
                },
            ],
        };

        export_todo_data(&test_data)?;

        let imported_data = import_todo_data()?;

        dbg!(&imported_data);
        assert_eq!(imported_data, test_data);

        let mut cache_file_path = dirs::cache_dir().expect("failed to get cache dir");
        cache_file_path.push(CACHE_FILE_NAME);

        Ok(())
    }
}
