<script setup>
import { ref } from 'vue'
import { app } from '@tauri-apps/api'
defineProps({
  msg: String
})

const i = ref(0)
let name = $ref('')
let version = $ref('')
let tauriVersion = $ref('')
;(async () => {
  name = await app.getName()
  version = await app.getVersion()
  tauriVersion = await app.getTauriVersion()
})()
</script>

<template>
  <h1 style="color: var(--el-color-primary)">
    <i-ep-element-plus /> Hello Element Plus!
  </h1>

  <el-button @click="i++">Incrase: {{ i }}</el-button>

  <el-descriptions title="App">
    <el-descriptions-item label="Name">
      {{ name }}
    </el-descriptions-item>
    <el-descriptions-item label="Version">
      {{ version }}
    </el-descriptions-item>
    <el-descriptions-item label="Tauri Version">
      {{ tauriVersion }}
    </el-descriptions-item>
  </el-descriptions>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
