<template>
  <div class="p-5 flex flex-col gap-4">
    <p class="text-sm text-slate-300">Review and confirm the import details:</p>

    <!-- Summary -->
    <div class="bg-slate-800 rounded border border-slate-700 p-4 space-y-3">
      <div>
        <span class="text-xs text-slate-500 block">Title</span>
        <span class="text-sm text-slate-200">{{ importStore.title }}</span>
      </div>
      <div>
        <span class="text-xs text-slate-500 block">Source File</span>
        <span class="text-sm text-slate-200 break-all">{{ importStore.filePath }}</span>
      </div>
      <div>
        <span class="text-xs text-slate-500 block">Destination</span>
        <span class="text-sm text-slate-200 break-all">{{ importStore.outputPath }}</span>
      </div>
    </div>

    <!-- Error display -->
    <p v-if="importStore.error" class="text-sm text-red-400">
      {{ importStore.error }}
    </p>

    <!-- Actions -->
    <div class="flex gap-3 justify-end pt-2 border-t border-slate-800">
      <button
        @click="importStore.goBack()"
        class="px-4 py-2 text-sm text-slate-400 hover:text-slate-200 transition-colors"
      >
        Back
      </button>
      <button
        @click="handleImport"
        :disabled="importing"
        class="px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-slate-700 disabled:text-slate-500 text-white rounded text-sm font-medium transition-colors"
      >
        {{ importing ? 'Starting...' : 'Import' }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useImportStore } from '../../stores/import'

const importStore = useImportStore()
const importing = ref(false)

const handleImport = async (): Promise<void> => {
  importing.value = true
  await importStore.confirmImport()
}
</script>
