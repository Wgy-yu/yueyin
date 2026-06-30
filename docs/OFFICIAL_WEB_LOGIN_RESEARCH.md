# Mineradio 官方网页登录迁移调研

## 1. 调研结论

Mineradio 并非使用网易云接口生成二维码。它的真实流程是：

1. Electron 创建独立 `BrowserWindow`。
2. 打开网易云或 QQ 音乐官方登录网页。
3. 用户在官方页面扫码、确认登录。
4. Electron 从该窗口专用 session 的 Cookie Store 读取全部 Cookie，包括 HttpOnly Cookie。
5. 按允许域名过滤并按关键字段排序，拼成请求 Cookie header。
6. 检测登录字段满足条件后关闭窗口，将会话同步给应用后端。

Tauri v2 可以复刻这个模型，但 Cookie 必须在 **Rust 侧**读取，不能在前端调用不存在的 `WebviewWindow.eval()`，也不能依赖 `document.cookie`。

## 2. Mineradio 源码证据

实现位于 `D:\Dev\Repos_self\Mineradio\desktop\main.js`：

- 网易云官方地址：`https://music.163.com/#/login`
- QQ 官方地址：`https://y.qq.com/n/ryqq/profile`
- 网易云窗口：`openNeteaseMusicLoginWindow()`
- QQ 窗口：`openQQMusicLoginWindow()`
- Cookie 读取：`cookieSession.cookies.get({})`
- 检查周期：1200ms
- 网易云成功条件：存在 `MUSIC_U`
- QQ 基础登录条件：有效 UIN 加登录 key
- QQ 可播放条件：有效 UIN 加 `qm_keyst`、`qqmusic_key`、`music_key` 或 `wxskey`

原项目还使用独立持久化 partition 隔离两个平台的网页登录会话，并按域名过滤 Cookie，避免把无关域 Cookie 注入音乐接口。

## 3. Tauri v2 API 调研

当前项目解析到 Tauri `2.11.3`。Rust API 中 `tauri::webview::WebviewWindow` 提供：

- `cookies()`：读取 WebView Cookie Store 的全部 HTTP(S) Cookie。
- `cookies_for_url(url)`：读取指定 URL 对应的 Cookie。
- 返回值包含 HttpOnly 和 Secure Cookie。
- `clear_all_browsing_data()`：清理登录 WebView 数据。
- `navigate()`：处理登录过程中的同窗口跳转。
- `eval()`：Rust 侧存在，但本登录方案不依赖它读取 Cookie。

官方文档特别说明：Windows 上在同步 command 或事件回调中读取 Cookie 可能因 WebView2 问题发生死锁，应使用 async command，并在独立任务/线程读取。

资料：

