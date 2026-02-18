<template>
  <div class="flex items-center gap-2 text-xs">
    <!-- Checking -->
    <template v-if="checking">
      <svg
        class="w-4 h-4 text-yellow-400 animate-spin"
        fill="none"
        viewBox="0 0 24 24"
      >
        <circle
          class="opacity-25"
          cx="12"
          cy="12"
          r="10"
          stroke="currentColor"
          stroke-width="4"
        />
        <path
          class="opacity-75"
          fill="currentColor"
          d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
        />
      </svg>
      <span class="text-yellow-400">Checking...</span>
    </template>

    <!-- Valid -->
    <template v-else-if="valid === true">
      <svg class="w-4 h-4 text-green-400" fill="currentColor" viewBox="0 0 20 20">
        <path
          fill-rule="evenodd"
          d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
          clip-rule="evenodd"
        />
      </svg>
      <span class="text-green-400">{{ label || 'Valid' }}</span>
    </template>

    <!-- Invalid -->
    <template v-else-if="valid === false">
      <svg class="w-4 h-4 text-red-400" fill="currentColor" viewBox="0 0 20 20">
        <path
          fill-rule="evenodd"
          d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
          clip-rule="evenodd"
        />
      </svg>
      <span class="text-red-400">{{ label || 'Invalid' }}</span>
    </template>

    <!-- Unchecked -->
    <template v-else>
      <div class="w-4 h-4 rounded-full border border-slate-600" />
      <span class="text-slate-500">{{ label || 'Not checked' }}</span>
    </template>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  valid?: boolean | null
  checking?: boolean
  label?: string
}>()
</script>
