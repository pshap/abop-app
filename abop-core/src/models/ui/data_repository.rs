//! Data repository for managing application data operations

use crate::error::Result;
use crate::models::{Audiobook, Library, Progress};

use super::types::AppData;

/// Repository for managing application data with clear separation from UI state
#[derive(Debug, Clone)]
pub struct DataRepository {
    data: AppData,
}

impl DataRepository {
    /// Creates a new data repository with default data
    #[must_use]
    pub fn new() -> Self {
        Self {
            data: AppData::default(),
        }
    }

    /// Creates a data repository from existing data
    #[must_use]
    pub const fn from_data(data: AppData) -> Self {
        Self { data }
    }

    /// Gets a reference to the underlying data
    #[must_use]
    pub const fn data(&self) -> &AppData {
        &self.data
    }    /// Gets a mutable reference to the underlying data
    pub fn data_mut(&mut self) -> &mut AppData {
        &mut self.data
    }

    /// Consumes the repository and returns the data
    #[must_use]
    pub fn into_data(self) -> AppData {
        self.data
    }

    // Library management methods

    /// Adds a library to the repository
    pub fn add_library(&mut self, library: Library) {
        self.data.libraries.push(library);
    }

    /// Removes a library by ID, returns true if library was found and removed
    pub fn remove_library(&mut self, library_id: &str) -> bool {
        let initial_len = self.data.libraries.len();
        self.data.libraries.retain(|lib| lib.id != library_id);
        self.data.libraries.len() != initial_len
    }

    /// Gets a library by ID
    #[must_use]
    pub fn get_library(&self, library_id: &str) -> Option<&Library> {
        self.data.libraries.iter().find(|lib| lib.id == library_id)
    }

    /// Gets a mutable reference to a library by ID
    pub fn get_library_mut(&mut self, library_id: &str) -> Option<&mut Library> {
        self.data
            .libraries
            .iter_mut()
            .find(|lib| lib.id == library_id)
    }

    /// Gets all libraries
    #[must_use]
    pub fn libraries(&self) -> &[Library] {
        &self.data.libraries
    }

    /// Gets all libraries as mutable slice
    pub fn libraries_mut(&mut self) -> &mut [Library] {
        &mut self.data.libraries
    }

    // Audiobook management methods

    /// Adds an audiobook to the repository
    pub fn add_audiobook(&mut self, audiobook: Audiobook) {
        self.data.audiobooks.push(audiobook);
    }

    /// Removes an audiobook by ID, returns true if audiobook was found and removed
    pub fn remove_audiobook(&mut self, audiobook_id: &str) -> bool {
        let initial_len = self.data.audiobooks.len();
        self.data.audiobooks.retain(|book| book.id != audiobook_id);
        self.data.audiobooks.len() != initial_len
    }

    /// Gets an audiobook by ID
    #[must_use]
    pub fn get_audiobook(&self, audiobook_id: &str) -> Option<&Audiobook> {
        self.data
            .audiobooks
            .iter()
            .find(|book| book.id == audiobook_id)
    }

    /// Gets a mutable reference to an audiobook by ID
    pub fn get_audiobook_mut(&mut self, audiobook_id: &str) -> Option<&mut Audiobook> {
        self.data
            .audiobooks
            .iter_mut()
            .find(|book| book.id == audiobook_id)
    }

    /// Gets audiobooks for a specific library
    #[must_use]
    pub fn audiobooks_for_library(&self, library_id: &str) -> Vec<&Audiobook> {
        self.data
            .audiobooks
            .iter()
            .filter(|book| book.library_id == library_id)
            .collect()
    }

    /// Gets all audiobooks
    #[must_use]
    pub fn audiobooks(&self) -> &[Audiobook] {
        &self.data.audiobooks
    }

    /// Gets all audiobooks as mutable slice
    pub fn audiobooks_mut(&mut self) -> &mut [Audiobook] {
        &mut self.data.audiobooks
    }

    // Progress management methods

    /// Updates or creates progress for an audiobook
    pub fn update_progress(&mut self, audiobook_id: &str, position_seconds: u64) -> Result<()> {
        if let Some(progress) = self
            .data
            .progress
            .iter_mut()
            .find(|p| p.audiobook_id == audiobook_id)
        {
            progress.update_position(position_seconds);
        } else {
            let new_progress = Progress::new(audiobook_id, position_seconds);
            self.data.progress.push(new_progress);
        }
        Ok(())
    }

    /// Removes progress for an audiobook
    pub fn remove_progress(&mut self, audiobook_id: &str) -> bool {
        let initial_len = self.data.progress.len();
        self.data
            .progress
            .retain(|p| p.audiobook_id != audiobook_id);
        self.data.progress.len() != initial_len
    }

    /// Gets progress for a specific audiobook
    #[must_use]
    pub fn get_progress(&self, audiobook_id: &str) -> Option<&Progress> {
        self.data
            .progress
            .iter()
            .find(|p| p.audiobook_id == audiobook_id)
    }

    /// Gets mutable progress for a specific audiobook
    pub fn get_progress_mut(&mut self, audiobook_id: &str) -> Option<&mut Progress> {
        self.data
            .progress
            .iter_mut()
            .find(|p| p.audiobook_id == audiobook_id)
    }

    /// Gets all progress entries
    #[must_use]
    pub fn progress(&self) -> &[Progress] {
        &self.data.progress
    }

    /// Gets all progress entries as mutable slice
    pub fn progress_mut(&mut self) -> &mut [Progress] {
        &mut self.data.progress
    }

    /// Gets recently played audiobooks (based on progress)
    #[must_use]
    pub fn recently_played_audiobooks(&self) -> Vec<&Audiobook> {
        let recent_audiobook_ids: Vec<&str> = self
            .data
            .progress
            .iter()
            .filter(|p| p.is_recently_played())
            .map(|p| p.audiobook_id.as_str())
            .collect();

        self.data
            .audiobooks
            .iter()
            .filter(|book| recent_audiobook_ids.contains(&book.id.as_str()))
            .collect()
    }

    // Utility methods

    /// Gets the total number of items in the repository
    #[must_use]
    pub const fn item_counts(&self) -> (usize, usize, usize) {
        (
            self.data.libraries.len(),
            self.data.audiobooks.len(),
            self.data.progress.len(),
        )
    }

    /// Clears all data from the repository
    pub fn clear(&mut self) {
        self.data.libraries.clear();
        self.data.audiobooks.clear();
        self.data.progress.clear();
    }

    /// Checks if the repository is empty
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.data.libraries.is_empty()
            && self.data.audiobooks.is_empty()
            && self.data.progress.is_empty()
    }
}

impl Default for DataRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl From<AppData> for DataRepository {
    fn from(data: AppData) -> Self {
        Self::from_data(data)
    }
}

impl From<DataRepository> for AppData {
    fn from(repository: DataRepository) -> Self {
        repository.into_data()
    }
}
