<template>
  <div class="flex flex-col gap-1">
    <button
      v-for="provider in providers"
      :key="provider.id"
      @click="$emit('select', provider.id)"
      class="w-full text-left px-3 py-2.5 rounded transition-colors"
      :class="
        provider.id === selected
          ? 'bg-blue-600 text-white'
          : 'text-slate-300 hover:bg-slate-800'
      "
    >
      <div class="font-medium text-sm">{{ provider.title }}</div>
      <div
        class="text-xs mt-0.5"
        :class="provider.id === selected ? 'text-blue-200' : 'text-slate-500'"
      >
        {{ provider.description }}
      </div>
    </button>
  </div>
</template>

<script setup lang="ts">
import type { ProviderInfo } from '@khaos/shared'

defineProps<{
  providers: ProviderInfo[]
  selected: string
}>()

defineEmits<{
  select: [providerId: string]
}>()
</script>
