# CmdBar v2 重构设计文档

> 撰写时间：2026-11-08 · 状态：设计定稿，进入实施
> 本文是动工前的总览与决策记录，实施范围以本文为准。

---

## 1. 现状总览（v1.1.0 事实基线）

| 项 | 现状 |
| --- | --- |
| 技术栈 | Tauri 2 + Vue 3 + TypeScript + Vite 6；Rust 侧仅有 1 个 `run_cmd` command |
| 窗口 | `decorations: false` + `transparent: true` + `shadow: false`，圆角靠 CSS `border-radius: 14px`，红绿灯是 3 个手写 HTML 圆形按钮 |
| 主题 | 暗红/黑渐变（Alastor），强调色 `#E43C44`，无 blur（v1.1 已移除） |
| 数据 | **唯一持久化是 `localStorage['cmdbar-commands']`**，物理位置由 WebView2 用户数据目录托管，与 bundle identifier `com.PestyMore.cmdbar` 绑定；无备份/迁移/版本字段，`JSON.parse` 无异常保护 |
| 命令模型 | `{ id: number, name: string, path: string }`，只存一个脚本路径 |
| 执行 | `cmd /c start "" <path>`，只 `spawn()` 不校验、不捕获、不支持提权，错误直接回显原始字符串 |
| 自启 | `tauri-plugin-autostart` + 前端 `onMounted` 每次强制 `enable()`；Rust `setup` 每次 `show()` 后立刻 `minimize()`；无托盘、无单实例，最小化后没有恢复入口 |
| 图标 | 手写内联 SVG + Vite 模板 favicon；应用图标为高光渐变的拟物图（`app-icon.png`），1.37 MB 且未被引用 |
| 工程 | 版本号 1.0.0 / 1.1.0 三处不一致；无 `Cargo.lock`、无 CI、无 CSP；`csp: null`；`.cargo/config.toml` 硬编码 `127.0.0.1:1080` 代理（会破坏别人和 CI 的构建）；`bundle.targets: "all"` |
| 无用物 | `src-tauri/2`（npm 输出误重定向）、`public/wallpaper.png`(4.55 MB)、`public/tauri.svg`、`public/vite.svg`、`src/assets/vue.svg`、`app-icon.png`、`repomix.config.json`、移动端图标 `android/`+`ios/`、重复的 `@2x-1` 图标 |

### 明确存在的问题

1. 数据看不见、摸不着、备份不了：藏在 WebView2 的 localStorage，清缓存 / 换 identifier 即丢失。
2. 每次启动都强制注册自启且立刻最小化，用户无法关闭，也没有托盘可恢复 —— 这不是"正常软件"的行为。
3. 命令无法以管理员身份运行；`cmd /c start` 对含空格、`&`、`^` 的路径与引号处理脆弱。
4. 暗红主题 + 深色弹窗 + 手写 SVG，与"轻量、好看"的目标不符；存在大量模板残留。
5. 依赖未使用（`tauri-plugin-opener`、`serde`、`serde_json` 在 Rust 侧 0 引用；前端 plugin-opener 0 引用）。
6. 无 CSP、无锁文件、无 CI、无类型检查门禁；自定义对话框用 `ref` 存 `Promise.resolve`，存在悬挂风险。

---

## 2. 目标与边界

### 硬性保留（形式）

- **窗口四周圆角**：透明窗口 + CSS `border-radius`，四角均裁切。
- **左上角 macOS 红绿灯**：三枚圆形按钮，红 / 黄 / 绿，关闭 / 最小化 / 最大化，位于标题栏最左。

### 功能目标（基本不变）

- 自定义命令 → 添加 / 编辑 / 删除 / 一键运行；小脚本优先。
- 移除开机自启动、首次启动静默启动及配套逻辑，改回普通桌面程序：启动即显示窗口，关闭即退出。
- 数据落到 `%APPDATA%\com.cmdbar\commands.json`（Roaming），可读、可备份、可手工编辑。
- 支持**按需管理员提权**（UAC）。
- 轻量：无背景图、无毛玻璃、无运行时图片资源、无重型 UI 库、依赖能删就删。
- 全新暖黄米白（莱茵生命配色取向）UI、Lucide 图标、全新扁平几何应用图标。
- 补齐 `Cargo.lock`、CSP、CI、统一版本号与可读文档。

