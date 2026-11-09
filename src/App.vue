<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import {
  Check,
  CircleAlert,
  FolderOpen,
  Pencil,
  Play,
  Plus,
  Search,
  ShieldCheck,
  Terminal,
  Trash2,
  X,
} from "@lucide/vue";
import MacTitleBar from "./components/MacTitleBar.vue";
import CommandEditor from "./components/CommandEditor.vue";
import type { CommandItem, StorePayload } from "./types";

const commands = ref<CommandItem[]>([]);
const dataDir = ref("");
const keyword = ref("");
const loading = ref(true);
const searchInput = ref<HTMLInputElement | null>(null);

const editorOpen = ref(false);
const editorIsNew = ref(false);
const editorItem = ref<CommandItem>(blank());

const deleteOpen = ref(false);
const deleteItem = ref<CommandItem>(blank());

const toast = ref({ show: false, text: "", kind: "ok" as "ok" | "err" });
let toastTimer: number | undefined;

const filtered = computed(() => {
  const q = keyword.value.trim().toLowerCase();
  if (!q) return commands.value;
  return commands.value.filter(
    (item) =>
      item.name.toLowerCase().includes(q) ||
      item.command.toLowerCase().includes(q) ||
      item.workdir.toLowerCase().includes(q),
  );
});

function blank(): CommandItem {
  return {
    id: Date.now().toString(36) + Math.random().toString(36).slice(2, 8),
    name: "",
    command: "",
    workdir: "",
    runAsAdmin: false,
    createdAt: Date.now(),
    lastRunAt: null,
  };
}

function notify(text: string, kind: "ok" | "err" = "ok") {
  toast.value = { show: true, text, kind };
  if (toastTimer) window.clearTimeout(toastTimer);
  toastTimer = window.setTimeout(() => {
    toast.value.show = false;
  }, 3200);
}

function formatLastRun(value: number | null): string {
  if (!value) return "尚未运行";
  const diff = Date.now() - value;
  if (diff < 60_000) return "刚刚运行";
  if (diff < 3_600_000) return `${Math.floor(diff / 60_000)} 分钟前运行`;
  if (diff < 86_400_000) return `${Math.floor(diff / 3_600_000)} 小时前运行`;
  return `${new Date(value).toLocaleDateString("zh-CN")} 运行`;
}

function onKeydown(event: KeyboardEvent) {
  if (event.key === "Escape") {
    if (editorOpen.value) editorOpen.value = false;
    else if (deleteOpen.value) deleteOpen.value = false;
    else if (keyword.value) keyword.value = "";
    return;
  }
  if (event.key === "/" && !editorOpen.value && !deleteOpen.value) {
    const target = event.target as HTMLElement | null;
    if (target && (target.tagName === "INPUT" || target.tagName === "TEXTAREA")) return;
    event.preventDefault();
    searchInput.value?.focus();
  }
}

onMounted(async () => {
  const legacy = localStorage.getItem("cmdbar-commands");
  try {
    const payload = await invoke<StorePayload>("load_store", { legacy });
    commands.value = payload.commands;
    dataDir.value = payload.dataDir;
    if (legacy) localStorage.removeItem("cmdbar-commands");
    if (payload.warning) notify(payload.warning, "err");
  } catch (error) {
    notify(`读取命令失败：${error}`, "err");
  } finally {
    loading.value = false;
  }
  window.addEventListener("keydown", onKeydown);
});

onUnmounted(() => {
  window.removeEventListener("keydown", onKeydown);
  if (toastTimer) window.clearTimeout(toastTimer);
});

function openAdd() {
  editorItem.value = blank();
  editorIsNew.value = true;
  editorOpen.value = true;
}

function openEdit(item: CommandItem) {
  editorItem.value = { ...item };
  editorIsNew.value = false;
  editorOpen.value = true;
}

async function saveEditor(item: CommandItem) {
  const next = editorIsNew.value
    ? [...commands.value, item]
    : commands.value.map((existing) => (existing.id === item.id ? item : existing));
  try {
    await invoke("save_commands", { commands: next });
    commands.value = next;
    editorOpen.value = false;
    notify(editorIsNew.value ? "已添加命令" : "已保存修改");
  } catch (error) {
    notify(`保存失败：${error}`, "err");
  }
}

