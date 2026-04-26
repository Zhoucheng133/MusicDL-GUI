<template>
  <TitleBar v-if="!isWindows" />
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';
import TitleBar from './components/TitleBar.vue';
import { useTheme } from 'vuetify';
const theme = useTheme()

import { platform } from '@tauri-apps/plugin-os';
const currentPlatform = platform();

let isWindows = currentPlatform=='windows';

onMounted(async ()=>{
  const appWindow = getCurrentWindow()
  appWindow.show();
  const systemTheme = await appWindow.theme();
  theme.change(systemTheme || 'light')
  await appWindow.listen('tauri://theme-changed', (event) => {
    theme.change(event.payload as string)
  })
})
</script>