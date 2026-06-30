# Melovibe 登录重构方案

## 1. 当前阻塞

当前实现尝试通过前端 `WebviewWindow` 打开网易云或 QQ 音乐登录页，并调用：

```ts
win.eval<string>("document.cookie")
```

该方案在 Tauri v2 中不可行：

- 前端 `WebviewWindow` API 没有 `eval()` 方法。
- 第三方页面与主应用属于不同来源，不能从主窗口读取其页面状态。
- 登录 Cookie 通常包含 `HttpOnly` 字段，即使在登录页自身执行 `document.cookie` 也无法取得完整会话。
- 给任意远程登录页开放脚本注入会扩大安全边界，不应为读取 Cookie 添加宽泛 capability。

因此，不应继续寻找 `WebviewWindow.eval()` 的替代写法。应根据平台选择后端二维码协议或受控 WebView Cookie Store。

## 2. 推荐结论

### 网易云：后端二维码登录

恢复现有的二维码 key → 生成二维码 → 后端轮询流程。Rust 端必须从二维码确认接口响应头的 `Set-Cookie` 中提取并保存登录 Cookie，而不是只解析 JSON body。

这是当前最短、最稳定的路径，且项目已经具备：

- `music_qr_key`
- `music_qr_create`
- `music_qr_check`
- `CookieStore`
- 前端二维码 service 方法

### QQ 音乐：分阶段处理

第一阶段保留手动 Cookie 登录，不伪装成已经支持稳定的网页登录。

第二阶段只有在确认 Windows WebView2 Cookie Manager 能稳定读取 QQ 登录会话，并完成安全评审后，才增加独立登录 WebView。不要把网易云和 QQ 强行塞进同一个登录实现。

## 3. 网易云实施方案

### 3.1 Rust 请求层

目前 `weapi_post()` 直接调用 `resp.json()`，响应头在解析后丢失。为二维码检查增加一个保留响应头的请求路径：

1. 发送 `login/qrcode/client/login` 请求。
2. 在消费 body 前读取所有 `Set-Cookie` 响应头。
3. 每条 Cookie 只保留第一个 `;` 前的 `name=value`，移除 `Path`、`Expires`、`HttpOnly`、`SameSite` 等属性。
4. 按 Cookie 名去重，并与轮询期间已有 Cookie 合并。
5. 解析 JSON body。
6. 当状态码为 `803` 时，将合并后的 Cookie 返回给 command。

不要把所有 `weapi_post()` 都改成复杂响应类型。Ponytail 原则：只为二维码登录增加当前需要的最小 helper，等其他接口确实需要响应头时再抽象。

建议返回类型：

```rust
pub struct QrCheckResult {
    pub body: serde_json::Value,
    pub cookie: String,
}
```

### 3.2 Tauri command

`music_qr_check` 负责：

- 调用 `netease::qr_check`。
- `code == 803` 时验证 Cookie 至少包含 `MUSIC_U=`。
- 通过 `CookieStore::set_netease_cookie()` 保存。
- 再调用一次 `netease::login_status()` 验证会话。
- 向前端返回明确状态，而不是让前端猜测字段位置。

推荐返回：

```json
{
  "code": 803,
  "success": true,
  "loggedIn": true,
  "profile": {
    "userId": "...",
    "nickname": "...",
    "avatar": "..."
  }
}
```

Cookie 绝不能返回前端、写入日志或显示在错误信息中。

### 3.3 前端状态机

恢复 `LoginModal.vue` 的网易云二维码状态：

```text
idle → loading → waiting → scanned → success
                         ↘ expired
                         ↘ error
```

状态码：

- `800`：二维码过期，停止轮询，允许刷新。
- `801`：等待扫码。
- `802`：已扫码，等待手机确认。
- `803`：登录成功，停止轮询并刷新账号状态。

轮询建议每 2 秒一次；组件卸载、切换平台、关闭弹窗或成功后必须清除 timer。禁止并行启动多个轮询。

### 3.4 Cookie Store

当前 `.netease-cookie` 和 `.qq-cookie` 明文文件仅可作为迁移阶段临时实现。后续应迁移到系统凭据存储或加密存储，但不要阻塞本次登录修复。

最低要求：

- 应用数据目录由 `app.path().app_data_dir()` 获取。
- 文件不进入 Git。
- 日志不输出 Cookie。
- 登出时清空内存和磁盘内容。

