import { describe, it, expect, beforeEach, vi } from 'vitest'
import { mount } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import { useProjectsStore } from '../stores/projects'
import type { Project } from '@khaos/shared'

// Mock Tauri
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

import { invoke } from '@tauri-apps/api/core'

describe('Projects Integration Tests', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.clearAllMocks()
  })

  function createMockProject(overrides: Partial<Project> = {}): Project {
    return {
      id: `proj-${Math.random()}`,
      title: 'Test Project',
      author: 'Test Author',
      path: '/home/user/projects/test',
      scene_count: 5,
      modified: Math.floor(Date.now() / 1000),
      ...overrides,
    }
  }

  // ===== DISCOVERY FLOW =====
  describe('Discovery Flow', () => {
    it('should complete full discovery flow', async () => {
      const mockProjects = [
        createMockProject({ id: '1', title: 'First' }),
        createMockProject({ id: '2', title: 'Second' }),
      ]

      ;(invoke as any).mockResolvedValue(mockProjects)

      const store = useProjectsStore()

      // Step 1: Load projects
      expect(store.loading).toBe(false)
      const loadPromise = store.loadProjects()
      expect(store.loading).toBe(true)

      // Step 2: Wait for completion
      await loadPromise
      expect(store.loading).toBe(false)

      // Step 3: Verify state is populated
      expect(store.projects).toEqual(mockProjects)
      expect(store.hasProjects).toBe(true)
      expect(store.error).toBeNull()

      // Step 4: Verify UI would render correctly
      expect(store.filteredProjects).toHaveLength(2)
    })

    it('should handle discovery errors gracefully', async () => {
      const errorMsg = 'Projects directory not found'
      ;(invoke as any).mockRejectedValue(new Error(errorMsg))

      const store = useProjectsStore()

      await store.loadProjects()

      expect(store.projects).toEqual([])
      expect(store.error).toContain('Projects directory not found')
      expect(store.loading).toBe(false)
    })

    it('should be called on component mount', async () => {
      // This would be verified in app.vue integration
      // Here we verify the store action works
      const mockProjects = [createMockProject()]
      ;(invoke as any).mockResolvedValue(mockProjects)

      const store = useProjectsStore()
      await store.loadProjects()

      expect(store.projects).toEqual(mockProjects)
    })
  })

  // ===== SEARCH FLOW =====
  describe('Search Flow', () => {
    beforeEach(() => {
      const store = useProjectsStore()
      store.projects = [
        createMockProject({ id: '1', title: 'Screenplay One', author: 'Author A' }),
        createMockProject({ id: '2', title: 'Story Two', author: 'Author B' }),
        createMockProject({ id: '3', title: 'Script Three', author: 'Author A' }),
      ]
    })

    it('should filter projects when user types', async () => {
      const store = useProjectsStore()

      expect(store.filteredProjects).toHaveLength(3)

      store.setSearch('screenplay')

      expect(store.filteredProjects).toHaveLength(1)
      expect(store.filteredProjects[0].title).toBe('Screenplay One')
    })

    it('should update filtering when search changes', async () => {
      const store = useProjectsStore()

      store.setSearch('author a')
      expect(store.filteredProjects).toHaveLength(2)

      store.setSearch('author b')
      expect(store.filteredProjects).toHaveLength(1)

      store.setSearch('')
      expect(store.filteredProjects).toHaveLength(3)
    })

    it('should be case-insensitive', () => {
      const store = useProjectsStore()

      store.setSearch('SCREENPLAY')
      expect(store.filteredProjects).toHaveLength(1)

      store.setSearch('screenplay')
      expect(store.filteredProjects).toHaveLength(1)

      store.setSearch('ScReEnPlAy')
      expect(store.filteredProjects).toHaveLength(1)
    })

    it('should handle partial matches', () => {
      const store = useProjectsStore()

      store.setSearch('play')
      expect(store.filteredProjects).toHaveLength(1)

      store.setSearch('story')
      expect(store.filteredProjects).toHaveLength(1)

      store.setSearch('script')
      expect(store.filteredProjects).toHaveLength(1)
    })

    it('should clear filtering when search is cleared', () => {
      const store = useProjectsStore()

      store.setSearch('screenplay')
      expect(store.filteredProjects).toHaveLength(1)

      store.setSearch('')
      expect(store.filteredProjects).toHaveLength(3)
    })
  })

  // ===== PROJECT SELECTION FLOW =====
  describe('Project Selection Flow', () => {
    it('should select project and update state', () => {
      const store = useProjectsStore()
      const project = createMockProject({ id: '1', title: 'Selected' })

      expect(store.selected).toBeNull()

      store.selectProject(project)

      expect(store.selected).toEqual(project)
      expect(store.selected?.id).toBe('1')
    })

    it('should handle consecutive selections', () => {
      const store = useProjectsStore()
      const project1 = createMockProject({ id: '1' })
      const project2 = createMockProject({ id: '2' })

      store.selectProject(project1)
      expect(store.selected?.id).toBe('1')

      store.selectProject(project2)
      expect(store.selected?.id).toBe('2')
    })

    it('should preserve selected project during filtering', () => {
      const store = useProjectsStore()
      const selectedProject = createMockProject({
        id: '1',
        title: 'Screenplay',
      })

      store.projects = [
        selectedProject,
        createMockProject({ id: '2', title: 'Story' }),
      ]

      store.selectProject(selectedProject)
      expect(store.selected?.id).toBe('1')

      store.setSearch('story')
      expect(store.selected?.id).toBe('1') // Still selected
      expect(store.filteredProjects).not.toContain(selectedProject) // But not in filtered list
    })
  })

  // ===== RECENT PROJECTS FLOW =====
  describe('Recent Projects Flow', () => {
    it('should show most recent projects first', async () => {
      const store = useProjectsStore()
      const now = Math.floor(Date.now() / 1000)

      store.projects = [
        createMockProject({ id: '1', title: 'Oldest', modified: now - 1000 }),
        createMockProject({ id: '2', title: 'Middle', modified: now - 500 }),
        createMockProject({ id: '3', title: 'Newest', modified: now }),
      ]

      const recent = store.recentProjects
      expect(recent[0].id).toBe('3') // Newest
      expect(recent[1].id).toBe('2') // Middle
      expect(recent[2].id).toBe('1') // Oldest
    })

    it('should limit recent projects to 5', () => {
      const store = useProjectsStore()
      const now = Math.floor(Date.now() / 1000)

      store.projects = Array.from({ length: 10 }, (_, i) =>
        createMockProject({
          id: String(i),
          modified: now - i,
        })
      )

      expect(store.recentProjects).toHaveLength(5)
    })

    it('should show recent projects when not searching', () => {
      const store = useProjectsStore()
      const now = Math.floor(Date.now() / 1000)

      store.projects = [
        createMockProject({ id: '1', modified: now }),
        createMockProject({ id: '2', modified: now - 1000 }),
      ]

      store.search = ''

      expect(store.isSearching).toBe(false)
      expect(store.recentProjects).toHaveLength(2)
    })

    it('should hide recent projects when searching', () => {
      const store = useProjectsStore()

      store.projects = [
        createMockProject({ id: '1', title: 'Alpha' }),
        createMockProject({ id: '2', title: 'Beta' }),
      ]

      store.search = 'Alpha'

      expect(store.isSearching).toBe(true)
      // In UI, recent projects section would be hidden
      // But the getter should still work
      expect(store.recentProjects).toHaveLength(2)
    })
  })

  // ===== COMBINED WORKFLOWS =====
  describe('Combined Workflows', () => {
    it('should handle load -> search -> select flow', async () => {
      const mockProjects = [
        createMockProject({ id: '1', title: 'Screenplay One' }),
        createMockProject({ id: '2', title: 'Story Two' }),
      ]

      ;(invoke as any).mockResolvedValue(mockProjects)

      const store = useProjectsStore()

      // Step 1: Load
      await store.loadProjects()
      expect(store.projects).toHaveLength(2)

      // Step 2: Search
      store.setSearch('screenplay')
      expect(store.filteredProjects).toHaveLength(1)

      // Step 3: Select from filtered results
      store.selectProject(store.filteredProjects[0])
      expect(store.selected?.title).toBe('Screenplay One')
    })

    it('should handle error recovery flow', async () => {
      ;(invoke as any).mockRejectedValueOnce(new Error('Network error'))

      const store = useProjectsStore()

      // Step 1: Failed load
      await store.loadProjects()
      expect(store.error).toContain('Network error')
      expect(store.projects).toHaveLength(0)

      // Step 2: Retry
      const mockProjects = [createMockProject()]
      ;(invoke as any).mockResolvedValueOnce(mockProjects)

      await store.loadProjects()
      expect(store.error).toBeNull()
      expect(store.projects).toHaveLength(1)
    })

    it('should maintain state integrity across operations', () => {
      const store = useProjectsStore()
      const project1 = createMockProject({ id: '1', title: 'First' })
      const project2 = createMockProject({ id: '2', title: 'Second' })

      store.projects = [project1, project2]
      store.selectProject(project1)
      store.setSearch('first')

      // All state should be consistent
      expect(store.selected?.id).toBe('1')
      expect(store.search).toBe('first')
      expect(store.filteredProjects).toHaveLength(1)
      expect(store.projects).toHaveLength(2)
    })
  })

  // ===== EDGE CASE FLOWS =====
  describe('Edge Case Flows', () => {
    it('should handle empty project collection', async () => {
      ;(invoke as any).mockResolvedValue([])

      const store = useProjectsStore()
      await store.loadProjects()

      expect(store.projects).toHaveLength(0)
      expect(store.hasProjects).toBe(false)
      expect(store.filteredProjects).toHaveLength(0)
      expect(store.recentProjects).toHaveLength(0)
    })

    it('should handle single project workflow', () => {
      const store = useProjectsStore()
      const project = createMockProject()

      store.projects = [project]
      store.selectProject(project)

      expect(store.hasProjects).toBe(true)
      expect(store.recentProjects).toHaveLength(1)
      expect(store.selected).toEqual(project)
    })

    it('should handle rapid state changes', () => {
      const store = useProjectsStore()
      const projects = [
        createMockProject({ id: '1', title: 'A' }),
        createMockProject({ id: '2', title: 'B' }),
      ]

      store.projects = projects

      // Rapid search changes
      store.setSearch('a')
      store.setSearch('b')
      store.setSearch('')
      store.setSearch('a')

      expect(store.search).toBe('a')
      expect(store.filteredProjects).toHaveLength(1)
    })

    it('should handle project with all missing optional fields', () => {
      const store = useProjectsStore()
      const project: Project = {
        id: 'minimal',
        title: 'Minimal Project',
        path: '/path',
        scene_count: 0,
        modified: Math.floor(Date.now() / 1000),
      }

      store.projects = [project]
      expect(store.projects).toHaveLength(1)
      expect(store.projects[0].author).toBeUndefined()
    })
  })
})
