# Melovibe（悦音）重构路线图

## 1. 项目目标

将 Mineradio 复刻为基于 Vue 3、TypeScript、Pinia、Tauri v2 和 SQLite 的桌面应用 Melovibe。

原项目的前端约 2.6 万行，HTML、CSS、业务状态、Three.js 渲染、音频分析、接口调用和本地存储集中在 `public/index.html`。重构不采用一次性重写，而采用按功能域逐步替换的方式：每迁移一个功能，就完成组件化、类型化、持久化和回归验证。

当前进度：应用外壳、无边框窗口控制、窗口圆角和启动页已初步重构。搜索、播放、歌词、账号、歌单、视觉系统、数据库和服务接口尚未完整迁移。

## 2. 重构原则

所有前端页面、组件和交互动效必须先阅读并遵循 `FRONTEND_REPLICATION_SPEC.md`；涉及静态资源时同时遵循 `FRONTEND_ASSET_MANIFEST.md`。不得凭模型审美重新设计，不得在缺少可靠视觉输入时描述或生成图片信息。

### 强制使用 Ponytail Skill

所有重构阶段必须先加载并遵循 `ponytail` skill，默认使用 **full** 强度。执行者开始工作时需要明确声明正在使用 Ponytail；若当前环境没有该 skill，应停止实现并先安装或请求提供，不能悄悄忽略此要求。

Ponytail 在本项目中的具体约束：

- 先完整理解原功能调用链，再选择最小可行改动；不能用“少读代码”冒充简化。
- 优先复用项目已有实现，其次使用 Rust/TypeScript 标准库、Tauri/Vue 原生能力和现有依赖。
- 不为未来需求预建接口、工厂、插件系统、通用仓储基类或只有一个实现的抽象层。
- 一个阶段只建立当前功能真实需要的表、组件、命令和类型，后续阶段需要时再扩展。
- 优先删除重复代码；能用数据库约束、CSS 或平台能力完成的，不额外写应用层框架。
- 已知存在容量或性能上限的刻意简化，必须用 `ponytail:` 注释说明上限及何时升级。
- 非平凡逻辑至少保留一个最小可运行检查；不为简单代码搭建庞大测试框架。
- 绝不简化掉数据安全、输入校验、错误处理、数据库迁移、凭据保护和无障碍基础。

### 涉及动画时强制使用 GSAP Skills

凡是迁移、实现、重构或评审 GSAP 动画，必须先加载 `gsap-core` skill；若当前环境没有对应 skill，应停止该动画任务并先安装或请求提供，不得凭印象编写 GSAP API。

按任务内容追加加载专项 skill：

| 动画任务 | 必须使用的 skill |
|---|---|
| `gsap.to/from/fromTo`、ease、stagger、响应式动画 | `gsap-core` |
| 多阶段编排、启动/退出序列、可暂停或反向动画 | `gsap-timeline` |
| 动画掉帧、批量元素、Three.js 属性 tween、性能评审 | `gsap-performance` |
| Vue 生命周期与组件清理 | `gsap-frameworks` |
| ScrollTrigger 滚动驱动 | `gsap-scrolltrigger` |
| Flip、Draggable、MotionPath 等插件 | `gsap-plugins` |
| clamp、mapRange、wrap 等 GSAP 工具 | `gsap-utils` |

项目约束：

- 简单 hover、颜色或单段过渡优先 CSS；只有需要编排、运行时控制、复杂缓动或动态值时才使用 GSAP。
- Vue 组件中的 tween、timeline、matchMedia 和事件必须在卸载时 kill/revert，禁止遗留动画实例。
- 优先动画 `x/y/scale/rotation/autoAlpha`，避免频繁动画 `top/left/width/height` 导致布局抖动。
- 多步骤动画使用 timeline，不用堆叠 delay 模拟时间轴。
- 必须通过 `gsap.matchMedia()` 尊重 `prefers-reduced-motion`。
- Three.js 动画只 tween 数值对象或 uniforms，不在每帧创建新 tween。
- 重构 Mineradio 现有动画时，先记录原 duration、ease、stagger、overwrite 和清理行为，再保持等效迁移。

1. **行为优先于代码美观**：先保证与 Mineradio 一致，再优化结构。
2. **按垂直功能切片**：一个阶段同时完成 UI、状态、服务、Tauri 命令和数据表，不留下半套系统。
3. **渲染与业务隔离**：Three.js/WebGL 不读取 Vue DOM，不直接修改 Pinia；通过明确的事件和快照通信。
4. **数据库是持久化主源**：结构化数据进入 SQLite；临时 UI 状态才使用内存或 localStorage。
5. **前端不直连敏感或受限接口**：跨域代理、文件访问、缓存和凭据通过 Rust/Tauri 后端处理。
6. **每个阶段都可运行**：禁止长期维护一个无法启动的“大迁移分支”。

