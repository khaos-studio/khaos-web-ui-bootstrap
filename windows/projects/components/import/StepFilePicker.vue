<template>
  <div class="p-5 flex flex-col gap-4">
    <p class="text-sm text-slate-300">
      Select a screenplay file to import (.fountain, .fdx, .sbx, .md)
    </p>

    <!-- Selected file display -->
    <div v-if="selectedFile" class="flex items-center gap-2 p-3 bg-slate-800 rounded border border-slate-700">
      <svg class="w-5 h-5 text-green-400 flex-shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
        <path stroke-linecap="round" stroke-linejoin="round" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
      <span class="text-sm text-slate-200 truncate" :title="selectedFile">{{ selectedFile }}</span>
      <button @click="clearFile" class="ml-auto text-slate-400 hover:text-slate-200 flex-shrink-0">
        <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
          <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
        </svg>
      </button>
    </div>

    <!-- Browse button -->
    <button
      @click="browseFile"
      :disabled="browsing"
      class="w-full py-3 border-2 border-dashed border-slate-600 hover:border-blue-500 rounded-lg text-sm text-slate-300 hover:text-blue-400 transition-colors flex items-center justify-center gap-2"
    >
      <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
        <path stroke-linecap="round" stroke-linejoin="round" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
      </svg>
      {{ browsing ? 'Opening file picker...' : selectedFile ? 'Choose a different file' : 'Browse for screenplay file' }}
    </button>

    <!-- Manual path input (secondary) -->
    <div>
      <label class="text-xs text-slate-500 mb-1 block">Or enter path manually:</label>
      <input
        v-model="manualPath"
        type="text"
        placeholder="/path/to/screenplay.fountain"
        class="w-full bg-slate-800 border border-slate-700 rounded px-3 py-2 text-sm text-slate-200 placeholder-slate-500 focus:outline-none focus:border-blue-500"
        @keydown.enter="submitManualPath"
      />
    </div>

    <!-- Error display -->
    <p v-if="importStore.error" class="text-sm text-red-400">
      {{ importStore.error }}
    </p>

    <!-- Actions -->
    <div class="flex gap-3 justify-end pt-2 border-t border-slate-800">
      <button
        @click="importStore.closeWizard()"
        class="px-4 py-2 text-sm text-slate-400 hover:text-slate-200 transition-colors"
      >
        Cancel
      </button>
      <button
        @click="handleNext"
        :disabled="!selectedFile"
        class="px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-slate-700 disabled:text-slate-500 text-white rounded text-sm font-medium transition-colors"
      >
        Next
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { useImportStore } from '../../stores/import'

const importStore = useImportStore()

const selectedFile = ref('')
const manualPath = ref('')
const browsing = ref(false)

const browseFile = async (): Promise<void> => {
  browsing.value = true
  importStore.error = null
  try {
    const result = await open({
      multiple: false,
      filters: [
        {
          name: 'Screenplay Files',
          extensions: ['fountain', 'fdx', 'sbx', 'md'],
        },
      ],
    })
    if (result) {
      selectedFile.value = result as string
      manualPath.value = ''
    }
  } catch (err) {
    importStore.error = err instanceof Error ? err.message : String(err)
  } finally {
    browsing.value = false
  }
}

const clearFile = (): void => {
  selectedFile.value = ''
  importStore.error = null
}

const submitManualPath = (): void => {
  if (manualPath.value.trim()) {
    selectedFile.value = manualPath.value.trim()
    handleNext()
  }
}

const handleNext = async (): Promise<void> => {
  if (selectedFile.value) {
    await importStore.setFile(selectedFile.value)
  }
}
</script>
