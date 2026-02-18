<template>
  <div class="space-y-6">
    <!-- Scene Analysis -->
    <template v-if="entityType === 'scenes' && sceneAnalysis">
      <AnalysisSection v-if="sceneAnalysis.summary" title="Summary" :content="sceneAnalysis.summary" />
      <AnalysisSection v-if="sceneAnalysis.narrative_role" title="Narrative Role" :content="sceneAnalysis.narrative_role" />
      <AnalysisSection v-if="sceneAnalysis.emotional_tone" title="Emotional Tone" :content="sceneAnalysis.emotional_tone" />
      <AnalysisSection v-if="sceneAnalysis.stakes" title="Stakes" :content="sceneAnalysis.stakes" />
      <AnalysisSection v-if="sceneAnalysis.pacing" title="Pacing" :content="sceneAnalysis.pacing" />
      <AnalysisSection v-if="sceneAnalysis.genre_tone" title="Genre & Tone" :content="sceneAnalysis.genre_tone" />
      <AnalysisListSection v-if="sceneAnalysis.themes?.length" title="Themes" :items="sceneAnalysis.themes" />
      <AnalysisSection v-if="sceneAnalysis.theme_details" title="Theme Details" :content="sceneAnalysis.theme_details" />
      <AnalysisListSection v-if="sceneAnalysis.plot_beats?.length" title="Plot Beats" :items="sceneAnalysis.plot_beats" />
      <AnalysisListSection v-if="sceneAnalysis.speaking_chars?.length" title="Speaking Characters" :items="sceneAnalysis.speaking_chars" />
      <AnalysisListSection v-if="sceneAnalysis.non_speaking_chars?.length" title="Non-Speaking Characters" :items="sceneAnalysis.non_speaking_chars" />
      <AnalysisListSection v-if="sceneAnalysis.callbacks?.length" title="Callbacks" :items="sceneAnalysis.callbacks" />
      <AnalysisListSection v-if="sceneAnalysis.sets_up?.length" title="Sets Up" :items="sceneAnalysis.sets_up" />
      <AnalysisListSection v-if="sceneAnalysis.strengths?.length" title="Strengths" :items="sceneAnalysis.strengths" />
      <AnalysisListSection v-if="sceneAnalysis.development_areas?.length" title="Development Areas" :items="sceneAnalysis.development_areas" />
      <AnalysisListSection v-if="sceneAnalysis.visual_symbols?.length" title="Visual Symbols" :items="sceneAnalysis.visual_symbols" />
      <AnalysisSection v-if="sceneAnalysis.location_significance" title="Location Significance" :content="sceneAnalysis.location_significance" />
    </template>

    <!-- Character Analysis -->
    <template v-if="entityType === 'characters' && characterAnalysis">
      <AnalysisSection v-if="characterAnalysis.summary" title="Summary" :content="characterAnalysis.summary" />
      <AnalysisSection v-if="characterAnalysis.arc" title="Character Arc" :content="characterAnalysis.arc" />
      <AnalysisSection v-if="characterAnalysis.emotional_journey" title="Emotional Journey" :content="characterAnalysis.emotional_journey" />
      <AnalysisSection v-if="characterAnalysis.arc_quality" title="Arc Quality" :content="characterAnalysis.arc_quality" />
      <AnalysisListSection v-if="characterAnalysis.key_turning_points?.length" title="Key Turning Points" :items="characterAnalysis.key_turning_points" />
      <AnalysisListSection v-if="characterAnalysis.traits?.length" title="Traits" :items="characterAnalysis.traits" />
      <AnalysisListSection v-if="characterAnalysis.goals?.length" title="Goals" :items="characterAnalysis.goals" />
      <AnalysisListSection v-if="characterAnalysis.conflicts?.length" title="Conflicts" :items="characterAnalysis.conflicts" />
      <AnalysisSection v-if="characterAnalysis.background" title="Background" :content="characterAnalysis.background" />
      <AnalysisSection v-if="characterAnalysis.stakes" title="Stakes" :content="characterAnalysis.stakes" />
      <AnalysisListSection v-if="characterAnalysis.relationships?.length" title="Relationships" :items="characterAnalysis.relationships" />
      <AnalysisSection v-if="characterAnalysis.dialogue_voice" title="Dialogue Voice" :content="characterAnalysis.dialogue_voice" />
      <AnalysisListSection v-if="characterAnalysis.dialogue_patterns?.length" title="Dialogue Patterns" :items="characterAnalysis.dialogue_patterns" />
      <AnalysisSection v-if="characterAnalysis.dialogue_subtext" title="Dialogue Subtext" :content="characterAnalysis.dialogue_subtext" />
      <AnalysisListSection v-if="characterAnalysis.themes?.length" title="Themes" :items="characterAnalysis.themes" />
      <AnalysisSection v-if="characterAnalysis.thematic_role" title="Thematic Role" :content="characterAnalysis.thematic_role" />
      <AnalysisListSection v-if="characterAnalysis.symbolic_elements?.length" title="Symbolic Elements" :items="characterAnalysis.symbolic_elements" />
      <AnalysisSection v-if="characterAnalysis.narrative_role" title="Narrative Role" :content="characterAnalysis.narrative_role" />
      <AnalysisSection v-if="characterAnalysis.genre_fit" title="Genre Fit" :content="characterAnalysis.genre_fit" />
    </template>

    <!-- Location Analysis -->
    <template v-if="entityType === 'locations' && locationAnalysis">
      <AnalysisSection v-if="locationAnalysis.summary" title="Summary" :content="locationAnalysis.summary" />
      <AnalysisSection v-if="locationAnalysis.atmosphere" title="Atmosphere" :content="locationAnalysis.atmosphere" />
      <AnalysisSection v-if="locationAnalysis.environment" title="Environment" :content="locationAnalysis.environment" />
      <AnalysisSection v-if="locationAnalysis.visual_context" title="Visual Context" :content="locationAnalysis.visual_context" />
      <AnalysisSection v-if="locationAnalysis.significance" title="Significance" :content="locationAnalysis.significance" />
      <AnalysisListSection v-if="locationAnalysis.traits?.length" title="Traits" :items="locationAnalysis.traits" />
      <AnalysisListSection v-if="locationAnalysis.changes?.length" title="Changes" :items="locationAnalysis.changes" />
      <AnalysisSection v-if="locationAnalysis.narrative_role" title="Narrative Role" :content="locationAnalysis.narrative_role" />
      <AnalysisSection v-if="locationAnalysis.story_role" title="Story Role" :content="locationAnalysis.story_role" />
      <AnalysisListSection v-if="locationAnalysis.plot_anchors?.length" title="Plot Anchors" :items="locationAnalysis.plot_anchors" />
      <AnalysisListSection v-if="locationAnalysis.symbols?.length" title="Symbols" :items="locationAnalysis.symbols" />
      <AnalysisListSection v-if="locationAnalysis.themes?.length" title="Themes" :items="locationAnalysis.themes" />
      <AnalysisSection v-if="locationAnalysis.thematic_role" title="Thematic Role" :content="locationAnalysis.thematic_role" />
      <AnalysisListSection v-if="locationAnalysis.character_connections?.length" title="Character Connections" :items="locationAnalysis.character_connections" />
      <AnalysisSection v-if="locationAnalysis.production_notes" title="Production Notes" :content="locationAnalysis.production_notes" />
    </template>

    <!-- No analysis -->
    <div v-if="!hasAnalysis" class="text-center text-slate-500 py-8">
      <p class="text-sm">No analysis available yet.</p>
      <p class="text-xs mt-1">Click "Analyze" to generate AI analysis.</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { SceneAnalysis, CharacterAnalysis, LocationAnalysis, DashboardSection } from '@khaos/shared'

