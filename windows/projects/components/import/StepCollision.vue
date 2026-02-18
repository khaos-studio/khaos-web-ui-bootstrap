<template>
  <div class="p-5 flex flex-col gap-4">
    <!-- Warning banner -->
    <div class="bg-yellow-900/30 border border-yellow-700/50 rounded p-3 flex items-start gap-2">
      <svg class="w-5 h-5 text-yellow-400 flex-shrink-0 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
        <path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd" />
      </svg>
      <div>
        <p class="text-sm text-yellow-200 font-medium">Project already exists</p>
        <p class="text-xs text-yellow-300/70 mt-0.5 break-all">{{ importStore.outputPath }}</p>
      </div>
    </div>

    <!-- Radio options -->
    <div class="space-y-2">
      <!-- Overwrite -->
      <label class="flex items-start gap-3 p-3 bg-slate-800 rounded border border-slate-700 hover:border-slate-600 cursor-pointer transition-colors">
        <input
          type="radio"
          v-model="choice"
          value="overwrite"
          class="mt-0.5 accent-blue-500"
        />
        <div>
          <span class="text-sm text-slate-200">Overwrite existing project</span>
          <p class="text-xs text-slate-500 mt-0.5">Replace the project at the current path</p>
        </div>
      </label>

      <!-- Suggested names -->
      <label
        v-for="name in importStore.collision?.suggested_names || []"
        :key="name"
        class="flex items-start gap-3 p-3 bg-slate-800 rounded border border-slate-700 hover:border-slate-600 cursor-pointer transition-colors"
      >
        <input
          type="radio"
          v-model="choice"
          :value="name"
          class="mt-0.5 accent-blue-500"
        />
        <div>
          <span class="text-sm text-slate-200">Rename to: {{ name }}</span>
          <p class="text-xs text-slate-500 mt-0.5">{{ name }}.kspd</p>
        </div>
      </label>

      <!-- Go back -->
      <label class="flex items-start gap-3 p-3 bg-slate-800 rounded border border-slate-700 hover:border-slate-600 cursor-pointer transition-colors">
        <input
          type="radio"
          v-model="choice"
          value="back"
          class="mt-0.5 accent-blue-500"
        />
        <span class="text-sm text-slate-200">Go back and change title</span>
      </label>
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
        Cancel
      </button>
      <button
        @click="handleContinue"
        :disabled="!choice"
        class="px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-slate-700 disabled:text-slate-500 text-white rounded text-sm font-medium transition-colors"
      >
        Continue
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useImportStore } from '../../stores/import'

const importStore = useImportStore()
const choice = ref('overwrite')

const handleContinue = async (): Promise<void> => {
  if (choice.value === 'back') {
    importStore.goBack()
    return
  }

  if (choice.value === 'overwrite') {
    await importStore.confirmImport(importStore.outputPath, true)
    return
  }

  // A suggested name was chosen — resolve its full path
  try {
    const newPath = await invoke<string>('resolve_import_path', {
      title: choice.value,
    })
    await importStore.confirmImport(newPath, false)
  } catch (err) {
    importStore.error = err instanceof Error ? err.message : String(err)
  }
}
</script>
