<template>
  <span :class="['inline-flex items-center gap-1.5 text-xs font-medium', colorClass]">
    <span v-if="state === 'analyzing'" class="relative flex h-2 w-2">
      <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-amber-400 opacity-75"></span>
      <span class="relative inline-flex rounded-full h-2 w-2 bg-amber-500"></span>
    </span>
    <span v-else :class="['inline-block h-2 w-2 rounded-full', dotClass]"></span>
    <span v-if="showLabel">{{ label }}</span>
  </span>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { AnalysisState } from '@khaos/shared'

const props = withDefaults(
  defineProps<{
    state: AnalysisState
    showLabel?: boolean
  }>(),
  { showLabel: false },
)

const colorClass = computed(() => {
  switch (props.state) {
    case 'analyzed':
      return 'text-emerald-400'
    case 'analyzing':
      return 'text-amber-400'
    case 'failed':
      return 'text-red-400'
    default:
      return 'text-slate-500'
  }
})

const dotClass = computed(() => {
  switch (props.state) {
    case 'analyzed':
      return 'bg-emerald-500'
    case 'failed':
      return 'bg-red-500'
    default:
      return 'bg-slate-600'
  }
})

const label = computed(() => {
  switch (props.state) {
    case 'analyzed':
      return 'Analyzed'
    case 'analyzing':
      return 'Analyzing...'
    case 'failed':
      return 'Failed'
    default:
      return 'Pending'
  }
})
</script>
