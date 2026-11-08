#[cfg(windows)]
mod platform {
    use std::ffi::c_void;
    use std::path::Path;
    use std::ptr;

    const SW_SHOWNORMAL: i32 = 1;
    const ERROR_CANCELLED: u32 = 1223;
    const SE_ERR_ACCESSDENIED: isize = 5;

    #[link(name = "shell32")]
    unsafe extern "system" {
        fn ShellExecuteW(
            hwnd: *mut c_void,
            operation: *const u16,
            file: *const u16,
            parameters: *const u16,
            directory: *const u16,
            show_cmd: i32,
        ) -> isize;
    }

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetLastError() -> u32;
    }

    fn wide(value: &str) -> Vec<u16> {
        value.encode_utf16().chain(std::iter::once(0)).collect()
    }

    pub fn launch(command: &str, workdir: Option<&Path>, as_admin: bool) -> Result<(), String> {
        let shell = std::env::var("COMSPEC").unwrap_or_else(|_| "cmd.exe".to_string());
        // /D 跳过 AutoRun；/S 只剥掉最外层引号，内部引号与特殊字符原样保留。
        let parameters = wide(&format!("/D /S /C \"{}\"", command.trim()));
        let operation = wide(if as_admin { "runas" } else { "open" });
        let file = wide(&shell);
        let directory = workdir.map(|p| wide(&p.to_string_lossy()));
        let directory_ptr = directory.as_ref().map_or(ptr::null(), |v| v.as_ptr());

        let result = unsafe {
            ShellExecuteW(
                ptr::null_mut(),
                operation.as_ptr(),
                file.as_ptr(),
                parameters.as_ptr(),
                directory_ptr,
                SW_SHOWNORMAL,
            )
        };

        if result > 32 {
            return Ok(());
        }
        let last_error = unsafe { GetLastError() };
        if last_error == ERROR_CANCELLED {
            return Err("已取消管理员授权（UAC），命令未执行".to_string());
        }
        if result == SE_ERR_ACCESSDENIED {
            return Err("权限不足，无法以管理员身份运行".to_string());
        }
        Err(format!("启动失败（Windows 错误码 {result}）"))
    }

    pub fn open_dir(path: &Path) -> Result<(), String> {
        std::process::Command::new("explorer.exe")
            .arg(path)
            .spawn()
            .map(|_| ())
            .map_err(|e| format!("打开数据目录失败：{e}"))
    }
}

#[cfg(not(windows))]
mod platform {
    use std::path::Path;

    pub fn launch(command: &str, workdir: Option<&Path>, as_admin: bool) -> Result<(), String> {
        if as_admin {
            return Err("管理员提权目前仅支持 Windows".to_string());
        }
        let mut child = std::process::Command::new("sh");
        child.arg("-c").arg(command);
        if let Some(dir) = workdir {
            child.current_dir(dir);
        }
        child
            .spawn()
            .map(|_| ())
            .map_err(|e| format!("启动失败：{e}"))
    }

    pub fn open_dir(path: &Path) -> Result<(), String> {
        let program = if cfg!(target_os = "macos") {
            "open"
        } else {
            "xdg-open"
        };
        std::process::Command::new(program)
            .arg(path)
            .spawn()
            .map(|_| ())
            .map_err(|e| format!("打开数据目录失败：{e}"))
    }
}

pub use platform::{launch, open_dir};