- [Tauri WebviewWindow Rust API](https://docs.rs/tauri/latest/tauri/webview/struct.WebviewWindow.html)
- [Tauri Webview Rust API](https://docs.rs/tauri/latest/tauri/webview/struct.Webview.html)
- [Wry WebView API](https://docs.rs/wry/latest/wry/struct.WebView.html)

## 4. 正确架构

```text
Vue LoginModal
  │ invoke("music_open_web_login", { provider })
  ▼
Rust async command
  │ WebviewWindowBuilder 打开官方 HTTPS 页面
  │ 每 1200ms 调用 cookies()/cookies_for_url()
  │ 按平台域名过滤与验证
  │ CookieStore 持久化
  │ 调用真实账号接口验证
  ▼
返回 LoginInfo / emit 登录结果
  │
  ▼
Pinia 刷新账号状态，关闭登录弹窗
```

前端只发起登录并显示状态，不接触 Cookie 内容。

## 5. Rust 实施方案

### 5.1 新增 command

建议新增一个命令：

```rust
#[tauri::command]
pub async fn music_open_web_login(
    provider: String,
    app: tauri::AppHandle,
    cookies: tauri::State<'_, CookieStore>,
) -> Result<LoginInfo, String>
```

不要为两个平台建立接口、工厂和策略层。Ponytail：一个 command 内按 `provider` 分支，复用小型纯函数完成域名过滤和成功条件判断。

### 5.2 创建官方登录窗口

Rust 使用：

```rust
use tauri::{WebviewUrl, WebviewWindowBuilder};

let window = WebviewWindowBuilder::new(
    &app,
    label,
    WebviewUrl::External(login_url.parse().map_err(|e| e.to_string())?),
)
.title(title)
.inner_size(940.0, 760.0)
.min_inner_size(780.0, 580.0)
.center()
.build()
.map_err(|e| e.to_string())?;
```

要求：

- 标签固定为 `music-login-netease` / `music-login-qq`，打开前关闭或聚焦旧窗口，避免重复窗口。
- 只加载官方 HTTPS 地址。
- 外部非白名单跳转交给系统浏览器或拒绝。
- 不在 URL、标题、日志和事件 payload 中放 Cookie。
- 不通过前端 `@tauri-apps/api/webviewWindow` 创建远程窗口。

### 5.3 Cookie 轮询

在 async command 或 `tauri::async_runtime::spawn` 中每 1200ms读取：

```rust
let cookies = window.cookies().map_err(|e| e.to_string())?;
```

为了兼容平台和跳转域名，推荐读取全部 Cookie 后严格过滤，而不是只读取单个 URL：

- 网易云允许：`163.com`、`music.163.com`、其子域、`netease.com` 及其子域。
- QQ 允许：`qq.com`、其子域、`qqmusic.qq.com`。

构造 header 时：

- 只保留非空的 `name=value`。
- 按 Cookie 名去重。
- 优先输出关键 Cookie，其余允许域 Cookie随后输出。
- 绝不输出 Cookie attribute。

### 5.4 登录成功条件

网易云：

```text
MUSIC_U 非空
```

QQ 基础登录：

```text
有效 uin/qqmusic_uin/wxuin/p_uin
+
qm_keyst/qqmusic_key/music_key/p_skey/skey/微信 token 中至少一个
```

QQ 可播放登录：

```text
有效 UIN
+
qm_keyst/qqmusic_key/music_key/wxskey 中至少一个
```

QQ 扫码成功但尚未获得播放 key 时，可像 Mineradio 一样导航到官方 player 页面暖机，再继续检查。此行为仅在确认基础登录后执行一次。

### 5.5 持久化和验证

满足 Cookie 条件后：

1. 保存到现有 `CookieStore`。
2. 调用对应真实账号接口验证身份。
3. 验证通过才返回 `loggedIn: true`。
4. 关闭官方登录窗口。

如果真实接口验证失败，保留网页登录窗口并继续短暂轮询，或返回明确错误；禁止仅凭 Cookie 名直接在 UI 宣布登录成功。

## 6. 窗口关闭与超时

- 用户关闭窗口：返回可识别的 `LOGIN_CANCELLED`，不是通用异常。
- 建议 5 分钟超时，超时后关闭窗口并返回 `LOGIN_TIMEOUT`。
- 应用退出或 LoginModal 卸载时可调用 `music_cancel_web_login` 关闭登录窗口。
- 每次结束必须停止轮询任务。
- 登录窗口已存在时再次点击应聚焦，不要再启动第二个轮询。

## 7. 前端修改

`LoginModal.vue`：

- 保留“打开官方登录窗口”交互，与 Mineradio 一致。
- 删除 `win.eval("document.cookie")` 和前端 Cookie polling。
- 调用 `music_open_web_login` 并等待结果。
- 显示 `opening / waiting / success / cancelled / error`。
- Cookie 手动导入可以保留为故障回退，但不应作为主要入口。

`account.ts`：

- 新增 `openWebLogin(provider)`，调用 service 后更新对应账号状态。
- Store 不保存窗口实例、timer 或 Cookie。

`src/services/music.ts`：

- 新增类型化 `openWebLogin(source)`。
- 返回 `LoginInfo`，不暴露 Cookie 字段。

## 8. Capability 修正

当前未提交改动为前端创建 WebView 添加了：

```json
"core:webview:allow-create-webview-window"
"core:webview:allow-webview-close"
```

改为 Rust command 创建窗口后，前端不需要这些能力，应移除。只保留调用已注册 command 所需的现有权限。

Rust 侧 `WebviewWindowBuilder` 不依赖前端 webview-create capability。

## 9. 依赖与版本

- 不新增 Cookie 插件。
- 不新增浏览器自动化依赖。
- 使用 Tauri 2.11.3 已有 Rust Cookie API。
- 因 cookie 返回类型的依赖可能在 Tauri 小版本变化，建议把 `tauri = "2"` 固定到至少 minor：`tauri = "2.11"`，具体是否修改版本应与现有 lockfile 一起评估。

## 10. 测试与验收

### 纯函数测试

- 网易云/QQ 域名过滤。
- Cookie header 去重和优先顺序。
- 网易云 `MUSIC_U` 判定。
- QQ 基础登录和可播放登录判定。

### 手动验收

1. 点击网易云“网页登录”，打开官方 `music.163.com` 页面。
2. 官方页面扫码并确认后，应用自动同步真实账号。
3. Cookie 中的 HttpOnly 字段可以被 Rust 获取，前端从未接收 Cookie。
4. 重启应用后保持登录。
5. 登出后 API 会话和官方登录 WebView 数据按产品要求清理。
6. QQ 官方窗口能登录，必要时只执行一次 player 暖机。
7. 关闭窗口、超时、重复点击均不会遗留轮询或孤儿窗口。
8. 无效/不完整 Cookie 不会被判定为成功。

### 构建检查

```powershell
npm run build
Push-Location src-tauri
cargo test
cargo check
Pop-Location
```

## 11. 文件范围

- `src-tauri/src/commands/music.rs`
- `src-tauri/src/services/cookie.rs`（仅复用/补纯函数）
- `src-tauri/src/lib.rs`
- `src/components/LoginModal.vue`
- `src/stores/account.ts`
- `src/services/music.ts`
- `src-tauri/capabilities/default.json`

不要修改接口二维码服务，除非决定保留为降级方案；官方网页登录修复不应顺带重构搜索、播放或歌单。

## 12. Git 提交

按项目规范独立提交：

```text
修复：通过 Rust WebView Cookie Store 完成官方登录
```

当前已有未提交的错误尝试。实施前逐项审查并在原文件上修正，保留其他人的无关改动；禁止使用 destructive checkout。

## 13. 交接提示词

```text
请修复 D:\Dev\Repos_self\yueyin 的官方网页登录。开始前加载 tauri-v2 和 ponytail skill，完整阅读 docs/OFFICIAL_WEB_LOGIN_RESEARCH.md，并对照 D:\Dev\Repos_self\Mineradio\desktop\main.js 的 openNeteaseMusicLoginWindow、openQQMusicLoginWindow 和 session.cookies.get 实现。

目标不是接口二维码。必须由 Rust WebviewWindowBuilder 打开网易云/QQ 音乐官方 HTTPS 页面，并在 async command 或独立异步任务中调用 Rust WebviewWindow.cookies()/cookies_for_url() 获取包含 HttpOnly/Secure 的 Cookie。严格按平台域名过滤、按关键字段验证和排序，保存到现有 CookieStore，再调用真实账号接口验证。Cookie 不得返回 Vue、不得写日志。删除前端 win.eval("document.cookie") 和前端 Cookie polling；前端只调用类型化 command 并显示状态。移除不再需要的前端 webview create capability。

遵循 Ponytail：复用现有 CookieStore、LoginInfo 和平台判定逻辑，不引入插件或通用认证框架。为域名过滤和登录条件留下最小 Rust 测试。完成后运行 npm run build、cargo test、cargo check，创建单独 Git 提交并报告哈希。
```
