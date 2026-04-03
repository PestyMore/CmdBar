<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window';
const appWindow = getCurrentWindow();
</script>
<template>
  <div class="titlebar" data-tauri-drag-region>
    <div class="controls-container">
      <button class="mac-btn close" @click="appWindow.close()"></button>
      <button class="mac-btn minimize" @click="appWindow.minimize()"></button>
      <button class="mac-btn maximize" @click="appWindow.toggleMaximize()"></button>
    </div><div class="drag-region" data-tauri-drag-region></div>
  </div>
</template>
<style scoped>
.titlebar { height: 38px; background: transparent; display: flex; align-items: center; position: relative; width: 100%; flex-shrink: 0; user-select: none; -webkit-app-region: drag; border-bottom: 1px solid rgba(255, 255, 255, 0.1);  }
.controls-container { display: flex; margin-left: 14px; gap: 8px; align-items: center; -webkit-app-region: no-drag; z-index: 10; }
.mac-btn { width: 12px; height: 12px; border-radius: 50%; border: none; padding: 0; cursor: pointer; transition: all 0.2s ease; box-shadow: inset 0 0 0 1px rgba(0,0,0,0.1); }
.close { background-color: #ff5f57; } .minimize { background-color: #febc2e; } .maximize { background-color: #28c840; }
.mac-btn:hover { filter: brightness(0.85); transform: scale(1.05); } .mac-btn:active { filter: brightness(0.7); transform: scale(0.95); }
.drag-region { flex: 1; height: 100%; }
</style>
