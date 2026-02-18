<template>
  <div class="p-5 flex flex-col gap-4">
    <!-- Selected file info -->
    <div class="text-xs text-slate-500 truncate" :title="importStore.filePath">
      File: {{ importStore.filePath }}
    </div>

    <!-- Title input -->
    <div>
      <label class="text-sm text-slate-300 mb-2 block">Project title</label>
      <input
        ref="titleInput"
        v-model="titleValue"
        type="text"
        placeholder="My Screenplay"
        maxlength="255"
        class="w-full bg-slate-800 border border-slate-700 rounded px-3 py-2 text-sm text-slate-200 placeholder-slate-500 focus:outline-none focus:border-blue-500"
        @keydown.enter="handleNext"
      />
      <div class="flex justify-between mt-1">
        <span v-if="importStore.error" class="text-xs text-red-400">{{ importStore.error }}</span>
        <span v-else class="text-xs text-transparent">.</span>
        <span class="text-xs text-slate-500">{{ titleValue.length }}/255</span>
      </div>
    </div>

    <!-- Actions -->
    <div class="flex gap-3 justify-end pt-2 border-t border-slate-800">
      <button
        @click="importStore.goBack()"
        class="px-4 py-2 text-sm text-slate-400 hover:text-slate-200 transition-colors"
      >
        Back
      </button>
      <button
        @click="handleNext"
        :disabled="!titleValue.trim() || loading"
        class="px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-slate-700 disabled:text-slate-500 text-white rounded text-sm font-medium transition-colors"
      >
        {{ loading ? 'Checking...' : 'Next' }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, nextTick } from 'vue'
import { useImportStore } from '../../stores/import'

const importStore = useImportStore()

const titleValue = ref(importStore.title || '')
const loading = ref(false)
const titleInput = ref<HTMLInputElement | null>(null)

onMounted(async () => {
  await nextTick()
  titleInput.value?.focus()
})

const handleNext = async (): Promise<void> => {
  if (!titleValue.value.trim() || loading.value) return
  loading.value = true
  try {
    await importStore.setTitle(titleValue.value)
  } finally {
    loading.value = false
  }
}
</script>
