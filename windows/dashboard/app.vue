<template>
  <div class="h-screen flex flex-col bg-slate-950 text-slate-100">
    <!-- Header -->
    <DashboardHeader
      :title="store.projectTitle"
      :summary="store.summary"
      :progress="store.analysisProgress"
      :analyzing="store.analyzing"
      :daemon-running="store.daemonStatus?.running ?? false"
      @analyze-all="store.analyzeAll()"
    />

    <!-- Error Banner -->
    <div v-if="store.error && !dismissedError" class="bg-red-900 border-b border-red-800 px-4 py-3 flex items-center justify-between">
      <div class="flex items-center gap-3">
        <svg class="w-5 h-5 text-red-200 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
          <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
        </svg>
        <p class="text-sm text-red-100">{{ store.error }}</p>
      </div>
      <button @click="dismissedError = true" class="text-red-200 hover:text-red-100 transition-colors flex-shrink-0 ml-2" aria-label="Dismiss">
        <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
          <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
        </svg>
      </button>
    </div>

    <!-- Loading Overlay -->
    <div v-if="store.loading" class="absolute inset-0 bg-black/50 flex items-center justify-center z-50 rounded-lg">
      <div class="bg-slate-800 rounded-lg px-6 py-4 flex flex-col items-center gap-3">
        <svg class="w-8 h-8 text-slate-300 animate-spin" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
        </svg>
        <p class="text-slate-300 text-sm">Loading project...</p>
      </div>
    </div>

    <!-- No Project State -->
    <div v-if="!store.projectPath && !store.loading" class="flex-1 flex items-center justify-center">
      <div class="text-center text-slate-500 space-y-3">
        <svg class="w-16 h-16 mx-auto" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="1">
          <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 12.75V12A2.25 2.25 0 014.5 9.75h15A2.25 2.25 0 0121.75 12v.75m-8.69-6.44l-2.12-2.12a1.5 1.5 0 00-1.061-.44H4.5A2.25 2.25 0 002.25 6v12a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18V9a2.25 2.25 0 00-2.25-2.25h-5.379a1.5 1.5 0 01-1.06-.44z" />
        </svg>
        <p class="text-lg font-medium text-slate-400">No project loaded</p>
        <p class="text-sm">Select a project from the Projects window to begin.</p>
      </div>
    </div>

    <!-- Main Content -->
    <main v-else-if="!store.loading" class="flex-1 flex overflow-hidden">
      <!-- Left Sidebar: Section Navigation -->
      <aside class="w-48 border-r border-slate-800 bg-slate-900/50 flex-shrink-0">
        <SectionNav
          :active="store.currentSection"
          :scenes-count="store.scenes.length"
          :characters-count="store.characters.length"
          :locations-count="store.locations.length"
          @select="store.switchSection"
        />
      </aside>

      <!-- Content Area -->
      <div class="flex-1 flex flex-col overflow-hidden">
        <!-- Detail Panel (when item selected) -->
        <DetailPanel
          v-if="store.detailOpen && store.selectedItem"
          :item="store.selectedItem"
          :analysis="store.detailAnalysis"
          :entity-type="store.currentSection"
          @close="store.closeDetail()"
          @analyze="store.analyzeItem"
        />

        <!-- Item List (default view) -->
        <ItemList
          v-else
          :items="store.currentItemsWithState"
          :selected-id="store.selectedId"
          @select="store.selectItem"
          @analyze="store.analyzeItem"
        />
      </div>
    </main>

    <!-- Footer -->
    <footer v-if="store.projectPath" class="border-t border-slate-800 bg-slate-900 px-4 py-2 flex items-center justify-between text-xs text-slate-500">
      <div class="flex items-center gap-4">
        <span v-if="store.detailOpen">
          <kbd class="px-1 py-0.5 bg-slate-800 rounded text-slate-400">Esc</kbd> Back
        </span>
        <span>
          <kbd class="px-1 py-0.5 bg-slate-800 rounded text-slate-400">1</kbd>
          <kbd class="px-1 py-0.5 bg-slate-800 rounded text-slate-400">2</kbd>
          <kbd class="px-1 py-0.5 bg-slate-800 rounded text-slate-400">3</kbd> Sections
        </span>
      </div>
      <span class="truncate max-w-xs" :title="store.projectPath">{{ store.projectPath }}</span>
    </footer>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { useDashboardStore } from './stores/dashboard'
