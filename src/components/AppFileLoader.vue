<script setup lang="ts">
export interface AppFileLoaderProps {
  modelValue?: File,
}

export interface AppFileLoaderEmits {
  (e: 'update:modelValue', value: File): void,
}

defineProps<AppFileLoaderProps>();
const emit = defineEmits<AppFileLoaderEmits>();

const computeFilePath = (event: Event) => {
  const target = event.target as HTMLInputElement;
  const [file] = target.files;

  emit('update:modelValue', file);
}
</script>

<template>
  <input class="hidden-input" type="file" @change="computeFilePath" />
</template>
