use crate::models::models::TodoData;
use dirs;
use eyre::OptionExt;
use std::fs::File;

pub fn export_todo_data(data: &TodoData) -> eyre::Result<()> {
    let mut cache_file_path =
        dirs::cache_dir().ok_or_eyre("failed to get cache dir suitable for each OS")?;
    cache_file_path.push("ratatodo_cache.json");

    dbg!(&cache_file_path);

    let file = File::create(cache_file_path)?;

    serde_json::to_writer(file, data)?;
    Ok(())
}

pub fn import_todo_data() -> eyre::Result<TodoData> {
    let mut cache_file_path =
        dirs::cache_dir().ok_or_eyre("failed to get cache dir suitable for each OS")?;
    cache_file_path.push("ratatodo_cache.json");

    dbg!(&cache_file_path);

    if !cache_file_path.exists() {
        return Ok(TodoData::default());
    }

    let file = File::open(cache_file_path)?;

    let data = serde_json::from_reader(file)?;

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

        let mut cache_file_path = dirs::cache_dir().ok_or_eyre("failed to get cache dir")?;
        cache_file_path.push("ratatodo_cache.json");

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

        let mut cache_file_path = dirs::cache_dir().ok_or_eyre("failed to get cache dir")?;
        cache_file_path.push("ratatodo_cache.json");

        Ok(())
    }
}
