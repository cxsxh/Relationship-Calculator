# 关系计算器

一个使用 `Rust + eframe/egui` 开发的 Windows 原生桌面关系计算器，支持多步亲属关系输入、本地推导、常见叫法展示，以及部分方言别名展示。

## 功能特性

- 多步关系链输入，当前支持最多 `10` 步
- 自动推导标准称呼
- 展示常见叫法、方言叫法和中性称呼
- 展示关系路径和内部编码
- 全部本地计算，不联网、不依赖外部服务
- 发布版支持无控制台黑窗启动

## 项目结构

```text
src/
  app.rs                UI 界面
  models.rs             结果模型与关系枚举
  data/
    relations.rs        可选关系定义
    titles.rs           称呼规则、别名和说明
  engine/
    calculator.rs       计算入口
    resolver.rs         关系推导规则
    formatter.rs        结果格式化
tests/
  calculator_tests.rs   关键链路测试
```

## 环境要求

- 操作系统：Windows 10/11 x64
- Rust 工具链：`stable`
- 当前验证环境：
  - `cargo 1.95.0`
  - `rustc 1.95.0`
  - Host：`x86_64-pc-windows-msvc`
  - 默认目标：`x86_64-pc-windows-msvc`

如果本机没有 `MSVC` 工具链，需要先安装 Visual C++ Build Tools。

## 本地运行

```powershell
cargo run
```

## 构建发布

```powershell
cargo build --release
```

生成文件位于：

```text
target\x86_64-pc-windows-msvc\release\relationship_calculator.exe
```

## 测试

```powershell
cargo test -j 1
```

当前已验证结果：

- `12 passed; 0 failed`

## 开源交付说明

- 已清理根目录构建产物、IDE 配置和无用临时文件
- 已补充 `.gitignore`，避免提交 `target`、`.idea`、`.pdb`、日志和临时文件
- 已扫描常见敏感字段和证书/私钥文件，当前未发现明显敏感信息
- 项目当前依赖本地 `vendor` 补丁目录，以确保当前 Windows 环境可稳定构建

当前接入的本地补丁依赖共 `14` 个：

- `ahash`
- `crc32fast`
- `getrandom`
- `libc`
- `memoffset`
- `num-traits`
- `paste`
- `proc-macro2`
- `quote`
- `serde`
- `serde_core`
- `syn`
- `thiserror`
- `zerocopy`

## 提交建议

首次提交到 GitHub 前，建议执行：

```powershell
git add .
git status
cargo test -j 1
```

如果你希望仓库保持最小体积，提交前确认 `target` 未被重新生成并加入索引。
