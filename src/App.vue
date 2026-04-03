<script setup lang="ts">
import MacTitleBar from "./components/MacTitleBar.vue";
import CmdBar from "./components/CmdBar.vue";
import { onMounted, onUnmounted } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { register } from '@tauri-apps/plugin-global-shortcut';

const appWindow = getCurrentWindow();

const setupWindowLogic = async () => {
  try {
    // 注册全局快捷键 Alt+Space 唤出/隐藏
    await register('Alt+Space', async () => {
      const isMinimized = await appWindow.isMinimized();
      if (isMinimized) {
        await appWindow.unminimize();
        await appWindow.setFocus();
      } else {
        await appWindow.minimize();
      }
    });
  } catch (err) { console.warn('快捷键注册失败:', err); }
};

const handleKeydown = async (e: KeyboardEvent) => {
  // Ctrl+W 或是 Esc 键，最小化窗口
  if (((e.key === 'w' || e.key === 'W') && (e.ctrlKey || e.metaKey)) || e.key === 'Escape') {
    e.preventDefault();
    await appWindow.minimize();
  }
};

onMounted(() => {
  setupWindowLogic();
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
