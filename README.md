# CmdBar

一个轻量、好看的 Windows 小工具：把常用的脚本和命令收进一个窗口，点一下就跑。

- 暖黄米白界面（莱茵生命配色取向），四周圆角 + 左上角 macOS 红绿灯
- 命令保存在 `%APPDATA%\com.cmdbar\commands.json`，看得见、能备份、能手工编辑
- 支持**按需管理员提权**（UAC），CmdBar 自身始终保持普通权限
- 没有开机自启、没有静默启动、没有托盘常驻 —— 就是一个正常的桌面程序
- 无背景图、无毛玻璃、无重型 UI 框架，图标取自 Lucide

> 重构决策见 [`docs/REFACTOR-v2.md`](docs/REFACTOR-v2.md)，隐私审计见 [`docs/PRIVACY-AUDIT.md`](docs/PRIVACY-AUDIT.md)。

---

## 下载与运行

GitHub Actions 每次构建产出两个文件（Releases 页面同样提供）：

| 文件 | 说明 |
| --- | --- |
| `CmdBar-portable.exe` | **免安装，双击直接运行**。不写注册表、不装服务、不注册自启动 |
| `CmdBar_2.1.0_x64-setup.exe` | NSIS 安装包，带开始菜单项与卸载程序，适合长期使用 |

**是的，portable 那个可以直接跑。** 两个 exe 的核心代码完全相同，区别只在分发方式。

两点说明：

