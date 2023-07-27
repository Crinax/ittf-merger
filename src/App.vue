<script setup lang="ts">
import { setup } from './states/app/app-state.ts';

const {
  generateFile,
  isLoading,
  hasError,
  hasSuccess,
  result,
  error,
  notLoadedOrHasErrors,
} = setup();
</script>

<template>
  <div class="app-container" v-loading.fullscreen.lock="isLoading">
    <Transition name="slide">
      <el-alert
        v-if="hasError"
        title="Произошла ошибка!"
        type="error"
        :description="error"
        effect="dark"
        show-icon
      />
    </Transition>

    <TransitionGroup name="hide">
      <el-button
        key="generate-button"
        v-if="notLoadedOrHasErrors"
        type="primary"
        @click="generateFile"
      >
        Сформировать Excel-файл
      </el-button>

      <el-button
        key="open-file"
        v-if="hasSuccess"
        type="success"
      >
        Открыть файл
      </el-button>
    </TransitionGroup>
  </div>
</template>

<style scoped>
@import './styles/app/app-style.css';
</style>
