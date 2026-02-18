// Shared TypeScript types
export interface Project {
  id: string
  title: string
  author?: string
  path: string
  scene_count: number
  modified: number
}

export interface SystemInfo {
  platform: string
  arch: string
  daemon_connected: boolean
}

// Import wizard types
export type ImportStep = 'file' | 'title' | 'confirm' | 'collision' | 'execute' | 'result'
export type ImportStatus = 'idle' | 'in_progress' | 'success' | 'failed'

export interface CollisionInfo {
  existing_path: string
  suggested_names: string[]
}

export interface ImportProgress {
  phase: string
  line: string
}

export interface ImportResult {
  success: boolean
  project_id?: string
  output_path?: string
  error?: string
}

// Settings types
export interface SettingsConfig {
  provider: string
  model?: string
  projects_root?: string
}

export interface ProviderInfo {
  id: string
  title: string
  description: string
  default_model: string
  requires_key: boolean
  env_var?: string
}

export interface SettingsLoadResult {
  config: SettingsConfig
  providers: ProviderInfo[]
}

export interface ProviderValidationResult {
  valid: boolean
  checks_run: string[]
  errors: string[]
}

export interface DaemonCheckResult {
  reachable: boolean
  version?: string
  error?: string
}

export interface OllamaStatus {
  installed: boolean
  model_available: boolean
  error?: string
}

// Dashboard types

export interface ProjectSummary {
  scenes: number
  characters: number
  locations: number
  compositions: number
}

export interface SceneSummary {
  id: string
  index: number
  slugline: string
  duration: string
  word_count: number
  line_count: number
  characters: string[]
}

export interface CharacterSummary {
  id: string
  name: string
  dialogue_lines: number
  words: number
  scene_count: number
  percentage: number
}

export interface LocationSummary {
  id: string
  name: string
  scene_count: number
  page_count: number
}

export interface SceneAnalysis {
  title?: string
  scene_number?: string
  summary?: string
  narrative_role?: string
  emotional_tone?: string
  stakes?: string
  themes: string[]
  theme_details?: string
  plot_beats: string[]
  heading?: string
  pacing?: string
  estimated_runtime?: string
  length_pages?: string
  scene_type?: string
  time_of_day?: string
  genre_tone?: string
  style_notes?: string
  speaking_chars: string[]
  non_speaking_chars: string[]
  non_speaking_details: string[]
  callbacks: string[]
  continues_from: string[]
  sets_up: string[]
  strengths: string[]
  development_areas: string[]
  visual_symbols: string[]
  location_significance?: string
}

export interface CharacterAnalysis {
  summary?: string
  arc?: string
  emotional_journey?: string
  end_state?: string
  arc_quality?: string
  key_turning_points: string[]
  traits: string[]
  goals: string[]
  conflicts: string[]
  background?: string
  stakes?: string
  contradictions?: string
  vulnerabilities: string[]
  relationships: string[]
  dialogue_voice?: string
  dialogue_function?: string
  dialogue_patterns: string[]
  dialogue_subtext?: string
  themes: string[]
  thematic_role?: string
  symbolic_elements: string[]
  narrative_role?: string
  narrative_importance?: string
  narrative_plot_function?: string
  genre_fit?: string
}

export interface LocationAnalysis {
  summary?: string
  atmosphere?: string
  environment?: string
  visual_context?: string
  significance?: string
  traits: string[]
  changes: string[]
  narrative_role?: string
  story_role?: string
  plot_anchors: string[]
  symbols: string[]
  themes: string[]
  thematic_role?: string
  location_type?: string
  region?: string
  classification?: string
  character_connections: string[]
  production_notes?: string
  structural_observations?: string
}

export type AnalysisState = 'pending' | 'analyzing' | 'analyzed' | 'failed'
export type DashboardSection = 'scenes' | 'characters' | 'locations'

export interface AnalysisResult {
  success: boolean
  item_type: string
  item_id: string
  error?: string
}

export interface AnalysisIndex {
  scenes: string[]
  characters: string[]
  locations: string[]
}

export interface SceneDetail {
  summary: SceneSummary
  analysis?: SceneAnalysis
}

export interface CharacterDetail {
  summary: CharacterSummary
  analysis?: CharacterAnalysis
}

export interface LocationDetail {
  summary: LocationSummary
  analysis?: LocationAnalysis
}

export interface DaemonStatus {
  running: boolean
  project_path?: string
  watching: boolean
  busy: boolean
  queue_depth: number
}
