<template>
  <v-dialog v-model="showSettings" width="auto" style="user-select: none; -webkit-user-drag: none; -webkit-user-select: none;">
    <v-card width="400"
      prepend-icon="mdi-cog"
      title="设置">
      <v-card-text style="padding-bottom: 0;">
        <div style="margin-top: 5px; display: flex; gap: 10px; align-items: center;">
          <v-text-field
            label="工作目录"
            hide-details
            v-model="workdir"
            :readonly="true"
            autocomplete="off"
          ></v-text-field>
          <v-btn icon="mdi-folder-open" @click="selectWorkDir" variant="text"></v-btn>
        </div>
        <div style="margin-top: 5px; display: flex; gap: 10px; align-items: center;">
          <v-text-field
            label="保存路径"
            hide-details
            v-model="downloadPath"
            :readonly="true"
            autocomplete="off"
          ></v-text-field>
          <v-btn icon="mdi-folder-open" @click="selectDir" variant="text"></v-btn>
        </div>
        <div style="margin-top: 10px;">
          <v-select 
            label="编码"
            :hide-details="true"
            :items="encodeList"
            v-model="encode"
            @update:model-value="encodeChanged"
          ></v-select>
        </div>
        <div style="margin-top: 10px;">
          <v-checkbox
            label="记住当前配置"
            density="compact"
            :hide-details="true"
            v-model="saveConfig"
            @change="saveConfigHandler"
          ></v-checkbox>
        </div>
      </v-card-text>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn variant="flat" @click="showSettings=false" color="rgb(20, 118, 108)">完成</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import Store, { encodeList } from '../store';
import { storeToRefs } from 'pinia';
import { message, open } from '@tauri-apps/plugin-dialog';
const store=Store();

let { downloadPath, encode, workdir, saveConfig, noConfirm }=storeToRefs(store);

async function saveConfigHandler(){
  if(saveConfig.value){
    const isExist = await store.pathCheck(downloadPath.value);
    if(!isExist){
      saveConfig.value = false;
      await message('下载路径不存在', { title: '无法记住当前配置', kind: 'error' });
      return;
    }
    localStorage.setItem("noConfirm", "true");
    noConfirm.value = true;
  }else{
    localStorage.setItem("noConfirm", "false");
    noConfirm.value = false;
  }
}

const showSettings=ref(false);
const show=()=>{
  showSettings.value=true;
}
function encodeChanged(){
  localStorage.setItem("encode", encode.value);
}

async function selectWorkDir(){
  const file = await open({
    multiple: false,
    directory: true,
  });
  if(file!=null){
    workdir.value = file;
    localStorage.setItem("workdir", file);
  }
}

async function selectDir(){
  const file = await open({
    multiple: false,
    directory: true,
  });
  if(file!=null){
    downloadPath.value = file;
    localStorage.setItem("downloadPath", file);
  }
}

defineExpose({
  show
});
</script>