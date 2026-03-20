<script setup lang="ts">
import MacTitleBar from "./components/MacTitleBar.vue";
import CmdBar from "./components/CmdBar.vue";
</script>
<template>
  <div class="window-layout">
    <div class="wallpaper-bg"></div>
    <MacTitleBar />
    <CmdBar />
  </div>
</template>
<style>
:root { font-family: -apple-system, BlinkMacSystemFont, "SF Pro Text", "Segoe UI", Roboto, Helvetica, sans-serif; font-size: 14px; font-weight: 400; color: #ffffff; background-color: transparent !important; }
body, #app { margin: 0; padding: 0; width: 100vw; height: 100vh; overflow: hidden; background-color: transparent !important; }
.window-layout { position: relative; width: 100vw; height: 100vh; display: flex; flex-direction: column; border-radius: 14px; overflow: hidden; border: 1px solid rgba(255, 255, 255, 0.15); box-sizing: border-box; }
.wallpaper-bg { position: absolute; top: -20px; left: -20px; right: -20px; bottom: -20px; background-color: #1c1c1e; background-image: url('/wallpaper.png'); background-size: cover; background-position: center; filter: blur(25px) brightness(0.65) saturate(150%); z-index: -1; pointer-events: none; }
</style>
