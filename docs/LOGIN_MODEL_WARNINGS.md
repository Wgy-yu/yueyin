# 登录重构：后续模型注意事项

以下问题在此前改动中真实发生过。后续模型处理登录功能前必须阅读本文件、`OFFICIAL_WEB_LOGIN_RESEARCH.md` 和 Mineradio 对应源码。

## 之前改错的问题

1. **没有先读原项目实现。**
   - 错误地把 Mineradio 登录理解为接口二维码。
   - 实际实现是 Electron `BrowserWindow` 打开网易云/QQ 官方网页，再从该窗口 Session Cookie Store 同步会话。

2. **调用不存在的前端 API。**
   - 使用了前端 `WebviewWindow.eval("document.cookie")`。
   - Tauri v2 前端 `WebviewWindow` 没有该返回值 API；即使执行页面脚本，`document.cookie` 也无法读取 HttpOnly Cookie。

3. **把正确的官方网页登录退化成接口二维码。**
   - 导致二维码资源请求失败，界面出现白色空块和“获取二维码失败”。
   - 也偏离了原项目为避开接口二维码风控而采用官方网页的设计。

4. **只根据 `uin=` 判断 QQ 登录成功。**
   - QQ 还必须具备有效登录票据；播放能力需要 `qm_keyst`、`qqmusic_key`、`music_key` 或 `wxskey` 等字段。

5. **让前端直接接触 Cookie。**
   - Cookie 不应返回 Vue、进入 Pinia、写入日志或错误提示。
   - 正确做法是 Rust 读取、过滤、持久化，并只向前端返回账号信息。

6. **登录界面被自行重新设计。**
   - 使用了泛化青色卡片、错误的窄弹窗和接口二维码布局。
   - 原项目对弹窗宽度、平台色、官方入口卡、按钮和状态文案均有明确源码参数。

7. **为错误的前端建窗方案增加 capability。**
   - Rust command 创建官方 WebView 时不需要前端 `allow-create-webview-window` 权限。
   - 不应为了绕过架构问题扩大前端权限。

## 正确实现约束

- 官方地址必须是：
  - 网易云：`https://music.163.com/#/login`
  - QQ：`https://y.qq.com/n/ryqq/profile`
- Rust 使用 `WebviewWindowBuilder` 创建官方登录窗口。
- Rust async command/独立任务调用 `WebviewWindow.cookies()`。
- Cookie 必须按平台允许域过滤，包含 HttpOnly/Secure Cookie。
- 网易云至少验证 `MUSIC_U`，并调用真实账号接口。
- QQ 验证 UIN 与有效 key；缺少播放 key 时最多执行一次 player 页面暖机。
- 前端只显示 opening、waiting、success、error 状态。
- 关闭弹窗时必须关闭官方登录窗口并停止轮询。
- 重复点击不得创建多个窗口或多个轮询。
- UI 参数遵循 `FRONTEND_REPLICATION_SPEC.md`，不能自行美化。

## 开始修改前必须检查

```text
D:\Dev\Repos_self\Mineradio\desktop\main.js
  openNeteaseMusicLoginWindow
  openQQMusicLoginWindow
  readNeteaseLoginCookieHeader
  readQQLoginCookieHeader

D:\Dev\Repos_self\yueyin\docs\OFFICIAL_WEB_LOGIN_RESEARCH.md
D:\Dev\Repos_self\yueyin\docs\FRONTEND_REPLICATION_SPEC.md
```

涉及 Tauri 必须加载 `tauri-v2` skill；重构必须加载 `ponytail` skill。不要在未验证当前 Tauri 版本 API 的情况下凭记忆写代码。