- 依赖系统自带的 **WebView2 Runtime**（Windows 10/11 一般已预装）。若双击没反应，安装一次 [WebView2 Evergreen Runtime](https://developer.microsoft.com/microsoft-edge/webview2/) 即可。
- "免安装"不等于"零写入"：命令数据仍会写到 `%APPDATA%\com.cmdbar\commands.json`，见下一节。

## 使用

### 添加命令

点右上角「新建命令」，填写：

| 字段 | 说明 |
| --- | --- |
| 显示名称 | 卡片上显示的名字，如 `清理临时文件` |
| 命令行 | 要执行的**完整命令行**，如 `C:\scripts\clean.cmd --all` |
| 工作目录（可选） | 脚本的相对路径基准；留空则继承当前目录 |
| 以管理员身份运行 | 勾选后执行时弹出 UAC 授权窗口 |

点击卡片即运行，右上角三个小按钮分别是运行 / 编辑 / 删除。卡片可用 `Tab` 聚焦，回车或空格执行。

### 快捷键

| 按键 | 作用 |
| --- | --- |
| `/` | 聚焦搜索框（在输入框内输入 `/` 不会触发） |
| `Esc` | 关闭弹窗 / 清空搜索 |
| `Enter` | 在编辑弹窗中保存；聚焦卡片时运行 |

### 命令行的写法

命令统一交给 `cmd.exe /D /S /C` 执行，所以下面这些都能用：

```bat
C:\scripts\clean.cmd --all
python "C:\My Scripts\report.py" --today
npm run dev
echo hello && pause
dir C:\ /b > "%TEMP%\listing.txt"
start "" notepad.exe
```

- 支持 `.bat` / `.cmd`、管道、`&&`、环境变量、内建命令。
- 路径含空格时请自己加引号，外层引号由程序处理，不要额外包一层。
- 脚本会打开一个控制台窗口（方便看输出、用 `pause`）；启动 GUI 程序且不想让控制台驻留时，用 `start "" app.exe`。

### 管理员权限

只有**勾选了「以管理员身份运行」**的命令才会走 `runas` 提权，Windows 会弹出一次 UAC。取消授权时界面会提示「已取消管理员授权（UAC），命令未执行」，不会留下半吊子进程。

CmdBar 本身**不要求**管理员权限，也不需要以管理员身份启动整个程序；普通命令完全不会打扰你。若脚本需要修改系统目录、注册表或运行 `netsh` / `sc` 之类的命令，勾上这一项即可。

---

## 数据存哪儿

```
%APPDATA%\com.cmdbar\commands.json
```

窗口底部会显示这个**不含用户名**的短路径，悬停可看完整绝对路径，点「打开」直接跳到目录。

文件内容是缩进过的 JSON，可直接手工编辑或复制备份：

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

- 每次增改删都是**临时文件 + 原子替换**写入，不会写坏文件。
- 时间戳是 Unix 毫秒（`createdAt` 创建时间、`lastRunAt` 上次运行时间）。
- 手工编辑后缺字段会自动补全；若 JSON 损坏，程序会把它备份成 `commands.json.bak` 再重建空列表，并给出提示，不会静默吞掉你的文件。
- 从 v1.x 升级时，旧的 `localStorage` 数据会在首次启动自动迁移到这里，迁移成功后旧键被清除。
- 备份/换机：直接复制这个 `commands.json`；删除它（以及同目录的 `.bak`）等于彻底清空。

---

## 隐私与数据边界

这是公有仓库，分发产物任何人都能下载，所以这里说清楚：

- **CmdBar 不联网、不上传、不遥测。** 命令只写在本机 `%APPDATA%\com.cmdbar\commands.json`。
- **仓库和 CI 产物里不含任何命令数据。** CI 在全新的 GitHub runner 上只编译、不启动应用；工作流里有「隐私检查」步骤，产物目录一旦出现 `commands.json`、`.env`、数据库或备份文件就直接失败并停止上传。完整审计见 [`docs/PRIVACY-AUDIT.md`](docs/PRIVACY-AUDIT.md)。
- **为什么下载后一打开就看到旧命令？** 因为便携版读取的是你这台机器上**同一份 Roaming 数据文件**，不是 exe 内置的。v1 的数据原本在 WebView2 的 `localStorage`，v2 首次启动迁移到了 Roaming。数据跟着机器走，不跟着 exe 走；换一台电脑再用同一个 exe，列表就是空的。
- `com.cmdbar` 是固定的全局目录，任何能以你身份运行的程序都能读到这个明文 JSON。**不要把密码、Token、密钥写进命令行**，敏感信息请放在脚本自己的安全存储里。
- 界面会显示命令名与路径，截图外发前请自行确认内容；底部已改为显示 `%APPDATA%\com.cmdbar`，不再暴露 Windows 用户名。
- Git 提交作者邮箱来自本机 `git config`，v1 原始提交中已存在。如需隐藏，可在 GitHub 启用 `noreply` 邮箱后自行修改本地配置（本项目不改你的 git 配置）；历史提交里的邮箱需要单独重写历史才能去除。

---

## 开发

需要 Node.js 20+、Rust stable、WebView2 Runtime。

```bash
npm ci
npm run dev        # 只跑前端
npm run tauri dev  # 完整开发模式
npm run build      # vue-tsc 类型检查 + vite 构建
npx tauri build --bundles nsis   # 产出安装包与便携 exe
```

本机若需要走代理（例如 `http://127.0.0.1:7890`），用环境变量注入，不要写进仓库：

```powershell
$env:HTTPS_PROXY = "http://127.0.0.1:7890"
$env:HTTP_PROXY  = "http://127.0.0.1:7890"
```

仓库内不含 `.cargo/config.toml`，避免把本机代理配置带进 CI 或别人的机器。

### 技术栈

| 层 | 内容 |
| --- | --- |
| 前端 | Vue 3 + TypeScript + Vite + `@lucide/vue`，纯 CSS 变量主题 |
| 后端 | Rust + Tauri 2，`serde_json` 读写数据文件，`ShellExecuteW` 执行命令与提权 |
| 窗口 | `decorations: false` + `transparent: true`，CSS 圆角与投影；红绿灯为自定义标题栏按钮 |

无 `tauri-plugin-autostart`、`plugin-opener`、`plugin-store` 等额外插件：数据读写和打开目录都由几个自定义 command 完成。

---

## CI

`.github/workflows/build.yml` 在 `windows-latest` 上执行，push 到 `main`、PR、手动触发或打 `v*` 标签时启动：

`npm ci` → `cargo fmt --check` → `npm run build` → `npx tauri build --bundles nsis`（失败自动重试一次）→ 整理成两个扁平命名的 exe → **隐私检查** → 上传 artifact；打标签时额外创建 GitHub Release。

产物目录 `release-artifacts/` 与数据文件 `commands.json` 都在 `.gitignore` 里，不会被提交。

---

## 版本

- **v2.1.0**：UI/UX 打磨（顶部标识线改为普通布局元素、移除 hover 位移以消除遮挡、操作按钮常驻、键盘可达、减弱动效）；底部路径脱敏；数据文件损坏时自动备份并提示；CI 增加隐私检查、产物扁平命名与构建重试；补隐私审计文档。
- **v2.0.0**：全面重构。数据迁到 Roaming 目录、按需管理员提权、暖黄米白主题、全新图标、移除开机自启与静默启动、补齐 CSP / `Cargo.lock` / CI。
- v1.1.0：暗红主题、开机自启与静默启动（已移除）。
