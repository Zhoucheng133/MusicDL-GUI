<template>
  <v-dialog v-model="showProgressDialog" width="auto" style="user-select: none; -webkit-user-drag: none; -webkit-user-select: none;">
    <v-card
      width="400"
      prepend-icon="mdi-download"
      :title="downloading ? '下载中...' : '转码中...'"
    >
      <v-progress-linear
        color="rgb(20, 118, 108)"
        :model-value="progress"
        v-if="downloading"
      ></v-progress-linear>
      <v-progress-linear
        color="rgb(20, 118, 108)"
        v-else
        :indeterminate="true"
      ></v-progress-linear>
      <v-card-actions>
        <v-btn text @click="cancelDownload">取消</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script lang="ts" setup>
import { computed } from 'vue';
import Store from '../store';
import { storeToRefs } from 'pinia';

const store=Store();
const { downloadProgress, showProgressDialog, downloading }=storeToRefs(store)

const progress=computed(()=>{
  return downloadProgress.value * 100;
})

const cancelDownload=()=>{
  // store.cancelDownload();
}
</script>