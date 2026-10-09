# OpenCake

免费开源的人像精修工具：批量修图、联机实时出片、AI 模型可自由替换。定位是像素蛋糕（PixCake）的开源平替。

- **零订阅、离线可用**：不按张收费，不联网激活。
- **AI 模型槽**：人脸检测、磨皮、祛瑕、抠图、调色等能力都抽象成可替换的槽，支持本地 ONNX（CUDA / CoreML / WinML / DirectML / CPU）、ComfyUI、HTTP 三种后端。
- **联机拍摄**：自研 Rust PTP（Windows WPD、macOS ImageCaptureCore、Linux nusb、PTP/IP），热文件夹与内置 FTP 兜底，libgphoto2 作 Linux 备选。
- **底座**：计划软 fork [RapidRAW](https://github.com/CyberTimon/RapidRAW)（AGPL-3.0），新增功能放独立 crate，按月合并上游。

> 状态：早期骨架。上游 RapidRAW 代码尚未并入；当前仓库只有设计文档和 `camera-core` / `camera-ptp` / `oc-models` 三个起步 crate。

## 目录

| 路径 | 内容 |
|---|---|
| `docs/design.md` | 详细设计方案 v2.1 |
| `docs/x-t30.md` | 富士 X-T30 联机实验方案与实机测试清单 |
| `crates/camera-core` | `CameraDriver` trait、能力位、事件 |
| `crates/camera-ptp` | 原生 PTP 驱动 + `oc-ptp-probe` 调试工具 |
| `crates/oc-models` | 模型卡 / 配方解析与模型槽注册表 |
| `models/` | 模型卡 YAML（不含权重） |
| `recipes/` | 修图配方 |
| `bench/` | 相机兼容记录与性能基准 |

## 首个实验机型

富士 X-T30（一代）官方不支持联机拍摄，但固件为云台开放了 USB 静态拍摄控制。我们用它验证自研 PTP 路线，测试进度见带 `x-t30` 标签的 issue。

## 许可

AGPL-3.0-or-later。厂商相机 SDK 不进入本仓库，只通过独立进程的相机桥接入。贡献请使用 DCO 签名（`git commit -s`）。