## 3. 目标架构

```text
src/
├─ app/                 # 启动、路由、全局错误和生命周期
├─ components/          # 无业务归属的通用组件
├─ features/
│  ├─ splash/           # 启动页与 shader
│  ├─ window/           # 标题栏、窗口状态
│  ├─ search/           # 搜索框、历史、结果
│  ├─ player/           # 播放器、进度、音量、音质
│  ├─ queue/            # 播放队列
│  ├─ lyrics/           # 歌词获取、同步、桌面歌词
│  ├─ library/          # 收藏、歌单、最近播放
│  ├─ account/          # 网易云/QQ 登录与账号状态
│  ├─ podcast/          # 播客及 DJ 节拍图
│  ├─ visualizer/       # Three.js 场景、粒子、相机、shader
│  ├─ presets/          # 视觉参数、用户存档、导入导出
│  ├─ weather/          # 天气与天气电台
│  └─ settings/         # 设置、更新和诊断
├─ stores/              # 仅跨功能域的 Pinia store
├─ services/            # Tauri IPC/API 适配层
├─ db/                  # 前端数据 DTO 与 repository 接口
├─ types/               # 共享领域类型
├─ utils/               # 无副作用工具
└─ workers/             # 可下放的音频/图像计算

src-tauri/src/
├─ commands/            # 薄 IPC 命令入口
├─ db/                  # SQLite 连接、迁移、repository
├─ domain/              # Rust 领域结构
├─ services/            # 音乐源、缓存、下载、更新、系统能力
├─ audio/               # 后台分析、节拍缓存
└─ lib.rs               # 插件、状态和命令注册
```

## 4. 数据库方案

建议使用 SQLite，Rust 侧优先选择 `rusqlite` 或 `sqlx`。首版优先 `rusqlite + migrations`，依赖少且适合桌面单用户应用。

核心表：

| 表 | 用途 |
|---|---|
| `app_settings` | 通用设置、模式、音质和界面偏好 |
| `accounts` | 平台、用户标识、展示信息；敏感令牌需单独加密保存 |
| `tracks` | 统一歌曲元数据与来源标识 |
| `playlists` / `playlist_tracks` | 歌单与曲目顺序 |
| `play_queue` | 当前队列、顺序和恢复位置 |
| `play_history` | 最近播放、次数和最后进度 |
| `favorites` | 收藏关系 |
| `search_history` | 搜索历史和使用次数 |
| `lyrics_cache` | 原始歌词、翻译、罗马音和更新时间 |
| `beat_maps` | 节拍分析结果、算法版本和音频指纹 |
| `visual_presets` | 视觉预设 JSON、版本和默认标记 |
| `background_assets` | 自定义背景文件路径、哈希和元数据 |
| `podcast_progress` | 播客播放进度 |
| `cache_entries` | 接口缓存索引、过期时间和磁盘路径 |

迁移要求：

- 使用递增 migration 文件，禁止启动时临时拼 SQL 改表。
- 数据库文件通过 Tauri `app_data_dir` 获取，禁止硬编码路径。
- 首次启动将 Mineradio localStorage/IndexedDB 数据导入 SQLite；导入成功后写迁移标记，但保留原数据一段版本周期。
- Token、Cookie 等敏感数据不得以明文 JSON 写入普通表，使用系统凭据存储或加密字段。

## 5. 分阶段路线

### 阶段 0：建立迁移基线

- 为 Mineradio 关键流程录屏、截图并保存默认数据样本。
- 列出全部 API、localStorage key、IndexedDB object store 和 server.js 路由。
- 为 `yueyin` 建立 ESLint、格式化、单元测试和基础 CI。
- 定义统一 `Track`、`Playlist`、`Account`、`LyricLine`、`VisualPreset` 类型。

完成标准：两边关键行为有可比对清单，构建和测试命令稳定。

### 阶段 1：应用基础设施与 SQLite

- 建立 Tauri capability、错误类型、IPC 返回协议和日志。
- 初始化 SQLite、迁移器和 repository。
- 建立 `settingsStore`、`appStore`，实现设置读写和窗口状态恢复。
- 把版本、系统信息等现有命令移入规范模块。

完成标准：设置能跨重启恢复，数据库可自动创建、升级和备份。

### 阶段 2：音频核心与播放队列

- 拆分 `AudioEngine`、`PlayerControls`、`ProgressBar`、`VolumeControl`、`NowPlaying`。
- 建立 `playerStore` 和 `queueStore`，禁止组件直接操作 Audio 对象。
- 迁移播放、暂停、切歌、进度、音量、循环、随机、队列恢复。
- 将播放历史、当前队列和断点写入 SQLite。

