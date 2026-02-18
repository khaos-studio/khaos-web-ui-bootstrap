import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useImportStore } from '../../stores/import'
import type { CollisionInfo } from '@khaos/shared'

// Mock the Tauri APIs
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

vi.mock('@tauri-apps/api/window', () => ({
  getCurrentWindow: () => ({
    listen: vi.fn().mockResolvedValue(() => {}),
  }),
}))

import { invoke } from '@tauri-apps/api/core'

describe('Import Store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.clearAllMocks()
  })

  // ===== INITIAL STATE =====
  describe('Initial State', () => {
    it('should initialize with default values', () => {
      const store = useImportStore()

      expect(store.isOpen).toBe(false)
      expect(store.step).toBe('file')
      expect(store.filePath).toBe('')
      expect(store.title).toBe('')
      expect(store.outputPath).toBe('')
      expect(store.collision).toBeNull()
      expect(store.logs).toEqual([])
      expect(store.status).toBe('idle')
      expect(store.error).toBeNull()
      expect(store.requestId).toBeNull()
    })
  })

  // ===== OPEN/CLOSE WIZARD =====
  describe('openWizard', () => {
    it('should set isOpen and reset state', () => {
      const store = useImportStore()

      // Mutate some state first
      store.step = 'title' as any
      store.filePath = '/some/file'
      store.error = 'some error'

      store.openWizard()

      expect(store.isOpen).toBe(true)
      expect(store.step).toBe('file')
      expect(store.filePath).toBe('')
      expect(store.error).toBeNull()
    })
  })

  describe('closeWizard', () => {
    it('should set isOpen to false and reset state', async () => {
      const store = useImportStore()
      store.isOpen = true
      store.step = 'title' as any

      await store.closeWizard()

      expect(store.isOpen).toBe(false)
      expect(store.step).toBe('file')
    })

    it('should attempt to cancel active parse', async () => {
      const store = useImportStore()
      store.isOpen = true
      store.status = 'in_progress' as any
      store.requestId = 'test-request-id'
      ;(invoke as any).mockResolvedValue(undefined)

      await store.closeWizard()

      expect(invoke).toHaveBeenCalledWith('cancel_parse', { requestId: 'test-request-id' })
    })
  })

  // ===== SET FILE =====
  describe('setFile', () => {
    it('should validate and advance to title step on success', async () => {
      ;(invoke as any).mockResolvedValue(undefined)

      const store = useImportStore()
      await store.setFile('/path/to/screenplay.fountain')

      expect(store.filePath).toBe('/path/to/screenplay.fountain')
      expect(store.step).toBe('title')
      expect(store.error).toBeNull()
    })

    it('should set error on validation failure', async () => {
      ;(invoke as any).mockRejectedValue('Unsupported file type')

      const store = useImportStore()
      await store.setFile('/path/to/file.txt')

      expect(store.step).toBe('file') // Should not advance
      expect(store.error).toBe('Unsupported file type')
    })
  })

  // ===== SET TITLE =====
  describe('setTitle', () => {
    it('should reject empty title', async () => {
      const store = useImportStore()
      await store.setTitle('')

      expect(store.error).toBe('Title cannot be empty')
      expect(store.step).toBe('file')
    })

    it('should reject whitespace-only title', async () => {
      const store = useImportStore()
      await store.setTitle('   ')

      expect(store.error).toBe('Title cannot be empty')
    })

    it('should reject title over 255 chars', async () => {
      const store = useImportStore()
      await store.setTitle('a'.repeat(256))

      expect(store.error).toBe('Title cannot exceed 255 characters')
    })

    it('should advance to confirm step when no collision', async () => {
      // check_import_collision returns null (no collision)
      ;(invoke as any)
        .mockResolvedValueOnce(null) // check_import_collision
        .mockResolvedValueOnce('/projects/My_Project.kspd') // resolve_import_path

      const store = useImportStore()
      await store.setTitle('My Project')

      expect(store.title).toBe('My Project')
      expect(store.step).toBe('confirm')
      expect(store.collision).toBeNull()
      expect(store.outputPath).toBe('/projects/My_Project.kspd')
    })

    it('should advance to collision step when collision exists', async () => {
      const collisionInfo: CollisionInfo = {
        existing_path: '/projects/My_Project.kspd',
        suggested_names: ['My_Project_1', 'My_Project_2'],
      }
      ;(invoke as any).mockResolvedValueOnce(collisionInfo)

      const store = useImportStore()
      await store.setTitle('My Project')

      expect(store.title).toBe('My Project')
      expect(store.step).toBe('collision')
      expect(store.collision).toEqual(collisionInfo)
      expect(store.outputPath).toBe('/projects/My_Project.kspd')
    })
  })

  // ===== CONFIRM IMPORT =====
  describe('confirmImport', () => {
    it('should start import and advance to execute step', async () => {
      ;(invoke as any).mockResolvedValue('request-123')

      const store = useImportStore()
      store.filePath = '/path/to/file.fountain'
      store.title = 'My Project'
      store.outputPath = '/projects/My_Project.kspd'

      await store.confirmImport()

      expect(store.status).toBe('in_progress')
      expect(store.step).toBe('execute')
      expect(store.requestId).toBe('request-123')
    })

    it('should handle start_parse failure', async () => {
      ;(invoke as any).mockRejectedValue('khaos-tools not found')

      const store = useImportStore()
      store.filePath = '/path/to/file.fountain'
      store.title = 'My Project'
      store.outputPath = '/projects/My_Project.kspd'

      await store.confirmImport()

      expect(store.status).toBe('failed')
      expect(store.step).toBe('result')
      expect(store.error).toBe('khaos-tools not found')
    })

    it('should accept override path and overwrite flag', async () => {
      ;(invoke as any).mockResolvedValue('request-456')

      const store = useImportStore()
      store.filePath = '/path/to/file.fountain'
      store.title = 'My Project'
      store.outputPath = '/projects/My_Project.kspd'

      await store.confirmImport('/projects/My_Project_1.kspd', false)

      expect(invoke).toHaveBeenCalledWith('start_parse', {
        filePath: '/path/to/file.fountain',
        title: 'My Project',
        outputPath: '/projects/My_Project_1.kspd',
        overwrite: false,
      })
    })
  })

  // ===== CANCEL IMPORT =====
  describe('cancelImport', () => {
    it('should set failed status and cancel message', async () => {
      ;(invoke as any).mockResolvedValue(undefined)

      const store = useImportStore()
      store.requestId = 'req-123'

      await store.cancelImport()

      expect(store.status).toBe('failed')
      expect(store.error).toBe('Import cancelled by user')
      expect(store.step).toBe('result')
    })
  })

  // ===== GO BACK =====
  describe('goBack', () => {
    it('should navigate from title to file', () => {
      const store = useImportStore()
      store.step = 'title' as any

      store.goBack()

      expect(store.step).toBe('file')
    })

    it('should navigate from confirm to title', () => {
      const store = useImportStore()
      store.step = 'confirm' as any

      store.goBack()

      expect(store.step).toBe('title')
    })

    it('should navigate from collision to title', () => {
      const store = useImportStore()
      store.step = 'collision' as any

      store.goBack()

      expect(store.step).toBe('title')
    })

    it('should clear error on goBack', () => {
      const store = useImportStore()
      store.step = 'title' as any
      store.error = 'some error'

      store.goBack()

      expect(store.error).toBeNull()
    })
  })

  // ===== RESET =====
  describe('reset', () => {
    it('should reset all state to defaults', () => {
      const store = useImportStore()

      // Mutate everything
      store.step = 'result' as any
      store.filePath = '/some/path'
      store.title = 'My Title'
      store.outputPath = '/some/output'
      store.collision = { existing_path: '/x', suggested_names: ['a'] }
      store.logs = ['line1', 'line2']
      store.status = 'success' as any
      store.error = 'some error'
      store.requestId = 'req-1'

      store.reset()

      expect(store.step).toBe('file')
      expect(store.filePath).toBe('')
      expect(store.title).toBe('')
      expect(store.outputPath).toBe('')
      expect(store.collision).toBeNull()
      expect(store.logs).toEqual([])
      expect(store.status).toBe('idle')
      expect(store.error).toBeNull()
      expect(store.requestId).toBeNull()
    })
  })

  // ===== GETTERS =====
  describe('Getters', () => {
    it('canGoBack should be true for title/confirm/collision steps', () => {
      const store = useImportStore()

      store.step = 'title' as any
      expect(store.canGoBack).toBe(true)

      store.step = 'confirm' as any
      expect(store.canGoBack).toBe(true)

      store.step = 'collision' as any
      expect(store.canGoBack).toBe(true)
    })

    it('canGoBack should be false for file/execute/result steps', () => {
      const store = useImportStore()

      store.step = 'file'
      expect(store.canGoBack).toBe(false)

      store.step = 'execute' as any
      expect(store.canGoBack).toBe(false)

      store.step = 'result' as any
      expect(store.canGoBack).toBe(false)
    })

    it('isImporting should reflect in_progress status', () => {
      const store = useImportStore()

      store.status = 'idle'
      expect(store.isImporting).toBe(false)

      store.status = 'in_progress' as any
      expect(store.isImporting).toBe(true)
    })

    it('isComplete should reflect success or failed status', () => {
      const store = useImportStore()

      store.status = 'idle'
      expect(store.isComplete).toBe(false)

      store.status = 'success' as any
      expect(store.isComplete).toBe(true)

      store.status = 'failed' as any
      expect(store.isComplete).toBe(true)
    })
  })
})
