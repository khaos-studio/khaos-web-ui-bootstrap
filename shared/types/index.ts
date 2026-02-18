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