const props = defineProps<{
  entityType: DashboardSection
  analysis: SceneAnalysis | CharacterAnalysis | LocationAnalysis | null
}>()

const sceneAnalysis = computed(() => props.entityType === 'scenes' ? props.analysis as SceneAnalysis | null : null)
const characterAnalysis = computed(() => props.entityType === 'characters' ? props.analysis as CharacterAnalysis | null : null)
const locationAnalysis = computed(() => props.entityType === 'locations' ? props.analysis as LocationAnalysis | null : null)
const hasAnalysis = computed(() => props.analysis !== null)

// Sub-components for clean rendering
const AnalysisSection = {
  props: { title: String, content: String },
  template: `
    <div>
      <h4 class="text-xs font-semibold text-slate-400 uppercase tracking-wide mb-1.5">{{ title }}</h4>
      <p class="text-sm text-slate-200 leading-relaxed whitespace-pre-wrap">{{ content }}</p>
    </div>
  `,
}

const AnalysisListSection = {
  props: { title: String, items: Array },
  template: `
    <div>
      <h4 class="text-xs font-semibold text-slate-400 uppercase tracking-wide mb-1.5">{{ title }}</h4>
      <ul class="space-y-1">
        <li v-for="(item, i) in items" :key="i" class="text-sm text-slate-200 flex gap-2">
          <span class="text-slate-600 select-none">·</span>
          <span>{{ item }}</span>
        </li>
      </ul>
    </div>
  `,
}
</script>
