import { defineStore } from 'pinia'
import { ref, computed, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'
import type { Project } from '@khaos/shared'

export const useProjectsStore = defineStore('projects', () => {
  // State
  const projects = ref<Project[]>([])
  const search = ref<string>('')
  const selected = ref<Project | null>(null)
  const loading = ref<boolean>(false)
  const error = ref<string | null>(null)

  // Actions
  const loadProjects = async (): Promise<void> => {
    loading.value = true
    error.value = null

    try {
      const result = await invoke<Project[]>('discover_projects')
      projects.value = result
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err)
      error.value = errorMessage
      console.error('Failed to load projects:', errorMessage)
    } finally {
      loading.value = false
    }
  }

  const setSearch = (query: string): void => {
    search.value = query
  }

  const selectProject = (project: Project): void => {
    selected.value = project
  }

  const deleteProject = async (projectId: string): Promise<void> => {
    try {
      await invoke<void>('delete_project', {
        projectId,
      })

      // Remove from projects list
      projects.value = projects.value.filter((p) => p.id !== projectId)
      console.log(`Project deleted: ${projectId}`)
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err)
      error.value = `Failed to delete project: ${errorMessage}`
      console.error('Failed to delete project:', errorMessage)
      throw err
    }
  }

  /**
   * Set project as active and handle recent projects + cross-window communication
   * This method:
   * 1. Calls backend to update recent projects list
   * 2. Emits app event to notify other windows (Dashboard)
   */
  const setActiveProject = async (project: Project): Promise<void> => {
    try {
      // Call backend to update recent projects and save to config
      await invoke<void>('set_active_project', {
        projectId: project.id,
      })

      // Get current window and emit app event for Dashboard window to listen
      const appWindow = getCurrentWindow()
      await appWindow.emit('app:project-selected', {
        projectId: project.id,
        projectTitle: project.title,
      })

      console.log(`Project selected and broadcast: ${project.title}`)
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err)
      error.value = `Failed to set active project: ${errorMessage}`
      console.error('Failed to set active project:', errorMessage)
    }
  }

  // Getters
  const filteredProjects = computed(() => {
    if (!search.value) {
      return projects.value
    }

    const queryLower = search.value.toLowerCase()
    return projects.value.filter((project) => {
      const titleMatch = project.title.toLowerCase().includes(queryLower)
      const authorMatch = project.author?.toLowerCase().includes(queryLower) ?? false
      const pathMatch = project.path.toLowerCase().includes(queryLower)

      return titleMatch || authorMatch || pathMatch
    })
  })

  const recentProjects = computed(() => {
    return [...projects.value]
      .sort((a, b) => b.modified - a.modified)
      .slice(0, 5)
  })

  const hasProjects = computed(() => {
    return projects.value.length > 0
  })

  const isSearching = computed(() => {
    return search.value.length > 0
  })

  // Watch for changes to selected project and trigger side effects
  watch(selected, async (newSelected) => {
    if (newSelected) {
      // When a project is selected, set it as active
      await setActiveProject(newSelected)
    }
  })

  return {
    // State
    projects,
    search,
    selected,
    loading,
    error,
    // Actions
    loadProjects,
    setSearch,
    selectProject,
    setActiveProject,
    deleteProject,
    // Getters
    filteredProjects,
    recentProjects,
    hasProjects,
    isSearching,
  }
})
