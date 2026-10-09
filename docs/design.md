---
title: OpenCake 详细设计方案 v2.1
subtitle: 像素蛋糕（PixCake）免费开源平替 · 批量人像精修 + 联机实时修图 + AI 模型可配置
date: 2026-10-08
---

# 0. 一页结论

- **定位**：零订阅、离线可用、参数可编辑的商业人像精修工具。像素蛋糕按导出张数收费（¥0.20–0.37/张，额度一年作废不结转），需联网激活，Windows 上只支持 NVIDIA 显卡加速。这三点正是开源版的切入口。
- **市场空白**：目前没有真正开源的同类产品。Evoto（与像素蛋糕同属真景科技）、美图云修/Kumoo 都按张或订阅收费；Retouch4me、Portraiture 是买断插件，但没有联机和批量工作流；云享传靓仔只开源了外围工具，引擎和模型闭源。
- **底座**：软 fork **RapidRAW**（AGPL-3.0，约 1.04 万星，2–3 周一版）。它已覆盖约 70–80% 的非人像能力（RAW 解码、GPU 管线、蒙版、批量 CLI、macOS/Linux 联机雏形），我们新增功能放独立 crate，按月合并上游。
- **AI 模型**：所有能力抽象成"模型槽"，三种后端（本地 ONNX / ComfyUI / HTTP）。默认模型全部选 MIT/Apache/BSD 权重；非商用权重只作可选下载，并在界面上明确提示。
- **联机**：不只靠 libgphoto2。自研 Rust PTP 核心，每个系统走原生通道（Windows WPD、macOS ImageCaptureCore、Linux nusb、PTP/IP），热文件夹和内置 FTP 兜底；厂商 SDK 只放在独立进程的"相机桥"里，不进主程序、不随发行包分发。
- **性能目标**（估算，待实测）：USB 3 联机，从按快门到出 AI 修图预览，典型 ≤2.5 秒，最差 ≤4 秒。

# 1. 对标分析：像素蛋糕拆解

## 1.1 产品形态
Lightroom 式非破坏编辑器 + AI 滑块 + 批量同步 + 预设 + 联机拍摄 + 在线选片交付（像素芝士小程序）。最新 10.0 版（2026-09-10）新增 AI 点修、局部 AI 追色、宠物精修，并把手动调色、液化、修补工具的导出改为免费。

## 1.2 核心机制（开源版必须复刻）
1. **AI 滑块的统一模式**：检测人脸/人体 → 生成蒙版或变形场 → 暴露滑块 → 用户可用画笔擦回 → 同步到其他照片。同步时其他照片会**重新计算**，同步的是"意图参数"而不是像素。
2. **人物属性体系**：每张脸被分为女性、男性、儿童、老年女性、老年男性；参数可以按类别、对全部人或对单个人设置，单个人可跨照片匹配。这是批量人像能成立的关键。
3. **磨皮**：中性灰、质感、高低频、水润四种；肤质分哑光、缎面、水光；肤色有透亮、美白、红润和 AI 肤色统一。

## 1.3 联机拍摄
- 支持机型（按官方表统计）：佳能约 223 款、尼康 163、索尼 54、富士 28，另有松下、徕卡、奥林巴斯、理光、哈苏 X2D II 等少量。
- 连接方式：USB（专业/快速模式自动切换）、无线、内置 FTP。
- 实时功能：新照片沿用上一张效果或套用预设；到达时标记闭眼、曝光异常、虚焦；叠加图层与命名模板；9.2 起可在电脑上调光圈等参数。

## 1.4 计费与痛点

| 套餐 | 年费 | 张数 | 单价 |
|---|---|---|---|
| 轻享 | ¥299 | 800 | ¥0.37 |
| 臻享 | ¥3,380 | 12,000 | ¥0.28 |
| 专享 | ¥8,980 | 36,000 | ¥0.25 |
| 豪华 | ¥17,800 | 90,000 | ¥0.20 |

用户主要抱怨：限量限时计费、必须联网、出片千篇一律、滑块拉满仍不够又没有图层做细修、AI 液化弱。AIGC 另扣点数，第三方工具包 ¥45–399/年。

# 2. 产品范围与优先级

**P0（MVP，必须有）**

