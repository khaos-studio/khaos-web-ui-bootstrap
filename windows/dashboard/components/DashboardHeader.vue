<template>
  <header class="border-b border-slate-800 bg-slate-900 px-4 py-3 flex items-center justify-between">
    <div class="min-w-0">
      <h1 class="text-lg font-semibold text-slate-100 truncate">{{ title }}</h1>
      <div class="flex items-center gap-3 mt-0.5">
        <span class="text-xs text-slate-400 flex items-center gap-1">
          <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="1.5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M3.375 19.5h17.25m-17.25 0a1.125 1.125 0 0 1-1.125-1.125M3.375 19.5h1.5C5.496 19.5 6 18.996 6 18.375m-2.625 0V5.625m17.25 12.75v-1.5" />
          </svg>
          {{ summary?.scenes ?? 0 }} scenes
        </span>
        <span class="text-xs text-slate-400 flex items-center gap-1">
          <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="1.5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 6a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0ZM4.501 20.118a7.5 7.5 0 0 1 14.998 0" />
          </svg>
          {{ summary?.characters ?? 0 }} characters
        </span>
        <span class="text-xs text-slate-400 flex items-center gap-1">
          <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="1.5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M15 10.5a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z M19.5 10.5c0 7.142-7.5 11.25-7.5 11.25S4.5 17.642 4.5 10.5a7.5 7.5 0 1 1 15 0Z" />
          </svg>
          {{ summary?.locations ?? 0 }} locations
        </span>
        <span v-if="daemonRunning" class="text-xs text-emerald-400 flex items-center gap-1">
          <span class="inline-block h-1.5 w-1.5 rounded-full bg-emerald-500"></span>
          Daemon
        </span>
      </div>
    </div>
    <div class="flex items-center gap-2 flex-shrink-0">
      <span v-if="progress" class="text-xs text-slate-400 tabular-nums">
        {{ progress.analyzed }}/{{ progress.total }} analyzed
      </span>
      <button
        @click="$emit('analyzeAll')"
        :disabled="analyzing"
        :class="[
          'px-3 py-1.5 text-sm font-medium rounded transition-colors',
          analyzing
            ? 'bg-slate-700 text-slate-400 cursor-not-allowed'
            : 'bg-blue-600 hover:bg-blue-700 text-white',
        ]"
      >
        <span v-if="analyzing" class="flex items-center gap-2">
          <svg class="w-3.5 h-3.5 animate-spin" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
          </svg>
          Analyzing...
        </span>
        <span v-else>Analyze All</span>
      </button>
    </div>
  </header>
</template>

<script setup lang="ts">
import type { ProjectSummary } from '@khaos/shared'

defineProps<{
  title: string
  summary: ProjectSummary | null
  progress: { analyzed: number; total: number } | null
  analyzing: boolean
  daemonRunning: boolean
}>()

defineEmits<{
  analyzeAll: []
}>()
</script>