function askDelete(item: CommandItem) {
  deleteItem.value = item;
  deleteOpen.value = true;
}

async function removeCommand() {
  const next = commands.value.filter((item) => item.id !== deleteItem.value.id);
  try {
    await invoke("save_commands", { commands: next });
    commands.value = next;
    deleteOpen.value = false;
    notify("已删除命令");
  } catch (error) {
    notify(`删除失败：${error}`, "err");
  }
}

async function run(item: CommandItem) {
  try {
    await invoke("run_command", { id: item.id });
    const target = commands.value.find((existing) => existing.id === item.id);
    if (target) target.lastRunAt = Date.now();
    notify(
      item.runAsAdmin
        ? `已请求以管理员身份运行「${item.name}」`
        : `已启动「${item.name}」`,
    );
  } catch (error) {
    notify(String(error), "err");
  }
}

async function openDataDir() {
  try {
    await invoke("open_data_dir");
  } catch (error) {
    notify(String(error), "err");
  }
}
</script>

<template>
  <div class="app-shell">
    <div class="window">
      <div class="accent-line" aria-hidden="true" />
      <MacTitleBar />

      <main class="content">
        <header class="page-head">
          <div class="head-copy">
            <div class="head-line">
              <h1>命令</h1>
              <span class="count">{{ commands.length }} 条</span>
            </div>
            <p>点击卡片即可运行，小脚本随手可达</p>
          </div>
          <div class="head-actions">
            <label class="search">
              <Search :size="15" />
              <input
                ref="searchInput"
                v-model="keyword"
                type="search"
                placeholder="搜索名称或命令"
                aria-label="搜索命令"
                spellcheck="false"
              />
              <button v-if="keyword" class="clear" title="清空搜索" aria-label="清空搜索" @click="keyword = ''">
                <X :size="13" />
              </button>
            </label>
            <button class="btn btn-primary" @click="openAdd"><Plus :size="16" />新建命令</button>
          </div>
        </header>

        <section class="list-area">
          <div v-if="loading" class="empty">正在读取命令…</div>

          <div v-else-if="!commands.length" class="empty">
            <div class="empty-icon"><Terminal :size="26" /></div>
            <strong>还没有命令</strong>
            <span>把常用脚本加进来，之后点一下就能跑</span>
            <button class="btn btn-primary" @click="openAdd"><Plus :size="15" />新建第一个命令</button>
          </div>

          <div v-else-if="!filtered.length" class="empty">
            <div class="empty-icon"><Search :size="24" /></div>
            <strong>没有匹配的命令</strong>
            <span>换个关键词试试</span>
          </div>

          <div v-else class="grid">
            <article
              v-for="item in filtered"
              :key="item.id"
              class="cmd-card"
              tabindex="0"
              :aria-label="'运行 ' + item.name"
              @click="run(item)"
              @keydown.enter.prevent="run(item)"
              @keydown.space.prevent="run(item)"
            >
              <div class="card-top">
                <div class="cmd-icon"><Terminal :size="17" /></div>
                <div class="card-title">
                  <div class="name" :title="item.name">{{ item.name }}</div>
                  <div class="meta">
                    <span v-if="item.runAsAdmin" class="badge"><ShieldCheck :size="11" />管理员</span>
                    <span class="last">{{ formatLastRun(item.lastRunAt) }}</span>
                  </div>
                </div>
              </div>

              <div class="cmd-text" :title="item.command">{{ item.command }}</div>
              <div v-if="item.workdir" class="workdir" :title="item.workdir">工作目录 · {{ item.workdir }}</div>

              <div class="card-actions">
                <button class="icon-btn" title="运行" aria-label="运行" @click.stop="run(item)"><Play :size="14" /></button>
                <button class="icon-btn" title="编辑" aria-label="编辑" @click.stop="openEdit(item)"><Pencil :size="14" /></button>
                <button class="icon-btn danger" title="删除" aria-label="删除" @click.stop="askDelete(item)"><Trash2 :size="14" /></button>
              </div>
            </article>
          </div>
        </section>

        <footer class="footer">
          <div class="dir">
            <FolderOpen :size="14" />
            <span class="dir-label">数据目录</span>
            <span class="dir-path" :title="dataDir">%APPDATA%\com.cmdbar</span>
            <button class="btn btn-ghost open-btn" @click="openDataDir">打开</button>
          </div>
          <span class="hint">/ 搜索 · Esc 关闭 · 支持管理员提权</span>
        </footer>
      </main>

      <CommandEditor
        v-if="editorOpen"
        :initial="editorItem"
        :is-new="editorIsNew"
        @save="saveEditor"
        @cancel="editorOpen = false"
      />

      <Transition name="fade">
        <div v-if="deleteOpen" class="overlay" @click.self="deleteOpen = false">
          <div class="panel confirm" role="alertdialog" aria-label="确认删除命令">
            <div class="confirm-icon"><CircleAlert :size="20" /></div>
            <h3>删除这条命令？</h3>
            <p>「{{ deleteItem.name }}」将从列表和数据文件中移除，此操作不可撤销。</p>
            <div class="confirm-actions">
              <button class="btn" @click="deleteOpen = false">取消</button>
              <button class="btn btn-danger" @click="removeCommand"><Trash2 :size="15" />删除</button>
            </div>
          </div>
        </div>
      </Transition>

      <Transition name="fade">
        <div v-if="toast.show" class="toast" :class="toast.kind" role="status">
          <Check v-if="toast.kind === 'ok'" :size="15" />
          <CircleAlert v-else :size="15" />
          <span>{{ toast.text }}</span>
        </div>
      </Transition>
    </div>
  </div>
