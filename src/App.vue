<template>

</template>

<script setup>
import { onMounted } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';

onMounted(()=>{
  const appWindow = getCurrentWindow()
  appWindow.show();
})
</script>