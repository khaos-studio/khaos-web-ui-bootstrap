<template>
  <div class="w-full">
    <!-- Search input container with focus state -->
    <div class="relative">
      <input
        v-model="localSearch"
        type="text"
        class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all"
        :placeholder="placeholder"
        @input="handleInput"
      />

      <!-- Clear button - shown when text is present -->
      <div
        v-if="localSearch.length > 0"
        class="absolute right-3 top-1/2 transform -translate-y-1/2 flex items-center gap-2"
      >
        <!-- Character count -->
        <span class="text-xs text-gray-500">{{ localSearch.length }}</span>

        <!-- Clear button -->
        <button
          @click="clearSearch"
          class="text-gray-400 hover:text-gray-600 transition-colors"
          type="button"
          aria-label="Clear search"
        >
          <svg
            class="w-4 h-4"
            fill="currentColor"
            viewBox="0 0 20 20"
          >
            <path
              fill-rule="evenodd"
              d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
              clip-rule="evenodd"
            />
          </svg>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { useProjectsStore } from '../stores/projects'

// Props
interface Props {
  placeholder?: string
}

const props = withDefaults(defineProps<Props>(), {
  placeholder: 'Search by title, author, path...'
})

// Store and state
const store = useProjectsStore()
const localSearch = ref<string>('')

// Watch for external changes to store.search and sync back to input
watch(
  () => store.search,
  (newSearch) => {
    localSearch.value = newSearch
  }
)

// Debounce timer for search input
let debounceTimer: ReturnType<typeof setTimeout> | null = null

/**
 * Handle input with debounce to avoid excessive store updates
 * Debounce delay: 300ms
 */
const handleInput = (): void => {
  // Clear existing timer
  if (debounceTimer) {
    clearTimeout(debounceTimer)
  }

  // Set new debounce timer
  debounceTimer = setTimeout(() => {
    store.setSearch(localSearch.value)
    debounceTimer = null
  }, 300)
}

/**
 * Clear search input and store
 */
const clearSearch = (): void => {
  localSearch.value = ''
  if (debounceTimer) {
    clearTimeout(debounceTimer)
  }
  store.setSearch('')
}
</script>