</template>

<style scoped>
.content {
  position: relative;
  z-index: 1;
  display: flex;
  flex: 1;
  flex-direction: column;
  min-height: 0;
  padding: 18px 24px 0;
}

.page-head {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 16px;
  flex: 0 0 auto;
  padding-bottom: 16px;
}

.head-line {
  display: flex;
  align-items: center;
  gap: 9px;
}

.head-line h1 {
  margin: 0;
  font-size: 21px;
  font-weight: 700;
  letter-spacing: 0.01em;
}

.count {
  padding: 2px 8px;
  border-radius: 20px;
  background: var(--accent-soft);
  color: var(--accent-strong);
  font-size: 11px;
  font-weight: 700;
}

.head-copy p {
  margin: 5px 0 0;
  color: var(--muted);
  font-size: 12px;
}

.head-actions {
  display: flex;
  align-items: center;
  gap: 10px;
}

.search {
  display: flex;
  align-items: center;
  gap: 7px;
  width: 188px;
  height: 34px;
  padding: 0 9px;
  border: 1px solid var(--border-strong);
  border-radius: 9px;
  background: var(--surface);
  color: var(--muted);
  transition: border-color 0.15s ease, box-shadow 0.15s ease;
}

.search:focus-within {
  border-color: var(--accent);
  box-shadow: 0 0 0 3px rgba(224, 163, 46, 0.14);
}

.search input {
  flex: 1;
  min-width: 0;
  border: none;
  outline: none;
  background: transparent;
  font-size: 12.5px;
}

.search input::placeholder {
  color: #b4ab97;
}

.search input::-webkit-search-cancel-button {
  display: none;
}

.clear {
  display: flex;
  align-items: center;
  padding: 2px;
  border: none;
  background: transparent;
  color: var(--muted);
}

.clear:hover {
  color: var(--danger);
}

.list-area {
  flex: 1;
  min-height: 0;
  overflow-y: auto;
  padding: 0 4px 14px 0;
}

.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(255px, 1fr));
  align-content: start;
  gap: 12px;
}

.empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 8px;
  height: 100%;
  min-height: 200px;
  color: var(--muted);
  font-size: 12.5px;
}

.empty strong {
  color: var(--ink-soft);
  font-size: 14px;
}

