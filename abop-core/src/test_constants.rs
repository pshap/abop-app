//! Common constants used in tests to avoid repeated string allocations
#![allow(missing_docs)]

/// Common test audiobook constants
pub mod audiobook {
    pub const TEST_ID: &str = "audiobook-123";
    pub const TEST_TITLE: &str = "Test Book";
    pub const TEST_TITLE_1: &str = "Test Book 1";
    pub const TEST_TITLE_2: &str = "Test Book 2";

    pub const TEST_AUTHOR: &str = "Test Author";
    pub const TEST_AUTHOR_1: &str = "Test Author 1";
    pub const TEST_AUTHOR_2: &str = "Test Author 2";

    pub const TEST_NARRATOR: &str = "Test Narrator";
    pub const TEST_DESCRIPTION: &str = "Test Description";
    pub const TEST_GENRE: &str = "Audiobook";

    // Bookmark and chapter constants
    pub const TEST_CHAPTER: &str = "Chapter 1";
    pub const TEST_BOOKMARK_NAME: &str = "Important";
    pub const TEST_NOTE: &str = "Key plot point";
    // File and path constants
    pub const TEST_PATH: &str = "/path/to/book.mp3";
    pub const TEST_FILENAME: &str = "book.mp3";
    pub const TEST_EXTENSION: &str = "mp3";
    pub const TEST_TITLE_BOOK: &str = "The Great Book";
    pub const TEST_AUTHOR_JANE: &str = "Jane Doe";
    pub const TEST_PATH_NO_EXT: &str = "/path/with.no.extension";
    pub const TEST_FILENAME_NO_EXT: &str = "with.no.extension";
    pub const TEST_EXTENSION_ALT: &str = "extension";
}

/// Common test library constants
pub mod library {
    pub const TEST_ID: &str = "lib-123";
    pub const TEST_NAME: &str = "Test Library";
    pub const TEST_PATH: &str = "/test/path";
}

/// Common test metadata constants
pub mod metadata {
    pub const TEST_TITLE: &str = "Test Title";
    pub const TEST_ARTIST: &str = "Test Artist";
    pub const TEST_ALBUM: &str = "Test Album";
    pub const TEST_NARRATOR: &str = "Test Narrator";
    pub const TEST_GENRE: &str = "Audiobook";
    pub const TEST_PUBLISHER: &str = "Test Publisher";
    pub const TEST_LANGUAGE: &str = "en";
    pub const TEST_DESCRIPTION: &str = "Test description";
}

/// Common test error messages
pub mod error {
    pub const DISK_FULL: &str = "disk full";
    pub const CANNOT_BE_EMPTY: &str = "cannot be empty";
    pub const NOT_FOUND: &str = "not found";
    pub const ALREADY_EXISTS: &str = "already exists";
}

/// Common test entity names
pub mod entity {
    pub const AUDIOBOOK: &str = "Audiobook";
    pub const LIBRARY: &str = "Library";
    pub const BOOKMARK: &str = "Bookmark";
    pub const PROGRESS: &str = "Progress";
}

/// Common test field names
pub mod field {
    pub const TITLE: &str = "title";
    pub const NAME: &str = "name";
    pub const ID: &str = "id";
    pub const PATH: &str = "path";
}

/// Common test file constants
pub mod file {
    pub const TEST_MP3: &str = "test1.mp3";
    pub const TEST_M4B: &str = "test2.m4b";
    pub const TEST_TXT: &str = "test3.txt";
    pub const TEST_FLAC: &str = "subdir/test4.flac";
    pub const TEST_CONTENT: &str = "test content";
    pub const MEMORY_DB: &str = ":memory:";
}

/// Common test service constants  
pub mod service {
    pub const TEST_DB_PATH: &str = "test.db";
    pub const TEST_CONFIG_PATH: &str = "/etc/config";
}