完成标准：本地歌曲可完整播放，重启后恢复队列和进度。

### 阶段 3：搜索与在线音乐源

- 拆分搜索输入、模式切换、历史、结果列表和加载/错误态。
- 为网易云、QQ 等建立统一 `MusicProvider` 接口。
- 将原 `server.js` 中必要代理逐步迁入 Rust service；暂未迁移的接口通过兼容适配层调用。
- 搜索历史进入数据库，接口结果使用带 TTL 的缓存。

完成标准：搜索、获取播放地址、封面和基础元数据可用，错误可恢复。

### 阶段 4：歌词、歌单与账号

- 拆分歌词同步、舞台歌词、桌面歌词和布局设置。
- 迁移登录状态、二维码/网页登录、账号切换。
- 迁移收藏、用户歌单、歌单详情和导入队列。
- 歌词缓存、收藏和歌单快照进入 SQLite。

完成标准：登录后能浏览并播放用户内容，歌词同步准确。

### 阶段 5：Three.js 视觉引擎

- 将渲染代码拆成 `RendererHost`、`SceneController`、`CameraController`、`ParticleSystem`、`CoverPipeline`、`LyricStage` 和 `Shelf3D`。
- Shader 独立为 `.glsl` 文件或 TS 字符串模块，统一管理 uniforms。
- 渲染循环只消费不可变状态快照；Pinia 更新按需节流。
- 迁移封面粒子、鼠标交互、涟漪、相机、3D 歌单架和舞台歌词。
- 加入显存资源释放、窗口隐藏降帧和自适应质量。

完成标准：视觉预设与原项目主要效果一致，切歌无资源泄漏，普通核显可稳定运行。

### 阶段 6：视觉预设与高级功能

- 迁移 DIY 面板、参数分组、颜色实验室、预设切换和导入导出。
- `visual_presets` 保存 schemaVersion，提供旧版本升级函数。
- 迁移自定义背景、裁剪、AI 深度、节拍分析和缓存管理。
- CPU 密集任务移到 Rust 后台线程或 Web Worker。

完成标准：默认存档、用户预设和背景跨重启可恢复，旧数据可导入。

### 阶段 7：播客、天气、更新与收尾

- 迁移播客 DJ、天气电台、拖放、本地文件、更新和诊断。
- 删除兼容 `server.js` 的剩余路径。
- 清理原 index 依赖、重复 CSS、localStorage 临时桥接和死代码。
- 完成安装包、数据升级、崩溃恢复和性能回归。

完成标准：核心功能不依赖 Mineradio 运行时，安装/升级/卸载流程可靠。

## 6. 组件边界规则

- 页面组件负责组合，不放网络请求、SQL 或 Three.js 初始化。
- Feature 组件只调用本域 composable/store，不跨域读取内部状态。
- Store 保存业务状态，不保存 DOM、WebGLRenderer、AudioContext 等不可序列化对象。
- Service 负责 IPC 和外部接口，返回类型化 DTO。
- Repository 负责持久化，不包含界面逻辑。
- 渲染引擎由 class/module 管理生命周期，并提供 `mount/update/dispose`。

## 7. 测试与验收

- 单元测试：数据转换、队列规则、歌词解析、预设迁移。
- Rust 测试：repository、migration、缓存策略和路径安全。
- 组件测试：播放器按钮、搜索状态、窗口控制和设置表单。
- 端到端冒烟：启动 → 搜索 → 播放 → 歌词 → 切歌 → 重启恢复。
- 视觉回归：启动页、Home、播放态、歌单架、歌词舞台和 DIY 面板固定截图。
- 性能基线：启动时间、内存、GPU 帧耗、切歌延迟和数据库查询时间。

每个迁移 PR 必须说明：替代了原 index 的哪一段、数据如何迁移、如何回退、验证了哪些流程。

同时必须附一段 Ponytail 自检：本次复用了什么、拒绝了哪些不必要抽象、新增依赖是否不可避免、是否存在带 `ponytail:` 标记的刻意技术上限。

## 8. Git 提交要求

每完成一个独立重构单元，都必须立即创建一次 Git 提交。Git 提交是“完成”的组成部分，不允许只改代码、不提交，也不允许把多个无关功能攒进同一次提交。

执行规则：

