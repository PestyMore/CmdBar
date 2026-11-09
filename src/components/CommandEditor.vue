<script setup lang="ts">
import { onMounted, reactive, ref } from "vue";
import { Save, ShieldCheck, X } from "@lucide/vue";
import type { CommandItem } from "../types";

const props = defineProps<{ initial: CommandItem; isNew: boolean }>();
const emit = defineEmits<{
  save: [item: CommandItem];
  cancel: [];
}>();

const form = reactive<CommandItem>({ ...props.initial });
const error = ref("");
const nameInput = ref<HTMLInputElement | null>(null);

onMounted(() => nameInput.value?.focus());

function submit() {
  const name = form.name.trim();
  const command = form.command.trim();
  if (!name || !command) {
    error.value = "名称和命令都不能为空";
    return;
  }
  emit("save", { ...form, name, command, workdir: form.workdir.trim() });
}
</script>

<template>
  <div class="overlay" @click.self="emit('cancel')">
    <form class="panel editor" @submit.prevent="submit">
      <div class="panel-head">
        <div>
          <h3>{{ isNew ? "新建命令" : "编辑命令" }}</h3>
          <p>把常用的脚本或程序加进来，随时一键运行</p>
        </div>
        <button class="icon-btn" type="button" title="关闭" aria-label="关闭" @click="emit('cancel')">
          <X :size="16" />
        </button>
      </div>

      <label class="field">
        <span class="field-label">显示名称</span>
        <input
          ref="nameInput"
          v-model="form.name"
          class="field-input"
          type="text"
          placeholder="如：清理临时文件"
        />
      </label>

      <label class="field">
        <span class="field-label">命令行</span>
        <input
          v-model="form.command"
          class="field-input mono"
          type="text"
          placeholder="如：C:\scripts\clean.cmd --all"
        />
        <span class="field-hint">通过 cmd.exe 执行，支持 .bat / .cmd、管道、&amp;&amp; 与环境变量；含空格路径请加引号</span>
      </label>

      <label class="field">
        <span class="field-label">工作目录 <em>可选</em></span>
        <input
          v-model="form.workdir"
          class="field-input mono"
          type="text"
          placeholder="留空则继承当前目录"
        />
      </label>

      <label class="admin-row">
        <ShieldCheck :size="17" />
        <span class="admin-copy">
          <strong>以管理员身份运行</strong>
          <small>运行时弹出 UAC 授权，CmdBar 本身保持普通权限</small>
        </span>
        <input v-model="form.runAsAdmin" class="checkbox" type="checkbox" />
      </label>

      <p v-if="error" class="form-error">{{ error }}</p>

      <div class="panel-actions">
        <button class="btn" type="button" @click="emit('cancel')">取消</button>
        <button class="btn btn-primary" type="submit"><Save :size="15" />保存</button>
      </div>
    </form>
  </div>
</template>

<style scoped>
.editor {
  padding: 20px 22px 18px;
}

.panel-head {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 12px;
  margin-bottom: 18px;
}

.panel-head h3 {
  margin: 0 0 4px;
  font-size: 17px;
  font-weight: 700;
}

.panel-head p {
  margin: 0;
  color: var(--muted);
  font-size: 12px;
}

.field-hint {
  display: block;
  margin-top: 6px;
  color: var(--muted);
  font-size: 11px;
  line-height: 1.5;
}

.admin-row {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 11px 12px;
  border: 1px solid var(--border);
  border-radius: 10px;
  background: var(--accent-soft);
  color: var(--accent-strong);
  cursor: pointer;
}

.admin-copy {
  display: flex;
  flex: 1;
  flex-direction: column;
  gap: 2px;
  min-width: 0;
}

.admin-copy strong {
  color: var(--ink);
  font-size: 12.5px;
}

.admin-copy small {
  color: var(--ink-soft);
  font-size: 11px;
  line-height: 1.4;
}

.checkbox {
  width: 16px;
  height: 16px;
  accent-color: var(--accent);
  cursor: pointer;
}

.form-error {
  margin: 10px 0 0;
  color: var(--danger);
  font-size: 12px;
}

.panel-actions {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  margin-top: 18px;
}
</style>