### 非目标（本次不做）

系统托盘、全局热键、单实例、自动更新、运行日志/输出回传、深色模式、命令分组、云同步、多语言、单元测试框架。理由见 §8。

---

## 3. 关键决策与备选方案

### 3.1 数据存储位置与形式

**选定**：Windows `%APPDATA%\com.cmdbar\commands.json`（即 Roaming）。非 Windows 回落 `$XDG_CONFIG_HOME/com.cmdbar` 或 `~/.config/com.cmdbar`，保证代码可跨平台编译。用 `serde_json` 美化输出（2 空格缩进）+ 临时文件原子替换写入。

| 备选 | 取舍 |
| --- | --- |
| `tauri-plugin-store` / `app_config_dir()` | `app_config_dir()` 得到的是 `%APPDATA%\com.PestyMore.cmdbar`，与指定目录名不符；plugin-store 多一层依赖与权限。**未采用** |
| 把 identifier 改成 `com.cmdbar` 再用 `app_config_dir()` | 目录名对了，但会换掉 WebView2 配置目录，导致旧 localStorage 读不到、无法迁移。**未采用** → 保留 `com.PestyMore.cmdbar` |
| SQLite / `.db` | 单用户几十条记录，杀鸡用牛刀，且不可手工编辑。**未采用** |
| 继续 localStorage | 无法满足"看得见的 Roaming 目录"。**未采用** |

**迁移**：保留 identifier 使旧 WebView2 配置目录不变；前端首启把 `localStorage['cmdbar-commands']` 原样传给 Rust，Rust 在 `commands.json` 不存在时把旧的 `{id,name,path}` 转成新结构并落盘；成功后前端删除旧键。旧数据不会丢。

**格式**（`version` 便于后续迁移；时间戳用 Unix 毫秒，避免引入 `chrono`/`time`）：

```json
{
  "version": 1,
  "commands": [
    {
      "id": "m5k2p9a4x",
      "name": "清理临时文件",
      "command": "C:\\scripts\\clean.cmd --all",
      "workdir": "C:\\scripts",
      "runAsAdmin": true,
      "createdAt": 1762588800000,
      "lastRunAt": 1762589900000
    }
  ]
}
```

### 3.2 命令模型与执行

由单一 `path` 升级为**完整命令行** `command` + 可选 `workdir` + `runAsAdmin`（旧 `path` 迁移为 `command`）。这样 `.cmd/.bat`、`python x.py`、管道、`&&`、环境变量都能用，且不再依赖 `start` 的路径猜测。

执行统一走 `%COMSPEC%`（`cmd.exe`）：

```
cmd.exe /D /S /C "<用户命令行>"
```

`/D` 跳过 AutoRun，`/S` 让 cmd 只剥掉最外层引号、内部引号原样保留 —— 解决含空格路径与内嵌引号的老问题。工作目录用 `lpDirectory` 传入并在执行前校验存在。

**启动方式**：Windows 侧直接 FFI 调 `shell32!ShellExecuteW`，`open` = 普通运行，`runas` = 触发 UAC 提权，两者共用一条代码路径。不加 `windows-sys`/`windows` 依赖（零额外编译负担）。

| 备选 | 取舍 |
| --- | --- |
| `cmd /c start "" <path>`（现状） | 引号/特殊字符易碎、不能提权。**弃用** |
| `powershell Start-Process -Verb RunAs` | 多起一个 PowerShell（数百 ms）、转义地狱、闪控制台。**未采用** |
| `runas.exe /user:Administrator` | 要求手输管理员密码而非 UAC。**未采用** |
| 给整个 app 加 `requireAdministrator` manifest | 每次开软件都弹 UAC，且所有子进程默认提权，违背"普通软件"。**未采用** |
| 直接 `CreateProcess` 解析命令行 | 需 `CommandLineToArgvW`，且不支持 cmd 内建命令/管道。**未采用** |

**提权语义**：仅被勾选的命令提权，CmdBar 自身始终以普通权限运行；用户取消 UAC（`ERROR_CANCELLED = 1223`）时返回中文提示而不是原始错误串。控制台窗口保留（脚本要看输出、可 `pause`）；需要启动 GUI 程序且不驻留控制台时，命令行写 `start "" app.exe`。

