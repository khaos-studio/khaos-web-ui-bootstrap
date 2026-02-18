<template>
  <div class="flex-1 overflow-y-auto">
    <!-- Empty state -->
    <div v-if="items.length === 0" class="flex flex-col items-center justify-center h-full text-slate-500 gap-3 p-8">
      <svg class="w-12 h-12" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="1">
        <path stroke-linecap="round" stroke-linejoin="round" d="M20.25 7.5l-.625 10.632a2.25 2.25 0 01-2.247 2.118H6.622a2.25 2.25 0 01-2.247-2.118L3.75 7.5M10 11.25h4M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125z" />
      </svg>
      <p class="text-sm">No items found</p>
    </div>

    <!-- Item rows -->
    <div v-else class="divide-y divide-slate-800/50">
      <button
        v-for="item in items"
        :key="item.id"
        @click="$emit('select', item.id)"
        :class="[
          'w-full flex items-center gap-3 px-4 py-3 text-left transition-colors group',
          selectedId === item.id
            ? 'bg-blue-600/10 border-l-2 border-blue-500'
            : 'hover:bg-slate-800/50 border-l-2 border-transparent',
        ]"
      >
        <StateBadge :state="item.state" />
        <div class="flex-1 min-w-0">
          <p class="text-sm font-medium text-slate-200 truncate">{{ item.title }}</p>
          <p class="text-xs text-slate-500 truncate">{{ item.subtitle }}</p>
        </div>
        <button
          v-if="item.state === 'pending' || item.state === 'failed'"
          @click.stop="$emit('analyze', item.id)"
          class="opacity-0 group-hover:opacity-100 px-2 py-1 text-xs bg-blue-600/20 text-blue-400 rounded hover:bg-blue-600/30 transition-all"
          title="Analyze"
        >
          <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2">
            <path stroke-linecap="round" stroke-linejoin="round" d="M5.25 5.653c0-.856.917-1.398 1.667-.986l11.54 6.347a1.125 1.125 0 0 1 0 1.972l-11.54 6.347a1.125 1.125 0 0 1-1.667-.986V5.653Z" />
          </svg>
        </button>
        <span v-if="item.state === 'analyzing'" class="text-xs text-amber-400">
          <svg class="w-4 h-4 animate-spin" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
          </svg>
        </span>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { AnalysisState } from '@khaos/shared'
import StateBadge from './StateBadge.vue'

defineProps<{
  items: Array<{
    id: string
    title: string
    subtitle: string
    state: AnalysisState
    error?: string
  }>
  selectedId: string | null
}>()

defineEmits<{
  select: [id: string]
  analyze: [id: string]
}>()
</script>
