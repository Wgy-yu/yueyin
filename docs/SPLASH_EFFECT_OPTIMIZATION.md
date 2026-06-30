# Melovibe 启动页特效优化方案

## 目标

在保留当前 Melovibe 启动页红、青、金三色流动线场、文字分镜和 5.2 秒时间轴的基础上，提高画面精致度与运行流畅度，并增加线条随鼠标位置自然避让的交互。

本轮不改变品牌排版、窗口圆角、进入方式及启动页整体色调。

## 当前实现

- 入口组件：`src/components/SplashScreen.vue`
- WebGL：单个全屏三角形 + fragment shader
- 降级：Canvas 2D 粒子、光线与碎片
- 当前最高渲染倍率：`devicePixelRatio` 上限 1.6
- Shader 的主要成本：每像素多次调用线场函数，每次含 6 次循环、噪声、三角函数与距离场计算

## 优化要求

### 1. 自适应渲染倍率

- 初始倍率建议为 `min(devicePixelRatio, 1.25)`。
- 使用滑动平均记录最近约 60–90 帧的帧耗。
- 平均帧耗高于 19ms 时，将倍率每次降低 0.1，最低 0.85。
- 平均帧耗低于 14ms 且稳定至少 120 帧时，将倍率每次提高 0.05，最高 1.35。
- 调整倍率后重新设置 canvas 尺寸和 viewport，但不要重建 shader program。
- CSS 尺寸始终保持窗口大小，由浏览器负责平滑放大。

### 2. Shader 成本控制

- 保留红、青、金三个主通道。
- 将只在高潮阶段可见的额外 tunnel 线场按时间窗启用，避免全时间轴计算。
- 可将线场循环从 6 层降到 5 层，并重新微调强度，确保视觉密度基本不变。
- 合并重复的 `sin`、`length`、旋转和时间变量计算。
- 噪声只用于最终细微颗粒，不承担主要线条结构。
- 禁止引入额外纹理、第三方渲染库或 CPU 粒子同步。

### 3. 鼠标避让交互

新增 uniform：

```glsl
uniform vec2 uPointer;
uniform float uPointerStrength;
```

- `uPointer` 使用与 shader 中 `p` 相同的坐标系，包含宽高比修正。
- 鼠标离开窗口时，将强度平滑衰减到 0，不要瞬间消失。
- JavaScript 端对目标指针位置做阻尼插值，推荐：`current += (target - current) * 0.08`。
- 按下鼠标时可将目标强度由约 0.65 提高到 1.0，松开后恢复。
- 避让半径建议为画面短边归一化后的 0.18–0.26。

建议的 shader 变形思路：

```glsl
vec2 delta = uv - uPointer;
float distanceToPointer = length(delta);
float influence = 1.0 - smoothstep(0.05, 0.24, distanceToPointer);
vec2 direction = delta / max(distanceToPointer, 0.001);
vec2 tangent = vec2(-direction.y, direction.x);
uv += direction * influence * 0.10 * uPointerStrength;
uv += tangent * influence * 0.035 * sin(uTime * 1.6) * uPointerStrength;
```

避让必须作用在线场采样坐标 `uv` 上，而不是在最终颜色上挖透明洞。线条应沿鼠标周围弯曲、分流，再在边缘自然汇合。鼠标中心可增加极弱的金色或青色边缘高光，但不得出现明显圆环、光球或“菊花状”爆点。

### 4. 时间与生命周期

- 使用 `requestAnimationFrame` 的时间戳计算连续时间，避免累计定时器漂移。
- 页面不可见时暂停绘制；恢复后重置上一帧时间，避免动画突然跳跃。
- 组件卸载时清理 animation frame、resize、pointermove、pointerleave、pointerdown、pointerup 和 visibilitychange 监听。
- `prefers-reduced-motion` 继续使用 Canvas 2D 降级，不启用鼠标流场。

### 5. 视觉精致度

- 线条主体保持纤细，高潮时增加密度而不是单纯增加亮度。
- 红、青通道错位控制在轻微可感知范围，避免色散过重。
- 金色只作为焦点和过渡色，不应覆盖主要青红层次。
- 鼠标移动停止后，流场需在约 300–500ms 内稳定，不持续抖动。
- 交互不能影响 Melovibe 字标的可读性。

## 性能目标

- 1920×1080、普通集成显卡：启动动画尽量稳定在 55–60 FPS。
- 连续低于 50 FPS 时应自动降低渲染倍率。
- 主线程 pointer 事件不做布局读取，不产生 Vue 响应式高频更新。
- 单帧不得创建数组、对象或重新编译 shader。
- WebGL 不可用时仍能正常进入现有 Canvas 2D fallback。

## 验收清单

- Melovibe 字体动画、5 秒后可点击进入等现有行为不变。
- 鼠标靠近线条时可看到自然分流，而不是圆形遮罩。
- 鼠标离开后线场平滑恢复。
- 快速移动鼠标时无明显跳变、拖影断层或闪烁。
- 最大化、缩放窗口后指针坐标仍准确。
- 开启“减少动态效果”时不运行 WebGL 交互。
- `npm run build` 和 `cargo check` 均通过。

## 实施提示词

```text
请优化 D:\Dev\Repos_self\yueyin 项目的 Melovibe 启动页特效。

先完整阅读 docs/SPLASH_EFFECT_OPTIMIZATION.md 和 src/components/SplashScreen.vue。保持现有品牌文字、字体分镜、颜色方向、5.2 秒动画时间轴、点击进入逻辑、窗口圆角及 Canvas 2D fallback 不变。

需要完成：
1. 为 WebGL canvas 增加基于实际帧耗的自适应渲染倍率，范围约 0.85–1.35，避免高 DPI 下固定 1.6 倍渲染造成卡顿。
2. 优化 fragment shader 的重复距离场、三角函数和循环计算；保留红、青、金三通道及当前视觉层次。
3. 增加鼠标流场避让。通过 uPointer 和 uPointerStrength uniform，把鼠标坐标平滑传入 shader；线条必须在采样坐标上绕开鼠标并沿切线轻微弯折，不能用颜色遮罩挖洞，不能出现明显圆环、光球或花瓣爆点。
4. 指针位置和强度使用阻尼插值；移出窗口后平滑衰减，按下时可稍微增强。
5. 页面不可见时暂停绘制，组件卸载时清理全部监听器和 animation frame。
6. 高频指针数据不要放进 Vue 响应式状态，单帧不要创建对象或数组，不要引入新依赖。
7. 保持 prefers-reduced-motion 的现有降级行为。

完成后运行 npm run build，并在 src-tauri 目录运行 cargo check。汇报修改文件、关键性能策略、验证结果以及仍存在的视觉取舍。不要顺带修改其他页面。
```