**IPC 面收敛**：新增 `run_command(id)` 由 Rust 按 id 从存储里查命令再执行，前端不传命令行字符串；成功后 Rust 自己更新 `lastRunAt` 并落盘，避免前后端双写不一致。

### 3.3 窗口与主题

- 保留 `decorations:false / transparent:true`；Rust `setup` 仅保留 `set_background_color(透明)`，**删掉 `show(); minimize()`** 与 autostart 插件。
- `#app` 留 12px 透明内边距，`.window` 上用 CSS `box-shadow` 画柔和投影，圆角 16px + `overflow:hidden` 裁四角 —— 比开 `shadow:true` 的系统方形阴影更贴合圆角，且不增加 GPU 负担。
- 新增 `minWidth/minHeight/center`，`dragDropEnabled:false`。
- **配色（暖黄米白 / 莱茵生命取向）**：背景 `#FBF7EC`，卡片 `#FFFDF8`，描边 `#E9DFC6`，墨色文字 `#3B3428`，次要文字 `#8C8371`，琥珀主色 `#E0A32E`（hover `#C98C1E`），浅琥珀底 `#FBF0D6`，危险色 `#C15B41`，红绿灯沿用 `#ff5f57 / #febc2e / #28c840`。全部走 CSS 变量，无渐变、无 blur、无图片。
- 图标统一 **Lucide**（`lucide-vue-next`，tree-shaking 后只打包用到的若干个），替换手写 SVG。
- 交互：卡片整块可点击运行，编辑/删除按钮 `@click.stop`；搜索框即时过滤；Esc 关闭弹窗或清空搜索（**不再最小化**）；错误/结果用轻量 toast 替代原来的 Promise 模拟对话框；删除确认改为受控状态机，消除悬挂 Promise。
- 底部展示数据目录并提供"打开"按钮，直接回答"数据存哪了"。

### 3.4 应用图标

**选定**：扁平几何、无高光/无渐变/无文字的 `>_` 提示符：墨色圆角方底 + 琥珀色折角与下划线，2~3 个形状，16px 下仍可辨识。用 Pillow 生成 1024×1024 源图（临时文件，完成后删除），再由 `tauri icon` 生成全尺寸集，最后删除移动端与重复图标，只保留桌面所需。

| 备选 | 取舍 |
| --- | --- |
| 沿用 `app-icon.png`（玻璃拟物 + 文字） | 用户明确表示"太像 AI"。**弃用** |
| 引入图标字体 / 位图贴图 | 增加体积、不轻量。**未采用** |

### 3.5 工程化

- 版本统一 `2.0.0`（`package.json` / `Cargo.toml` / `tauri.conf.json`）。
- 删除 `tauri-plugin-autostart`、前后端 `plugin-opener`；Rust 保留并用好 `serde`/`serde_json`。
- capabilities 移除 `autostart:default`；补显式 CSP（生产严格、`devCsp` 放行 Vite HMR）。
- 删除仓库内硬编码代理 `.cargo/config.toml`（原 1080 已失效），`.cargo/` 加入 `.gitignore`；本地需要时通过 `HTTPS_PROXY=http://127.0.0.1:7890` 环境变量注入，CI 用 GitHub 托管 runner 直连，不写死任何代理。
- 生成并提交 `Cargo.lock`；`package-lock.json` 随依赖变更重新生成。
- CI：`.github/workflows/build.yml`，`windows-latest`，push `main` / PR / tag / 手动触发；`npm ci` → `npx tauri build --bundles nsis` → 上传 NSIS 安装包 artifact；tag 时附加 Release。

---

## 4. 文件变更清单