1. 开始前运行 `git status -sb`，识别并保留用户已有的无关改动。
2. 一次只重构一个可独立验证的功能，例如“SQLite 初始化”“播放队列持久化”或“搜索历史组件化”。
3. 完成该功能后运行对应测试、`npm run build`，涉及 Rust 时再运行 `cargo check`。
4. 只暂存本功能涉及的文件，禁止在混合工作区中直接使用 `git add -A`。
5. 检查 `git diff --cached`，确认没有构建产物、凭据、用户文件或无关修改。
6. 创建一个原子提交，再开始下一个重构单元。
7. 测试未通过时不得提交；若失败来自已存在且与本次无关的问题，必须在交付说明中提供证据。
8. 未经用户明确要求，不自动推送远端、不强推、不改写历史。

提交信息采用中文、祈使式和单一目的：

```text
重构：拆分播放队列状态
重构：接入 SQLite 设置存储
修复：恢复窗口状态监听
测试：覆盖歌词时间轴解析
文档：记录视觉引擎迁移边界
```

禁止使用 `update`、`fix stuff`、`修改代码` 等无法说明范围的提交信息。

每次交付必须列出提交哈希、提交信息、验证命令和结果；若本轮未产生提交，则该重构单元不视为完成。

## 9. 建议的近期三个里程碑

1. **M1 基础持久化**：完成 SQLite、设置表、repository 和迁移框架。
2. **M2 可用播放器**：完成音频核心、播放队列、进度与重启恢复。
3. **M3 在线闭环**：完成搜索、播放地址、歌词与基础歌单。

在 M1–M3 完成前，不建议优先迁移复杂 Three.js 视觉系统。先让数据和播放链路稳定，后续视觉层才能拥有清晰、可靠的输入边界。

## 10. 重构提交记录

| 日期 | 提交 | 范围 | 验证 |
|---|---|---|---|
| 2026-06-30 | `重构：拆分歌单组件并完善基础歌单功能` | 拆分 Home 与歌单面板组件；补齐登录态恢复后的歌单自动获取、歌单/曲目错误态；网易云歌单接口返回登录用户标识 | `npm run build`、`cargo test`、`cargo check` |
| 2026-06-30 | `动效：补齐歌单架入场和播放视觉挂载` | 将歌单列表从普通抽屉调整为右侧浮动卡片栈，详情列表增加 stagger 进场；修复播放视觉引擎挂载时机和显式清理，保证播放时能接收音频分析 | `npm run build`、`cargo test` |
| 2026-06-30 | `动效：复刻播放舞台和 Three.js 歌单架` | 播放态退出首页 DOM；按原项目 183×183 网格、相机基线和远程封面代理恢复彩色粒子封面；歌单卡改为共享 Three.js 场景的 CanvasTexture 弧形架；中央歌词改为舞台大字；音频改走 Rust 字节代理以恢复时长、频谱和歌词时钟 | `npm run build`、`cargo test`、Tauri 实机截图 |

## 11. 交接提示词

```text
请按照 D:\Dev\Repos_self\yueyin\docs\REFACTOR_ROADMAP.md 推进 Melovibe 重构。原项目位于 D:\Dev\Repos_self\Mineradio，目标项目位于 D:\Dev\Repos_self\yueyin。

开始前必须加载并使用 ponytail skill，强度为 full，并在进度消息中明确说明。遵循 YAGNI、标准库/平台原生能力优先、复用优先、最少文件和最短正确 diff；不要创建单实现接口、预留式工厂、通用基类或当前阶段用不到的数据库表。若环境没有 ponytail skill，先停止并报告，不得自行跳过。

凡任务涉及 GSAP，必须先加载 gsap-core；根据任务追加 gsap-timeline、gsap-performance、gsap-frameworks、gsap-scrolltrigger、gsap-plugins 或 gsap-utils。简单动画优先 CSS；GSAP 实例必须随 Vue 组件卸载而清理，必须支持 prefers-reduced-motion。若所需 GSAP skill 不可用，停止动画实现并报告。

本次只执行路线图中的一个明确阶段，不做跨阶段大改。开始前定位 Mineradio public/index.html 和 server.js 中对应功能，列出行为、存储 key、接口和依赖；然后在 yueyin 中按 feature/store/service/repository 分层迁移。保持现有 UI 和行为，不擅自重新设计。所有 Tauri 命令需注册并配置 capability，结构化持久化使用 SQLite migration，不新增 localStorage 技术债。完成后运行 npm run build、cargo check 和该阶段相关测试，并记录已迁移范围、兼容桥接、数据迁移与剩余风险。

每完成一个独立重构单元，必须只暂存该单元的文件并创建一次原子 Git 提交，然后才能继续下一个单元。提交前检查 git status、git diff --cached 和验证结果；禁止混入用户已有改动，禁止把多个功能合并提交，未经要求不要推送。交付时列出提交哈希、提交信息及验证结果，没有提交就不算完成。

交付时补充 Ponytail 自检：说明复用项、跳过的不必要设计、新增依赖理由，以及任何使用 ponytail: 注释标记的简化上限。
```
