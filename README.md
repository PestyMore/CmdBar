## CmdBar v1.1.0 更新日志 (Changelog)

本次更新为一次重大重构，核心目标是极致的性能优化与核心体验的完善。

### 🚀 新增功能 (Features)
- **开机自启 (Autostart):** 安装后，程序首次运行会自动注册到系统启动项。
- **静默启动 (Silent Startup):** 程序启动时将自动最小化至任务栏，不打扰用户当前工作。

### ✨ 优化与改进 (Improvements & Optimizations)
- **极致性能优化 (Performance Overhaul):** 彻底移除了消耗资源的毛玻璃特效（`blur`）与背景图片，应用启动和响应速度得到质的飞跃。
- **全新 Alastor 主题 (New Alastor Theme):** 采用高性能的 CSS 渐变背景，重塑了 UI 的色彩风格，参考自 Alastor 的暗红与黑配色。
- **命令项点击体验优化 (Improved Click Area):** 增大了命令列表项的有效点击区域，现在只要鼠标悬浮的区域均可触发运行。
- **新增 Esc 键隐藏窗口 (Minimize with Esc key):** 在窗口激活时，按下 `Esc` 键可快速将窗口最小化。


# CmdBar

🍏 **Windows ”快速命令启动器**

CmdBar 是一个基于 Tauri v2 + Vue 3 构建的轻量级桌面快捷启动工具。它将 macOS 与 iOS 的拟物化玻璃质感（Glassmorphism）带到了 Windows 系统中，让你能以最优雅的方式一键管理和运行常用的本地脚本与程序。

## ✨ 特性

- **极致视效**：纯正的 macOS 风格控制键与全局高斯模糊背景，打破 Windows 传统直角与死板边框。
- **沉浸交互**：告别系统原生丑陋弹窗，内置纯手工打磨的果味 Alert/Confirm 提示框。
- **极简管理**：以双列网格结构直观呈现你的所有指令，支持一键运行、编辑名称与路径、安全删除。
- **轻量如羽**：得益于 Tauri v2 的底层优化，打包后的独立 `.exe` 体积仅有几 MB，零负担后台常驻。

## 🛠️ 技术栈

- **前端**: Vue 3 + TypeScript + Vite
- **后端**: Rust + Tauri v2
- **UI**: 纯手工 CSS（高斯模糊、Grid 布局、透明穿透技术）

## 🚀 最佳实践指南

虽然 CmdBar 支持直接填入 `.exe` 甚至 `.py` 的路径，但为了获得最佳的体验（尤其是需要查看报错信息或进行命令行交互时），**强烈建议使用 `.cmd` 或 `.bat` 脚本作为中转**。

**示例 `run_tree.cmd`:**
```batch
@echo off
chcp 65001 >nul
echo 正在运行脚本...
python "C:\YourPath\script.py"
pause >nul