- 项目/图库管理，Lightroom 式星级、旗标快捷键
- RAW 解码与基础调色（GPU）
- 人脸/人体检测层 + 人物属性体系（按类/全部/单人）
- 核心磨皮（频率分离 + AI 匀肤）、一键祛瑕
- 核心液化（五官、脸型、身形），带背景保护
- 影棚背景清理、证件照换底
- 局部调色蒙版（皮肤、头发、衣服、天空、背景）
- 预设 + 批量同步（相对/绝对/按人脸三种语义）
- 联机：热文件夹、内置 FTP、Windows/Linux 原生 PTP
- 实时修图队列 + 第二屏客户视图
- 分层 PSD/TIFF 导出
- 可替换 AI 模型（模型槽）

**P1**：AI 选片（闭眼/虚焦/美学）、AI 追色与色彩风格、更多五官工具（眼睛、牙齿、头发）、换天空、实时取景、macOS 原生联机、局域网 iPad 选片、厂商 SDK 桥、对话式修图助手。

**P2**：ComfyUI 生成式工具（扩图、换装、布景）、点修、合影换表情、妆容、宠物。

# 3. 总体架构

```
┌──────────────── 前端 React 19 (RapidRAW UI + src/portrait/ 新面板) ────────────────┐
│  图库 · 精修 · 人物面板 · 批量同步 · 联机会话 · 模型库 · 配方编辑 · 客户视图窗口   │
└──────────────┬──────────────────────────────────────────────────┬──────────────────┘
               │ Tauri 命令 / Channel(Raw) / 自定义协议 liveview://│
┌──────────────▼──────────────── Rust 核心 (AGPL) ──────────────────▼──────────────────┐
│ oc-schema   强类型 sidecar：faces[]、recipe、session，JSON Schema + 版本迁移       │
│ oc-portrait 人脸/人体分析缓存、按人参数、美型参数→位移场、部件蒙版                   │
│ oc-gpu-ext  WGSL：网格液化、频率分离磨皮、多通道部件蒙版、取景 LUT 预览              │
│ oc-pipeline 调度：交互 > 联机 > 批量；GPU/显存预算                                   │
│ oc-batch    同步语义、追色、任务持久化、CLI                                          │
│ oc-tether   CameraDriver trait、ingest 队列(SQLite)、liveview、remote-view(axum)    │
│ 上游：rawler(LGPL) RAW 解码 · wgpu uber-shader · 蒙版 · 导出                         │
└───────┬─────────────────────────────┬─────────────────────────────┬────────────────┘
        │ oc-models (MIT/Apache SDK)  │ 相机桥协议 (本地 socket)    │
   ┌────▼─────┬──────────┬──────┐  ┌──▼────────────────────────┐
   │OnnxLocal │ ComfyUI  │ Http │  │ bridge-sony / canon / nikon│ (独立进程, 用户自建)
   └──────────┴──────────┴──────┘  └────────────────────────────┘
```

## 3.1 技术栈
- 保留：Rust 2024、Tauri 2、wgpu（上游锁 29，因 Apple P3 色偏）、WGSL、React 19 + zustand + konva、rawler、ort（load-dynamic）、rayon、tokio。
- 新增：notify（热文件夹）、windows crate（WPD/PTP）、nusb、mtp-rs（PTP 基础，Apache-2.0）、schemars、jsonrpsee 或 tonic（桥协议）、keyring（API 密钥）、axum（局域网选片）、SQLite（导入队列与图库）。
- RAW：主用 rawler，LibRaw 作可选回退（Rust 绑定需自维护）。

## 3.2 RapidRAW 现状与改造点

| 项 | RapidRAW 现状 | 改造 |
|---|---|---|
| 代码结构 | 单 crate，若干 100 KB 级大文件 | 新功能全部放 `crates/oc-*`，对上游文件只加挂钩 |
| GPU 管线 | 一个 76 KB uber-shader，2048 px 分块，16-bit 浮点 | 前插"几何/液化 pass"，后插"人像局部 pass"；中期参考 vkdt 改成节点图 |
| 液化 | CPU 计算，存成 8-bit 像素补丁 | 重写为 GPU 网格变形，sidecar 只存参数（控制点/位移场） |
| 修饰笔刷 | CPU 平滑，存像素 | 重写为 GPU 频率分离，参数化存储 |
| 蒙版 | 加减交组合 + AI 主体/天空/前景/深度，上限 32 个 | 新增人脸部件、人体部件蒙版类型，突破 32 上限 |
| AI | ort rc.10，未开 GPU EP（全 CPU），模型 URL 写死 | 换成 oc-models 注册表，启用 GPU EP，作为对上游的 PR 切入点 |
| 联机 | macOS/Linux，gphoto2，有取景和自动套预设 | 换成 CameraDriver 抽象；补上机身快门新文件入库（读代码推断上游缺失） |
| 批量 | 已有 headless CLI | 增加按人脸同步、追色 |
| 云服务 | 计划中的付费云（Clerk 登录） | fork 中移除 |

