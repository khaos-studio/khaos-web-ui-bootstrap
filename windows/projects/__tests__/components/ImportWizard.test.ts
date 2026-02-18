import { describe, it, expect, beforeEach, vi } from 'vitest'
import { mount } from '@vue/test-utils'
import { setActivePinia, createPinia } from 'pinia'
import ImportWizard from '../../components/ImportWizard.vue'
import { useImportStore } from '../../stores/import'

// Mock the Tauri APIs
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

vi.mock('@tauri-apps/api/window', () => ({
  getCurrentWindow: () => ({
    listen: vi.fn().mockResolvedValue(() => {}),
    emit: vi.fn().mockResolvedValue(undefined),
  }),
}))

// Mock the dialog plugin
vi.mock('@tauri-apps/plugin-dialog', () => ({
  open: vi.fn(),
}))

describe('ImportWizard', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.clearAllMocks()
  })

  it('should render when mounted', () => {
    const wrapper = mount(ImportWizard)
    expect(wrapper.find('h2').text()).toBe('Import Project')
  })

  it('should display step indicator', () => {
    const wrapper = mount(ImportWizard)
    const steps = wrapper.findAll('[class*="rounded-full"]')
    expect(steps.length).toBe(5)
  })

  it('should show file step label initially', () => {
    const wrapper = mount(ImportWizard)
    expect(wrapper.text()).toContain('Step 1 of 4')
    expect(wrapper.text()).toContain('Select screenplay file')
  })

  it('should have a close button', () => {
    const wrapper = mount(ImportWizard)
    const closeBtn = wrapper.find('[aria-label="Close"]')
    expect(closeBtn.exists()).toBe(true)
  })

  it('should call closeWizard when close button is clicked', async () => {
    const wrapper = mount(ImportWizard)
    const store = useImportStore()
    const closeSpy = vi.spyOn(store, 'closeWizard')

    const closeBtn = wrapper.find('[aria-label="Close"]')
    await closeBtn.trigger('click')

    expect(closeSpy).toHaveBeenCalled()
  })

  it('should highlight the active step', () => {
    const wrapper = mount(ImportWizard)
    // First step indicator should have blue background
    const stepIndicators = wrapper.findAll('[class*="rounded-full"]')
    expect(stepIndicators[0].classes()).toContain('bg-blue-600')
  })

  it('should update step label when step changes', async () => {
    const store = useImportStore()
    store.step = 'title'

    const wrapper = mount(ImportWizard)
    expect(wrapper.text()).toContain('Step 2 of 4')
    expect(wrapper.text()).toContain('Name your project')
  })
})
