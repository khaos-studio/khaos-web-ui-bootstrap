// Shared types for command handlers and IPC

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(crate = "serde")]
pub struct Project {
    pub id: String,
    pub title: String,
    pub author: Option<String>,
    pub path: String,
    pub scene_count: usize,
    pub modified: i64,  // Unix timestamp
}

/// Manifest.json structure found in KSPD projects
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct ProjectManifest {
    pub title: Option<String>,
    pub author: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

impl Project {
    /// Generate a unique ID from the project path
    pub fn id_from_path(path: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        path.hash(&mut hasher);
        format!("proj_{:x}", hasher.finish())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct SystemInfo {
    pub platform: String,
    pub arch: String,
    pub daemon_connected: bool,
}

// Import wizard types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct CollisionInfo {
    pub existing_path: String,
    pub suggested_names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct ImportResult {
    pub success: bool,
    pub project_id: Option<String>,
    pub output_path: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct DaemonCheckResult {
    pub reachable: bool,
    pub version: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct ProviderValidationResult {
    pub valid: bool,
    pub checks_run: Vec<String>,
    pub errors: Vec<String>,
}

// Settings types

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct SettingsConfig {
    pub provider: String,
    pub model: Option<String>,
    pub projects_root: Option<String>,
}

impl Default for SettingsConfig {
    fn default() -> Self {
        Self {
            provider: "ollama".to_string(),
            model: None,
            projects_root: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct ProviderInfo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub default_model: String,
    pub requires_key: bool,
    pub env_var: Option<String>,
}

impl ProviderInfo {
    pub fn all() -> Vec<ProviderInfo> {
        vec![
            ProviderInfo {
                id: "ollama".to_string(),
                title: "Ollama".to_string(),
                description: "Local LLM inference via Ollama".to_string(),
                default_model: "qwen3".to_string(),
                requires_key: false,
                env_var: None,
            },
            ProviderInfo {
                id: "openai".to_string(),
                title: "OpenAI".to_string(),
                description: "GPT models via OpenAI API".to_string(),
                default_model: "gpt-4o".to_string(),
                requires_key: true,
                env_var: Some("OPENAI_API_KEY".to_string()),
            },
            ProviderInfo {
                id: "mistralai".to_string(),
                title: "Mistral AI".to_string(),
                description: "Mistral models via Mistral API".to_string(),
                default_model: "mistral-large-latest".to_string(),
                requires_key: true,
                env_var: Some("MISTRALAI_API_KEY".to_string()),
            },
            ProviderInfo {
                id: "anthropic".to_string(),
                title: "Anthropic".to_string(),
                description: "Claude models via Anthropic API".to_string(),
                default_model: "claude-sonnet-4-20250514".to_string(),
                requires_key: true,
                env_var: Some("ANTHROPIC_API_KEY".to_string()),
            },
            ProviderInfo {
                id: "groq".to_string(),
                title: "Groq".to_string(),
                description: "Fast inference via Groq API".to_string(),
                default_model: "llama-3.3-70b-versatile".to_string(),
                requires_key: true,
                env_var: Some("GROQ_API_KEY".to_string()),
            },
            ProviderInfo {
                id: "mock".to_string(),
                title: "Mock".to_string(),
                description: "Mock provider for testing (no API calls)".to_string(),
                default_model: "mock-model".to_string(),
                requires_key: false,
                env_var: None,
            },
        ]
    }

    pub fn find(id: &str) -> Option<ProviderInfo> {
        Self::all().into_iter().find(|p| p.id == id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct OllamaStatus {
    pub installed: bool,
    pub model_available: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct SettingsLoadResult {
    pub config: SettingsConfig,
    pub providers: Vec<ProviderInfo>,
}

// Dashboard types

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct ProjectSummary {
    pub scenes: usize,
    pub characters: usize,
    pub locations: usize,
    pub compositions: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct SceneSummary {
    pub id: String,
    pub index: usize,
    pub slugline: String,
    pub duration: String,
    pub word_count: usize,
    pub line_count: usize,
    pub characters: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct CharacterSummary {
    pub id: String,
    pub name: String,
    pub dialogue_lines: usize,
    pub words: usize,
    pub scene_count: usize,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct LocationSummary {
    pub id: String,
    pub name: String,
    pub scene_count: usize,
    pub page_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct GraphSummary {
    pub total_entities: usize,
    pub total_relationships: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct SceneAnalysis {
    pub title: Option<String>,
    pub scene_number: Option<String>,
    pub summary: Option<String>,
    pub narrative_role: Option<String>,
    pub emotional_tone: Option<String>,
    pub stakes: Option<String>,
    pub themes: Vec<String>,
    pub theme_details: Option<String>,
    pub plot_beats: Vec<String>,
    pub heading: Option<String>,
    pub pacing: Option<String>,
    pub estimated_runtime: Option<String>,
    pub length_pages: Option<String>,
    pub scene_type: Option<String>,
    pub time_of_day: Option<String>,
    pub genre_tone: Option<String>,
    pub style_notes: Option<String>,
    pub speaking_chars: Vec<String>,
    pub non_speaking_chars: Vec<String>,
    pub non_speaking_details: Vec<String>,
    pub callbacks: Vec<String>,
    pub continues_from: Vec<String>,
    pub sets_up: Vec<String>,
    pub strengths: Vec<String>,
    pub development_areas: Vec<String>,
    pub visual_symbols: Vec<String>,
    pub location_significance: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct CharacterAnalysis {
    pub summary: Option<String>,
    pub arc: Option<String>,
    pub emotional_journey: Option<String>,
    pub end_state: Option<String>,
    pub arc_quality: Option<String>,
    pub key_turning_points: Vec<String>,
    pub traits: Vec<String>,
    pub goals: Vec<String>,
    pub conflicts: Vec<String>,
    pub background: Option<String>,
    pub stakes: Option<String>,
    pub contradictions: Option<String>,
    pub vulnerabilities: Vec<String>,
    pub relationships: Vec<String>,
    pub dialogue_voice: Option<String>,
    pub dialogue_function: Option<String>,
    pub dialogue_patterns: Vec<String>,
    pub dialogue_subtext: Option<String>,
    pub themes: Vec<String>,
    pub thematic_role: Option<String>,
    pub symbolic_elements: Vec<String>,
    pub narrative_role: Option<String>,
    pub narrative_importance: Option<String>,
    pub narrative_plot_function: Option<String>,
    pub genre_fit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct LocationAnalysis {
    pub summary: Option<String>,
    pub atmosphere: Option<String>,
    pub environment: Option<String>,
    pub visual_context: Option<String>,
    pub significance: Option<String>,
    pub traits: Vec<String>,
    pub changes: Vec<String>,
    pub narrative_role: Option<String>,
    pub story_role: Option<String>,
    pub plot_anchors: Vec<String>,
    pub symbols: Vec<String>,
    pub themes: Vec<String>,
    pub thematic_role: Option<String>,
    pub location_type: Option<String>,
    pub region: Option<String>,
    pub classification: Option<String>,
    pub character_connections: Vec<String>,
    pub production_notes: Option<String>,
    pub structural_observations: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(crate = "serde")]
#[serde(rename_all = "lowercase")]
pub enum AnalysisState {
    Pending,
    Analyzing,
    Analyzed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct ItemWithState<T: Clone> {
    pub item: T,
    pub state: AnalysisState,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct AnalysisResult {
    pub success: bool,
    pub item_type: String,
    pub item_id: String,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct AnalysisIndex {
    pub scenes: Vec<String>,
    pub characters: Vec<String>,
    pub locations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct SceneDetail {
    pub summary: SceneSummary,
    pub analysis: Option<SceneAnalysis>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct CharacterDetail {
    pub summary: CharacterSummary,
    pub analysis: Option<CharacterAnalysis>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct LocationDetail {
    pub summary: LocationSummary,
    pub analysis: Option<LocationAnalysis>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct DaemonStatus {
    pub running: bool,
    pub project_path: Option<String>,
    pub watching: bool,
    pub busy: bool,
    pub queue_depth: usize,
}