# 4. 人像精修引擎设计

## 4.1 分析层（每张图一次，结果缓存）
1. 人脸检测（YuNet）→ 关键点（MediaPipe FaceMesh-V2 + blendshapes；先按两眼连线把脸转正，避免侧倾超过 8° 时精度下降）
2. 人脸/人体解析：MediaPipe Selfie Multiclass（头发、脸部皮肤、身体皮肤、衣服）；BiSeNet 作可选的五官细分
3. 人体骨骼：RTMPose / DWPose
4. 人脸特征向量（SFace）→ 跨照片同一个人聚类
5. 属性分类（性别/年龄段）→ 人物属性体系
6. 质量分：闭眼（blendshape）、清晰度（拉普拉斯方差）、曝光

分析结果写入 sidecar 的 `faces[]`：`{id, person_id, bbox, landmarks, attrs, params_override}`。

## 4.2 磨皮（中性灰效果的开源实现）
- **基线（纯 shader，可实时）**：引导滤波做频率分离 → 低频层用皮肤蒙版加权平滑 → 高频层保留纹理并可调"质感"强度 → 叠加 dodge & burn 图修光影。
- **AI 层**：阿里 ABPN 皮肤模型（ModelScope `damo/cv_unet_skin-retouching`，Apache-2.0，约 490 MB），输出匀肤结果和瑕疵分割；需手动拆成 ONNX（原管线混用 PyTorch 和 TensorFlow）。
- **合成**：AI 结果作为 blend_layer，按滑块强度混合到基线上，用户可用画笔擦回。四种磨皮风格 = 基线参数 + AI 强度的不同预设组合。
- **长期**：用 CRHD-3K 的思路（Unsplash 可商用照片）发起社区成对数据集，训练自有的 dodge/burn 预测网络。FFHQR、PPR10K、CelebAMask-HQ、FFHQ 均有非商用限制，不用于默认模型训练。

## 4.3 祛瑕
瑕疵分割（ABPN 自带）→ 掩膜膨胀 → LaMa / MI-GAN 修补（均 Apache/MIT）→ 只在皮肤蒙版内生效，痣可设白名单。

## 4.4 液化与美型
- GPU 网格变形（MLS 移动最小二乘），控制点由关键点和骨骼点驱动。
- 五官：瘦脸、下巴、额头、眼睛大小与位置、鼻翼、嘴型；身形：瘦腰、长腿、瘦臂、肩宽。
- **背景保护**：变形场乘以（1 − 背景直线/门框检测权重），并用人体分割限制作用区域，解决"墙歪了"的老问题。
- sidecar 只存语义参数（如 `slim_face: 0.3`），不同照片重新计算位移场，这才能批量同步。

## 4.5 调色与追色
- 自动调色：Image-Adaptive-3DLUT（作者称 4K 下不到 2 ms）/ AdaInt / NILUT。
- AI 追色：样片和客片分别做语义分区（皮肤、头发、天空、背景、衣服），每区做统计色彩迁移，最后生成一个 3D LUT + 区域蒙版的组合，可编辑、可同步。

## 4.6 背景与其他
- 抠图：BiRefNet（MIT，高精度）；MODNet 用于实时预览。
- 交互选区：SAM 2.1（Apache-2.0）。
- 天空：skyseg（MIT）。
- 超分：Real-ESRGAN（BSD-3）。
- 面部修复默认 RestoreFormer++（Apache-2.0，需先打通 ONNX 导出）；GFPGAN 有非商用部分、CodeFormer 为 S-Lab 非商用，均只作可选。

## 4.7 批量同步语义
- **相对**：在每张图原有参数上加减（适合曝光不一致的组图）。
- **绝对**：直接覆盖。
- **按人**：参数绑定到人物类别或单个 person_id，跨照片自动匹配。
- 同步后进入低优先级队列重算，界面上先显示缩略预览。

