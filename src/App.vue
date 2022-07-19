<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import HelloWorld from './components/HelloWorld.vue'
import Editor from './components/Editor.vue'
import { invoke } from '@tauri-apps/api'
import { type } from '@tauri-apps/api/os';

import { ref } from 'vue'


// do not use same name with ref
const textarea = ref('')

const onSave = async () => {
  // let hosts = await invoke('read_hosts', { "osType": osType });
  const osType = await type();
  console.log(textarea.value)
  invoke('write_hosts', { 'osType': osType, 'hosts': textarea.value }).then(_ => {
    ElMessage({
      message: '修改成功.',
      type: 'success',
    })
  }).catch(e => {
    ElMessage.error(`修改失败: ${e.message}`)
  });

}


onMounted(() => {
  console.log('onMounted')
  loadHosts()
})


// 加载 hosts
const loadHosts = async () => {
  const osType = await type();
  let hosts = await invoke('read_hosts', { 'osType': osType });
  console.log(hosts)
  textarea.value = hosts
}


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