| 文件 | 动作 |
| --- | --- |
| `docs/REFACTOR-v2.md`、`README.md` | 新增 / 重写 |
| `.github/workflows/build.yml` | 新增 |
| `src-tauri/src/lib.rs` | 重写：builder、command 注册、透明背景 |
| `src-tauri/src/store.rs` | 新增：`com.cmdbar` 读写、迁移、原子写 |
| `src-tauri/src/runner.rs` | 新增：`ShellExecuteW` 执行 / UAC 提权、非 Win 回落 |
| `src-tauri/Cargo.toml`、`tauri.conf.json`、`capabilities/default.json` | 重写配置 |
| `src-tauri/icons/*` | 重新生成 + 删除移动端/重复项 |
| `src/App.vue` | 重写：窗口壳 + 状态 + 列表 + 弹窗 + toast |
| `src/components/MacTitleBar.vue` | 重写样式，保留红绿灯结构 |
| `src/components/CommandEditor.vue` | 新增（替代 `AddEditCmdModal.vue`） |
| `src/types.ts`、`src/styles.css`、`src/main.ts` | 新增 / 更新 |
| `index.html`、`vite.config.ts`、`package.json`、`.gitignore` | 更新 |
| `src/components/CmdBar.vue`、`src/components/AddEditCmdModal.vue` | 删除（职责并入 App.vue / CommandEditor.vue） |
| `src-tauri/2`、`app-icon.png`、`public/*`、`src/assets/vue.svg`、`repomix.config.json`、`.cargo/config.toml`、`icons/android/`、`icons/ios/` | 删除 |

---

## 5. 验证清单（成功判据）

1. `npm ci` 成功；`npm run build`（`vue-tsc --noEmit` + `vite build`）零错误。
2. `cargo check --manifest-path src-tauri/Cargo.toml` 零错误。
3. `npx tauri build --bundles nsis` 产出 `.exe`，启动后**窗口正常显示**、不最小化、不注册启动项。
4. 窗口四角圆角、左上角红黄绿三按钮可关闭/最小化/最大化。
5. 新增命令后 `%APPDATA%\com.cmdbar\commands.json` 出现且为可读 JSON；重启应用数据仍在；删除/编辑同步落盘。
6. 勾选"管理员运行"的命令点击后弹出 UAC；取消时显示中文提示；确认后脚本以管理员权限执行。
7. 命令行含空格路径（`python "C:\Program Files\x\a.py"`）能正确执行。
8. 旧版 localStorage 数据在首次启动后自动出现在新界面，旧键被清除。
9. `git push` 后 GitHub Actions 跑绿，artifact 可下载。
10. 仓库中不再有 autostart 字样、不再有失效代理配置、不再有未引用的大图片。

---

## 6. 风险与对策

| 风险 | 对策 |
| --- | --- |
| `transparent` 窗口下 CSS 投影在 WebView2 合成异常 | 投影只是装饰；`.window` 本体不透明，最坏情况退化为无阴影，功能不受影响 |
| `ShellExecuteW` 返回值语义（>32 成功）与 `GetLastError` 时序 | 直接在调用后读取，映射 `ERROR_CANCELLED` / `SE_ERR_ACCESSDENIED` 为中文提示 |
| `devCsp` 字段若不被当前 Tauri 版本接受 | 由 `cargo check` / `tauri build` 在验证阶段暴露，届时退回单 `csp` 并放宽 HMR |
| 旧数据迁移依赖 WebView2 配置目录不变 | 已决定保留 `com.PestyMore.cmdbar`；即使迁移失败也只是空列表，不阻塞启动（`unwrap_or_default`） |
| CI 首次 Rust 编译耗时 | 加 `Swatinem/rust-cache`，`--bundles nsis` 跳过 MSI/WiX |

---

## 7. 代理与网络

- 本机代理：`http://127.0.0.1:7890`。仅用于本地 `npm` / `cargo` / `git` 的环境变量，**不写入仓库**。
- 全局 `%USERPROFILE%\.cargo\config.toml` 已指向 7890 且开启 `git-fetch-with-cli`；仓库内不再放置 `.cargo/config.toml`，避免污染 CI 与他人。
- CI 运行在 GitHub 托管 `windows-latest`，直连 npm / crates.io，无需代理。

---

## 8. 后续可选优化方向（本次不实现，供决策）

