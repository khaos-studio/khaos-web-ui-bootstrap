<template>
  <div class="h-screen flex flex-col bg-slate-950 text-slate-100">
    <!-- Header Section -->
    <header class="border-b border-slate-800 bg-slate-900 px-4 py-4 flex items-center justify-between">
      <div>
        <h1 class="text-lg font-semibold">Projects</h1>
        <p class="text-sm text-slate-400">
          {{ isSearching ? `${store.filteredProjects.length} results` : `${store.projects.length} projects` }}
        </p>
      </div>
      <div class="flex items-center gap-2">
        <button
          @click="openSettings"
          class="p-2 text-slate-400 hover:text-slate-200 hover:bg-slate-800 rounded transition-colors"
          title="Settings (⌘,)"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="1.5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.325.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 0 1 1.37.49l1.296 2.247a1.125 1.125 0 0 1-.26 1.431l-1.003.827c-.293.241-.438.613-.43.992a7.723 7.723 0 0 1 0 .255c-.008.378.137.75.43.991l1.004.827c.424.35.534.955.26 1.43l-1.298 2.247a1.125 1.125 0 0 1-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.47 6.47 0 0 1-.22.128c-.331.183-.581.495-.644.869l-.213 1.281c-.09.543-.56.94-1.11.94h-2.594c-.55 0-1.019-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 0 1-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 0 1-1.369-.49l-1.297-2.247a1.125 1.125 0 0 1 .26-1.431l1.004-.827c.292-.24.437-.613.43-.991a6.932 6.932 0 0 1 0-.255c.007-.38-.138-.751-.43-.992l-1.004-.827a1.125 1.125 0 0 1-.26-1.43l1.297-2.247a1.125 1.125 0 0 1 1.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.086.22-.128.332-.183.582-.495.644-.869l.214-1.28Z" />
            <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z" />
          </svg>
        </button>
        <button
          @click="handleCreate"
          class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded transition-colors text-sm font-medium"
        >
          + Create
        </button>
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

    <!-- Delete Confirmation Dialog -->
    <div v-if="deleteConfirmation.project" class="absolute inset-0 bg-black/50 flex items-center justify-center z-50">
      <div class="bg-slate-800 rounded-lg px-6 py-4 max-w-sm w-full mx-4 border border-slate-700">
        <h2 class="text-lg font-semibold text-slate-100 mb-2">Delete Project?</h2>
        <p class="text-slate-300 mb-1">
          <span class="font-medium">{{ deleteConfirmation.project.title }}</span>
        </p>
        <p class="text-sm text-slate-400 mb-4">
          This will permanently remove the project directory and cannot be undone.
        </p>
        <div class="flex gap-3 justify-end">
          <button
            @click="cancelDelete"
            class="px-4 py-2 bg-slate-700 hover:bg-slate-600 text-slate-100 rounded transition-colors font-medium text-sm"
          >
            Cancel
          </button>
          <button
            @click="confirmDelete"
            class="px-4 py-2 bg-red-600 hover:bg-red-700 text-white rounded transition-colors font-medium text-sm"
            :disabled="deleteConfirmation.deleting"
          >
            <span v-if="deleteConfirmation.deleting" class="flex items-center gap-2">
              <svg class="w-4 h-4 animate-spin" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
              </svg>
              Deleting...
            </span>
            <span v-else>Delete</span>
          </button>
        </div>
      </div>
    </div>

    <!-- Import Wizard -->
    <ImportWizard v-if="importStore.isOpen" />

    <!-- Main Content Area -->
    <main class="flex-1 flex flex-col overflow-hidden">
      <!-- Search Section -->
      <div class="px-4 py-3 border-b border-slate-800 bg-slate-900 flex-shrink-0">
        <ProjectSearch placeholder="Search by title, author, path..." />
      </div>

      <!-- Main Projects List Section -->
      <div class="flex-1 overflow-hidden">
        <ProjectList @selected="handleProjectSelected" @delete="handleDelete" />
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref, computed, reactive } from 'vue'
import { useProjectsStore } from './stores/projects'
import { useImportStore } from './stores/import'
import { invoke } from '@tauri-apps/api/core'
import type { Project } from '@khaos/shared'
import ProjectSearch from './components/ProjectSearch.vue'
import ProjectList from './components/ProjectList.vue'
import ImportWizard from './components/ImportWizard.vue'

// Stores
const store = useProjectsStore()
const importStore = useImportStore()

// Local state
const dismissedError = ref<boolean>(false)
const deleteConfirmation = reactive<{ project: Project | null; deleting: boolean }>({
  project: null,
  deleting: false,
})

// Computed
const isSearching = computed(() => store.isSearching)

/**
 * Open the Settings window via Tauri command
 */
const openSettings = async (): Promise<void> => {
  try {
    await invoke('open_settings_window')
  } catch (err) {
    console.error('Failed to open settings:', err)
  }
}

/**
 * Handle Cmd+, keyboard shortcut (macOS standard for Preferences)
 */
const handleKeydown = (e: KeyboardEvent): void => {
  if (e.metaKey && e.key === ',') {
    e.preventDefault()
    openSettings()
  }
}

// Lifecycle
onMounted(async () => {
  window.addEventListener('keydown', handleKeydown)
  await store.loadProjects()
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown)
})

/**
 * Handle project selection from child components
 */
const handleProjectSelected = (project: Project): void => {
  console.log('Project selected:', project.title)
}

/**
 * Handle create new project — opens the import wizard
 */
const handleCreate = (): void => {
  importStore.openWizard()
}

/**
 * Show delete confirmation dialog
 */
const handleDelete = (project: Project): void => {
  deleteConfirmation.project = project
  deleteConfirmation.deleting = false
}

/**
 * Cancel deletion
 */
const cancelDelete = (): void => {
  deleteConfirmation.project = null
  deleteConfirmation.deleting = false
}

/**
 * Confirm and execute deletion
 */
const confirmDelete = async (): Promise<void> => {
  if (!deleteConfirmation.project) return

  deleteConfirmation.deleting = true
  try {
    await store.deleteProject(deleteConfirmation.project.id)
    deleteConfirmation.project = null
    deleteConfirmation.deleting = false
  } catch (err) {
    console.error('Failed to delete project:', err)
    deleteConfirmation.deleting = false
  }
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