# 5. AI 模型槽系统

## 5.1 槽与契约

| 槽 | 必须输出 | 默认模型 | 许可 |
|---|---|---|---|
| face_detect | 框 + 5 点 | YuNet | MIT |
| face_landmark | 关键点 (+blendshapes) | MediaPipe FaceMesh-V2 | Apache-2.0（仅 TFLite，需转换） |
| face_parse / human_parse | 多类蒙版 | MediaPipe Selfie Multiclass | Apache-2.0 |
| body_pose | 关键点 | RTMPose / DWPose | Apache-2.0 |
| skin_retouch | 混合层 | ABPN | Apache-2.0 |
| blemish | 蒙版 | ABPN 瑕疵分支 | Apache-2.0 |
| inpaint | 图像 | LaMa / MI-GAN | Apache-2.0 / MIT |
| matting | alpha | BiRefNet / MODNet | MIT / Apache-2.0 |
| interactive_select | 蒙版 | SAM 2.1 | Apache-2.0 |
| sky_seg | 蒙版 | skyseg | MIT |
| upscale | 图像 | Real-ESRGAN | BSD-3 |
| face_restore | 图像 | RestoreFormer++ | Apache-2.0 |
| color_auto / color_match | 3D LUT | Image-Adaptive-3DLUT 等 | 见模型卡 |
| face_embed | 归一化向量 | OpenCV Zoo SFace | Apache-2.0 |
| cull | 分数 | blendshapes + 清晰度 | Apache-2.0 |
| aigc | 图像 | FLUX.2 [klein] 4B、Qwen-Image-Edit | Apache-2.0（经 ComfyUI） |

**需避开作为默认的**：RMBG-2.0（权重 CC BY-NC）、InsightFace 全系（含 SCRFD、buffalo_l，非商用）、EdgeFace（CC BY-NC-SA）、MAT（CC BY-NC）、CodeFormer、FLUX dev 系列、BeautyGRPO（基于 FLUX Kontext dev）。

## 5.2 许可分级（写进模型卡并在界面提示）
`permissive`（MIT/Apache/BSD）→ `copyleft` → `openrail` → `data-nc`（权重宽松但训练数据非商用）→ `non-commercial`。后两级下载前需用户确认。注：模型权重是数据，不与 AGPL 代码"链接"，但其自身许可仍约束使用者。

## 5.3 模型卡 `model-card.yaml`（节选）

```yaml
schema: opencake/model-card@1
id: birefnet-portrait-fp16
slot: matting
license: { code: MIT, weights: MIT, tier: permissive, requires_ack: false }
io:
  inputs:  [{ name: input_image, dtype: float16, layout: NCHW, shape: [1,3,1024,1024],
              color: RGB, range: [0,1], normalize: { mean: [0.485,0.456,0.406], std: [0.229,0.224,0.225] } }]
  outputs: [{ name: output, kind: alpha, activation: sigmoid }]
preprocess:  { roi: person_bbox, resize: { mode: letterbox } }
postprocess: { resize_back: bilinear, refine: guided_filter }
runtime:
  backend: local-onnx
  precision: [fp16, fp32]
  ep_preference: [cuda, coreml, winml, dml, cpu]
  vram_mb: { fp16: 3500 }
  static_shapes: true
performance: [{ device: "RTX 3060 12G", ep: cuda, latency_ms: null }]   # 社区实测回填
ui: { display_name_zh: 人像抠图（高精度）, default_for_slot: true }
source: { files: [{ name: model.onnx, sha256: "...", mirrors: [huggingface, modelscope] }] }
```

## 5.4 配方 `recipe.yaml`（节选）

```yaml
schema: opencake/recipe@1
id: studio-portrait-female
steps:
  - slot: face_detect
  - slot: face_landmark
  - slot: face_parse
  - slot: skin_retouch    { strength: 0.6, texture: 0.4, style: neutral-grey }
  - slot: blemish         { then: inpaint, keep_moles: true }
  - op: liquify           { slim_face: 0.25, eye_size: 0.1, protect_background: true }
  - slot: color_match     { reference: refs/sample-01.jpg, regions: [skin, hair, background] }
apply_to: { person_class: [female, older_female] }
fallback: { skin_retouch: shader-baseline }      # 模型缺失或显存不足时的降级
```

