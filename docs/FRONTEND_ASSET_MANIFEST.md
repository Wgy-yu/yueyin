# Mineradio 前端资产清单

## 规则

本清单不描述图片内容，不写图片提示词，不根据文件名推测视觉语义。只记录文件、大小、源码引用及技术用途。

## 本地运行依赖

| 源文件 | 大小 | 源码引用 | 技术用途 |
|---|---:|---|---|
| `public/vendor/three.r128.min.js` | 603445 B | `index.html:9` | Three.js r128 运行时 |
| `public/vendor/music-tempo.min.js` | 14537 B | `index.html:10`，运行时还存在动态加载路径 | 节拍分析运行时 |
| `public/vendor/music-tempo.LICENCE` | 1089 B | 随 vendor 分发 | 许可证文件 |
| `public/vendor/gsap.min.js` | 72927 B | `index.html:11` | GSAP 3.15.0 运行时 |

## 数据资产

| 源文件 | 大小 | 源码引用 | 技术用途 |
|---|---:|---|---|
| `public/assets/skull-decimation-points.bin` | 1048320 B | `index.html:6628` | 由 Three.js 代码读取的二进制点数据 |
| `public/default-user-fx-archive.json` | 2306 B | 首次启动默认视觉参数逻辑 | 默认用户视觉参数快照 |

## 字体来源

原项目通过 Google Fonts URL 加载以下字体族：

- `Cinzel Decorative`：700、900。
- `Inter`：200、300、400、500、600、700。
- `JetBrains Mono`：400、500、600、700。
- `Noto Sans SC`：400、500、700、900。
- `UnifrakturCook`：700。

迁移策略待产品决定：继续联网加载，或在获得合法字体文件后改成本地打包。不得自行下载、复制或替换字体文件。

## 代码内资产

- 界面图标主要为 `index.html` 内联 SVG path。
- 启动页噪声为 CSS data URL SVG filter。
- 多个 Three.js 材质纹理由运行时 Canvas/DataTexture 生成。
- 用户封面、背景图片和视频为运行时输入，不属于仓库静态资产。

迁移内联 SVG 时必须复制原 path、viewBox、stroke/fill 和尺寸，不重新绘制。

## Tauri 目标资产位置

建议保持：

```text
public/vendor/
public/assets/
public/default-user-fx-archive.json
```

使用 Vite `public/` 原样复制机制，避免给二进制资产增加无必要的 import loader。

## 迁移检查

- 文件哈希或字节数与源项目一致。
- 生产构建后文件仍存在于 `dist` 对应路径。
- 动态 fetch 路径不被 Vite 重命名。
- 许可证文件随依赖分发。
- 未添加未经来源确认的图片资产。
