<template>
  <div class="h-screen flex flex-col bg-slate-950 text-slate-100">
    <!-- Header Section -->
    <header class="border-b border-slate-800 bg-slate-900 px-4 py-4">
      <div class="flex flex-col gap-1">
        <h1 class="text-lg font-semibold">Projects</h1>
        <p class="text-sm text-slate-400">
          {{ isSearching ? `${store.filteredProjects.length} results` : `${store.projects.length} projects discovered` }}
        </p>
      </div>
    </header>

    <!-- Error Banner Section -->
    <div v-if="store.error && !dismissedError" class="bg-red-900 border-b border-red-800 px-4 py-3 flex items-center justify-between">
      <div class="flex items-center gap-3">
        <svg class="w-5 h-5 text-red-200 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
          <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
        </svg>
        <div class="flex-1">
          <p class="text-sm text-red-100">
            {{ store.error }}
          </p>
        </div>
      </div>
      <button
        @click="dismissedError = true"
        class="text-red-200 hover:text-red-100 transition-colors flex-shrink-0 ml-2"
        aria-label="Dismiss error"
      >
        <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
          <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
        </svg>
      </button>
    </div>

    <!-- Loading State Overlay -->
    <div v-if="store.loading" class="absolute inset-0 bg-black/50 flex items-center justify-center z-50 rounded-lg">
      <div class="bg-slate-800 rounded-lg px-6 py-4 flex flex-col items-center gap-3">
        <svg class="w-8 h-8 text-slate-300 animate-spin" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
        </svg>
        <p class="text-slate-300 text-sm">Discovering projects...</p>
      </div>
    </div>

    <!-- Main Content Area -->
    <main class="flex-1 flex flex-col overflow-hidden">
      <!-- Search Section -->
      <div class="px-4 py-3 border-b border-slate-800 bg-slate-900 flex-shrink-0">
        <ProjectSearch placeholder="Search by title, author, path..." />
      </div>

      <!-- Recent Projects Section -->
      <div v-if="store.recentProjects.length > 0 && !isSearching" class="px-4 py-3 border-b border-slate-800 flex-shrink-0">
        <h2 class="text-sm font-semibold text-slate-300 mb-3">Recently Opened</h2>
        <div class="flex gap-3 overflow-x-auto pb-2 scrollbar-hide">
          <ProjectCard
            v-for="project in store.recentProjects"
            :key="project.path"
            :project="project"
            class="w-64 flex-shrink-0"
            @selected="handleProjectSelected"
          />
        </div>
      </div>

      <!-- Main Projects List Section -->
      <div class="flex-1 overflow-hidden">
        <ProjectList @selected="handleProjectSelected" />
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref, computed } from 'vue'
import { useProjectsStore } from './stores/projects'
import type { Project } from '@khaos/shared'
import ProjectSearch from './components/ProjectSearch.vue'
import ProjectCard from './components/ProjectCard.vue'
import ProjectList from './components/ProjectList.vue'

// Store
const store = useProjectsStore()

// Local state
const dismissedError = ref<boolean>(false)

// Computed
const isSearching = computed(() => store.isSearching)

// Lifecycle
onMounted(async () => {
  await store.loadProjects()
})

/**
 * Handle project selection from child components
 */
const handleProjectSelected = (project: Project): void => {
  // Project is already selected in the store via ProjectCard
  // Additional logic can be added here for navigation or other actions
  console.log('Project selected:', project.title)
}
</script>

<style scoped>
/* Custom scrollbar for recent projects scroll */
:deep(.scrollbar-hide) {
  -ms-overflow-style: none;
  scrollbar-width: none;
}

:deep(.scrollbar-hide::-webkit-scrollbar) {
  display: none;
}
</style>
