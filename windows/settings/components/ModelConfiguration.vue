<template>
  <div v-if="provider" class="flex flex-col gap-4">
    <!-- Provider Info -->
    <div>
      <h3 class="text-sm font-medium text-slate-300">{{ provider.title }}</h3>
      <p class="text-xs text-slate-500 mt-1">{{ provider.description }}</p>
    </div>

    <!-- Model Input -->
    <div>
      <label class="block text-xs font-medium text-slate-400 mb-1">
        Model override
      </label>
      <input
        type="text"
        :value="modelValue"
        @input="$emit('update:modelValue', ($event.target as HTMLInputElement).value)"
        :placeholder="provider.default_model"
        class="w-full bg-slate-800 border border-slate-700 rounded px-3 py-2 text-sm text-slate-200 placeholder-slate-600 focus:outline-none focus:border-blue-500 transition-colors"
      />
      <p class="text-xs text-slate-500 mt-1">
        Effective model: <span class="text-slate-300">{{ effectiveModel }}</span>
      </p>
    </div>

    <!-- API Key Status -->
    <div v-if="provider.requires_key" class="text-xs">
      <span class="text-slate-400">API key status: </span>
      <span v-if="validation?.valid" class="text-green-400">SET</span>
      <span v-else-if="validation && !validation.valid" class="text-red-400">
        missing {{ provider.env_var }}
      </span>
      <span v-else class="text-slate-500">not checked</span>
    </div>

    <div v-else-if="provider.id === 'ollama'" class="text-xs">
      <span class="text-slate-400">Status: </span>
      <span v-if="validating" class="text-yellow-400">checking...</span>
      <span v-else-if="validation?.valid" class="text-green-400">ready</span>
      <span v-else-if="validation && !validation.valid" class="text-red-400">
        {{ validation.errors[0] }}
      </span>
      <span v-else class="text-slate-500">not checked</span>
    </div>

    <div v-else class="text-xs">
      <span class="text-slate-400">API key status: </span>
      <span class="text-slate-500">not required</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { ProviderInfo, ProviderValidationResult } from '@khaos/shared'

defineProps<{
  provider?: ProviderInfo
  modelValue: string
  effectiveModel: string
  validation: ProviderValidationResult | null
  validating: boolean
}>()

defineEmits<{
  'update:modelValue': [value: string]
}>()
</script>