import DashboardHeader from './components/DashboardHeader.vue'
import SectionNav from './components/SectionNav.vue'
import ItemList from './components/ItemList.vue'
import DetailPanel from './components/DetailPanel.vue'

const store = useDashboardStore()
const dismissedError = ref(false)

// Event listener cleanup handles
const unlisteners: Array<() => void> = []

// Keyboard shortcuts
function handleKeydown(e: KeyboardEvent) {
  // Section switching: 1, 2, 3
  if (e.key === '1' && !e.metaKey && !e.ctrlKey) {
    store.switchSection('scenes')
  } else if (e.key === '2' && !e.metaKey && !e.ctrlKey) {
    store.switchSection('characters')
  } else if (e.key === '3' && !e.metaKey && !e.ctrlKey) {
    store.switchSection('locations')
  }

  // Escape to close detail
  if (e.key === 'Escape' && store.detailOpen) {
    store.closeDetail()
  }
}

onMounted(async () => {
  window.addEventListener('keydown', handleKeydown)

  // Try to set up event listeners, but don't fail if they're not allowed
  try {
    // Listen for project selection from Projects window
    const unlistenProject = await listen<{ project_title: string; project_path: string }>(
      'app:project-selected',
      async (event) => {
        console.log('[app.vue] Event received: app:project-selected', event.payload)
        const { project_title, project_path } = event.payload
        try {
          console.log('[app.vue] Loading from event project_path:', project_path)
          await store.loadProject(project_path, project_title)
          console.log('[app.vue] Project loaded successfully')
        } catch (err) {
          console.error('[app.vue] Failed to load project:', err)
          store.error = String(err)
        }
      },
    )
    unlisteners.push(unlistenProject)

    // Listen for analysis started events
    const unlistenStarted = await listen<{ item_type: string; item_id: string; operation: string }>(
      'daemon:analysis-started',
      (event) => {
        store.onAnalysisStarted(event.payload)
      },
    )
    unlisteners.push(unlistenStarted)

    // Listen for analysis progress events (from daemon or CLI)
    const unlistenProgress = await listen<{ item_type: string; item_id: string; progress: number; status: string }>(
      'daemon:analysis-progress',
      (event) => {
        store.onAnalysisProgress(event.payload)
      },
    )
    unlisteners.push(unlistenProgress)

    // Listen for analysis completion events
    const unlistenCompleted = await listen<{ item_type: string; item_id: string; success: boolean; error?: string }>(
      'daemon:analysis-completed',
      (event) => {
        store.onAnalysisCompleted(event.payload)
      },
    )
    unlisteners.push(unlistenCompleted)
  } catch (err) {
    console.error('[app.vue] CRITICAL: Failed to set up event listeners:', err)
    console.error('[app.vue] Error details:', String(err))
  }

  // Check URL params for direct project loading (e.g., from deep link)
  console.log('[app.vue] Checking URL params...')
  console.log('[app.vue] window.location.search:', window.location.search)
  const params = new URLSearchParams(window.location.search)
  const projectPath = params.get('project')
  const projectTitle = params.get('title')
  console.log('[app.vue] projectPath from URL:', projectPath)
  if (projectPath) {
    console.log('[app.vue] Loading project from URL param:', projectPath)
    try {
      await store.loadProject(projectPath, projectTitle || undefined)
      console.log('[app.vue] Project loaded successfully')
    } catch (err) {
      console.error('[app.vue] Failed to load project from URL param:', err)
      store.error = String(err)
    }
  } else {
    console.log('[app.vue] No project param in URL')
  }
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown)
  unlisteners.forEach((fn) => fn())
})
</script>

<style scoped>
:deep(.scrollbar-hide) {
  -ms-overflow-style: none;
  scrollbar-width: none;
}

:deep(.scrollbar-hide::-webkit-scrollbar) {
  display: none;
}
</style>
