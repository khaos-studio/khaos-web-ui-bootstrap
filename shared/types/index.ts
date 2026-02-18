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
