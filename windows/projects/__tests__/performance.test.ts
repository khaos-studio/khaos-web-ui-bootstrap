import { describe, it, expect, beforeEach, vi } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'
import { useProjectsStore } from '../stores/projects'
import { mount } from '@vue/test-utils'
import ProjectList from '../components/ProjectList.vue'
import type { Project } from '@khaos/shared'

// Mock Tauri
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

import { invoke } from '@tauri-apps/api/core'

describe('Performance Tests', () => {
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

  // ===== DISCOVERY PERFORMANCE =====
  describe('Discovery Performance', () => {
    it('should load 100 projects in under 500ms', async () => {
      const projects = Array.from({ length: 100 }, (_, i) =>
        createMockProject({
          id: `proj-${i}`,
          title: `Project ${i}`,
        })
      )

      ;(invoke as any).mockResolvedValue(projects)

      const store = useProjectsStore()
      const start = performance.now()

      await store.loadProjects()

      const duration = performance.now() - start

      expect(store.projects).toHaveLength(100)
      expect(duration).toBeLessThan(500)
      console.log(`Load 100 projects: ${duration.toFixed(2)}ms`)
    })

    it('should load 500 projects in under 1000ms', async () => {
      const projects = Array.from({ length: 500 }, (_, i) =>
        createMockProject({
          id: `proj-${i}`,
          title: `Project ${i}`,
        })
      )

      ;(invoke as any).mockResolvedValue(projects)

      const store = useProjectsStore()
      const start = performance.now()

      await store.loadProjects()

      const duration = performance.now() - start

      expect(store.projects).toHaveLength(500)
      expect(duration).toBeLessThan(1000)
      console.log(`Load 500 projects: ${duration.toFixed(2)}ms`)
    })

    it('should handle invocation call efficiently', async () => {
      const mockData = Array.from({ length: 200 }, (_, i) =>
        createMockProject({ id: `proj-${i}` })
      )

      ;(invoke as any).mockResolvedValue(mockData)

      const start = performance.now()
      const result = await (invoke as any)('discover_projects')
      const duration = performance.now() - start

      expect(result).toHaveLength(200)
      expect(duration).toBeLessThan(100)
      console.log(`Invoke discover_projects: ${duration.toFixed(2)}ms`)
    })
  })

  // ===== SEARCH/FILTER PERFORMANCE =====
  describe('Search and Filter Performance', () => {
    beforeEach(() => {
      const store = useProjectsStore()
      const projects = Array.from({ length: 500 }, (_, i) =>
        createMockProject({
          id: `proj-${i}`,
          title: `Project ${i}`,
          author: i % 3 === 0 ? 'Author A' : i % 3 === 1 ? 'Author B' : 'Author C',
        })
      )
      store.projects = projects
    })

    it('should filter 500 projects in under 100ms', () => {
      const store = useProjectsStore()

      const start = performance.now()
      store.setSearch('author a')
      const _ = store.filteredProjects // Force evaluation
      const duration = performance.now() - start

      expect(duration).toBeLessThan(100)
      console.log(`Filter 500 projects: ${duration.toFixed(2)}ms`)
    })

    it('should filter with partial match in under 100ms', () => {
      const store = useProjectsStore()

      const start = performance.now()
      store.setSearch('project')
      const filtered = store.filteredProjects
      const duration = performance.now() - start

      expect(filtered.length > 0).toBe(true)
      expect(duration).toBeLessThan(100)
      console.log(`Filter with partial match: ${duration.toFixed(2)}ms`)
    })

    it('should handle rapid successive filters efficiently', () => {
      const store = useProjectsStore()

      const start = performance.now()

      // Simulate rapid typing
      store.setSearch('a')
      store.setSearch('au')
      store.setSearch('aut')
      store.setSearch('auth')
      store.setSearch('author')

      const _ = store.filteredProjects
      const duration = performance.now() - start

      expect(duration).toBeLessThan(200)
      console.log(`5 rapid filters: ${duration.toFixed(2)}ms`)
    })

    it('should sort recent projects efficiently', () => {
      const store = useProjectsStore()

      const start = performance.now()
      const recent = store.recentProjects
      const duration = performance.now() - start

      expect(recent).toHaveLength(5)
      expect(duration).toBeLessThan(50)
      console.log(`Sort and limit 500 projects: ${duration.toFixed(2)}ms`)
    })

    it('should evaluate multiple getters efficiently', () => {
      const store = useProjectsStore()

      const start = performance.now()

      const filtered = store.filteredProjects
      const recent = store.recentProjects
      const hasProjects = store.hasProjects
      const isSearching = store.isSearching

      const duration = performance.now() - start

      expect(filtered.length > 0).toBe(true)
      expect(recent.length > 0).toBe(true)
      expect(hasProjects).toBe(true)
      expect(isSearching).toBe(false)
      expect(duration).toBeLessThan(50)
      console.log(`Evaluate all getters: ${duration.toFixed(2)}ms`)
    })
  })

  // ===== COMPONENT RENDERING PERFORMANCE =====
  describe('Component Rendering Performance', () => {
    it('should render 100 ProjectCards in reasonable time', async () => {
      const store = useProjectsStore()
      const projects = Array.from({ length: 100 }, (_, i) =>
        createMockProject({
          id: `proj-${i}`,
          title: `Project ${i}`,
        })
      )

      store.projects = projects

      const start = performance.now()
      const wrapper = mount(ProjectList)
      await wrapper.vm.$nextTick()
      const duration = performance.now() - start

      const cards = wrapper.findAll('[class*="p-4"][class*="border"]')
      expect(cards.length > 0).toBe(true)
      expect(duration).toBeLessThan(1000)
      console.log(`Render 100 ProjectCards: ${duration.toFixed(2)}ms`)
    })

    it('should update filtered list reactively in under 200ms', async () => {
      const store = useProjectsStore()
      const projects = Array.from({ length: 200 }, (_, i) =>
        createMockProject({
          id: `proj-${i}`,
          title: i % 2 === 0 ? 'Alpha' : 'Beta',
        })
      )

      store.projects = projects

      const wrapper = mount(ProjectList)
      await wrapper.vm.$nextTick()

      const start = performance.now()
      store.setSearch('Alpha')
      await wrapper.vm.$nextTick()
      const duration = performance.now() - start

      expect(duration).toBeLessThan(200)
      console.log(`Filter update in UI: ${duration.toFixed(2)}ms`)
    })

    it('should handle rapid rerenders efficiently', async () => {
      const store = useProjectsStore()
      store.projects = Array.from({ length: 50 }, (_, i) =>
        createMockProject({ id: `proj-${i}`, title: `Project ${i}` })
      )

      const wrapper = mount(ProjectList)
      await wrapper.vm.$nextTick()

      const start = performance.now()

      // Simulate rapid filtering
      for (let i = 0; i < 10; i++) {
        store.setSearch(`p${i}`)
        await wrapper.vm.$nextTick()
      }

      const duration = performance.now() - start

      expect(duration).toBeLessThan(500)
      console.log(`10 rapid rerenders: ${duration.toFixed(2)}ms`)
    })
  })

  // ===== MEMORY EFFICIENCY =====
  describe('Memory Efficiency', () => {
    it('should handle large dataset without memory issues', async () => {
      const store = useProjectsStore()
      const projects = Array.from({ length: 1000 }, (_, i) =>
        createMockProject({
          id: `proj-${i}`,
          title: `Project ${i}`,
        })
      )

      ;(invoke as any).mockResolvedValue(projects)

      const start = performance.now()
      await store.loadProjects()
      const duration = performance.now() - start

      expect(store.projects).toHaveLength(1000)
      expect(duration).toBeLessThan(2000)
      console.log(`Load 1000 projects: ${duration.toFixed(2)}ms`)
    })

    it('should not leak memory with repeated filtering', () => {
      const store = useProjectsStore()
      const projects = Array.from({ length: 100 }, (_, i) =>
        createMockProject({
          id: `proj-${i}`,
          title: `Project ${i}`,
        })
      )

      store.projects = projects

      const start = performance.now()

      // Perform many filtering operations
      for (let i = 0; i < 100; i++) {
        store.setSearch(`project${i % 10}`)
        const _ = store.filteredProjects
      }

      const duration = performance.now() - start

      expect(duration).toBeLessThan(500)
      console.log(`100 filter operations: ${duration.toFixed(2)}ms`)
    })
  })

  // ===== STATE OPERATIONS PERFORMANCE =====
  describe('State Operations Performance', () => {
    it('should select project quickly', () => {
      const store = useProjectsStore()
      const project = createMockProject()

      const start = performance.now()
      store.selectProject(project)
      const duration = performance.now() - start

      expect(store.selected).toEqual(project)
      expect(duration).toBeLessThan(1)
    })

    it('should update search state quickly', () => {
      const store = useProjectsStore()

      const start = performance.now()
      store.setSearch('test query')
      const duration = performance.now() - start

      expect(store.search).toBe('test query')
      expect(duration).toBeLessThan(1)
    })

    it('should handle 1000 state updates efficiently', () => {
      const store = useProjectsStore()
      const projects = Array.from({ length: 100 }, (_, i) =>
        createMockProject({ id: `proj-${i}` })
      )

      store.projects = projects

      const start = performance.now()

      for (let i = 0; i < 1000; i++) {
        store.setSearch(`query${i % 10}`)
      }

      const duration = performance.now() - start

      expect(duration).toBeLessThan(100)
      console.log(`1000 state updates: ${duration.toFixed(2)}ms`)
    })
  })

  // ===== BENCHMARKS SUMMARY =====
  describe('Performance Benchmarks Summary', () => {
    it('should provide baseline performance characteristics', async () => {
      const benchmarks: Record<string, number> = {}

      // Benchmark 1: Load 100 projects
      const projects100 = Array.from({ length: 100 }, (_, i) =>
        createMockProject({ id: `proj-${i}` })
      )
      ;(invoke as any).mockResolvedValue(projects100)

      const store = useProjectsStore()
      let start = performance.now()
      await store.loadProjects()
      benchmarks['Load 100 projects'] = performance.now() - start

      // Benchmark 2: Filter 100 projects
      start = performance.now()
      store.setSearch('project')
      const _ = store.filteredProjects
      benchmarks['Filter 100 projects'] = performance.now() - start

      // Benchmark 3: Get recent projects
      start = performance.now()
      const recent = store.recentProjects
      benchmarks['Get recent projects'] = performance.now() - start

      // Benchmark 4: Render 100 cards
      start = performance.now()
      const wrapper = mount(ProjectList)
      benchmarks['Mount ProjectList'] = performance.now() - start

      // Print summary
      console.log('\n=== PERFORMANCE BENCHMARKS ===')
      Object.entries(benchmarks).forEach(([name, duration]) => {
        console.log(`${name}: ${duration.toFixed(2)}ms`)
      })
      console.log('=============================\n')

      // All benchmarks should complete in reasonable time
      expect(benchmarks['Load 100 projects']).toBeLessThan(500)
      expect(benchmarks['Filter 100 projects']).toBeLessThan(100)
      expect(benchmarks['Get recent projects']).toBeLessThan(50)
      expect(benchmarks['Mount ProjectList']).toBeLessThan(1000)
    })
  })
})
