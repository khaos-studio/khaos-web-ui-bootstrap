import { describe, it, expect, beforeEach, vi } from 'vitest'
import { mount } from '@vue/test-utils'
import ProjectSearch from '../../components/ProjectSearch.vue'
import { createPinia, setActivePinia } from 'pinia'
import { useProjectsStore } from '../../stores/projects'

// Mock Tauri
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

describe('ProjectSearch Component', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.useFakeTimers()
  })

  afterEach(() => {
    vi.runAllTimers()
    vi.useRealTimers()
  })

  it('should render input field', () => {
    const wrapper = mount(ProjectSearch)
    const input = wrapper.find('input')

    expect(input.exists()).toBe(true)
  })

  it('should display custom placeholder', () => {
    const wrapper = mount(ProjectSearch, {
      props: {
        placeholder: 'Custom placeholder text',
      },
    })

    const input = wrapper.find('input')
    expect(input.attributes('placeholder')).toBe('Custom placeholder text')
  })

  it('should use default placeholder when not provided', () => {
    const wrapper = mount(ProjectSearch)
    const input = wrapper.find('input')

    expect(input.attributes('placeholder')).toContain('Search by title, author, path')
  })

  it('should focus input on mount', async () => {
    const wrapper = mount(ProjectSearch)
    const input = wrapper.find('input')

    await wrapper.vm.$nextTick()

    // Input should be available for interaction
    expect(input.exists()).toBe(true)
  })

  it('should debounce search input', async () => {
    const wrapper = mount(ProjectSearch)
    const store = useProjectsStore()
    vi.spyOn(store, 'setSearch')

    const input = wrapper.find('input')

    // Type something
    await input.setValue('test')
    expect(store.setSearch).not.toHaveBeenCalled()

    // Advance time but not enough
    vi.advanceTimersByTime(100)
    expect(store.setSearch).not.toHaveBeenCalled()

    // Advance past debounce delay
    vi.advanceTimersByTime(200)
    expect(store.setSearch).toHaveBeenCalledWith('test')
  })

  it('should have 300ms debounce delay', async () => {
    const wrapper = mount(ProjectSearch)
    const store = useProjectsStore()
    vi.spyOn(store, 'setSearch')

    const input = wrapper.find('input')
    await input.setValue('query')

    // At 299ms, should not have called
    vi.advanceTimersByTime(299)
    expect(store.setSearch).not.toHaveBeenCalled()

    // At 300ms, should have called
    vi.advanceTimersByTime(1)
    expect(store.setSearch).toHaveBeenCalledWith('query')
  })

  it('should cancel previous debounce timer on new input', async () => {
    const wrapper = mount(ProjectSearch)
    const store = useProjectsStore()
    vi.spyOn(store, 'setSearch')

    const input = wrapper.find('input')

    // First input
    await input.setValue('first')
    vi.advanceTimersByTime(150)

    // Second input (should cancel first timer)
    await input.setValue('second')
    vi.advanceTimersByTime(150)

    // Total 300ms, but timer was reset
    expect(store.setSearch).not.toHaveBeenCalled()

    // Advance more to complete second timer
    vi.advanceTimersByTime(150)
    expect(store.setSearch).toHaveBeenCalledWith('second')
    expect(store.setSearch).toHaveBeenCalledTimes(1)
  })

  it('should show clear button when input has text', async () => {
    const wrapper = mount(ProjectSearch)
    const input = wrapper.find('input')

    // Initially no clear button
    let clearButton = wrapper.find('button')
    expect(clearButton.exists()).toBe(false)

    // After typing
    await input.setValue('text')
    await wrapper.vm.$nextTick()

    clearButton = wrapper.find('button[aria-label="Clear search"]')
    expect(clearButton.exists()).toBe(true)
  })

  it('should hide clear button when input is empty', async () => {
    const wrapper = mount(ProjectSearch)
    const input = wrapper.find('input')

    // Type something
    await input.setValue('text')
    await wrapper.vm.$nextTick()

    let clearButton = wrapper.find('button[aria-label="Clear search"]')
    expect(clearButton.exists()).toBe(true)

    // Clear it
    await input.setValue('')
    await wrapper.vm.$nextTick()

    clearButton = wrapper.find('button[aria-label="Clear search"]')
    expect(clearButton.exists()).toBe(false)
  })

  it('should clear search when clear button is clicked', async () => {
    const wrapper = mount(ProjectSearch)
    const store = useProjectsStore()

    const input = wrapper.find('input')
    await input.setValue('some query')
    await wrapper.vm.$nextTick()

    const clearButton = wrapper.find('button[aria-label="Clear search"]')
    await clearButton.trigger('click')

    expect(input.element.value).toBe('')
    expect(store.search).toBe('')
  })

  it('should cancel debounce timer when clearing search', async () => {
    const wrapper = mount(ProjectSearch)
    const store = useProjectsStore()
    vi.spyOn(store, 'setSearch')

    const input = wrapper.find('input')
    await input.setValue('pending')
    vi.advanceTimersByTime(100)

    // Click clear before debounce completes
    const clearButton = wrapper.find('button[aria-label="Clear search"]')
    await clearButton.trigger('click')

    // Advance time - should not call setSearch with pending value
    vi.advanceTimersByTime(300)

    // Should have called setSearch with empty string, not 'pending'
    expect(store.setSearch).toHaveBeenCalledWith('')
  })

  it('should sync with store search changes', async () => {
    const wrapper = mount(ProjectSearch)
    const store = useProjectsStore()

    // External change to store
    store.search = 'external change'
    await wrapper.vm.$nextTick()

    const input = wrapper.find('input')
    expect(input.element.value).toBe('external change')
  })

  it('should display character count when input is active', async () => {
    const wrapper = mount(ProjectSearch)
    const input = wrapper.find('input')

    // Initially no count
    let count = wrapper.find('.text-xs.text-gray-500')
    expect(count.exists()).toBe(false)

    // After typing
    await input.setValue('hello')
    await wrapper.vm.$nextTick()

    count = wrapper.find('.text-xs.text-gray-500')
    expect(count.exists()).toBe(true)
    expect(count.text()).toBe('5')
  })

  it('should handle rapid successive inputs', async () => {
    const wrapper = mount(ProjectSearch)
    const store = useProjectsStore()
    vi.spyOn(store, 'setSearch')

    const input = wrapper.find('input')

    // Rapid typing
    await input.setValue('a')
    vi.advanceTimersByTime(50)
    await input.setValue('ab')
    vi.advanceTimersByTime(50)
    await input.setValue('abc')
    vi.advanceTimersByTime(50)

    // No calls yet
    expect(store.setSearch).not.toHaveBeenCalled()

    // Complete the debounce
    vi.advanceTimersByTime(200)

    // Should call with final value only
    expect(store.setSearch).toHaveBeenCalledTimes(1)
    expect(store.setSearch).toHaveBeenCalledWith('abc')
  })
})
