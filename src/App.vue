<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
// import HelloWorld from './components/HelloWorld.vue'
import Editor from './components/Editor.vue'
import { invoke } from '@tauri-apps/api'
import { type } from '@tauri-apps/api/os';

import { ref } from 'vue'


// do not use same name with ref
const textarea = ref('')

const onSave = async () => {
  const osType = await type()
  const isElevated = await invoke('is_elevated')
  if (!isElevated) {
    ElMessage.warning(`非管理员模式运行，备份文件将保存在当前运行目录`)
  } 

  await invoke('backup_hosts', { 'osType': osType, 'isElevated': String(isElevated) })

  invoke('write_hosts', { 'osType': osType, 'hosts': textarea.value }).then(_ => {
    ElMessage({
      message: '修改成功.',
      type: 'success',
    })
  }).catch(e => {
    ElMessage.error(`修改失败: ${e.message}`)
  })

}
// 加载 hosts
const loadHosts = async () => {
  const osType = await type();
  let hosts = await invoke('read_hosts', { 'osType': osType });
  console.log(hosts)
  textarea.value = hosts
}

onMounted(() => {
  console.log('onMounted')
  loadHosts()
})

</script>

<template>
  <div class="editor">

    <Editor ref="cm" v-model:value="textarea"></Editor>
    <el-button class="icon" @click="loadHosts">刷新</el-button>
    <el-button class="icon" @click="onSave">保存</el-button>
  </div>
</template>

<style scoped>
.editor {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: row;
  align-items: flex-start;
}
</style>
