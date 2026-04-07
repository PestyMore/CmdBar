<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import AddEditCmdModal from './AddEditCmdModal.vue';

interface CmdItem { id: number; name: string; path: string; }

const cmds = ref<CmdItem[]>([]);
const showModal = ref(false);
const modalMode = ref<'add' | 'edit'>('add');
const editingCmd = ref<CmdItem | null>(null);
const tempName = ref('');
const tempPath = ref('');

const customDialog = ref({ show: false, title: '', message: '', type: 'alert' as 'alert' | 'confirm', resolve: null as ((value: boolean) => void) | null });
const showMessage = (msg: string, title = '提示') => new Promise<boolean>((resolve) => { customDialog.value = { show: true, title, message: msg, type: 'alert', resolve }; });
const showConfirm = (msg: string, title = '确认') => new Promise<boolean>((resolve) => { customDialog.value = { show: true, title, message: msg, type: 'confirm', resolve }; });
const handleDialogClose = (result: boolean) => { customDialog.value.show = false; if (customDialog.value.resolve) { customDialog.value.resolve(result); customDialog.value.resolve = null; } };

onMounted(() => { const saved = localStorage.getItem('cmdbar-commands'); if (saved) cmds.value = JSON.parse(saved); });
const saveToStorage = () => { localStorage.setItem('cmdbar-commands', JSON.stringify(cmds.value)); };

const runCmd = async (cmd: CmdItem) => {
  try { await invoke('run_cmd', { path: cmd.path }); } 
  catch (error) { await showMessage(`[${cmd.name}] 启动失败\n\n详细信息: ${error}`, '执行错误'); }
};

const openAddModal = () => { modalMode.value = 'add'; tempName.value = ''; tempPath.value = ''; showModal.value = true; };
const openEditModal = (cmd: CmdItem) => { modalMode.value = 'edit'; editingCmd.value = cmd; tempName.value = cmd.name; tempPath.value = cmd.path; showModal.value = true; };

const saveCmd = () => {
  if (!tempName.value.trim() || !tempPath.value.trim()) { showMessage('名称和路径不能为空。', '提示'); return; }
  if (modalMode.value === 'add') {
    const newId = cmds.value.length ? Math.max(...cmds.value.map(c => c.id)) + 1 : 1;
    cmds.value.push({ id: newId, name: tempName.value.trim(), path: tempPath.value.trim() });
  } else if (editingCmd.value) {
    const target = cmds.value.find(c => c.id === editingCmd.value!.id);
    if (target) { target.name = tempName.value.trim(); target.path = tempPath.value.trim(); }
  }
  saveToStorage(); showModal.value = false;
};

const deleteCmd = async (id: number) => {
  if (await showConfirm('确定要删除这条启动项吗？\n此操作不可撤销。', '确认删除')) {
    cmds.value = cmds.value.filter(cmd => cmd.id !== id);
    saveToStorage();
  }
};
</script>

<template>
  <div class="cmd-container">
    <div class="header">
      <h1 class="title">Commands</h1>
      <button class="add-btn" @click="openAddModal" title="添加新命令"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg></button>
    </div>
    <div class="list-area">
      <div v-if="!cmds.length" class="empty-state"><p>暂无命令，点击右上角 "+" 添加</p></div>
      <div class="list" v-else>
        <div v-for="cmd in cmds" :key="cmd.id" class="cmd-item" @click="runCmd(cmd)">
          <div class="info">
            <div class="name">{{ cmd.name }}</div>
            <div class="path">{{ cmd.path }}</div>
          </div>
          <div class="actions">
            <button class="icon-btn edit-btn" @click.stop="openEditModal(cmd)" title="编辑"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/><path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/></svg></button>
            <button class="icon-btn del-btn" @click.stop="deleteCmd(cmd.id)" title="删除"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="3 6 5 6 21 6"/><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/></svg></button>
          </div>
        </div>
      </div>
    </div>
    <AddEditCmdModal v-if="showModal" :mode="modalMode" v-model:name="tempName" v-model:path="tempPath" @save="saveCmd" @cancel="showModal = false" />
    <transition name="fade">
      <div v-if="customDialog.show" class="dialog-backdrop" @click.self="customDialog.type === 'alert' ? handleDialogClose(true) : handleDialogClose(false)">
        <div class="dialog-card">
          <h3 class="dialog-title">{{ customDialog.title }}</h3>
          <p class="dialog-message">{{ customDialog.message }}</p>
          <div class="dialog-actions" :class="{ 'single': customDialog.type === 'alert' }">
            <button v-if="customDialog.type === 'confirm'" class="dialog-btn cancel" @click="handleDialogClose(false)">取消</button>
            <button class="dialog-btn confirm" @click="handleDialogClose(true)">确定</button>
          </div>
        </div>
      </div>
    </transition>
  </div>
