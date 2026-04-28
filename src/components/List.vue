<template>
  <div class="list_container" v-if="list.length!=0 && !loading">
    <div class="list_header">
      <div class="list_header_content">
        <div class="list_header_center">序号</div>
        <div>标题</div>
        <div>艺人</div>
        <div class="list_header_center"><v-icon icon="mdi-timer-outline"></v-icon></div>
        <div class="list_header_center"><v-icon icon="mdi-link"></v-icon></div>
      </div>
      <div class="list_divider"></div>
      <div class="list_result_content" v-for="(item, index) in list" :key="index">
        <div class="list_header_center">{{ index+1 }}</div>
        <div class="list_name">{{ item.name }}</div>
        <div>{{ item.artist }}</div>
        <div class="list_header_center">{{ item.duration }}</div>
        <div class="list_header_center">
          <v-btn icon="mdi-download" variant="text" size="small" @click="downloadHandler(item)"></v-btn>
        </div>
      </div>
    </div>
  </div>
  <div class="list_empty" v-else-if="list.length==0 && !loading">
    <div class="list_empty_content">
      <v-icon icon="mdi-music-circle-outline"></v-icon>
      <div style="margin-top: 5px; font-size: 14px;">搜索关键词来查找</div>
    </div>
  </div>
  <div class="list_empty" v-else-if="loading">
    <v-progress-circular
      indeterminate
      :size="30"
      :width="4"
      color="rgb(20, 118, 108)"
    ></v-progress-circular>
  </div>
  <DownloadConfig ref="downloadConfigRef" :select-item="selectItem" />
</template>

<script lang="ts" setup>
import "../styles/list.css";
import Store, { ListItem } from "../store/index";
import { storeToRefs } from "pinia";
import { ref } from "vue";
import DownloadConfig from "./DownloadConfig.vue";

const store = Store();
const downloadConfigRef=ref();
let { list, loading, saveConfig } = storeToRefs(store);

let selectItem = ref<ListItem | null>(null);

function downloadHandler(item: ListItem) {
  if(saveConfig.value){
    store.download(item);
  }else{
    selectItem.value = item;
    downloadConfigRef.value.showDialog();
  }
}


</script>

<style scoped>
.list_name{
  width: 100%;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>