<script setup lang="ts">
defineProps<{ mode: 'add' | 'edit'; name: string; path: string; }>();
const emit = defineEmits(['update:name', 'update:path', 'save', 'cancel']);
</script>

<template>
  <div class="modal-backdrop" @click.self="emit('cancel')">
    <div class="modal-card">
      <h3 class="modal-title">{{ mode === 'add' ? '添加启动项' : '编辑启动项' }}</h3>
      <div class="form-group">
        <label>显示名称</label>
        <input type="text" :value="name" @input="e => emit('update:name', (e.target as HTMLInputElement).value)" placeholder="如: 运行目录树" autofocus />
      </div>
      <div class="form-group">
        <label>脚本绝对路径 (.cmd, .bat)</label>
        <input type="text" :value="path" @input="e => emit('update:path', (e.target as HTMLInputElement).value)" placeholder="如: C:\scripts\run_tree.cmd" />
      </div>
      <div class="modal-actions">
        <button class="btn cancel" @click="emit('cancel')">取消</button>
        <button class="btn save" @click="emit('save')">保存</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.modal-backdrop { position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0, 0, 0, 0.7);  display: flex; align-items: center; justify-content: center; z-index: 999; }
.modal-card { width: 340px; background: #180305;  border: 1px solid rgba(255, 255, 255, 0.15); border-radius: 16px; padding: 24px; box-shadow: 0 20px 40px rgba(0,0,0,0.3); text-align: center; }
.modal-title { margin: 0 0 20px 0; font-size: 18px; font-weight: 600; }
.form-group { margin-bottom: 16px; text-align: left; }
.form-group label { display: block; font-size: 12px; color: rgba(255, 255, 255, 0.7); margin-bottom: 8px; padding-left: 2px; }
.form-group input { width: 100%; box-sizing: border-box; background: rgba(0, 0, 0, 0.2); border: 1px solid rgba(255, 255, 255, 0.1); color: white; padding: 10px 12px; border-radius: 8px; font-size: 14px; outline: none; }
.form-group input:focus { border-color: #E43C44; }
.modal-actions { display: flex; gap: 12px; margin-top: 24px; }
.btn { flex: 1; padding: 12px; border-radius: 8px; border: none; font-size: 14px; font-weight: 600; cursor: pointer; }
.btn.cancel { background: rgba(255, 255, 255, 0.1); color: white; }
.btn.cancel:hover { background: rgba(255, 255, 255, 0.2); }
.btn.save { background: #E43C44; color: white; }
.btn.save:hover { background: #BA252C; }
</style>
