import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type {
  SettingsConfig,
  ProviderInfo,
  SettingsLoadResult,
  ProviderValidationResult,
  DaemonCheckResult,
} from '@khaos/shared'

export const useSettingsStore = defineStore('settings', () => {
  // State
  const config = ref<SettingsConfig>({ provider: 'ollama' })
  const providers = ref<ProviderInfo[]>([])
  const loading = ref(false)
  const saving = ref(false)
  const error = ref<string | null>(null)
  const validating = ref(false)
  const validation = ref<ProviderValidationResult | null>(null)
  const daemonStatus = ref<DaemonCheckResult | null>(null)
  const dirty = ref(false)

  // Actions
  const loadSettings = async (): Promise<void> => {
    loading.value = true
    error.value = null

    try {
      const result = await invoke<SettingsLoadResult>('load_settings')
      config.value = result.config
      providers.value = result.providers
      dirty.value = false
    } catch (err) {
      error.value = err instanceof Error ? err.message : String(err)
      console.error('Failed to load settings:', error.value)
    } finally {
      loading.value = false
    }
  }

  const saveSettings = async (): Promise<void> => {
    saving.value = true
    error.value = null

    try {
      await invoke<void>('save_settings', { config: config.value })
      dirty.value = false
    } catch (err) {
      error.value = err instanceof Error ? err.message : String(err)
      console.error('Failed to save settings:', error.value)
    } finally {
      saving.value = false
    }
  }

  const selectProvider = (providerId: string): void => {
    config.value = { ...config.value, provider: providerId, model: undefined }
    dirty.value = true
    validation.value = null
  }

  const setModel = (model: string): void => {
    config.value = {
      ...config.value,
      model: model.trim() || undefined,
    }
    dirty.value = true
  }

  const validateProvider = async (): Promise<void> => {
    validating.value = true
    validation.value = null

    try {
      const result = await invoke<ProviderValidationResult>(
        'validate_provider_config',
        { config: config.value },
      )
      validation.value = result
    } catch (err) {
      validation.value = {
        valid: false,
        checks_run: [],
        errors: [err instanceof Error ? err.message : String(err)],
      }
    } finally {
      validating.value = false
    }
  }

  const checkDaemonConnection = async (): Promise<void> => {
    try {
      daemonStatus.value = await invoke<DaemonCheckResult>(
        'check_daemon_connection',
      )
    } catch (err) {
      daemonStatus.value = {
        reachable: false,
        error: err instanceof Error ? err.message : String(err),
      }
    }
  }

  // Getters
  const currentProvider = computed((): ProviderInfo | undefined => {
    return providers.value.find((p) => p.id === config.value.provider)
  })

  const effectiveModel = computed((): string => {
    return config.value.model || currentProvider.value?.default_model || ''
  })

  const canSave = computed((): boolean => {
    return dirty.value && !saving.value && !loading.value
  })

  return {
    // State
    config,
    providers,
    loading,
    saving,
    error,
    validating,
    validation,
    daemonStatus,
    dirty,
    // Actions
    loadSettings,
    saveSettings,
    selectProvider,
    setModel,
    validateProvider,
    checkDaemonConnection,
    // Getters
    currentProvider,
    effectiveModel,
    canSave,
  }
})
