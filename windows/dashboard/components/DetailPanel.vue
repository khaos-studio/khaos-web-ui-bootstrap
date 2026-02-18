<template>
  <div class="flex flex-col h-full">
    <!-- Detail Header -->
    <div class="px-4 py-3 border-b border-slate-800 bg-slate-900 flex items-center justify-between flex-shrink-0">
      <div class="flex items-center gap-3 min-w-0">
        <button
          @click="$emit('close')"
          class="p-1 text-slate-400 hover:text-slate-200 hover:bg-slate-800 rounded transition-colors flex-shrink-0"
          title="Back (Esc)"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="1.5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 19.5 8.25 12l7.5-7.5" />
          </svg>
        </button>
        <div class="min-w-0">
          <h2 class="text-sm font-semibold text-slate-100 truncate">{{ item?.title }}</h2>
          <p class="text-xs text-slate-400 truncate">{{ item?.subtitle }}</p>
        </div>
      </div>
      <div class="flex items-center gap-2 flex-shrink-0">
        <StateBadge v-if="item" :state="item.state" :show-label="true" />
        <button
          v-if="item && (item.state === 'pending' || item.state === 'failed')"
          @click="$emit('analyze', item.id)"
          class="px-3 py-1.5 text-xs bg-blue-600 hover:bg-blue-700 text-white rounded transition-colors font-medium"
        >
          Analyze
        </button>
      </div>
    </div>

    <!-- Detail Content -->
    <div class="flex-1 overflow-y-auto p-4">
      <!-- Loading -->
      <div v-if="!analysis && item?.state === 'analyzing'" class="flex items-center justify-center h-32">
        <div class="flex flex-col items-center gap-2">
          <svg class="w-6 h-6 text-amber-400 animate-spin" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
          </svg>
          <p class="text-sm text-slate-400">Analyzing...</p>
        </div>
      </div>

      <!-- Analysis Content -->
      <AnalysisContent v-else :entity-type="entityType" :analysis="analysis" />
    </div>
  </div>
</template>

<script setup lang="ts">
import type { AnalysisState, SceneAnalysis, CharacterAnalysis, LocationAnalysis, DashboardSection } from '@khaos/shared'
import StateBadge from './StateBadge.vue'
import AnalysisContent from './AnalysisContent.vue'

defineProps<{
  item: { id: string; title: string; subtitle: string; state: AnalysisState } | null
  analysis: SceneAnalysis | CharacterAnalysis | LocationAnalysis | null
  entityType: DashboardSection
}>()

defineEmits<{
  close: []
  analyze: [id: string]
}>()
</script>
