//! Parallel execution framework for devtool
//!
//! This module provides the infrastructure for parallel execution of tool updates,
//! including dependency management, task scheduling, and progress reporting.

use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

/// Represents a tool that can be updated
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Tool {
    Homebrew,
    Rustup,
    Mise,
}

impl Tool {
    /// Get the display name for the tool
    pub fn display_name(&self) -> &'static str {
        match self {
            Tool::Homebrew => "Homebrew",
            Tool::Rustup => "Rustup",
            Tool::Mise => "Mise",
        }
    }
}

// ToolDependency struct removed - not currently used

/// Dependency graph for tool update ordering
#[derive(Debug)]
pub struct DependencyGraph {
    dependencies: HashMap<Tool, Vec<Tool>>,
    reverse_dependencies: HashMap<Tool, Vec<Tool>>,
}

impl DependencyGraph {
    /// Create a new dependency graph
    pub fn new() -> Self {
        Self {
            dependencies: HashMap::new(),
            reverse_dependencies: HashMap::new(),
        }
    }

    // add_dependency method removed - not currently used

    /// Get tools that have no dependencies (can be run first)
    pub fn get_ready_tools(&self, available_tools: &HashSet<Tool>) -> Vec<Tool> {
        available_tools
            .iter()
            .filter(|tool| {
                self.dependencies
                    .get(tool)
                    .map(|deps| deps.is_empty())
                    .unwrap_or(true)
            })
            .cloned()
            .collect()
    }

    /// Get tools that depend on the given tool
    pub fn get_dependent_tools(&self, tool: &Tool) -> Vec<Tool> {
        self.reverse_dependencies
            .get(tool)
            .cloned()
            .unwrap_or_default()
    }

    /// Check if a tool can be executed (all dependencies are satisfied)
    pub fn can_execute(&self, tool: &Tool, completed_tools: &HashSet<Tool>) -> bool {
        self.dependencies
            .get(tool)
            .map(|deps| deps.iter().all(|dep| completed_tools.contains(dep)))
            .unwrap_or(true)
    }
}

impl Default for DependencyGraph {
    fn default() -> Self {
        let graph = Self::new();

        // Define tool dependencies
        // Homebrew and Rustup can run in parallel
        // Mise can run in parallel with others
        // No dependencies for now, but this structure allows for future dependencies

        graph
    }
}

/// Task execution result
#[derive(Debug)]
pub struct TaskResult {
    pub tool: Tool,
    pub success: bool,
    pub output: String,
    // error field removed - not currently used
}

/// Parallel task scheduler
pub struct ParallelScheduler {
    // semaphore field removed - not currently used
    completed_tools: Arc<Mutex<HashSet<Tool>>>,
    dependency_graph: Arc<DependencyGraph>,
}

impl ParallelScheduler {
    /// Create a new parallel scheduler
    pub fn new(_max_concurrent: usize) -> Self {
        Self {
            // semaphore removed - not currently used
            completed_tools: Arc::new(Mutex::new(HashSet::new())),
            dependency_graph: Arc::new(DependencyGraph::default()),
        }
    }

    /// Execute tools in parallel with dependency management
    pub async fn execute_parallel(
        &self,
        tools: Vec<Tool>,
        update_fn: impl Fn(Tool) -> JoinHandle<Result<TaskResult>> + Send + Sync + 'static,
    ) -> Result<Vec<TaskResult>> {
        let mut results = Vec::new();
        let mut pending_tools: HashSet<Tool> = tools.into_iter().collect();
        let mut running_tasks: Vec<JoinHandle<Result<TaskResult>>> = Vec::new();

        while !pending_tools.is_empty() || !running_tasks.is_empty() {
            // Check for completed tasks
            let mut completed_indices = Vec::new();
            for (i, task) in running_tasks.iter().enumerate() {
                if task.is_finished() {
                    completed_indices.push(i);
                }
            }

            // Process completed tasks
            for &i in completed_indices.iter().rev() {
                let task = running_tasks.remove(i);
                if let Ok(result) = task.await? {
                    let tool = result.tool.clone();
                    results.push(result);

                    // Mark tool as completed
                    {
                        let mut completed = self.completed_tools.lock().await;
                        completed.insert(tool.clone());
                    }

                    // Check if any pending tools can now be executed
                    for dependent_tool in self.dependency_graph.get_dependent_tools(&tool) {
                        if pending_tools.contains(&dependent_tool) {
                            let can_execute = {
                                let completed = self.completed_tools.lock().await;
                                self.dependency_graph
                                    .can_execute(&dependent_tool, &completed)
                            };

                            if can_execute {
                                pending_tools.remove(&dependent_tool);
                                let task = update_fn(dependent_tool);
                                running_tasks.push(task);
                            }
                        }
                    }
                }
            }

            // Start new tasks if we have capacity and ready tools
            let ready_tools = {
                let _completed = self.completed_tools.lock().await;
                self.dependency_graph.get_ready_tools(&pending_tools)
            };

            for tool in ready_tools {
                if pending_tools.contains(&tool) {
                    pending_tools.remove(&tool);
                    let task = update_fn(tool);
                    running_tasks.push(task);
                }
            }

            // Small delay to prevent busy waiting
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependency_graph() {
        let mut graph = DependencyGraph::new();
        let homebrew = Tool::Homebrew;
        let rustup = Tool::Rustup;
        let mise = Tool::Mise;

        let available_tools: HashSet<Tool> =
            [homebrew.clone(), rustup.clone(), mise.clone()].into();

        let ready_tools = graph.get_ready_tools(&available_tools);
        assert_eq!(ready_tools.len(), 3); // All tools should be ready initially
    }

    #[test]
    fn test_tool_display_names() {
        assert_eq!(Tool::Homebrew.display_name(), "Homebrew");
        assert_eq!(Tool::Rustup.display_name(), "Rustup");
        assert_eq!(Tool::Mise.display_name(), "Mise");
    }
}
