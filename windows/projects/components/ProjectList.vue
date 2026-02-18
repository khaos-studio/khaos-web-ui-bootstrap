<template>
  <div class="h-full flex flex-col overflow-hidden">
    <!-- Empty state -->
    <div
      v-if="store.filteredProjects.length === 0 && !store.loading"
      class="flex-1 flex items-center justify-center"
    >
      <div class="text-center">
        <svg
          class="mx-auto h-12 w-12 text-slate-500 mb-4"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
          />
        </svg>
        <p v-if="store.isSearching" class="text-slate-400">
          No projects found matching your search
        </p>
        <p v-else class="text-slate-400">
          No projects discovered yet
        </p>
      </div>
    </div>

    <!-- Projects list - scrollable container -->
    <div v-else class="flex-1 overflow-y-auto">
      <div class="space-y-2 px-4 py-3">
        <ProjectCard
          v-for="project in store.filteredProjects"
          :key="project.path"
          :project="project"
          @selected="handleProjectSelected"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useProjectsStore } from '../stores/projects'
import type { Project } from '@khaos/shared'
import ProjectCard from './ProjectCard.vue'

// Store
const store = useProjectsStore()

// Emit event when project is selected
const emit = defineEmits<{
  selected: [project: Project]
}>()

/**
 * Handle project selection from child ProjectCard component
 */
const handleProjectSelected = (project: Project): void => {
  emit('selected', project)
}
</script>
