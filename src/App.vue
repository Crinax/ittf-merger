<script setup lang="ts">
import AppFileLoader from './components/AppFileLoader.vue'

import { ref, watch } from 'vue';

const errorMessage = ref('');
const file = ref<File | undefined>(undefined);

watch(file, (newFile, oldFile) => {
  if (newFile === undefined) {
    return;
  }

  if (newFile?.type.startsWith('image')) {
    errorMessage.value = 'Невозможно загрузить такой тип файла';
    file.value = oldFile;
  }

  errorMessage.value = '';
})
</script>

<template>
  <div class="container">
    <p v-if="errorMessage.length">
      {{ errorMessage }}
    </p>
    <app-file-loader v-model="file" />
  </div>
</template>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
}
</style>
