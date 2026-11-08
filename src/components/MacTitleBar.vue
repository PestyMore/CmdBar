<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";

const appWindow = getCurrentWindow();
</script>

<template>
  <div class="titlebar">
    <div class="drag" data-tauri-drag-region />
    <div class="lights">
      <button class="light close" title="关闭" @click="appWindow.close()" />
      <button class="light minimize" title="最小化" @click="appWindow.minimize()" />
      <button class="light maximize" title="最大化 / 还原" @click="appWindow.toggleMaximize()" />
    </div>
    <div class="brand">
      <span class="brand-mark" />
      <span>CmdBar</span>
    </div>
  </div>
</template>

<style scoped>
.titlebar {
  position: relative;
  display: flex;
  align-items: center;
  flex: 0 0 40px;
  height: 40px;
  padding: 0 14px;
  border-bottom: 1px solid var(--border);
  background: rgba(255, 253, 248, 0.62);
  user-select: none;
}

.drag {
  position: absolute;
  inset: 0;
  z-index: 0;
}

.lights {
  position: relative;
  z-index: 2;
  display: flex;
  align-items: center;
  gap: 8px;
}

.light {
  width: 12px;
  height: 12px;
  padding: 0;
  border: none;
  border-radius: 50%;
  box-shadow: inset 0 0 0 1px rgba(0, 0, 0, 0.08);
  transition: filter 0.15s ease, transform 0.15s ease;
}

.light:hover {
  filter: brightness(0.9);
  transform: scale(1.12);
}

.light:active {
  transform: scale(0.92);
}

.close {
  background: #ff5f57;
}

.minimize {
  background: #febc2e;
}

.maximize {
  background: #28c840;
}

.brand {
  position: absolute;
  left: 50%;
  z-index: 1;
  display: flex;
  align-items: center;
  gap: 7px;
  transform: translateX(-50%);
  color: var(--muted);
  font-size: 12px;
  font-weight: 700;
  letter-spacing: 0.06em;
  pointer-events: none;
}

.brand-mark {
  width: 9px;
  height: 9px;
  border-radius: 2px;
  background: var(--accent);
  transform: rotate(45deg);
}
</style>
