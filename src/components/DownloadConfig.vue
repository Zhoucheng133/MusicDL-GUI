<template>
  <v-dialog v-model="downloadDialog" width="auto" style="user-select: none; -webkit-user-drag: none; -webkit-user-select: none;">
    <v-card
      width="400"
      prepend-icon="mdi-download"
      :title="`下载 ${props.selectItem?.name}`"
    >
      <v-card-text style="padding-bottom: 0;">
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
        <v-btn variant="text" @click="downloadDialog = false">取消</v-btn>
        <v-btn variant="flat" @click="download" color="rgb(20, 118, 108)">下载</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
  <ProgressDialog ref="progressDialogRef" />
</template>

<script lang="ts" setup>
import { open } from '@tauri-apps/plugin-dialog';
import Store, { encodeList } from "../store/index";
import { storeToRefs } from 'pinia';
import { message } from '@tauri-apps/plugin-dialog';
import ProgressDialog from './ProgressDialog.vue';
import { ref } from 'vue';

const store = Store();

let { downloadPath, encode, saveConfig } = storeToRefs(store);

const downloadDialog=ref(false);

async function saveConfigHandler(){
  if(saveConfig.value){
    const isExist = await store.pathCheck(downloadPath.value);
    if(!isExist){
      saveConfig.value = false;
      await message('下载路径不存在', { title: '无法记住当前配置', kind: 'error' });
      return;
    }
    localStorage.setItem("saveConfig", "true");
    saveConfig.value = true;
  }else{
    localStorage.setItem("saveConfig", "false");
    saveConfig.value = false;
  }
}

const download=()=>{
  store.download(props.selectItem);
  downloadDialog.value = false;
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

function encodeChanged(){
  localStorage.setItem("encode", encode.value);
}

function showDialog(){ 
  downloadDialog.value = true;
}

const props = defineProps(["selectItem"])

defineExpose({
  showDialog
})
</script>