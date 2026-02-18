<template>
  <div class="p-5 flex flex-col gap-4">
    <!-- Spinner + status -->
    <div class="flex items-center gap-3">
      <svg class="w-5 h-5 text-blue-400 animate-spin" fill="none" viewBox="0 0 24 24">
        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
      </svg>
      <span class="text-sm text-slate-300">Importing project...</span>
    </div>

    <!-- Log viewport -->
    <div
      ref="logContainer"
      class="bg-slate-950 border border-slate-700 rounded p-3 h-48 overflow-y-auto font-mono text-xs text-slate-400 space-y-0.5"
    >
      <div v-for="(line, i) in importStore.logs" :key="i" class="whitespace-pre-wrap">
        {{ line }}
      </div>
      <div v-if="importStore.logs.length === 0" class="text-slate-600">
        Waiting for output...
      </div>
    </div>

    <!-- Actions -->
    <div class="flex gap-3 justify-end pt-2 border-t border-slate-800">
      <button
        @click="importStore.cancelImport()"
        class="px-4 py-2 bg-red-600 hover:bg-red-700 text-white rounded text-sm font-medium transition-colors"
      >
        Cancel Import
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, nextTick } from 'vue'
import { useImportStore } from '../../stores/import'

const importStore = useImportStore()
const logContainer = ref<HTMLElement | null>(null)

// Auto-scroll to bottom on new log lines
watch(
  () => importStore.logs.length,
  async () => {
    await nextTick()
    if (logContainer.value) {
      logContainer.value.scrollTop = logContainer.value.scrollHeight
    }
  }
)
</script>