.empty .btn {
  margin-top: 8px;
}

.empty-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 52px;
  height: 52px;
  margin-bottom: 4px;
  border: 1px solid var(--border);
  border-radius: 14px;
  background: var(--accent-soft);
  color: var(--accent-strong);
}

.cmd-card {
  position: relative;
  display: flex;
  flex-direction: column;
  gap: 9px;
  min-height: 116px;
  padding: 13px 14px;
  border: 1px solid var(--border);
  border-radius: 12px;
  background: var(--surface);
  cursor: pointer;
  /* 不使用 transform：透明窗口下位移会创建合成层，容易盖住顶部标识线 */
  transition: background 0.15s ease, border-color 0.15s ease, box-shadow 0.15s ease;
}

.cmd-card:hover,
.cmd-card:focus-visible {
  border-color: var(--accent);
  background: var(--surface-hover);
  box-shadow: 0 4px 14px rgba(76, 62, 32, 0.12);
}

.card-top {
  display: flex;
  align-items: center;
  gap: 10px;
  padding-right: 88px;
}

.cmd-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  flex: 0 0 32px;
  width: 32px;
  height: 32px;
  border-radius: 9px;
  background: var(--accent-soft);
  color: var(--accent-strong);
}

.card-title {
  min-width: 0;
}

.name {
  overflow: hidden;
  color: var(--ink);
  font-size: 14px;
  font-weight: 650;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.meta {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-top: 3px;
  font-size: 11px;
}

.badge {
  display: inline-flex;
  align-items: center;
  gap: 3px;
  padding: 1px 6px;
  border-radius: 5px;
  background: #f6ead1;
  color: #a9761a;
  font-weight: 600;
}

.last {
  color: var(--muted);
}

.cmd-text,
.workdir {
  overflow: hidden;
  color: var(--ink-soft);
  font-family: var(--mono);
  font-size: 11.5px;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.cmd-text {
  padding: 7px 9px;
  border-radius: 7px;
  background: var(--surface-soft);
}

.workdir {
  color: var(--muted);
  font-size: 10.5px;
}

.card-actions {
  position: absolute;
  top: 9px;
  right: 9px;
  display: flex;
  gap: 1px;
  opacity: 0.45;
  transition: opacity 0.15s ease;
}

.cmd-card:hover .card-actions,
.cmd-card:focus-within .card-actions {
  opacity: 1;
}

.footer {
  position: relative;
  z-index: 1;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  flex: 0 0 40px;
  height: 40px;
  border-top: 1px solid var(--border);
  color: var(--muted);
  font-size: 11px;
}

.dir {
  display: flex;
  align-items: center;
  gap: 6px;
  min-width: 0;
}

.dir-label {
  flex: 0 0 auto;
}

.dir-path {
  font-family: var(--mono);
  white-space: nowrap;
}

.open-btn {
  height: 24px;
  padding: 0 8px;
  font-size: 11px;
}

.hint {
  flex: 0 0 auto;
}

.confirm {
  max-width: 360px;
  text-align: center;
}

.confirm-icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 42px;
  height: 42px;
  border-radius: 12px;
  background: var(--danger-soft);
  color: var(--danger);
}

.confirm h3 {
  margin: 12px 0 6px;
  font-size: 16px;
}

.confirm p {
  margin: 0;
  color: var(--ink-soft);
  font-size: 12.5px;
  line-height: 1.6;
}

.confirm-actions {
  display: flex;
  justify-content: center;
  gap: 10px;
  margin-top: 20px;
}

.toast {
  position: absolute;
  bottom: 52px;
  left: 50%;
  z-index: 60;
  display: flex;
  align-items: center;
  gap: 8px;
  max-width: 80%;
  padding: 9px 14px;
  border-radius: 10px;
  background: var(--ink);
  color: #fffdf8;
  font-size: 12px;
  box-shadow: 0 8px 22px rgba(46, 42, 34, 0.28);
  transform: translateX(-50%);
}

.toast.err {
  background: var(--danger);
}
</style>