</template>

<style scoped>
.cmd-container { display: flex; flex-direction: column; flex: 1; padding: 0 24px; overflow: hidden; }
.header { display: flex; justify-content: space-between; align-items: center; padding: 10px 0 20px; flex-shrink: 0; }
.title { font-size: 28px; font-weight: 700; margin: 0; letter-spacing: -0.5px; }
.add-btn { width: 32px; height: 32px; border-radius: 50%; background: rgba(228, 60, 68, 0.15); border: 1px solid rgba(255, 255, 255, 0.2); color: white; display: flex; align-items: center; justify-content: center; cursor: pointer; transition: all 0.2s;  }
.add-btn:hover { background: rgba(255, 255, 255, 0.25); transform: scale(1.05); } .add-btn:active { transform: scale(0.95); } .add-btn svg { width: 18px; height: 18px; }
.list-area { flex: 1; overflow-y: auto; padding-bottom: 24px; } .list-area::-webkit-scrollbar { width: 0; }
.empty-state { text-align: center; margin-top: 50px; color: rgba(255, 255, 255, 0.5); font-size: 15px; }
.list { display: grid; grid-template-columns: repeat(2, 1fr); gap: 16px; }
.cmd-item { display: flex; align-items: center; background: rgba(228, 60, 68, 0.05); border: 1px solid rgba(255, 255, 255, 0.1); border-radius: 12px; padding: 14px 18px; transition: all 0.2s;  cursor: pointer; }
.cmd-item:hover { background: rgba(228, 60, 68, 0.15); transform: translateY(-2px); box-shadow: 0 4px 12px rgba(0,0,0,0.15); }
.info { flex: 1; overflow: hidden; }
.name { font-size: 16px; font-weight: 600; margin-bottom: 4px; color: #fff; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
.path { font-size: 12px; color: rgba(255, 255, 255, 0.6); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; font-family: monospace; }
.actions { display: flex; gap: 8px; margin-left: 12px; }
.icon-btn { width: 30px; height: 30px; border-radius: 8px; background: rgba(255, 255, 255, 0.1); border: none; color: rgba(255, 255, 255, 0.8); display: flex; align-items: center; justify-content: center; cursor: pointer; transition: all 0.2s; }
.icon-btn svg { width: 14px; height: 14px; }
.edit-btn:hover { background: #E43C44; color: white; } .del-btn:hover { background: #FF3B30; color: white; }
.dialog-backdrop { position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0, 0, 0, 0.7);  display: flex; align-items: center; justify-content: center; z-index: 1000; }
.dialog-card { width: 300px; background: #180305;  border: 1px solid rgba(255, 255, 255, 0.15); border-radius: 16px; box-shadow: 0 20px 40px rgba(0,0,0,0.3); text-align: center; overflow: hidden; }
.dialog-title { margin: 20px 20px 8px; font-size: 17px; font-weight: 600; color: white; }
.dialog-message { margin: 0 20px 20px; font-size: 14px; color: rgba(255, 255, 255, 0.7); line-height: 1.4; white-space: pre-wrap; word-break: break-all; }
.dialog-actions { display: flex; border-top: 1px solid rgba(255, 255, 255, 0.1); }
.dialog-actions.single .dialog-btn { width: 100%; }
.dialog-btn { flex: 1; padding: 14px 0; background: transparent; border: none; font-size: 16px; cursor: pointer; transition: background 0.2s; }
.dialog-btn.cancel { color: rgba(255, 255, 255, 0.9); border-right: 1px solid rgba(255, 255, 255, 0.1); font-weight: 400; }
.dialog-btn.confirm { color: #E43C44; font-weight: 600; }
.dialog-btn:hover { background: rgba(255, 255, 255, 0.05); } .dialog-btn:active { background: rgba(255, 255, 255, 0.1); }
.fade-enter-active, .fade-leave-active { transition: opacity 0.2s ease; } .fade-enter-from, .fade-leave-to { opacity: 0; }
</style>