## 4. QQ 登录方案

### 4.1 当前阶段

- UI 明确显示“QQ 音乐暂使用 Cookie 登录”。
- 校验至少包含 `uin=`，并接受 QQ 音乐实际需要的关键字段。
- 保存后调用一个真实的账号信息接口验证，而不是仅凭 `uin=` 判断 `loggedIn: true`。
- 验证失败时不保存无效 Cookie，或立即回滚。

### 4.2 后续 WebView 登录

如果确实需要官方网页登录，应在 Rust 侧使用 Tauri/Wry/WebView2 可用的 Cookie Manager，而不是前端脚本注入。实施前需验证：

- Tauri 当前版本是否公开了目标平台的 Cookie Store API。
- 是否能读取 HttpOnly Cookie。
- 登录窗口和后端 HTTP 请求是否共享同一 Cookie profile。
- Windows、macOS 的实现差异。
- 登录窗口关闭、超时、清 Cookie和多账号切换行为。

如果需要大量平台专用代码，先只实现 Windows，并通过 `#[cfg(target_os = "windows")]` 隔离；其他平台继续使用 Cookie 登录。

## 5. Capability 原则

后端二维码登录不需要创建远程 WebView，因此应移除仅为错误方案添加的：

```json
"core:webview:allow-create-webview-window"
"core:webview:allow-webview-close"
```

只有未来真正实现受控登录 WebView 时，才添加最小权限。不要开放通配窗口或远程页面脚本执行权限。

## 6. 文件修改范围

本轮建议只修改：

- `src-tauri/src/services/netease.rs`
- `src-tauri/src/commands/music.rs`
- `src/components/LoginModal.vue`
- `src/stores/account.ts`
- `src-tauri/capabilities/default.json`

如现有 `src/services/music.ts` 类型无法表达新响应，可做最小类型调整。不要顺带重构搜索、播放、歌单或视觉系统。

## 7. 测试与验收

### Rust 最小检查

- Set-Cookie 解析能去除属性并按名称去重。
- 不含 `MUSIC_U` 时不能标记登录成功。
- `803 + MUSIC_U` 会保存 Cookie。
- 登出后内存与磁盘均为空。

### 手动流程

1. 打开网易云登录弹窗并生成二维码。
2. 手机扫码后 UI 从 waiting 变为 scanned。
3. 手机确认后 UI 变为 success，弹窗关闭。
4. 顶部显示真实昵称和头像。
5. 重启应用后仍保持登录。
6. 登出并重启后仍为未登录。
7. 二维码过期可刷新，关闭弹窗后无后台轮询。
8. QQ Cookie 无效时不得显示登录成功。

### 构建检查

```powershell
npm run build
Push-Location src-tauri
cargo test
cargo check
Pop-Location
```

## 8. Git 提交要求

按项目重构规范独立提交：

```text
修复：通过响应头完成网易云二维码登录
```

只暂存本方案涉及文件。当前工作区已有未提交的登录尝试，实施者必须先检查 diff，在其基础上修正或明确回退，禁止覆盖其他无关改动。

## 9. 交接提示词

```text
请修复 D:\Dev\Repos_self\yueyin 的登录流程。开始前加载 tauri-v2 和 ponytail skill，并完整阅读 docs/LOGIN_REFACTOR_PLAN.md。

不要继续使用 WebviewWindow.eval()，也不要通过 document.cookie 读取第三方登录会话。网易云恢复后端二维码轮询：Rust 在二维码确认接口响应中读取 Set-Cookie，安全合并并保存 Cookie，music_qr_check 在 803 时验证 MUSIC_U、刷新账号状态并只向前端返回登录结果，不得把 Cookie 返回前端或写入日志。前端恢复二维码状态机并正确清理轮询。QQ 当前阶段保留手动 Cookie 登录，并使用真实接口验证，禁止只凭 uin= 宣布登录成功。移除错误 WebView 方案新增的多余 capability。

遵循 Ponytail：复用现有 QR commands、service 和 CookieStore，只增加解决响应头丢失所需的最小代码，不创建通用认证框架。完成后运行 npm run build、cargo test、cargo check，按文档要求创建独立 Git 提交，并汇报提交哈希与验证结果。
```
