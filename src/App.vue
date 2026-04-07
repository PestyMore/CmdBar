<script setup lang="ts">
import MacTitleBar from "./components/MacTitleBar.vue";
import CmdBar from "./components/CmdBar.vue";
import { onMounted, onUnmounted } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { enable, isEnabled } from '@tauri-apps/plugin-autostart';

const appWindow = getCurrentWindow();

// 保留开机自启逻辑
const setupAutostart = async () => {
  try {
    if (!(await isEnabled())) {
      await enable();
    }
  } catch (err) { console.error("自启配置异常:", err); }
};

// 仅保留 Esc 键最小化
const handleKeydown = async (e: KeyboardEvent) => {
  if (e.key === 'Escape') {
    e.preventDefault();
    await appWindow.minimize();
  }
};

onMounted(() => {
  setupAutostart();
  // 恢复最朴素的监听方式，去掉之前画蛇添足的捕获参数
  window.addEventListener('keydown', handleKeydown);
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown);
});
</script>

<template>
  <div class="window-layout">
    <MacTitleBar />
    <CmdBar />
  </div>
</template>

<style>
:root { font-family: -apple-system, BlinkMacSystemFont, "SF Pro Text", "Segoe UI", Roboto, Helvetica, sans-serif; font-size: 14px; font-weight: 400; color: #ffffff; background-color: transparent !important; }
body, #app { margin: 0; padding: 0; width: 100vw; height: 100vh; overflow: hidden; background-color: transparent !important; }
.window-layout { position: relative; width: 100vw; height: 100vh; display: flex; flex-direction: column; border-radius: 14px; overflow: hidden; border: 1px solid rgba(228, 60, 68, 0.3); box-sizing: border-box; background: linear-gradient(135deg, #110101 0%, #2a0003 50%, #110101 100%); }
</style>