## 5.5 三种后端
1. **OnnxLocal**：ort（锁定 rc 版本，目前 rc.13）；EP 顺序 CUDA → CoreML → WinML → DirectML → CPU，用 `error_on_failure` 探测避免静默回落 CPU；首次运行与 CPU 输出比对精度；会话池 + 显存预算，空闲卸载。注意：DirectML 已进入仅维护状态、opset 上限 20、不支持 DeformConv（BiRefNet 在 DML 上需实测）；WinML 要求 Win11 24H2；CUDA 需 ≥13.2，**不随包分发 CUDA**，加载用户已安装的。
2. **ComfyUI**（GPL-3.0，独立进程）：`/prompt` + websocket；沿用 RapidRAW AI-Connector 的优化（整图只传一次，之后只传蒙版和提示词）；工作流的输入输出节点映射写在模型卡里。
3. **Http**：OpenAI 兼容图像接口 + 自定义 REST 模板（URL、headers、请求体模板、返回解析路径），密钥存系统钥匙串。

# 6. 联机拍摄与实时修图

## 6.1 驱动分层

| 驱动 | 平台 | 说明 |
|---|---|---|
| 热文件夹 | 全平台 | notify 监听 + 去抖 + 写入完成判定；兼容任何厂商软件 |
| 内置 FTP | 全平台 | 索尼 α7 IV/α7 V、佳能 R5/R1 等可每拍自动上传 |
| ptp-native | Win (WPD) / Linux (nusb) / macOS (ImageCaptureCore) | 自研 Rust PTP 核心；Windows 无需换驱动，EDSDK 本身也走 WPD |
| PTP/IP | 全平台 | TCP 15740，Wi-Fi/以太网 |
| gphoto2 | Linux 默认，macOS 可选 | LGPL 动态链接；Windows 需 Zadig 换 WinUSB，会让其他软件看不到相机，不作默认 |
| 厂商桥 | 视 SDK | 独立进程，MIT，用户自备 SDK 自行构建 |

**为什么不以 libgphoto2 为主**：官方 README 写明不支持 Windows；macOS 13+ 的 ptpcamerad 会抢占 USB 设备且被杀后立刻重启；2020 年后的新机身不在官方远控列表；低端富士（X-A、X-E、X-T10/20/30）无法远控。

## 6.2 驱动接口（示意）
`CameraDriver::enumerate/open` → `CameraSession`：`caps()`（能力位驱动 UI 灰显）、`properties/set_property`（统一键 + 厂商原始值透传）、`capture`、`start_liveview`（帧流）、`download`、`events`（新对象/属性变化/断开/卡满）。佳能需轮询 GetEvent，尼康索尼有事件推送，统一转成 CameraEvent。驱动优先级：厂商桥（用户启用时）> ptp-native > gphoto2，可手动切换；同一时刻一台相机只被一个驱动打开。

## 6.3 导入流水线（SQLite 持久化，崩溃可恢复）
```
检测 → 下载(.part) → 校验 → 提交(重命名+fsync，可双写备份) → 索引(EXIF/序号/会话)
 → 提取内嵌 JPEG → AI 预览(会话配方, 预览分辨率) → 显示/推送第二屏
 → 全分辨率渲染(低优先级) → 自动导出(可选)
```
每段一个有界 channel 做背压；**下载段永远不阻塞**（相机缓存满了就拍不了）。断线重连后对比相机存储补拉缺失文件（参考 Capture One 的 ReTether）。会话含命名模板、绑定配方、"下一张覆盖参数"、双写目录。

## 6.4 延迟预算（估算，待实测）

| 阶段 | 参考数据 |
|---|---|
| 传输（42 MP，约 40 MB） | USB-C 1.8 s，USB 2 5.9 s；连拍约 0.8 s/张 |
| 提取内嵌 JPEG | 1–9 ms |
| AI 预览（检测+磨皮+液化+调色，预览尺寸） | 目标 300–600 ms |
| 推送显示 | <50 ms |
| **合计** | 典型 ≤2.5 s，最差 ≤4 s（USB 3） |

