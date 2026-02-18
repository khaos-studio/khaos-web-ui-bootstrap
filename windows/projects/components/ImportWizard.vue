<template>
  <div class="absolute inset-0 bg-black/60 flex items-center justify-center z-50">
    <div class="bg-slate-900 rounded-lg border border-slate-700 w-full max-w-lg mx-4 flex flex-col max-h-[90vh]">
      <!-- Header -->
      <div class="flex items-center justify-between px-5 py-4 border-b border-slate-700">
        <div>
          <h2 class="text-lg font-semibold text-slate-100">Import Project</h2>
          <p class="text-xs text-slate-400 mt-0.5">{{ stepLabel }}</p>
        </div>
        <button
          @click="importStore.closeWizard()"
          class="text-slate-400 hover:text-slate-200 transition-colors p-1"
          aria-label="Close"
        >
          <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <!-- Step Indicator -->
      <div class="flex items-center gap-1 px-5 py-3 border-b border-slate-800">
        <template v-for="(s, i) in steps" :key="s.key">
          <div
            class="flex items-center gap-1.5"
          >
            <div
              class="w-6 h-6 rounded-full flex items-center justify-center text-xs font-medium transition-colors"
              :class="stepClass(s.key)"
            >
              <svg v-if="isStepComplete(s.key)" class="w-3.5 h-3.5" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
              </svg>
              <span v-else>{{ i + 1 }}</span>
            </div>
            <span class="text-xs" :class="s.key === importStore.step ? 'text-slate-200' : 'text-slate-500'">
              {{ s.label }}
            </span>
          </div>
          <div v-if="i < steps.length - 1" class="flex-1 h-px bg-slate-700 mx-1" />
        </template>
      </div>

      <!-- Step Content -->
      <div class="flex-1 overflow-hidden">
        <StepFilePicker v-if="importStore.step === 'file'" />
        <StepTitle v-else-if="importStore.step === 'title'" />
        <StepConfirm v-else-if="importStore.step === 'confirm'" />
        <StepCollision v-else-if="importStore.step === 'collision'" />
        <StepExecute v-else-if="importStore.step === 'execute'" />
        <StepResult v-else-if="importStore.step === 'result'" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useImportStore } from '../stores/import'
import type { ImportStep } from '@khaos/shared'
import StepFilePicker from './import/StepFilePicker.vue'
import StepTitle from './import/StepTitle.vue'
import StepConfirm from './import/StepConfirm.vue'
import StepCollision from './import/StepCollision.vue'
import StepExecute from './import/StepExecute.vue'
import StepResult from './import/StepResult.vue'

const importStore = useImportStore()

const steps = [
  { key: 'file' as ImportStep, label: 'File' },
  { key: 'title' as ImportStep, label: 'Title' },
  { key: 'confirm' as ImportStep, label: 'Confirm' },
  { key: 'execute' as ImportStep, label: 'Import' },
  { key: 'result' as ImportStep, label: 'Done' },
]

const stepOrder: ImportStep[] = ['file', 'title', 'confirm', 'execute', 'result']

const currentStepIndex = computed(() => {
  // Map collision to confirm's position
  const mapped = importStore.step === 'collision' ? 'confirm' : importStore.step
  return stepOrder.indexOf(mapped)
})

const stepLabel = computed(() => {
  switch (importStore.step) {
    case 'file': return 'Step 1 of 4: Select screenplay file'
    case 'title': return 'Step 2 of 4: Name your project'
    case 'confirm': return 'Step 3 of 4: Confirm destination'
    case 'collision': return 'Step 3 of 4: Resolve name collision'
    case 'execute': return 'Step 4 of 4: Importing...'
    case 'result': return 'Complete'
    default: return ''
  }
})

const isStepComplete = (key: ImportStep): boolean => {
  const mapped = key === 'collision' ? 'confirm' : key
  const keyIndex = stepOrder.indexOf(mapped)
  return keyIndex < currentStepIndex.value
}

const stepClass = (key: ImportStep): string => {
  const mapped = key === 'collision' ? 'confirm' : key
  const current = importStore.step === 'collision' ? 'confirm' : importStore.step
  if (mapped === current) return 'bg-blue-600 text-white'
  if (isStepComplete(key)) return 'bg-green-600 text-white'
  return 'bg-slate-700 text-slate-400'
}
</script>
