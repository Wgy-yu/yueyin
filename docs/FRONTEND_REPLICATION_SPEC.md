# Melovibe 前端复刻规范

## 1. 适用范围

本规范用于将 `D:\Dev\Repos_self\Mineradio` 的前端迁移到 `D:\Dev\Repos_self\yueyin`。

规范只依据 Mineradio 源码中的 DOM、CSS、JavaScript 状态、现有资源引用和明确参数，不依据模型主观审美。开发者不得把“重构”理解为重新设计。

## 2. 禁止事项

- 禁止凭描述、记忆或通用 UI 模板重画页面。
- 禁止自行增加渐变、卡片、边框、光晕、插画、图标和装饰层。
- 禁止用 emoji、字符图标、临时 SVG 或占位框替代原有控件。
- 禁止描述、猜测或生成图片内容；模型没有获得可靠视觉输入时，不得写图片提示词或图片语义说明。
- 禁止仅凭截图估算已有源码中可以读取的数值。
- 禁止把所有组件统一成同一种青色玻璃风格。平台色、状态色和功能层级必须保留。
- 禁止修改文案、信息层级和操作顺序，除非用户明确要求。

## 3. 实施前置流程

每次迁移一个前端功能，必须先完成以下记录：

1. 在 Mineradio `public/index.html` 定位对应 DOM、CSS 和函数。
2. 记录选择器、尺寸、间距、圆角、颜色、字号、字重、层级和状态 class。
3. 记录显示/隐藏条件、交互入口、状态文案和动画参数。
4. 记录所有资源路径及引用位置，不解释图片内容。
5. 找到相邻组件共享的 token 和 helper，避免只抄孤立片段。
6. Vue 实现完成后逐项对照源码参数，不以“看起来差不多”为验收标准。

若无法从源码确定视觉规则，必须标记为 `待确认`，不能自行补完。

## 4. 全局基础规范

### 字体

```css
--font-sans: "Noto Sans SC", "PingFang SC", "HarmonyOS Sans SC",
  "Alibaba PuHuiTi", "Inter", -apple-system, BlinkMacSystemFont,
  system-ui, sans-serif;

--font-mono: "JetBrains Mono", "Geist Mono", "SF Mono",
  ui-monospace, monospace;
```

原项目还加载 `Cinzel Decorative`、`UnifrakturCook`。只有原选择器实际使用时才迁移，禁止为了“更酷”随意套用。

### 基础颜色 token

```css
--fc-bg: #08090b;
--fc-paper: #0e1014;
--fc-ink: #e8ecef;
--fc-ink-2: #d2d7dc;
--fc-muted: #8a9099;
--fc-hair: #1a1d22;
--fc-hair-2: #262a31;
--fc-accent: #00f5d4;
--fc-accent-hov: #00e0be;
--fc-blue: #2442ff;
--fc-warm: #f8f4ee;
--champagne: #f4d28a;
--champagne-deep: #9a6f2c;
--source-netease: #d95b67;
--source-qq: #00f5d4;
--source-local: #9db8cf;
```

### 窗口外壳

- 普通窗口圆角：`34px`。
- 裁剪：`clip-path: inset(0 round 34px)`。
- 阴影：`0 24px 80px rgba(0,0,0,.46)`。
- 使用 `transform: translateZ(0)` 保证固定层被裁剪。
- 最大化、全屏时圆角为 0、无 clip-path、无阴影。
- 自定义标题栏高度：`44px`。
- 窗口按钮：`38 × 30px`，圆角 `10px`，间距 `6px`。

### 层级基准

| 层 | 原项目 z-index |
|---|---:|
| 背景 | 0 |
| 主视觉 Canvas | 1 |
| Home/提示内容 | 3 起 |
| 搜索与顶部操作 | 10 |
| 设置/高级面板 | 18 起 |
| 登录引导 Canvas | 49 |
| 模态遮罩 | 50 |
| 启动页 | 300 |
| 桌面标题栏 | 500 |

组件化后仍需维持这套相对层级，禁止各组件随意使用极大 z-index。

## 5. 玻璃样式

原项目至少有两套玻璃系统，不能混用。

### 内容玻璃

```css
--glass-bg: linear-gradient(112deg,
  rgba(72,74,76,.62), rgba(24,27,30,.70) 48%, rgba(8,12,14,.74));
--glass-border: rgba(0,245,212,.30);
--glass-border-soft: rgba(255,255,255,.095);
```

主要用于搜索和内容面板。聚焦状态使用更亮背景及独立 focus shadow。

### 控件玻璃

```css
background: rgba(0,0,0,.10);
backdrop-filter: blur(12px) saturate(1.8) brightness(1.16);
```

主要用于保存面板和按钮，依赖多层 inset shadow。禁止用一个简单 `rgba + blur(20px)` 取代。

## 6. 登录弹窗强制规格

登录是当前重点，Vue 版本必须以以下源码参数为准。

### 遮罩与容器