## 6.5 实时取景
- libgphoto2 实测参考：旧款索尼 A7R II 2–3 fps，佳能 EOS R 约 40 fps，尼康 D800 60 fps（640×360）；帧宽多为 640–1024 px。
- 推流：默认自定义协议 `liveview://` 推 MJPEG（相机原生 JPEG，零重编码），前端 canvas 叠加对焦框/网格/人脸框；备选 Tauri Channel 传 Raw ArrayBuffer。不用事件/JSON/base64。Rust 侧用单槽 watch channel，只保留最新一帧。三个 webview 内核对 multipart 的支持需实测。
- 取景上的实时美颜不是刚需，v1 只做 LUT 风格预览。

## 6.6 第二屏与局域网选片
- 本机：Tauri 多窗口，扩展屏全屏"客户视图"，自动显示最新修图结果。
- 局域网：内嵌 axum + WebSocket 的 PWA，iPad/手机浏览器直接打开；星级、旗标、喜欢、评论、选片计数；默认关闭，开启后一次性二维码配对，只监听局域网网卡。这是对标像素芝士小程序、但不走云端的替代。


## 6.7 首个实验机型：富士 X-T30（一代）
- 官方不支持联机（Capture One 也标注不支持），libgphoto2 #509 拍摄报错并被标为相机限制。但其固件为云台/无人机开放了经 USB 的静态拍摄控制，只在"USB 视频拍摄 自动/固定"模式下暴露，开源项目大概率没在这个模式下测过。
- **实验三级**：①各连接模式下枚举 PTP DeviceInfo；②在云台模式下触发快门（标准 InitiateCapture → 富士厂商码 → 云台抓包重放）；③监听对象事件取回 RAF/JPEG（"固定"模式下照片不存卡，图像可能留在相机缓存待主机取走，重点验证）。
- **降级**：D1 读卡器/读卡器模式 + 热文件夹（保底，M2 前先做到插卡即出片）；D2 PC AutoSave Wi-Fi 传到热文件夹；D3 蓝牙自动传手机，仅 JPEG 用于现场预览；D4 第三方 Wi-Fi SD 卡；D5 HDMI 采集卡补实时取景。
- 不依赖富士 SDK（其许可禁止 GPL/LGPL）。详细方案与实机测试清单见 `x-t30-experiment.md`。

# 7. 许可与合规

- **主程序必须保持 AGPL-3.0**（上游无 CLA，无法给他人代码加例外）。
- **厂商 SDK 一律不进主进程**：在独立"相机桥"进程里加载，桥本身 MIT、独立仓库、不含 SDK 文件，与主程序走本地 socket + 公开协议（不用共享内存，GPL FAQ 视共享内存近似链接）。各 SDK 要点：富士 §3.9 明确禁止置于 GPL/LGPL 下；索尼 CrSDK 只能"不可分离地"并入应用且须告知用户相机保修失效（启用前加同意页）；佳能、尼康、松下现行条款需注册后阅读，待法务确认。
- 索尼 Camera Remote Command 是 PTP 协议文档，自研实现可开源（推断）；旧 PTP2 命令集据称部分机型 2027 起可能失效，直接按 PTP3 做。
- 新写的通用 crate（模型运行时、相机协议）可用 MIT/Apache 双许可，前提是不复制 RapidRAW 代码；贡献流程用 DCO 签名。
- AGPL 网络条款仅在将来提供托管服务时触发。
- **两份调研的分歧**：联机调研建议给 AGPL 代码加 GPLv3 §7 附加许可（需 CLA）；底座调研认为独立进程 + 协议通信即可不加例外。本方案采用后者，因为它不依赖上游授权，**请律师最终确认**。

# 8. 仓库结构

```
opencake/                    主仓库 AGPL-3.0（RapidRAW 软 fork）
├─ src/ + src/portrait/      前端
├─ src-tauri/                上游 app crate，只加挂钩
├─ crates/oc-schema | oc-portrait | oc-gpu-ext | oc-pipeline | oc-batch | oc-tether
│        camera-core | camera-ptp(transport-wpd/nusb/icc/ptpip) | camera-gphoto2
│        camera-hotfolder | camera-ftp | camera-bridge | ingest | liveview | remote-view
├─ models/                   模型卡 YAML（不含权重，含国内镜像）
├─ recipes/                  婚纱、证件照、写真、儿童……
└─ docs/ bench/

opencake-sdk/                独立仓库 MIT OR Apache-2.0
├─ oc-models/                模型槽运行时
├─ oc-tether-proto/          相机桥协议 + Rust/Python 客户端
└─ bridges/ sony-crsdk | canon-edsdk | nikon | fuji | lumix
```

