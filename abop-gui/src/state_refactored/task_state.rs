//! Background task state management
//!
//! This module handles all background task related state including task tracking,
//! progress monitoring, and task history.

/// Information about a background task
#[derive(Debug, Clone)]
pub struct TaskInfo {
    /// Unique identifier for the task
    pub id: String,
    /// Task type
    pub task_type: TaskType,
    /// Current progress (0.0 to 1.0)
    pub progress: f32,
    /// Task status message
    pub status: String,
    /// Whether the task is currently running
    pub is_running: bool,
    /// Whether the task has completed
    pub is_completed: bool,
    /// Error message if task failed
    pub error: Option<String>,
    /// Start time of the task
    pub start_time: chrono::DateTime<chrono::Local>,
    /// End time of the task if completed
    pub end_time: Option<chrono::DateTime<chrono::Local>>,
}

/// Types of background tasks that can be performed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskType {
    /// Scanning a library directory for audiobooks
    Scan,
    /// Processing audio files (e.g., extracting metadata)
    Process,
    /// Importing audiobooks from another source
    Import,
    /// Exporting audiobooks to another format
    Export,
    /// Saving application state
    Save,
}

impl std::fmt::Display for TaskType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Scan => write!(f, "Scanning"),
            Self::Process => write!(f, "Processing"),
            Self::Import => write!(f, "Importing"),
            Self::Export => write!(f, "Exporting"),
            Self::Save => write!(f, "Saving"),
        }
    }
}

impl Default for TaskInfo {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            task_type: TaskType::Scan,
            progress: 0.0,
            status: "Ready".to_string(),
            is_running: false,
            is_completed: false,
            error: None,
            start_time: chrono::Local::now(),
            end_time: None,
        }
    }
}

/// Background task management state
#[derive(Debug, Clone)]
pub struct TaskState {
    /// Current active task if any
    pub active_task: Option<TaskInfo>,
    /// List of recent tasks (for history/debugging)
    pub recent_tasks: Vec<TaskInfo>,
    /// Maximum number of tasks to keep in history
    pub max_task_history: usize,
    /// Whether state saving is in progress
    pub saving: bool,
    /// Progress of the current state save (0.0 to 1.0)
    pub save_progress: Option<f32>,
    /// Flag to indicate task state needs UI redraw
    pub needs_redraw: bool,
}

impl TaskState {
    /// Create new task state with specified history size
    #[must_use]
    pub fn with_history_size(max_history: usize) -> Self {
        Self {
            active_task: None,
            recent_tasks: Vec::new(),
            max_task_history: max_history,
            saving: false,
            save_progress: None,
            needs_redraw: false,
        }
    }

    /// Start a new task
    pub fn start_task(&mut self, task_type: TaskType, initial_status: String) -> String {
        let task_id = uuid::Uuid::new_v4().to_string();
        let task = TaskInfo {
            id: task_id.clone(),
            task_type,
            progress: 0.0,
            status: initial_status,
            is_running: true,
            is_completed: false,
            error: None,
            start_time: chrono::Local::now(),
            end_time: None,
        };

        // Move current active task to history if exists
        if let Some(current_task) = self.active_task.take() {
            self.add_to_history(current_task);
        }

        self.active_task = Some(task);
        self.needs_redraw = true;
        task_id
    }

    /// Update progress for the active task
    pub fn update_task_progress(&mut self, progress: f32, status: Option<String>) {
        if let Some(task) = &mut self.active_task {
            let progress = progress.clamp(0.0, 1.0);
            if task.progress != progress {
                task.progress = progress;
                self.needs_redraw = true;
            }
            if let Some(new_status) = status
                && task.status != new_status
            {
                task.status = new_status;
                self.needs_redraw = true;
            }
        }
    }

    /// Complete the active task successfully
    pub fn complete_task(&mut self, final_status: Option<String>) {
        if let Some(mut task) = self.active_task.take() {
            task.is_running = false;
            task.is_completed = true;
            task.progress = 1.0;
            task.end_time = Some(chrono::Local::now());
            if let Some(status) = final_status {
                task.status = status;
            }
            self.add_to_history(task);
            self.needs_redraw = true;
        }
    }

    /// Fail the active task with an error
    pub fn fail_task(&mut self, error_message: String) {
        if let Some(mut task) = self.active_task.take() {
            task.is_running = false;
            task.is_completed = false;
            task.error = Some(error_message.clone());
            task.status = format!("Failed: {error_message}");
            task.end_time = Some(chrono::Local::now());
            self.add_to_history(task);
            self.needs_redraw = true;
        }
    }

    /// Cancel the active task
    pub fn cancel_task(&mut self) {
        if let Some(mut task) = self.active_task.take() {
            task.is_running = false;
            task.is_completed = false;
            task.status = "Cancelled".to_string();
            task.end_time = Some(chrono::Local::now());
            self.add_to_history(task);
            self.needs_redraw = true;
        }
    }

    /// Start a save operation
    pub fn start_saving(&mut self) {
        self.saving = true;
        self.save_progress = Some(0.0);
        self.needs_redraw = true;
    }

    /// Update save progress
    pub fn update_save_progress(&mut self, progress: f32) {
        let progress = progress.clamp(0.0, 1.0);
        if self.save_progress != Some(progress) {
            self.save_progress = Some(progress);
            self.needs_redraw = true;
        }
    }

    /// Complete save operation
    pub fn complete_saving(&mut self) {
        if self.saving {
            self.saving = false;
            self.save_progress = None;
            self.needs_redraw = true;
        }
    }

    /// Get the current active task
    pub fn active_task(&self) -> Option<&TaskInfo> {
        self.active_task.as_ref()
    }

    /// Get task history
    pub fn task_history(&self) -> &[TaskInfo] {
        &self.recent_tasks
    }

    /// Clear task history
    pub fn clear_history(&mut self) {
        if !self.recent_tasks.is_empty() {
            self.recent_tasks.clear();
            self.needs_redraw = true;
        }
    }

    /// Check if any task is currently running
    pub fn has_active_task(&self) -> bool {
        self.active_task.as_ref().is_some_and(|t| t.is_running)
    }

    /// Check if saving is in progress
    pub fn is_saving(&self) -> bool {
        self.saving
    }

    /// Add a completed task to history
    fn add_to_history(&mut self, task: TaskInfo) {
        self.recent_tasks.push(task);
        
        // Trim history to max size
        if self.recent_tasks.len() > self.max_task_history {
            self.recent_tasks.drain(0..self.recent_tasks.len() - self.max_task_history);
        }
    }

    /// Check if the task state needs a redraw
    #[must_use]
    pub const fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    /// Clear the redraw flag (typically called after redraw is complete)
    pub fn clear_redraw_flag(&mut self) {
        self.needs_redraw = false;
    }
}

impl Default for TaskState {
    fn default() -> Self {
        Self::with_history_size(50) // Keep last 50 tasks by default
    }
}