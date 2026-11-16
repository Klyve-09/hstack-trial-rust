use std::{fs, io::Write, path::Path};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: u64,
    pub title: String,
    pub body: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notebook {
    next_id: u64,
    items: Vec<Item>,
}

impl Default for Notebook {
    fn default() -> Self {
        Self {
            next_id: 1,
            items: Vec::new(),
        }
    }
}

impl Notebook {
    pub fn load(path: &Path) -> Result<Self> {
        let contents = match fs::read(path) {
            Ok(contents) => contents,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Self::default());
            }
            Err(error) => return Err(error).with_context(|| format!("Read {}", path.display())),
        };
        serde_json::from_slice(&contents)
            .with_context(|| format!("Invalid JSON in {}", path.display()))
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let parent = path
            .parent()
            .context("Data file needs a parent directory")?;
        fs::create_dir_all(parent).with_context(|| format!("Create {}", parent.display()))?;
        let mut file =
            tempfile::NamedTempFile::new_in(parent).context("Create temporary data file")?;
        serde_json::to_writer_pretty(&mut file, self).context("Encode notebook")?;
        file.write_all(b"\n")?;
        file.as_file().sync_all().context("Flush data file")?;
        // Replace only after the complete notebook has been written successfully.
        file.persist(path)
            .with_context(|| format!("Save {}", path.display()))?;
        Ok(())
    }

    pub fn items(&self) -> &[Item] {
        &self.items
    }

    pub fn add(&mut self, title: &str, body: String) -> Result<u64> {
        let title = title.trim();
        if title.is_empty() {
            bail!("Title must not be empty");
        }
        if title.contains(['\n', '\r']) {
            bail!("Title must be a single line");
        }
        let id = self.next_id;
        self.next_id = id.checked_add(1).context("No more item IDs available")?;
        self.items.push(Item {
            id,
            title: title.to_owned(),
            body,
        });
        Ok(id)
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn remove(&mut self, id: u64) -> Result<()> {
        let index = self.items.iter().position(|item| item.id == id);
        match index {
            Some(index) => {
                self.items.remove(index);
                Ok(())
            }
            None => bail!("Item {id} not found"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_file_is_empty_without_creating_files() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested/items.json");
        assert!(Notebook::load(&path).unwrap().items().is_empty());
        assert!(!path.parent().unwrap().exists());
    }

    #[test]
    fn saves_reloads_and_replaces_existing_data() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested/items.json");
        let mut notebook = Notebook::default();
        notebook.add("  Groceries  ", "Milk\n사과".into()).unwrap();
        notebook.save(&path).unwrap();
        let mut loaded = Notebook::load(&path).unwrap();
        assert_eq!(loaded.items()[0].title, "Groceries");
        assert_eq!(loaded.items()[0].body, "Milk\n사과");
        loaded.remove(1).unwrap();
        loaded.save(&path).unwrap();
        let mut loaded = Notebook::load(&path).unwrap();
        assert!(loaded.items().is_empty());
        assert_eq!(loaded.add("Next", String::new()).unwrap(), 2);
    }

    #[test]
    fn invalid_titles_and_missing_ids_leave_items_unchanged() {
        let mut notebook = Notebook::default();
        for title in ["", "  ", "two\nlines", "two\rlines"] {
            assert!(notebook.add(title, String::new()).is_err());
        }
        assert_eq!(notebook.add("Keep", String::new()).unwrap(), 1);
        assert!(notebook.remove(99).is_err());
        assert_eq!(notebook.items().len(), 1);
    }

    #[test]
    fn corrupt_data_is_reported_and_preserved() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("items.json");
        fs::write(&path, "broken").unwrap();
        assert!(Notebook::load(&path).is_err());
        assert_eq!(fs::read_to_string(path).unwrap(), "broken");
    }
}
