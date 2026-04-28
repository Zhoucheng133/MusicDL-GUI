<template>
  <div class="search_bar">
    <v-select
      :items="clients"
      item-title="name"
      item-value="value"
      v-model="client"
      hide-details
    ></v-select>
    <v-text-field
      v-model="keyword"
      label="搜索"
      hide-details
      append-inner-icon="mdi-magnify"
      autocomplete="off"
      @click:append-inner="store.search"
      @keydown.enter="store.search"
    ></v-text-field>
    <v-btn icon="mdi-cog" variant="text" size="small" @click="showSettings"></v-btn>
  </div>
  <Settings ref="settingsRef" />
</template>

<script lang="ts" setup>
import { storeToRefs } from 'pinia';
import Store from '../store';
import Settings from './Settings.vue';
import { ref } from 'vue';
const store = Store();
let {client, keyword}=storeToRefs(store);

const settingsRef=ref();
const showSettings=()=>{
  settingsRef.value.show();
}

const clients=[
  {name: '网易云', value: 'NeteaseMusicClient'},
  {name: 'QQ音乐', value: 'QQMusicClient'},
  {name: '酷我音乐', value: 'KuwoMusicClient'},
  {name: '咪咕音乐', value: 'MiguMusicClient'},
  {name: '千千音乐', value: 'QianqianMusicClient'},
]
</script>