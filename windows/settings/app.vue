<template>
  <div class="h-screen flex flex-col bg-slate-950 text-slate-100">
    <!-- Header -->
    <header class="border-b border-slate-800 bg-slate-900 px-4 py-4">
      <h1 class="text-lg font-semibold">Settings</h1>
      <p class="text-sm text-slate-400">
        Provider/model configuration for analysis commands
      </p>
    </header>

    <!-- Error Banner -->
    <div
      v-if="store.error"
      class="bg-red-900 border-b border-red-800 px-4 py-3 flex items-center justify-between"
    >
      <p class="text-sm text-red-100">{{ store.error }}</p>
      <button
        @click="store.error = null"
        class="text-red-200 hover:text-red-100 transition-colors ml-2"
      >
        <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
          <path
            fill-rule="evenodd"
            d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
            clip-rule="evenodd"
          />
        </svg>
      </button>
    </div>

    <!-- Loading State -->
    <div
      v-if="store.loading"
      class="flex-1 flex items-center justify-center"
    >
      <div class="flex flex-col items-center gap-3">
        <svg
          class="w-8 h-8 text-slate-300 animate-spin"
          fill="none"
          viewBox="0 0 24 24"
        >
          <circle
            class="opacity-25"
            cx="12"
            cy="12"
            r="10"
            stroke="currentColor"
            stroke-width="4"
          />
          <path
            class="opacity-75"
            fill="currentColor"
            d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
          />
        </svg>
        <p class="text-slate-300 text-sm">Loading settings...</p>
      </div>
    </div>

    <!-- Main Content: Two-pane layout -->
    <div v-else class="flex-1 flex min-h-0">
      <!-- Left pane: Provider list -->
      <div class="w-64 border-r border-slate-800 p-3 overflow-y-auto">
        <p class="text-xs font-medium text-slate-500 uppercase tracking-wide mb-2">
          Provider
        </p>
        <ProviderSelector
          :providers="store.providers"
          :selected="store.config.provider"
          @select="store.selectProvider"
        />

        <!-- Daemon Status -->
        <div class="mt-4 pt-4 border-t border-slate-800">
          <p class="text-xs font-medium text-slate-500 uppercase tracking-wide mb-2">
            Daemon
          </p>
          <StatusIndicator
            :valid="store.daemonStatus?.reachable ?? null"
            :label="
              store.daemonStatus?.reachable
                ? `Connected${store.daemonStatus.version ? ` (${store.daemonStatus.version})` : ''}`
                : store.daemonStatus?.error || 'Not checked'
            "
          />
          <button
            @click="store.checkDaemonConnection"
            class="mt-2 text-xs text-blue-400 hover:text-blue-300 transition-colors"
          >
            Check connection
          </button>
        </div>
      </div>

      <!-- Right pane: Model configuration -->
      <div class="flex-1 p-4 overflow-y-auto">
        <ModelConfiguration
          :provider="store.currentProvider"
          :model-value="store.config.model || ''"
          :effective-model="store.effectiveModel"
          :validation="store.validation"
          :validating="store.validating"
          @update:model-value="store.setModel"
        />

        <!-- Validate button -->
        <div class="mt-4">
          <button
            @click="store.validateProvider"
            :disabled="store.validating"
            class="px-3 py-1.5 text-xs rounded transition-colors"
            :class="
              store.validating
                ? 'bg-slate-700 text-slate-500 cursor-not-allowed'
                : 'bg-slate-800 text-slate-300 hover:bg-slate-700'
            "
          >
            {{ store.validating ? 'Validating...' : 'Validate' }}
          </button>
        </div>

        <!-- Validation Results -->
        <div
          v-if="store.validation"
          class="mt-4 p-3 rounded text-xs"
          :class="
            store.validation.valid
              ? 'bg-green-900/30 border border-green-800'
              : 'bg-red-900/30 border border-red-800'
          "
        >
          <div class="flex items-center gap-2 mb-2">
            <StatusIndicator
              :valid="store.validation.valid"
              :label="store.validation.valid ? 'All checks passed' : 'Validation failed'"
            />
          </div>
          <div v-if="store.validation.errors.length > 0" class="mt-2 space-y-1">
            <p
              v-for="(err, i) in store.validation.errors"
              :key="i"
              class="text-red-400"
            >
              {{ err }}
            </p>
          </div>
        </div>
      </div>
    </div>

    <!-- Footer: Save/Cancel -->
    <footer class="border-t border-slate-800 bg-slate-900 px-4 py-3 flex items-center justify-end gap-3">
      <button
        @click="handleCancel"
        class="px-4 py-2 text-sm text-slate-400 hover:text-slate-200 transition-colors"
      >
        Cancel
      </button>
      <button
        @click="handleSave"
        :disabled="!store.canSave"
        class="px-4 py-2 text-sm rounded transition-colors"
        :class="
          store.canSave
            ? 'bg-blue-600 hover:bg-blue-700 text-white'
            : 'bg-slate-700 text-slate-500 cursor-not-allowed'
        "
      >
        {{ store.saving ? 'Saving...' : 'Save' }}
      </button>
    </footer>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { useSettingsStore } from './stores/settings'
import ProviderSelector from './components/ProviderSelector.vue'
import ModelConfiguration from './components/ModelConfiguration.vue'
import StatusIndicator from './components/StatusIndicator.vue'

const store = useSettingsStore()

onMounted(async () => {
  await store.loadSettings()
})

const handleSave = async () => {
  await store.saveSettings()
}

const handleCancel = async () => {
  await store.loadSettings()
}
</script>