# 9. 路线图

| 阶段 | 周期（估） | 交付 |
|---|---|---|
| M0 底座 | 4–6 周 | fork + 去云服务；oc-models 注册表替换写死模型；开 GPU EP；sidecar 强类型化；基准测试 |
| M1 人像核心 | 8–10 周 | 分析层 + 人物属性；GPU 频率分离磨皮 + ABPN；祛瑕；GPU 网格液化；按人批量同步；分层导出 |
| M2 联机 v1 | 6–8 周 | 热文件夹 + 内置 FTP + 导入队列 + AI 预览管线 + 第二屏；Windows WPD 与 Linux nusb PTP（佳能 EOS、尼康 Z 拍摄/下载/取景） |
| M3 扩展 | 8 周 | 追色、AI 选片、换天空、证件照；macOS ImageCaptureCore；索尼 PTP3、富士；PTP/IP；iPad 选片 |
| M4 生态 | 持续 | 厂商桥、ComfyUI/HTTP 后端与 AIGC 配方、配方市场、社区磨皮数据集 |

M2 结束时评估上游合并成本，过高则冻结在某个上游版本彻底分叉。

# 10. 风险与待验证

| 风险 | 缓解 |
|---|---|
| 磨皮质感追不上像素蛋糕 | 基线 shader + ABPN 先可用；社区成对数据集长期训练 |
| 上游大文件导致合并痛苦 | 新代码放独立 crate；月度合并 + bench 回归；M2 检查点 |
| ort 仍是 RC、DirectML 停更 | 锁版本；EP 探测 + 黑名单 + 精度比对 |
| 厂商 SDK 许可 | 独立进程桥 + 法务审查；优先自研 PTP |
| 新机身联机兼容 | 社区相机兼容矩阵；每品牌 2–3 台实机测试 |
| 依赖单一上游作者 | 2–3 名维护者；通用能力回馈上游 |

**待验证**：RTX 3060 / M1 实测各模型和全管线速度；RapidRAW 机身快门新文件不入库（读代码推断）；ABPN、MediaPipe、RestoreFormer++ 的 ONNX 导出；佳能/尼康/松下现行 SDK 条款；webview 对 MJPEG 的支持；部分模型与数据集许可（RetouchFormer、SegFace、AdaFace、SAM 3 等）。

# 11. 社区策略
- README 首屏写明"基于 RapidRAW 的人像向下游发行版"，通用能力（GPU EP、模型注册表、Windows 联机）优先提 PR 给上游，定位互补：上游走付费云，我们坚持零订阅本地优先。
- 中文影楼社区优先：中文文档、ModelScope/HF 国内镜像、B 站/小红书教程、配方分享（纯 YAML，无许可负担）。
- 低门槛贡献：模型卡、配方、相机兼容报告、翻译都是 good first issue。

# 12. 首选测试素材：X-T30 RAF（X-Trans IV，26.1MP）
- **选它的理由**：X-Trans 去马赛克是 RapidRAW 当前最弱环节（社区反馈不如 darktable/RawTherapee 的 Markesteijn 3-pass，后者 GPLv3 可移植进 AGPL）；体量适中，适合 RTX 3060/M1 基准；用户可持续产出人像和风光素材。
- **RAW 管线测试**：各压缩方式解码正确性；去马赛克质量对比（锐度、彩噪、树叶伪色、皮肤纹理）；解码与内嵌 JPEG 提取耗时；**预览一致性**——内嵌 JPEG 带胶片模拟色彩，换成 RAW 渲染会跳色，需要"胶片模拟近似 LUT"或半尺寸 RAW 快速渲染，两方案用此素材对比。
- **AI 槽测试**：X-Trans 伪影对检测/关键点/解析的影响（RAW 渲染 vs 机内 JPEG）；磨皮是否把彩噪当瑕疵或放大"蠕虫纹"；抠图发丝与天空分割；各槽耗时回填模型卡。
- 人像素材需被摄者书面授权才可用于公开基准或训练。

---
*详细调研附录（含全部来源链接）：research/01-pixcake-teardown.md、02-ai-models.md、03-tethering.md、04-codebase.md；X-T30 实验与测试清单：x-t30-experiment.md。性能数字除注明出处外均为估算。*
