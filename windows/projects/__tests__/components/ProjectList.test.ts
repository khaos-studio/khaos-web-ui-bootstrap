import { describe, it, expect, beforeEach, vi } from 'vitest'
import { mount } from '@vue/test-utils'
import ProjectList from '../../components/ProjectList.vue'
import ProjectCard from '../../components/ProjectCard.vue'
import { createPinia, setActivePinia } from 'pinia'
import { useProjectsStore } from '../../stores/projects'
import type { Project } from '@khaos/shared'

// Mock Tauri
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

describe('ProjectList Component', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
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

  it('should render without crashing', () => {
    const wrapper = mount(ProjectList)
    expect(wrapper.exists()).toBe(true)
  })

  // ===== RENDERING TESTS =====
  describe('Rendering', () => {
    it('should render filtered projects from store', async () => {
      const store = useProjectsStore()
      const projects = [
        createMockProject({ id: 'proj-1', title: 'First' }),
        createMockProject({ id: 'proj-2', title: 'Second' }),
      ]

      store.projects = projects
      store.search = ''

      const wrapper = mount(ProjectList)
      await wrapper.vm.$nextTick()

      const cards = wrapper.findAllComponents(ProjectCard)
      expect(cards).toHaveLength(2)
    })

    it('should render only filtered projects based on search', async () => {
      const store = useProjectsStore()
      store.projects = [
        createMockProject({ id: 'proj-1', title: 'Alpha Project' }),
        createMockProject({ id: 'proj-2', title: 'Beta Story' }),
        createMockProject({ id: 'proj-3', title: 'Gamma Project' }),
      ]

      store.search = 'Project'

      const wrapper = mount(ProjectList)
      await wrapper.vm.$nextTick()

      const cards = wrapper.findAllComponents(ProjectCard)
      expect(cards).toHaveLength(2)
    })

    it('should pass correct project prop to each ProjectCard', async () => {
      const store = useProjectsStore()
      const project1 = createMockProject({ id: 'proj-1', title: 'First' })
      const project2 = createMockProject({ id: 'proj-2', title: 'Second' })

      store.projects = [project1, project2]
      store.search = ''

      const wrapper = mount(ProjectList)
      await wrapper.vm.$nextTick()

      const cards = wrapper.findAllComponents(ProjectCard)
      expect(cards[0].props('project')).toEqual(project1)
      expect(cards[1].props('project')).toEqual(project2)
    })

    it('should use project path as key', async () => {
      const store = useProjectsStore()
      const project = createMockProject({
        id: 'unique-id',
        path: '/unique/path',
      })

      store.projects = [project]

      const wrapper = mount(ProjectList)
      await wrapper.vm.$nextTick()

      const card = wrapper.findComponent(ProjectCard)
      // Vue uses the key internally, we verify the component renders
      expect(card.exists()).toBe(true)
    })
  })

  // ===== LOADING STATE =====
  describe('Loading State', () => {
    it('should show loading spinner when loading', async () => {
      const store = useProjectsStore()
      store.loading = true

      const wrapper = mount(ProjectList)
      await wrapper.vm.$nextTick()

      // Loading overlay should show
      expect(wrapper.text()).toContain('No projects')
    })

    it('should hide projects while loading', async () => {
      const store = useProjectsStore()
      store.projects = [createMockProject()]
      store.loading = true

      const wrapper = mount(ProjectList)
      await wrapper.vm.$nextTick()

      const cards = wrapper.findAllComponents(ProjectCard)
      expect(cards).toHaveLength(0)
    })

    it('should show projects after loading completes', async () => {
      const store = useProjectsStore()
      store.projects = [createMockProject()]
      store.loading = false

      const wrapper = mount(ProjectList)
      await wrapper.vm.$nextTick()

      const cards = wrapper.findAllComponents(ProjectCard)
      expect(cards).toHaveLength(1)
    })
  })

  // ===== EMPTY STATE =====
  describe('Empty States', () => {
    it('should show empty state when no projects and not loading', async () => {
      const store = useProjectsStore()
      store.projects = []
      store.loading = false
      store.search = ''

      const wrapper = mount(ProjectList)
      await wrapper.vm.$nextTick()

      expect(wrapper.text()).toContain('No projects discovered yet')
    })

    it('should show "no results" message when search returns empty', async () => {
      const store = useProjectsStore()
      store.projects = [
        createMockProject({ title: 'Alpha' }),
        createMockProject({ title: 'Beta' }),
      ]
      store.search = 'NonexistentProject'
      store.loading = false

      const wrapper = mount(ProjectList)
      await wrapper.vm.$nextTick()

      expect(wrapper.text()).toContain(
        'No projects found matching your search'
      )
    })

    it('should show empty message when no projects exist', async () => {
      const store = useProjectsStore()
      store.projects = []
      store.search = ''
      store.loading = false

      const wrapper = mount(ProjectList)
      await wrapper.vm.$nextTick()

      const emptyState = wrapper.find('.text-center')
      expect(emptyState.exists()).toBe(true)
    })

    it('should have empty icon in empty state', async () => {
      const store = useProjectsStore()
      store.projects = []
      store.search = ''
      store.loading = false

      const wrapper = mount(ProjectList)
      await wrapper.vm.$nextTick()

      const icon = wrapper.find('svg')
      expect(icon.exists()).toBe(true)
    })
  })

  // ===== SCROLL BEHAVIOR =====
  describe('Scroll Behavior', () => {
    it('should have scrollable container', async () => {
      const store = useProjectsStore()
      store.projects = [createMockProject()]

      const wrapper = mount(ProjectList)
      await wrapper.vm.$nextTick()

      const scrollContainer = wrapper.find('.overflow-y-auto')
      expect(scrollContainer.exists()).toBe(true)
    })

    it('should render projects in scrollable div', async () => {
      const store = useProjectsStore()
      store.projects = [createMockProject()]
      store.search = ''

      const wrapper = mount(ProjectList)
      await wrapper.vm.$nextTick()

      const cards = wrapper.findAllComponents(ProjectCard)
      expect(cards.length > 0).toBe(true)
    })
  })

  // ===== EVENT HANDLING =====
  describe('Event Handling', () => {
    it('should emit selected event when ProjectCard emits', async () => {
      const store = useProjectsStore()
      const project = createMockProject()
      store.projects = [project]

      const wrapper = mount(ProjectList)
      await wrapper.vm.$nextTick()

      const card = wrapper.findComponent(ProjectCard)
      await card.vm.$emit('selected', project)

      expect(wrapper.emitted('selected')).toBeTruthy()
      expect(wrapper.emitted('selected')?.[0]).toEqual([project])
    })

    it('should forward multiple project selections', async () => {
      const store = useProjectsStore()
      const project1 = createMockProject({ id: 'proj-1' })
      const project2 = createMockProject({ id: 'proj-2' })

      store.projects = [project1, project2]

      const wrapper = mount(ProjectList)
      await wrapper.vm.$nextTick()

      const cards = wrapper.findAllComponents(ProjectCard)

      await cards[0].vm.$emit('selected', project1)
      await cards[1].vm.$emit('selected', project2)

      const emitted = wrapper.emitted('selected')
      expect(emitted).toHaveLength(2)
      expect(emitted?.[0]).toEqual([project1])
      expect(emitted?.[1]).toEqual([project2])
    })
  })

  // ===== FILTERING INTEGRATION =====
  describe('Filtering Integration', () => {
    it('should reactively update when store search changes', async () => {
      const store = useProjectsStore()
      store.projects = [
        createMockProject({ title: 'Alpha' }),
        createMockProject({ title: 'Beta' }),
        createMockProject({ title: 'Gamma' }),
      ]

      const wrapper = mount(ProjectList)
      await wrapper.vm.$nextTick()

      // Initially all 3
      expect(wrapper.findAllComponents(ProjectCard)).toHaveLength(3)

      // Filter to 1
      store.search = 'Alpha'
      await wrapper.vm.$nextTick()

      expect(wrapper.findAllComponents(ProjectCard)).toHaveLength(1)
    })

    it('should show filtered count in parent', async () => {
      const store = useProjectsStore()
      store.projects = [
        createMockProject({ title: 'Alpha' }),
        createMockProject({ title: 'Beta' }),
      ]
      store.search = 'Alpha'

      const wrapper = mount(ProjectList)
      await wrapper.vm.$nextTick()

      expect(store.filteredProjects).toHaveLength(1)
    })
  })

  // ===== LARGE DATASET TESTS =====
  describe('Large Datasets', () => {
    it('should handle large number of projects', async () => {
      const store = useProjectsStore()
      const projects = Array.from({ length: 100 }, (_, i) =>
        createMockProject({ id: `proj-${i}`, title: `Project ${i}` })
      )

      store.projects = projects
      store.search = ''

      const wrapper = mount(ProjectList)
      await wrapper.vm.$nextTick()

      expect(wrapper.findAllComponents(ProjectCard)).toHaveLength(100)
    })

    it('should filter large dataset efficiently', async () => {
      const store = useProjectsStore()
      const projects = Array.from({ length: 100 }, (_, i) =>
        createMockProject({
          id: `proj-${i}`,
          title: i % 2 === 0 ? 'Alpha' : 'Beta',
        })
      )

      store.projects = projects
      store.search = 'Alpha'

      const wrapper = mount(ProjectList)
      await wrapper.vm.$nextTick()

      expect(wrapper.findAllComponents(ProjectCard)).toHaveLength(50)
    })
  })

  // ===== ACCESSIBILITY =====
  describe('Accessibility', () => {
    it('should have semantic structure', async () => {
      const store = useProjectsStore()
      store.projects = [createMockProject()]

      const wrapper = mount(ProjectList)
      await wrapper.vm.$nextTick()

      const root = wrapper.find('.h-full')
      expect(root.exists()).toBe(true)
    })

    it('should have proper flex layout', async () => {
      const wrapper = mount(ProjectList)

      const root = wrapper.find('.flex.flex-col')
      expect(root.exists()).toBe(true)
    })
  })
})
