<template>
  <div class="p-3 border border-slate-700 rounded bg-slate-800 hover:bg-slate-700 transition-colors">
    <!-- Project info section -->
    <div class="mb-3">
      <!-- Project title -->
      <h3 class="text-sm font-semibold text-slate-100 mb-1 truncate">
        {{ project.title }}
      </h3>

      <!-- Author - if exists -->
      <p v-if="project.author" class="text-xs text-slate-400 mb-1">
        {{ project.author }}
      </p>

      <!-- Path - truncated -->
      <p class="text-xs text-slate-500 truncate" :title="project.path">
        {{ project.path }}
      </p>
    </div>

    <!-- Metadata row -->
    <div class="flex items-center justify-between gap-2 mb-3 text-xs">
      <!-- Scene count -->
      <span class="text-slate-400">
        {{ project.scene_count }} {{ project.scene_count === 1 ? 'scene' : 'scenes' }}
      </span>

      <!-- Modified date -->
      <span class="text-slate-500">
        {{ relativeTime }}
      </span>
    </div>

    <!-- Action buttons -->
    <div class="flex gap-2">
      <button
        @click="handleOpen"
        class="flex-1 px-2 py-1 bg-blue-600 hover:bg-blue-700 text-slate-100 text-xs rounded transition-colors font-medium"
      >
        Open
      </button>
      <button
        @click="handleDelete"
        class="flex-1 px-2 py-1 bg-red-600 hover:bg-red-700 text-slate-100 text-xs rounded transition-colors font-medium"
      >
        Delete
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { Project } from '@khaos/shared'

// Props
interface Props {
  project: Project
}

const props = defineProps<Props>()

/**
 * Format Unix timestamp to relative time string
 */
const relativeTime = computed<string>(() => {
  const now = Date.now()
  const timestamp = props.project.modified * 1000
  const diffMs = now - timestamp
  const diffSecs = Math.floor(diffMs / 1000)
  const diffMins = Math.floor(diffSecs / 60)
  const diffHours = Math.floor(diffMins / 60)
  const diffDays = Math.floor(diffHours / 24)

  if (diffSecs < 60) {
    return 'just now'
  } else if (diffMins < 60) {
    return `${diffMins}m ago`
  } else if (diffHours < 24) {
    return `${diffHours}h ago`
  } else if (diffDays < 30) {
    return `${diffDays}d ago`
  } else {
    const date = new Date(timestamp)
    return date.toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
      year: date.getFullYear() !== new Date().getFullYear() ? 'numeric' : undefined
    })
  }
})

// Emit events for actions
const emit = defineEmits<{
  selected: [project: Project]
  delete: [project: Project]
}>()

/**
 * Handle open project
 */
const handleOpen = (): void => {
  emit('selected', props.project)
}

/**
 * Handle delete project
 */
const handleDelete = (): void => {
  emit('delete', props.project)
}
</script>
