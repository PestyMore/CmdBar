# CmdBar

一个轻量、好看的 Windows 小工具：把常用的脚本和命令收进一个窗口，点一下就跑。

- 暖黄米白界面（莱茵生命配色取向），四周圆角 + 左上角 macOS 红绿灯
- 命令保存在 `%APPDATA%\com.cmdbar\commands.json`，看得见、能备份、能手工编辑
- 支持**按需管理员提权**（UAC），CmdBar 自身始终保持普通权限
- 没有开机自启、没有静默启动、没有托盘常驻 —— 就是一个正常的桌面程序
- 无背景图、无毛玻璃、无重型 UI 框架，图标取自 Lucide

> 完整的重构决策与备选方案见 [`docs/REFACTOR-v2.md`](docs/REFACTOR-v2.md)。

---

## 安装

从 [Releases](https://github.com/PestyMore/CmdBar/releases) 下载 `CmdBar_*_x64-setup.exe` 安装即可，或直接在 GitHub Actions 的构建产物里取 `src-tauri/target/release/cmdbar.exe` 单文件运行。

## 使用

### 添加命令

点右上角「新建命令」，填写：

| 字段 | 说明 |
| --- | --- |
| 显示名称 | 卡片上显示的名字，如 `清理临时文件` |
| 命令行 | 要执行的**完整命令行**，如 `C:\scripts\clean.cmd --all` |
| 工作目录（可选） | 脚本的相对路径基准；留空则继承当前目录 |
| 以管理员身份运行 | 勾选后执行时弹出 UAC 授权窗口 |

点击卡片即运行，右上角三个小按钮分别是运行 / 编辑 / 删除。顶部搜索框可按名称、命令行、工作目录即时过滤。按 `Esc` 关闭弹窗或清空搜索。

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

例如 `C:\Users\<你>\AppData\Roaming\com.cmdbar\commands.json`。窗口底部会显示完整路径，点「打开」可直接跳到目录。

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
- 从 v1.x 升级时，旧的 `localStorage` 数据会在首次启动自动迁移到这里，迁移成功后旧键被清除。
- 备份/换机：直接复制这个 `commands.json` 即可；删除它等于清空命令列表。

---

## 开发

需要 Node.js 20+、Rust stable、WebView2 Runtime（Windows 10/11 一般已自带）。

```bash
npm ci
npm run dev        # 只跑前端
npm run tauri dev  # 完整开发模式
npm run build      # vue-tsc 类型检查 + vite 构建
npx tauri build --bundles nsis   # 产出安装包
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

`npm ci` → `cargo fmt --check` → `npm run build` → `npx tauri build --bundles nsis` → 上传 NSIS 安装包与独立 exe 作为 artifact；打标签时额外创建 GitHub Release。

---

## 版本

- **v2.0.0**：全面重构。数据迁到 Roaming 目录、按需管理员提权、暖黄米白主题、全新图标、移除开机自启与静默启动、补齐 CSP / `Cargo.lock` / CI。
- v1.1.0：暗红主题、开机自启与静默启动（已移除）。
