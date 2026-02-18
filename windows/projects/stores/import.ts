import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'
import type { ImportStep, ImportStatus, CollisionInfo, ImportResult } from '@khaos/shared'

export const useImportStore = defineStore('import', () => {
  // State
  const isOpen = ref(false)
  const step = ref<ImportStep>('file')
  const filePath = ref('')
  const title = ref('')
  const outputPath = ref('')
  const collision = ref<CollisionInfo | null>(null)
  const logs = ref<string[]>([])
  const status = ref<ImportStatus>('idle')
  const error = ref<string | null>(null)
  const requestId = ref<string | null>(null)

  // Event unlisten handles
  let unlistenProgress: (() => void) | null = null
  let unlistenCompleted: (() => void) | null = null

  // Getters
  const canGoBack = computed(() => {
    return step.value === 'title' || step.value === 'confirm' || step.value === 'collision'
  })

  const isImporting = computed(() => status.value === 'in_progress')
  const isComplete = computed(() => status.value === 'success' || status.value === 'failed')

  // Actions
  const reset = (): void => {
    step.value = 'file'
    filePath.value = ''
    title.value = ''
    outputPath.value = ''
    collision.value = null
    logs.value = []
    status.value = 'idle'
    error.value = null
    requestId.value = null
  }

  const openWizard = (): void => {
    reset()
    isOpen.value = true
  }

  const closeWizard = async (): Promise<void> => {
    if (status.value === 'in_progress' && requestId.value) {
      try {
        await invoke('cancel_parse', { requestId: requestId.value })
      } catch {
        // Ignore cancel errors on close
      }
    }
    await cleanupListeners()
    isOpen.value = false
    reset()
  }

  const setFile = async (path: string): Promise<void> => {
    error.value = null
    try {
      await invoke('validate_import_file', { filePath: path })
      filePath.value = path
      step.value = 'title'
    } catch (err) {
      error.value = err instanceof Error ? err.message : String(err)
    }
  }

  const setTitle = async (value: string): Promise<void> => {
    error.value = null

    // Client-side validation
    const trimmed = value.trim()
    if (!trimmed) {
      error.value = 'Title cannot be empty'
      return
    }
    if (trimmed.length > 255) {
      error.value = 'Title cannot exceed 255 characters'
      return
    }

    title.value = trimmed

    try {
      // Check for collision
      const collisionResult = await invoke<CollisionInfo | null>('check_import_collision', {
        title: trimmed,
      })

      if (collisionResult) {
        collision.value = collisionResult
        outputPath.value = collisionResult.existing_path
        step.value = 'collision'
      } else {
        // Resolve the output path
        const resolvedPath = await invoke<string>('resolve_import_path', {
          title: trimmed,
        })
        outputPath.value = resolvedPath
        collision.value = null
        step.value = 'confirm'
      }
    } catch (err) {
      error.value = err instanceof Error ? err.message : String(err)
    }
  }

  const confirmImport = async (
    overridePath?: string,
    overwrite: boolean = false
  ): Promise<void> => {
    error.value = null
    const targetPath = overridePath || outputPath.value

    try {
      status.value = 'in_progress'
      step.value = 'execute'
      logs.value = ['Starting import...']

      // Set up event listeners before starting
      await setupEventListeners()

      const rid = await invoke<string>('start_parse', {
        filePath: filePath.value,
        title: title.value,
        outputPath: targetPath,
        overwrite,
      })

      requestId.value = rid
      outputPath.value = targetPath
    } catch (err) {
      status.value = 'failed'
      error.value = err instanceof Error ? err.message : String(err)
      step.value = 'result'
    }
  }

  const cancelImport = async (): Promise<void> => {
    if (requestId.value) {
      try {
        await invoke('cancel_parse', { requestId: requestId.value })
      } catch {
        // Ignore cancel errors
      }
    }
    status.value = 'failed'
    error.value = 'Import cancelled by user'
    step.value = 'result'
    await cleanupListeners()
  }

  const goBack = (): void => {
    error.value = null
    switch (step.value) {
      case 'title':
        step.value = 'file'
        break
      case 'confirm':
      case 'collision':
        step.value = 'title'
        break
    }
  }

  // Event listener management
  const setupEventListeners = async (): Promise<void> => {
    await cleanupListeners()
    const appWindow = getCurrentWindow()

    unlistenProgress = await appWindow.listen<{
      request_id: string
      phase: string
      progress: number
      line: string | null
    }>('daemon:parser-progress', (event) => {
      if (event.payload.line) {
        logs.value = [...logs.value, event.payload.line]
      }
    })

    unlistenCompleted = await appWindow.listen<{
      request_id: string
      success: boolean
      project_id: string | null
      error: string | null
    }>('daemon:parser-completed', (event) => {
      if (event.payload.success) {
        status.value = 'success'
      } else {
        status.value = 'failed'
        error.value = event.payload.error || 'Import failed'
      }
      step.value = 'result'
      cleanupListeners()
    })
  }

  const cleanupListeners = async (): Promise<void> => {
    if (unlistenProgress) {
      unlistenProgress()
      unlistenProgress = null
    }
    if (unlistenCompleted) {
      unlistenCompleted()
      unlistenCompleted = null
    }
  }

  return {
    // State
    isOpen,
    step,
    filePath,
    title,
    outputPath,
    collision,
    logs,
    status,
    error,
    requestId,
    // Getters
    canGoBack,
    isImporting,
    isComplete,
    // Actions
    openWizard,
    closeWizard,
    setFile,
    setTitle,
    confirmImport,
    cancelImport,
    goBack,
    reset,
  }
})
