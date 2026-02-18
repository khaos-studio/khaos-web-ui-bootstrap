import { describe, it, expect, beforeEach, vi } from 'vitest'
import { mount } from '@vue/test-utils'
import ProjectCard from '../../components/ProjectCard.vue'
import { createPinia, setActivePinia } from 'pinia'
import type { Project } from '@khaos/shared'

// Mock Tauri
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

describe('ProjectCard Component', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

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

  it('should render project title', () => {
    const project = createMockProject({ title: 'My Screenplay' })
    const wrapper = mount(ProjectCard, {
      props: { project },
    })

    expect(wrapper.text()).toContain('My Screenplay')
  })

  it('should render author when present', () => {
    const project = createMockProject({ author: 'John Doe' })
    const wrapper = mount(ProjectCard, {
      props: { project },
    })

    expect(wrapper.text()).toContain('John Doe')
  })

  it('should not render author section when missing', () => {
    const project = createMockProject({ author: undefined })
    const wrapper = mount(ProjectCard, {
      props: { project },
    })

    const authorSection = wrapper.find('.text-gray-600')
    expect(authorSection.exists()).toBe(false)
  })

  it('should render path with truncation', () => {
    const project = createMockProject({
      path: '/very/long/path/to/project/directory/that/should/be/truncated',
    })
    const wrapper = mount(ProjectCard, {
      props: { project },
    })

    // Should have the path in the title attribute for tooltip
    const pathElement = wrapper.find('.truncate')
    expect(pathElement.attributes('title')).toBe(
      '/very/long/path/to/project/directory/that/should/be/truncated'
    )
  })

  it('should render scene count badge', () => {
    const project = createMockProject({ scene_count: 7 })
    const wrapper = mount(ProjectCard, {
      props: { project },
    })

    expect(wrapper.text()).toContain('7')
    expect(wrapper.text()).toContain('scenes')
  })

  it('should use singular "scene" for single scene', () => {
    const project = createMockProject({ scene_count: 1 })
    const wrapper = mount(ProjectCard, {
      props: { project },
    })

    expect(wrapper.text()).toContain('1')
    expect(wrapper.text()).toContain('scene')
    expect(wrapper.text()).not.toContain('scenes')
  })

  it('should use plural "scenes" for multiple scenes', () => {
    const project = createMockProject({ scene_count: 3 })
    const wrapper = mount(ProjectCard, {
      props: { project },
    })

    expect(wrapper.text()).toContain('3')
    expect(wrapper.text()).toContain('scenes')
  })

  // ===== RELATIVE TIME TESTS =====
  describe('Relative Time Formatting', () => {
    it('should show "just now" for very recent projects', () => {
      const now = Math.floor(Date.now() / 1000)
      const project = createMockProject({ modified: now - 30 }) // 30 seconds ago

      const wrapper = mount(ProjectCard, {
        props: { project },
      })

      expect(wrapper.text()).toContain('just now')
    })

    it('should show minutes for projects modified minutes ago', () => {
      const now = Math.floor(Date.now() / 1000)
      const project = createMockProject({ modified: now - 300 }) // 5 minutes ago

      const wrapper = mount(ProjectCard, {
        props: { project },
      })

      expect(wrapper.text()).toContain('minute')
      expect(wrapper.text()).toContain('ago')
    })

    it('should show singular "minute" for 1 minute', () => {
      const now = Math.floor(Date.now() / 1000)
      const project = createMockProject({ modified: now - 60 })

      const wrapper = mount(ProjectCard, {
        props: { project },
      })

      expect(wrapper.text()).toContain('1 minute ago')
    })

    it('should show plural "minutes" for multiple minutes', () => {
      const now = Math.floor(Date.now() / 1000)
      const project = createMockProject({ modified: now - 600 }) // 10 minutes

      const wrapper = mount(ProjectCard, {
        props: { project },
      })

      expect(wrapper.text()).toContain('10 minutes ago')
    })

    it('should show hours for projects modified hours ago', () => {
      const now = Math.floor(Date.now() / 1000)
      const project = createMockProject({ modified: now - 3600 * 2 }) // 2 hours

      const wrapper = mount(ProjectCard, {
        props: { project },
      })

      expect(wrapper.text()).toContain('2 hours ago')
    })

    it('should show singular "hour" for 1 hour', () => {
      const now = Math.floor(Date.now() / 1000)
      const project = createMockProject({ modified: now - 3600 })

      const wrapper = mount(ProjectCard, {
        props: { project },
      })

      expect(wrapper.text()).toContain('1 hour ago')
    })

    it('should show days for projects modified days ago', () => {
      const now = Math.floor(Date.now() / 1000)
      const project = createMockProject({ modified: now - 3600 * 24 * 5 }) // 5 days

      const wrapper = mount(ProjectCard, {
        props: { project },
      })

      expect(wrapper.text()).toContain('5 days ago')
    })

    it('should show singular "day" for 1 day', () => {
      const now = Math.floor(Date.now() / 1000)
      const project = createMockProject({ modified: now - 3600 * 24 })

      const wrapper = mount(ProjectCard, {
        props: { project },
      })

      expect(wrapper.text()).toContain('1 day ago')
    })

    it('should show formatted date for older projects', () => {
      const now = Math.floor(Date.now() / 1000)
      const twoMonthsAgo = now - 3600 * 24 * 60 // 60 days
      const project = createMockProject({ modified: twoMonthsAgo })

      const wrapper = mount(ProjectCard, {
        props: { project },
      })

      // Should contain a date, not "ago"
      const text = wrapper.text()
      expect(text).not.toContain('ago')
    })
  })

  // ===== EVENT TESTS =====
  describe('Selection Events', () => {
    it('should emit selected event when clicked', async () => {
      const project = createMockProject()
      const wrapper = mount(ProjectCard, {
        props: { project },
      })

      const card = wrapper.find('.p-4')
      await card.trigger('click')

      expect(wrapper.emitted('selected')).toBeTruthy()
      expect(wrapper.emitted('selected')?.[0]).toEqual([project])
    })

    it('should call store selectProject when clicked', async () => {
      const project = createMockProject()
      const wrapper = mount(ProjectCard, {
        props: { project },
      })

      // Mock the store
      const { useProjectsStore } = await import('../../stores/projects')
      const store = useProjectsStore()
      vi.spyOn(store, 'selectProject')

      const card = wrapper.find('.p-4')
      await card.trigger('click')

      expect(store.selectProject).toHaveBeenCalledWith(project)
    })

    it('should have hover visual feedback', () => {
      const project = createMockProject()
      const wrapper = mount(ProjectCard, {
        props: { project },
      })

      const card = wrapper.find('.p-4')
      expect(card.classes()).toContain('hover:shadow-lg')
      expect(card.classes()).toContain('cursor-pointer')
    })
  })

  // ===== EDGE CASES =====
  describe('Edge Cases', () => {
    it('should handle project with very long title', () => {
      const longTitle = 'A'.repeat(200)
      const project = createMockProject({ title: longTitle })

      const wrapper = mount(ProjectCard, {
        props: { project },
      })

      expect(wrapper.text()).toContain(longTitle.substring(0, 50))
    })

    it('should handle project with zero scenes', () => {
      const project = createMockProject({ scene_count: 0 })
      const wrapper = mount(ProjectCard, {
        props: { project },
      })

      expect(wrapper.text()).toContain('0')
      expect(wrapper.text()).toContain('scenes')
    })

    it('should handle project with many scenes', () => {
      const project = createMockProject({ scene_count: 999 })
      const wrapper = mount(ProjectCard, {
        props: { project },
      })

      expect(wrapper.text()).toContain('999')
    })

    it('should handle project with special characters in title', () => {
      const project = createMockProject({
        title: 'Project @#$% & "Special" Chars',
      })

      const wrapper = mount(ProjectCard, {
        props: { project },
      })

      expect(wrapper.text()).toContain('Project')
    })

    it('should handle empty path gracefully', () => {
      const project = createMockProject({ path: '' })
      const wrapper = mount(ProjectCard, {
        props: { project },
      })

      expect(wrapper.exists()).toBe(true)
    })
  })
})