1. **真正的启动器形态**：单实例 + 托盘 + 全局热键（如 `Alt+Space`）唤起/隐藏，配合 `Esc` 收起。体验最好，但会重新引入"常驻后台"，与本次"正常软件"的诉求冲突，故留作下一版选项。
2. **运行记录与输出**：把每次执行的退出码、耗时、尾部输出写入 `history.jsonl`，在 UI 里查看失败原因。当前 `ShellExecuteW` 不等待子进程，需改为 `CreateProcess` + 管道，复杂度明显上升。
3. **命令分组 / 标签 / 置顶 / 最近使用排序**：命令超过约 20 条时收益才开始显现。
4. **导入导出 JSON / 配置云同步**：目前文件本身就是 JSON，手动复制即等价备份。
5. **深色模式与主题切换**：当前配色变量已集中，后续加 `[data-theme]` 成本很低。
6. **替代实现：完全去掉 Tauri**，用 PowerShell/WPF 或 AutoHotkey 做单文件脚本启动器 —— 体积更小但 UI 与维护性大幅下降，不建议。
7. **替代实现：Web 前端改为原生 HTML/JS**，去掉 Vue/Vite 依赖，构建产物更小；但状态管理与弹窗会回到手工 DOM 操作，可维护性变差，不划算。

---

## 9. v2.1 增补（实施后复盘）

> 本节覆盖 v2.0 发布后的实测反馈，与上文冲突处以本节为准。详细证据见 [`PRIVACY-AUDIT.md`](PRIVACY-AUDIT.md)。

### 9.1 实测结果

v2.0 已在本地完成 `vue-tsc` / `cargo check` / `cargo fmt --check` / `npx tauri build --bundles nsis` 全链路验证，安装包 1.04 MiB；实机启动确认窗口可见且未最小化、四角圆角与红绿灯正常、旧 `localStorage` 三条命令自动迁移到 Roaming。CI 首轮构建成功并产出可用 artifact。

### 9.2 隐私追问与结论

用户下载 CI 产物后一打开就看到旧命令，怀疑数据被打包进公有仓库。

结论：**没有泄露**。命令数据只在本机 Roaming 目录；仓库 HEAD、全部提交历史、`dist/`、便携 exe 与 NSIS 安装包的 UTF-8/UTF-16/GBK 字节扫描均无命中。原因是数据目录为固定绝对路径，任何构建在同一台机器上都会读同一份文件。CI 只编译不运行应用，runner 上不存在该数据。唯一额外发现是 Git 提交作者邮箱（v1 原始提交中已存在，来自用户本机 git 配置），已在文档里告知，未擅自修改用户 git 配置。

### 9.3 顶部标识线被遮挡的修复

v2.0 用 `.window::before`（绝对定位 + `z-index`）画顶部琥珀线，并让卡片 hover 时 `transform: translateY(-2px)`。透明 WebView2 窗口下，`transform` 会把卡片提升为合成层，容易在重绘时盖住或擦掉位于其他层级的细线。

v2.1 改为两处结构性修复：

- 琥珀线改成 `.window` 的**普通 flex 子元素** `.accent-line`，参与正常布局、不依赖 z-index，从机制上不可能被后代盖住；
- **移除 hover 的 `transform`**，只用边框色、背景色与 `box-shadow` 表达悬停；同时给标题栏/内容/页脚显式 z-index，堆叠顺序确定。

副作用是避免了合成层抖动，透明窗口的重绘更稳定。

### 9.4 其它 v2.1 决策

| 变更 | 理由 |
| --- | --- |
| 底部路径显示为 `%APPDATA%\com.cmdbar`，完整路径仅在 tooltip | 截图外发不泄露 Windows 用户名 |
| 卡片操作按钮常驻（低透明度），卡片 `Tab` 可聚焦、回车/空格可运行，`/` 聚焦搜索 | 解决"按钮 hover 才出现"的可发现性与键盘可达性 |
| `prefers-reduced-motion` 支持、全局 `:focus-visible` 轮廓、`aria-label` | 无障碍与减少动效 |
| 数据文件损坏时改为备份 `commands.json.bak` + 重建 + 返回 warning 提示 | 原来直接报错会诱导用户后续覆盖写，可能丢数据 |
| 缺省字段自动补全、空 id 自动补齐、首尾空白规范化 | 支持"手工编辑 JSON"这一用法 |
| `.gitignore` 增加 `commands.json*` 与 `release-artifacts/` | 防止误把数据文件拷进仓库 |
| CI 增加隐私检查、产物扁平化并显式命名 `CmdBar-portable.exe`、构建失败重试、`concurrency` / `timeout-minutes` | 公有仓库的分发安全与构建鲁棒性 |
| 文档新增隐私审计、便携版说明、快捷键表 | 直接回答"能不能免安装"和"数据从哪来" |