- 遮罩：`rgba(0,0,0,.78)`，叠加中心香槟色径向渐变。
- 基础弹窗背景：从 `rgba(24,23,26,.96)` 到 `rgba(12,11,12,.92)` 的 180° 渐变。
- 边框：`1px solid rgba(244,210,138,.16)`。
- 圆角：`18px`。
- padding：`32px`。
- 登录弹窗宽度：`min(470px, 92vw)`。
- 基础 shadow：`0 26px 90px rgba(0,0,0,.56)`，另有细描边和顶部内高光。
- 文本整体居中，简介卡片内部左对齐。

### 平台切换

- 外层胶囊：padding `4px`、间距 `8px`、圆角 `999px`。
- 单个按钮高 `32px`，字号 `11.5px`，字重 `700`。
- 网易云 active：背景 `rgba(217,91,103,.16)`，文字 `#ffd7dc`。
- QQ active：背景 `rgba(191,214,107,.16)`，文字 `#f3ffd1`。
- 禁止将两个平台 active 状态统一成青色。

### 产品简介区

- margin：`-6px 0 16px`。
- padding：`14px 15px`。
- 圆角：`14px`。
- kicker：`10px / 780 / .18em`，香槟色。
- title：`18px / 760 / 1.18`。
- body：`12px / 1.58`，白色 56% 透明度。
- 文案内容以原 DOM 为基准，品牌名替换可使用 Melovibe，不能删掉信息层级。

### 官方网页登录入口

- 入口容器：`200 × 200px`，圆角 `16px`，下边距 `16px`。
- 内按钮圆角：`12px`。
- 网易云使用源码中的红色系边框、文字与背景参数。
- QQ 使用源码中的黄绿色系边框、文字与背景参数。
- 标记字号：`22px`，label `10.5px`。
- 状态文本字号：`12px`，最小高度 `18px`，下边距 `14px`。
- 登录必须打开官方网页，具体后端流程遵循 `OFFICIAL_WEB_LOGIN_RESEARCH.md`。

### 底部操作

- 使用 flex 居中、间距 `10px`，登录弹窗允许 wrap。
- 普通按钮：padding `8px 20px`，字号 `12.5px`，圆角 `8px`。
- primary 使用香槟色体系，不使用全局青色 primary。
- 保留“取消”“先搜索一首歌”“我两个都要”“手动导入/网页登录”等真实状态对应入口；显示条件按原 JS 迁移。

### 动画

- 原项目模态使用 GSAP `openGsapModal` / `closeGsapModal`。
- 迁移时必须加载 `gsap-core`、`gsap-frameworks`；涉及时间轴时加载 `gsap-timeline`。
- 参数从原函数提取，禁止自行换成弹簧动画。
- Vue 卸载时必须 kill/revert。

## 7. 搜索区域规格

- 默认隐藏在顶部：`top: -76px`；桌面外壳为 `-92px`。
- peek：普通 `24px`，桌面外壳 `34px`。
- 搜索框高 `58px`，圆角 `22px`，水平 padding `20px`。
- 默认宽度 `min(520px,58vw)`；舞台模式 `min(360px,52vw)`。
- 输入字号 `13.5px`。
- 模式 tab 高 `24px`、圆角 `999px`、字号 `10.5px`。
- 搜索结果最大高度 `360px`，圆角 `14px`，滚动条宽 `3px`。

## 8. 动效规则

- 原项目已使用 GSAP 3.15.0；涉及 GSAP 必须遵循重构路线图的 skill 路由。
- CSS transition 的 duration、delay、ease 从原选择器复制。
- GSAP 的 duration、ease、stagger、overwrite 从原函数复制。
- WebGL/Three.js 动画保持渲染层实现，不改写成大量 Vue 响应式状态。
- 简单 hover 保留 CSS；可中断、编排、反向或 stagger 动画才使用 GSAP。
- 支持 `prefers-reduced-motion`，但不能删除正常模式的原动效。

## 9. 图标规范

- 优先复用原 DOM 中的 SVG path。
- SVG 保留原 viewBox、stroke/fill、尺寸和 stroke-width。
- 不使用 emoji。
- 不因组件化改成另一套图标库。
- 若原功能无图标，不新增图标。

## 10. 前端资产规则

资产登记见 `FRONTEND_ASSET_MANIFEST.md`。

- 只记录文件事实、大小、引用位置和技术用途。
- 不记录图片内容描述。
- 不根据文件名推测图片视觉内容。
- 用户上传资源只记录格式、路径、尺寸等可程序读取信息。
- 缺失资源必须报告，不得自动生成替代图片。

## 11. 组件化边界

- 组件拆分不得改变 DOM 层级、状态 class 和选择器语义。
- 第一轮允许保留原 class 名，确保参数可对照。
- 页面组件只组合；动画、账号、搜索、播放器各自归 feature/store/service。
- 不为每个小元素建立组件；有独立状态、生命周期或可复用行为时才拆。
- 一次只迁移一个视觉单元，并按路线图创建独立 Git 提交。

## 12. 验收模板

每个前端重构单元提交前填写：

```text
原 DOM 范围：
原 CSS 选择器：
原 JS 函数：
保留的状态 class：
复用的资源：
改动的参数：无 / 列明并说明用户授权
未确定项：
构建结果：
提交哈希：
```

“更现代”“更漂亮”“大致一致”不属于有效验收结论。
