import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type {
  ProjectSummary,
  SceneSummary,
  CharacterSummary,
  LocationSummary,
  SceneAnalysis,
  CharacterAnalysis,
  LocationAnalysis,
  AnalysisState,
  AnalysisIndex,
  AnalysisResult,
  DashboardSection,
  DaemonStatus,
} from '@khaos/shared'

export const useDashboardStore = defineStore('dashboard', () => {
  // State
  const projectPath = ref<string>('')
  const projectTitle = ref<string>('')
  const summary = ref<ProjectSummary | null>(null)
  const currentSection = ref<DashboardSection>('scenes')
  const scenes = ref<SceneSummary[]>([])
  const characters = ref<CharacterSummary[]>([])
  const locations = ref<LocationSummary[]>([])
  const analysisStates = ref<Record<string, AnalysisState>>({})
  const analysisErrors = ref<Record<string, string>>({})
  const selectedId = ref<string | null>(null)
  const detailOpen = ref<boolean>(false)
  const detailAnalysis = ref<SceneAnalysis | CharacterAnalysis | LocationAnalysis | null>(null)
  const loading = ref<boolean>(false)
  const error = ref<string | null>(null)
  const analyzing = ref<boolean>(false)
  const daemonStatus = ref<DaemonStatus | null>(null)

  function syncAnalyzingFlag() {
    analyzing.value = Object.values(analysisStates.value).some((s) => s === 'analyzing')
  }

  // Getters
  const currentItems = computed(() => {
    switch (currentSection.value) {
      case 'scenes':
        return scenes.value.map((s) => ({ id: s.id, title: s.slugline, subtitle: `${s.word_count} words · ${s.characters.length} chars` }))
      case 'characters':
        return characters.value.map((c) => ({ id: c.id, title: c.name, subtitle: `${c.dialogue_lines} lines · ${c.scene_count} scenes` }))
      case 'locations':
        return locations.value.map((l) => ({ id: l.id, title: l.name, subtitle: `${l.scene_count} scenes · ${l.page_count} pages` }))
      default:
        return []
    }
  })

  const currentItemsWithState = computed(() => {
    return currentItems.value.map((item) => ({
      ...item,
      state: (analysisStates.value[item.id] || 'pending') as AnalysisState,
      error: analysisErrors.value[item.id],
    }))
  })

  const selectedItem = computed(() => {
    if (!selectedId.value) return null
    return currentItemsWithState.value.find((item) => item.id === selectedId.value) ?? null
  })

  const analysisProgress = computed(() => {
    const items = currentItems.value
    const analyzed = items.filter((i) => analysisStates.value[i.id] === 'analyzed').length
    return { analyzed, total: items.length }
  })

  // Actions
  async function loadProject(path: string, title?: string) {
    console.log('[Dashboard Store] loadProject called with path:', path)
    projectPath.value = path
    projectTitle.value = title || path.split('/').pop()?.replace('.kspd', '') || 'Untitled'
    summary.value = null
    scenes.value = []
    characters.value = []
    locations.value = []
    analysisStates.value = {}
    analysisErrors.value = {}
    selectedId.value = null
    detailOpen.value = false
    detailAnalysis.value = null
    daemonStatus.value = null
    loading.value = true
    error.value = null

    try {
      console.log('[Dashboard Store] Starting parallel data loads...')
      
      const [summaryData, scenesData, charsData, locsData, indexData] = await Promise.all([
        invoke<ProjectSummary>('get_project_summary', { projectPath: path }).then(d => { console.log('[Dashboard Store] Summary loaded:', d); return d }).catch(e => { console.error('[Dashboard Store] get_project_summary FAILED:', e); throw e }),
        invoke<SceneSummary[]>('get_scenes', { projectPath: path }).then(d => { console.log('[Dashboard Store] Scenes loaded:', d?.length, 'scenes'); return d }).catch(e => { console.error('[Dashboard Store] get_scenes FAILED:', e); throw e }),
        invoke<CharacterSummary[]>('get_characters', { projectPath: path }).then(d => { console.log('[Dashboard Store] Characters loaded:', d?.length, 'chars'); return d }).catch(e => { console.error('[Dashboard Store] get_characters FAILED:', e); throw e }),
        invoke<LocationSummary[]>('get_locations', { projectPath: path }).then(d => { console.log('[Dashboard Store] Locations loaded:', d?.length, 'locs'); return d }).catch(e => { console.error('[Dashboard Store] get_locations FAILED:', e); throw e }),
        invoke<AnalysisIndex>('scan_analysis_index', { projectPath: path }).then(d => { console.log('[Dashboard Store] Analysis index loaded'); return d }).catch(e => { console.error('[Dashboard Store] scan_analysis_index FAILED:', e); throw e }),
      ])

      console.log('[Dashboard Store] All data loaded successfully')
      scenes.value = scenesData
      characters.value = charsData
      locations.value = locsData
      summary.value = {
        scenes: summaryData?.scenes ?? scenesData.length,
        characters: summaryData?.characters ?? charsData.length,
        locations: summaryData?.locations ?? locsData.length,
        compositions: summaryData?.compositions ?? 0,
      }

      // If summary endpoint returned placeholder zeros, trust hydrated lists.
      if (
        summary.value.scenes === 0 &&
        summary.value.characters === 0 &&
        summary.value.locations === 0 &&
        (scenesData.length > 0 || charsData.length > 0 || locsData.length > 0)
      ) {
        summary.value.scenes = scenesData.length
        summary.value.characters = charsData.length
        summary.value.locations = locsData.length
      }

      // Build analysis state from index
      const states: Record<string, AnalysisState> = {}
      for (const s of scenesData) states[s.id] = indexData.scenes.includes(s.id) ? 'analyzed' : 'pending'
      for (const c of charsData) states[c.id] = indexData.characters.includes(c.id) ? 'analyzed' : 'pending'
      for (const l of locsData) states[l.id] = indexData.locations.includes(l.id) ? 'analyzed' : 'pending'
      analysisStates.value = states

      console.log('[Dashboard Store] Project loaded:', { scenes: scenes.value.length, characters: characters.value.length, locations: locations.value.length })

      // Check daemon status
      try {
        daemonStatus.value = await invoke<DaemonStatus>('get_daemon_status', { projectPath: path })
      } catch (err) {
        console.warn('[Dashboard Store] Daemon status check failed:', err)
        daemonStatus.value = null
      }
    } catch (e) {
      const errorMsg = String(e)
      console.error('[Dashboard Store] Failed to load project:', errorMsg)
      error.value = errorMsg
      throw e
    } finally {
      loading.value = false
    }
  }

  function switchSection(section: DashboardSection) {
    currentSection.value = section
    selectedId.value = null
    detailOpen.value = false
    detailAnalysis.value = null
  }

  async function selectItem(id: string) {
    selectedId.value = id
    detailOpen.value = true
    detailAnalysis.value = null

    try {
      if (currentSection.value === 'scenes') {
        const detail = await invoke<{ summary: SceneSummary; analysis: SceneAnalysis | null }>('get_scene_detail', { projectPath: projectPath.value, sceneId: id })
        detailAnalysis.value = detail.analysis
      } else if (currentSection.value === 'characters') {
        const detail = await invoke<{ summary: CharacterSummary; analysis: CharacterAnalysis | null }>('get_character_detail', { projectPath: projectPath.value, characterId: id })
        detailAnalysis.value = detail.analysis
      } else if (currentSection.value === 'locations') {
        const detail = await invoke<{ summary: LocationSummary; analysis: LocationAnalysis | null }>('get_location_detail', { projectPath: projectPath.value, locationId: id })
        detailAnalysis.value = detail.analysis
      }
    } catch (e) {
      console.error('Failed to load detail:', e)
    }
  }

  function closeDetail() {
    detailOpen.value = false
    selectedId.value = null
    detailAnalysis.value = null
  }

  async function analyzeItem(id: string) {
    analysisStates.value[id] = 'analyzing'
    analysisErrors.value[id] = ''

    try {
      let result: AnalysisResult
      if (currentSection.value === 'scenes') {
        result = await invoke<AnalysisResult>('analyze_scene', { projectPath: projectPath.value, sceneId: id })
      } else if (currentSection.value === 'characters') {
        result = await invoke<AnalysisResult>('analyze_character', { projectPath: projectPath.value, characterId: id })
      } else {
        result = await invoke<AnalysisResult>('analyze_location', { projectPath: projectPath.value, locationId: id })
      }

      if (result.success) {
        if (!daemonStatus.value?.running) {
          // CLI path is synchronous from command return perspective; verify
          // output via index refresh instead of optimistic state transition.
          await refreshAnalysisStates()
        }
      } else {
        analysisStates.value[id] = 'failed'
        analysisErrors.value[id] = result.error || 'Analysis failed'
      }
    } catch (e) {
      analysisStates.value[id] = 'failed'
      analysisErrors.value[id] = String(e)
    } finally {
      syncAnalyzingFlag()
    }
  }

  async function analyzeAll() {
    analyzing.value = true
    const items = currentItems.value

    // Mark all pending items as analyzing
    for (const item of items) {
      if (analysisStates.value[item.id] !== 'analyzed') {
        analysisStates.value[item.id] = 'analyzing'
      }
    }

    try {
      const result = await invoke<AnalysisResult>('analyze_all', {
        projectPath: projectPath.value,
        section: currentSection.value,
      })

      if (!result.success) {
        error.value = result.error || 'Batch analysis failed'
      }

      // If CLI fallback (no daemon), refresh all states
      if (!daemonStatus.value?.running) {
        await refreshAnalysisStates()
      } else {
        // Daemon has queued async work; keep header in analyzing mode.
        analyzing.value = true
      }
    } catch (e) {
      error.value = String(e)
    } finally {
      if (!daemonStatus.value?.running) {
        syncAnalyzingFlag()
      }
    }
  }

  async function refreshAnalysisStates() {
    try {
      const indexData = await invoke<AnalysisIndex>('scan_analysis_index', { projectPath: projectPath.value })
      const states = { ...analysisStates.value }

      for (const s of scenes.value) {
        if (states[s.id] !== 'analyzing') {
          states[s.id] = indexData.scenes.includes(s.id) ? 'analyzed' : 'pending'
        }
      }
      for (const c of characters.value) {
        if (states[c.id] !== 'analyzing') {
          states[c.id] = indexData.characters.includes(c.id) ? 'analyzed' : 'pending'
        }
      }
      for (const l of locations.value) {
        if (states[l.id] !== 'analyzing') {
          states[l.id] = indexData.locations.includes(l.id) ? 'analyzed' : 'pending'
        }
      }

      analysisStates.value = states
      syncAnalyzingFlag()
    } catch (e) {
      console.error('Failed to refresh analysis states:', e)
    }
  }

  // Handle daemon events — called from app.vue event listeners
  function onAnalysisStarted(payload: { item_type: string; item_id: string; operation: string }) {
    analyzing.value = true
    if (payload.item_id && payload.item_id !== 'all') {
      analysisStates.value[payload.item_id] = 'analyzing'
    } else {
      for (const item of currentItems.value) {
        if (analysisStates.value[item.id] !== 'analyzed') {
          analysisStates.value[item.id] = 'analyzing'
        }
      }
    }
  }

  function onAnalysisProgress(payload: { item_type: string; item_id: string; progress: number; status: string }) {
    analyzing.value = true
    if (payload.item_id && payload.item_id !== 'all') {
      analysisStates.value[payload.item_id] = 'analyzing'
    }
  }

  async function onAnalysisCompleted(payload: { item_type: string; item_id: string; success: boolean; error?: string }) {
    if (payload.item_id && payload.item_id !== 'all') {
      analysisStates.value[payload.item_id] = payload.success ? 'analyzed' : 'failed'
      if (!payload.success && payload.error) {
        analysisErrors.value[payload.item_id] = payload.error
      }
      if (payload.success) {
        await refreshAnalysisStates()
        if (selectedId.value === payload.item_id && detailOpen.value) {
          await selectItem(payload.item_id)
        }
      } else {
        syncAnalyzingFlag()
      }
    } else {
      // Batch completion — refresh all states
      await refreshAnalysisStates()
    }
  }

  return {
    // State
    projectPath,
    projectTitle,
    summary,
    currentSection,
    scenes,
    characters,
    locations,
    analysisStates,
    analysisErrors,
    selectedId,
    detailOpen,
    detailAnalysis,
    loading,
    error,
    analyzing,
    daemonStatus,
    // Getters
    currentItems,
    currentItemsWithState,
    selectedItem,
    analysisProgress,
    // Actions
    loadProject,
    switchSection,
    selectItem,
    closeDetail,
    analyzeItem,
    analyzeAll,
    refreshAnalysisStates,
    onAnalysisStarted,
    onAnalysisProgress,
    onAnalysisCompleted,
  }
})
