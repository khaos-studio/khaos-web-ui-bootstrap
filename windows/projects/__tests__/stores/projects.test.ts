import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useProjectsStore } from '../../stores/projects'
import type { Project } from '@khaos/shared'

// Mock the Tauri invoke function
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

import { invoke } from '@tauri-apps/api/core'

describe('Projects Store', () => {
  beforeEach(() => {
    // Create a fresh pinia instance and make it active
    setActivePinia(createPinia())
    // Clear all mocks before each test
    vi.clearAllMocks()
  })

  // Helper to create mock projects
  function createMockProject(overrides: Partial<Project> = {}): Project {
    return {
      id: 'proj-1',
      title: 'Test Project',
      author: 'Test Author',
      path: '/home/user/projects/test',
      scene_count: 5,
      modified: Math.floor(Date.now() / 1000),
      ...overrides,
    }
  }

  // ===== STATE TESTS =====
  describe('Initial State', () => {
    it('should initialize with empty state', () => {
      const store = useProjectsStore()

      expect(store.projects).toEqual([])
      expect(store.search).toBe('')
      expect(store.selected).toBeNull()
      expect(store.loading).toBe(false)
      expect(store.error).toBeNull()
    })
  })

  // ===== ACTIONS: loadProjects =====
  describe('loadProjects action', () => {
    it('should load projects and populate state', async () => {
      const mockProjects = [
        createMockProject({ id: 'proj-1', title: 'First Project' }),
        createMockProject({ id: 'proj-2', title: 'Second Project' }),
      ]

      ;(invoke as any).mockResolvedValue(mockProjects)

      const store = useProjectsStore()
      await store.loadProjects()

      expect(store.projects).toEqual(mockProjects)
      expect(store.loading).toBe(false)
      expect(store.error).toBeNull()
    })

    it('should set loading flag during load', async () => {
      ;(invoke as any).mockImplementation(
        () => new Promise(resolve => setTimeout(() => resolve([]), 100))
      )

      const store = useProjectsStore()
      const loadPromise = store.loadProjects()

      expect(store.loading).toBe(true)

      await loadPromise
      expect(store.loading).toBe(false)
    })

    it('should handle errors gracefully', async () => {
      const errorMessage = 'Failed to discover projects'
      ;(invoke as any).mockRejectedValue(new Error(errorMessage))

      const store = useProjectsStore()
      await store.loadProjects()

      expect(store.projects).toEqual([])
      expect(store.error).toContain('Failed to discover projects')
      expect(store.loading).toBe(false)
    })

    it('should clear previous error on successful load', async () => {
      const store = useProjectsStore()

      // Simulate a previous error
      store.error = 'Previous error'

      ;(invoke as any).mockResolvedValue([createMockProject()])

      await store.loadProjects()

      expect(store.error).toBeNull()
    })
  })

  // ===== ACTIONS: setSearch =====
  describe('setSearch action', () => {
    it('should update search state', () => {
      const store = useProjectsStore()

      store.setSearch('test query')

      expect(store.search).toBe('test query')
    })

    it('should allow empty search', () => {
      const store = useProjectsStore()

      store.setSearch('something')
      store.setSearch('')

      expect(store.search).toBe('')
    })

    it('should handle special characters', () => {
      const store = useProjectsStore()

      store.setSearch('test@#$%query')

      expect(store.search).toBe('test@#$%query')
    })
  })

  // ===== ACTIONS: selectProject =====
  describe('selectProject action', () => {
    it('should update selected project', () => {
      const store = useProjectsStore()
      const project = createMockProject()

      store.selectProject(project)

      expect(store.selected).toEqual(project)
    })

    it('should replace previously selected project', () => {
      const store = useProjectsStore()
      const project1 = createMockProject({ id: 'proj-1' })
      const project2 = createMockProject({ id: 'proj-2' })

      store.selectProject(project1)
      expect(store.selected?.id).toBe('proj-1')

      store.selectProject(project2)
      expect(store.selected?.id).toBe('proj-2')
    })
  })

  // ===== GETTERS: filteredProjects =====
  describe('filteredProjects getter', () => {
    beforeEach(() => {
      const store = useProjectsStore()
      store.projects = [
        createMockProject({ id: 'proj-1', title: 'Screenplay Alpha', author: 'John Doe' }),
        createMockProject({ id: 'proj-2', title: 'Story Beta', author: 'Jane Smith' }),
        createMockProject({ id: 'proj-3', title: 'Script Gamma', author: 'Bob Johnson' }),
      ]
    })

    it('should return all projects when search is empty', () => {
      const store = useProjectsStore()
      store.search = ''

      expect(store.filteredProjects).toHaveLength(3)
    })

    it('should filter by title (case-insensitive)', () => {
      const store = useProjectsStore()
      store.search = 'screenplay'

      const filtered = store.filteredProjects
      expect(filtered).toHaveLength(1)
      expect(filtered[0].title).toBe('Screenplay Alpha')
    })

    it('should filter by author (case-insensitive)', () => {
      const store = useProjectsStore()
      store.search = 'john'

      const filtered = store.filteredProjects
      expect(filtered).toHaveLength(2) // John Doe and Bob Johnson
      expect(filtered.map(p => p.author)).toContain('John Doe')
    })

    it('should filter by path', () => {
      const store = useProjectsStore()
      // Update one project with specific path
      store.projects[0].path = '/home/user/projects/screenplay'
      store.projects[1].path = '/home/user/projects/story'

      store.search = '/screenplay'

      const filtered = store.filteredProjects
      expect(filtered).toHaveLength(1)
      expect(filtered[0].id).toBe('proj-1')
    })

    it('should support multiple filter matches', () => {
      const store = useProjectsStore()
      store.search = 'script'

      const filtered = store.filteredProjects
      expect(filtered).toHaveLength(1)
      expect(filtered[0].title).toContain('Script')
    })

    it('should return empty array when no matches', () => {
      const store = useProjectsStore()
      store.search = 'nonexistent'

      expect(store.filteredProjects).toHaveLength(0)
    })

    it('should handle special characters in search', () => {
      const store = useProjectsStore()
      store.search = '@#$%'

      expect(store.filteredProjects).toHaveLength(0)
    })

    it('should handle projects with missing author', () => {
      const store = useProjectsStore()
      store.projects = [
        createMockProject({ id: 'proj-1', title: 'With Author', author: 'John' }),
        createMockProject({ id: 'proj-2', title: 'No Author', author: undefined }),
      ]

      store.search = 'john'

      const filtered = store.filteredProjects
      expect(filtered).toHaveLength(1)
      expect(filtered[0].author).toBe('John')
    })
  })

  // ===== GETTERS: recentProjects =====
  describe('recentProjects getter', () => {
    it('should return projects sorted by modification time (newest first)', () => {
      const now = Math.floor(Date.now() / 1000)
      const store = useProjectsStore()

      store.projects = [
        createMockProject({ id: 'proj-1', title: 'Old', modified: now - 1000 }),
        createMockProject({ id: 'proj-2', title: 'New', modified: now }),
        createMockProject({ id: 'proj-3', title: 'Medium', modified: now - 500 }),
      ]

      const recent = store.recentProjects
      expect(recent[0].id).toBe('proj-2') // newest
      expect(recent[1].id).toBe('proj-3') // medium
      expect(recent[2].id).toBe('proj-1') // oldest
    })

    it('should limit to 5 projects', () => {
      const now = Math.floor(Date.now() / 1000)
      const store = useProjectsStore()

      store.projects = Array.from({ length: 10 }, (_, i) =>
        createMockProject({
          id: `proj-${i}`,
          modified: now - i
        })
      )

      expect(store.recentProjects).toHaveLength(5)
    })

    it('should return fewer than 5 if fewer projects exist', () => {
      const store = useProjectsStore()

      store.projects = [
        createMockProject({ id: 'proj-1' }),
        createMockProject({ id: 'proj-2' }),
      ]

      expect(store.recentProjects).toHaveLength(2)
    })

    it('should return empty array when no projects', () => {
      const store = useProjectsStore()
      store.projects = []

      expect(store.recentProjects).toHaveLength(0)
    })
  })

  // ===== GETTERS: hasProjects =====
  describe('hasProjects getter', () => {
    it('should return true when projects exist', () => {
      const store = useProjectsStore()
      store.projects = [createMockProject()]

      expect(store.hasProjects).toBe(true)
    })

    it('should return false when no projects', () => {
      const store = useProjectsStore()
      store.projects = []

      expect(store.hasProjects).toBe(false)
    })
  })

  // ===== GETTERS: isSearching =====
  describe('isSearching getter', () => {
    it('should return true when search has content', () => {
      const store = useProjectsStore()
      store.search = 'test'

      expect(store.isSearching).toBe(true)
    })

    it('should return false when search is empty', () => {
      const store = useProjectsStore()
      store.search = ''

      expect(store.isSearching).toBe(false)
    })
  })

  // ===== ERROR HANDLING =====
  describe('Error Handling', () => {
    it('should store error message when invoke fails', async () => {
      const errorMsg = 'Discover service unavailable'
      ;(invoke as any).mockRejectedValue(new Error(errorMsg))

      const store = useProjectsStore()
      await store.loadProjects()

      expect(store.error).toContain('Discover service unavailable')
    })

    it('should handle non-Error exceptions', async () => {
      ;(invoke as any).mockRejectedValue('String error')

      const store = useProjectsStore()
      await store.loadProjects()

      expect(store.error).toBe('String error')
    })
  })
})
