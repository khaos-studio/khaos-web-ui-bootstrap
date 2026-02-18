<template>
  <div class="p-5 flex flex-col gap-4">
    <!-- Success state -->
    <template v-if="importStore.status === 'success'">
      <div class="flex items-center gap-3">
        <svg class="w-8 h-8 text-green-400" fill="currentColor" viewBox="0 0 20 20">
          <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
        </svg>
        <div>
          <h3 class="text-base font-semibold text-green-400">Import Successful</h3>
          <p class="text-sm text-slate-400">Project imported successfully</p>
        </div>
      </div>

      <div class="bg-slate-800 rounded border border-slate-700 p-3">
        <span class="text-xs text-slate-500 block">Output</span>
        <span class="text-sm text-slate-200 break-all">{{ importStore.outputPath }}</span>
      </div>
    </template>

    <!-- Failure state -->
    <template v-else>
      <div class="flex items-center gap-3">
        <svg class="w-8 h-8 text-red-400" fill="currentColor" viewBox="0 0 20 20">
          <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
        </svg>
        <div>
          <h3 class="text-base font-semibold text-red-400">Import Failed</h3>
          <p class="text-sm text-slate-400">{{ importStore.error || 'The import could not be completed' }}</p>
        </div>
      </div>

      <!-- Expandable log viewer for failures -->
      <details v-if="importStore.logs.length > 0" class="group">
        <summary class="text-xs text-slate-500 cursor-pointer hover:text-slate-300 transition-colors">
          Show import logs ({{ importStore.logs.length }} lines)
        </summary>
        <div class="mt-2 bg-slate-950 border border-slate-700 rounded p-3 max-h-40 overflow-y-auto font-mono text-xs text-slate-400 space-y-0.5">
          <div v-for="(line, i) in importStore.logs" :key="i" class="whitespace-pre-wrap">
            {{ line }}
          </div>
        </div>
      </details>
    </template>

    <!-- Actions -->
    <div class="flex gap-3 justify-end pt-2 border-t border-slate-800">
      <button
        @click="handleDone"
        class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded text-sm font-medium transition-colors"
      >
        Done
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useImportStore } from '../../stores/import'
import { useProjectsStore } from '../../stores/projects'

const importStore = useImportStore()
const projectsStore = useProjectsStore()

const handleDone = async (): Promise<void> => {
  const wasSuccess = importStore.status === 'success'
  await importStore.closeWizard()

  // Refresh the project list if import was successful
  if (wasSuccess) {
    await projectsStore.loadProjects()
  }
}
</script>
