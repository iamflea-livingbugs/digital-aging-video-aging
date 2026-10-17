# 数字旧化 | 视频旧化 Digital aging – Video aging — ntsc-rs 中文汉化 + PAL 分支

> 基于 [ntsc-rs/ntsc-rs](https://github.com/ntsc-rs/ntsc-rs) 0.9.6 的独立下游分支，在上游 NTSC/VHS 复合视频信号仿真器之上做了两件事：
> 1. **简体中文界面汉化**——显示层查找表方案，预设/配置键名/保存格式与上游字节级兼容（上游明确无 i18n 计划，issue #451）；
> 2. **PAL 视频制式模拟**——新增「视频制式」设置项，复刻 PAL「逐行倒相 + 行间平均」的视觉特征（路线 A：模拟观众可见特征，不严格复刻物理参数）。

---

## 一、这是什么

一个把 NTSC/VHS 复合视频信号仿真器 [ntsc-rs](https://ntsc.rs) 的图形界面汉化成中文、并新增 PAL 制式模拟的独立分支。

- **汉化只翻译 GUI 显示文本**（菜单、设置项、提示、对话框、渲染卡片），未收录词条原样保留，预设名/文件名/路径/错误信息绝不会被改写
- **PAL 是新增能力而非改写**：只新增 1 个设置项（`video_standard`，id=62），默认值为 NTSC；旧预设 JSON 缺该字段时自动落回 NTSC，上游预设 100% 可导入
- **NTSC 行为不变**：选 NTSC 时算法路径与上游完全一致（有测试验证干净信号往返保真度两制式相同）

---

## 二、人类怎么操作

### 1. 前提环境（一次性准备）

| 依赖 | 版本 | 说明 |
|------|------|------|
| Visual Studio | 2026 Community（`C:\Program Files\Microsoft Visual Studio\18\Community`） | MSVC x64 工具链，编译 Rust 需要 |
| Rust 工具链 | 1.97.1（`C:\Users\Administrator\.cargo\bin`） | rustup 安装 |
| GStreamer | 1.26.11（`D:\_Software\OpenSource\gstreamer-msvc-1.26.11`） | 官方 MSI 解包（`msiexec /a`），非系统安装 |

### 2. 构建汉化版

在 PowerShell 中，一条命令完成环境设置 + 编译（建议存成 `build.ps1`）：

```powershell
# 1) 加载 VS 2026 开发环境（x64）
Import-Module "C:\Program Files\Microsoft Visual Studio\18\Community\Common7\Tools\Microsoft.VisualStudio.DevShell.dll"
Enter-VsDevShell -VsInstallPath "C:\Program Files\Microsoft Visual Studio\18\Community" -SkipAutomaticLocation -DevCmdArguments "-arch=x64 -no_logo" | Out-Null

# 2) 设置 GStreamer 环境（pkg-config 依赖）
$env:GSTREAMER_1_0_ROOT_MSVC_X86_64 = "D:\_Software\OpenSource\gstreamer-msvc-1.26.11\PFiles64\gstreamer\1.0\msvc_x86_64"
$env:PKG_CONFIG_PATH = "$env:GSTREAMER_1_0_ROOT_MSVC_X86_64\lib\pkgconfig"
$env:Path = "C:\Users\Administrator\.cargo\bin;$env:GSTREAMER_1_0_ROOT_MSVC_X86_64\bin;$env:Path"

# 3) 打包（产物在 build\ntsc-rs-windows-standalone\，自包含可直接分发）
cd D:\_Program\MyOpenSource\NTSC-loc
cargo xtask windows-bundle --release
```

> ⚠️ 每次新开终端都要重新执行上面的环境设置，终端不会自动记住。
> ⚠️ 必须先加载 VS DevShell 再编译，否则会撞上 Git 自带的 `link.exe`（链接阶段报错）。

> 💡 `cargo xtask windows-bundle` 会自动完成：编译 GUI → 把 GStreamer 全部 DLL 和插件按官方目录结构拷进发布文件夹。本机实测约 5 分钟（增量编译后更快），产物约 300 MB（含完整 GStreamer；上游 CI 用精简组件约 150 MB）。

### 3. 运行

**双击发布文件夹里的 `ntsc-rs-launcher.exe`**（或 `bin\ntsc-rs-standalone.exe`）：

```
build\ntsc-rs-windows-standalone\
├── ntsc-rs-launcher.exe      ← 双击这个
├── bin\
│   ├── ntsc-rs-standalone.exe
│   ├── ntsc-rs-cli.exe
│   └── *.dll                 ← GStreamer 核心 DLL 与 exe 同目录
├── lib\gstreamer\            ← GStreamer 插件
└── licenses\
```

直接拖入视频文件即可加载（也支持「文件 → 打开」）。界面即中文。

> ❗**不要直接双击 `target\release\ntsc-rs-standalone.exe`**：那个裸 exe 依赖 GStreamer DLL，只有在配置了 GStreamer PATH 的终端里才能跑，直接双击会报「找不到 gstpbutils-1.0-0.dll」。要分发或日常使用，请用上面 `cargo xtask windows-bundle` 打出来的自包含文件夹（整个文件夹一起拷走，不能只拷 exe）。

### 4. 验证翻译

```powershell
cargo test -p ntsc-rs-gui --lib translations
```

应输出 3 个测试全部 `ok`（`translates_gui_and_parameter_labels`、`preserves_unknown_text_and_user_content`、`settings_descriptors_are_covered`）。

---

## 三、我做了什么

### 改动文件清单

| 文件 | 改动 |
|------|------|
| `crates/gui/src/app/translations.rs` | **新增核心**：`tr()` 查找表，约 240 个词条，8 组分类注释，3 个单元测试 |
| `crates/gui/src/app/mod.rs` | 注册 `pub mod translations;` |
| `crates/gui/src/app/main.rs` | 全部 GUI 渲染处用 `tr()` 包裹：顶部菜单、主题/缩放、左标签页、预设面板、渲染设置、视频面板、about 对话框、图像序列对话框、错误屏幕 |
| `crates/gui/src/app/presets.rs` | 预设面板 8 处按钮/提示翻译（保存/删除/重命名/覆盖/另存为/重新加载/打开文件夹/错误提示） |
| `crates/gui/src/app/update_dialog.rs` | 更新检查对话框（窗口标题/加载中/已是最新/下载/错误/最新版本号） |
| `crates/gui/src/app/license_dialog.rs` | 许可证窗口标题 |
| `crates/gui/src/app/third_party_licenses_dialog.rs` | 第三方许可证窗口标题 + "使用方：" |
| `crates/gui/src/widgets/render_job.rs` | 渲染任务卡片（等待中/渲染中/已暂停/用时/错误/剩余时间/时间单位/打开所在文件夹） |
| `crates/ntscrs/src/settings/standard.rs` | **PAL 新增**：`VideoStandard` 枚举（NTSC/PAL）、`NtscEffect.video_standard` 字段、setting_id 62「视频制式」描述符 |
| `crates/ntscrs/src/ntsc.rs` | **PAL 核心**：`EffectCtx.pal` 标志、`pal_v_switch()` 逐行倒相（调制/解调/相位误差/相位噪声四处 Q 分量符号交替）、PAL 强制行间平均 |
| `crates/ntscrs/src/settings/easy.rs` | 简易模式固定 NTSC（新字段初始化） |
| `crates/ntscrs/tests/pal.rs` | **PAL 新增**：3 个行为测试（PAL≠NTSC 输出、相位误差表现为降饱和而非偏色、干净信号往返保真度与 NTSC 一致） |

### 覆盖范围

- **全部**标准模式设置项（62 个 setting_id，含 PAL 新增的「视频制式」）的标签、枚举选项、悬停描述
- 顶部菜单 5 项 + 全部子菜单、左侧「效果/渲染」标签页
- 渲染设置（编码器/位深/质量/压缩级别/隔行输出等）
- 视频面板（保存帧/复制帧/静音/分屏/缩放预览等）
- 4 个对话框（关于/许可证/第三方许可/更新检查）
- 渲染任务卡片全部状态文本

### 刻意保留英文的

- 技术术语：NTSC、VHS、H.264、FFV1、PNG、SP/LP/EP（磁带速度档）
- CLI 工具（ntsc-rs-cli）——命令行界面保持英文，技术工作流习惯
- 错误信息（Rust snafu 生成的底层错误）——保留原文便于搜索/上报
- 作者名、项目名、超链接（valadaptive / zhuker/ntsc 等）

### 验证结果（本机实测）

```
cargo test -p ntsc-rs-gui --lib          → 82 passed; 0 failed
cargo test -p ntsc-rs-gui --lib translations → 3 passed; 0 failed
cargo build --release --bin ntsc-rs-standalone → 1m 10s，零警告零错误（裸 exe：26.1 MB，不能脱离 GStreamer 单独运行）
cargo xtask windows-bundle --release     → 4m 53s，自包含发布文件夹（543 个 DLL，301.6 MB）
启动验证：干净环境下双击 bin\ntsc-rs-standalone.exe → 进程正常存活，无缺 DLL 报错
```

---

## 四、怎么实现的 / 结构如何（从工程视角）

### 设计决策：为什么是「显示层查找表」

上游无 i18n 基建，全部 UI 字符串硬编码在 Rust 源码里。给整个项目引入 egui 的 i18n 框架（`Translations` trait）成本高且侵入数据流；而**查找表**只需要在「字符串进入渲染函数的那一刻」拦截一次：

```
源代码字符串 ──→ tr(text) ──→ 渲染
                ┌───────┐
                │ 查表命中 → 返回中文
                │ 未命中  → 原样返回
                └───────┘
```

**为什么不改数据层**：ntsc-rs 的预设（`.json`）、配置文件、`SettingDescriptor` 的 `id` 都是**序列化契约**。一旦改键名，上游预设导入即坏、跨版本存档即坏。汉化只碰 `label`/`description` 这些「只进渲染函数、不进序列化」的字段，用 `tr()` 包一层即可——**数据层一行没动，上游预设 100% 兼容**。这是「模式正确 > 短期可用」的取舍：以后上游升级，我们只需 diff 新增字符串，而不是迁移数据。

### `tr()` 的签名为什么是 `&str -> &str`

```rust
pub fn tr(text: &str) -> &str { match text { "File" => "文件", ..., _ => text } }
```

返回 `&str` 而非 `String`：零拷贝，不分配堆内存，渲染热路径零开销；借用的生命周期绑定输入，查不到时直接返回原引用。这是 Rust 惯用做法（类似 `CStr::to_str` 的借用传播）。

### 翻译表的结构

`translations.rs` 按 UI 区域分 8 组注释：
顶部菜单 → 主题/缩放 → 左侧标签页 → 预设操作 → 渲染设置 → 视频面板 → 对话框 → 渲染任务卡片 → 效果设置参数 → 效果设置描述。

每组对应源码里的一个文件/函数区域，改的时候按组找，不用全表扫。

### 三个必须知道的坑（都已处理）

1. **多行字符串的键必须逐字符一致**。源码里带 `\` 续行符的字符串（如设置描述），`tr()` 的键也必须用同样的 `\` 续行写法，Rust 才会折叠出相同的字符串。否则查表永远 miss、界面显示英文。
2. **同名不同义的词必须拆键**。`"Light"` 在主题菜单是「浅色」，在色度低通滤镜档位是「轻度」。match 按顺序匹配，后者永远够不到。解决：表内保留滤镜义 `"Light" → "轻度"`，主题菜单调用点改用上下文键 `tr("Light theme") → "浅色"`。
3. **CJK 字体不用配**。`system_fonts.rs` 已通过 `fontique` 加载全系统回退字体（Windows 微软雅黑等），中文渲染开箱即用——这一步是源码自带的，不是我们改的。

### 数据流全景

```
效果设置（数据层，英文键名不动）
        │
        ▼
SettingDescriptor { id, label, description, ... }
        │  label / description 经过
        ▼
tr() 查找表 ──→ 中文渲染
        │
        ▼
预设 JSON（id 序列化，与上游字节级兼容）
```

### PAL 模拟的实现要点（路线 A）

PAL 与 NTSC 的核心区别在色度传输：NTSC 副载波相位恒定，相位误差直接表现为**色调偏移**；PAL 让 Q 分量逐行倒相，接收机再把相邻两行平均，相位误差被折算成**饱和度损失**（PAL 名字本身 = Phase Alternating Line）。

本分支只复刻观众可见的行为，不严格复刻副载波频率/场频等物理参数：

1. **逐行倒相**：`pal_v_switch(line)` 在奇数行对 Q 分量乘 -1，调制、解调、相位误差、相位噪声四个位置使用同一个开关，保证编解码自洽；
2. **行间平均**：PAL 模式下强制执行 `chroma_vert_blend`（相邻行色度平均，误差对消的前提）；
3. **误差交替**：相位误差/相位噪声逐行取反，经行间平均后色相抵消、幅度残留——视觉上就是「色脏、饱和度抖，但色调不跑」。

测试数据（同一张测试图、同一误差强度）：NTSC 相位误差边际差异 27.4（明显偏色），PAL 5.6（轻微降饱和）；无误差时两制式干净信号往返偏差一致（57.9 vs 57.3，差异在噪声随机性范围内）。

---

## 五、如何修改

### 改一个翻译

打开 `crates/gui/src/app/translations.rs`，找到对应组的行，改右侧中文即可。例如：

```rust
"Codec" => "编码器",   // ← 改成 "编码器" 或别的
```

改完跑 `cargo test -p ntsc-rs-gui --lib translations` 验证。

### 加一个此前没翻的词条

1. 在翻译表合适分组加一行：`"New English text" => "新中文",`
2. **词条必须与源码字符串逐字符一致**（含标点、空格、emoji、`\` 续行）
3. 如果该词出现在 GUI 里但没被 `tr()` 包住，需要在源码对应渲染处包一层 `tr("...")`

### 想让某个词恢复英文

删除翻译表中对应行即可——未收录词条会自动原样返回。

### 想整体切回英文界面

把 `translations.rs` 的 `tr()` 改成：

```rust
pub fn tr(text: &str) -> &str { text }
```

所有调用点自动失效，一行代码整体回退，源码其他部分零改动。

### 上游升级后同步汉化

`git fetch upstream && git merge upstream/main` 后，跑一次构建，把新增的英文 UI 字符串按上述流程补进翻译表即可。数据层无迁移成本。

---

## 六、补充说明

- 本分支基于上游 `0.9.6`。
- 汉化仅影响 `crates/gui`（图形界面）；PAL 功能改动 `crates/ntscrs`（核心算法），但所有改动都是**加法**（新枚举/新字段 id=62/新测试），默认 NTSC 路径与上游一致；CLI 未改动。
- 发布形态：独立仓库（非 fork 关系），README 与本文件均标注上游来源，各 crate 原许可证文件完整保留（核心库 MIT/ISC/Apache-2.0，GUI GPL-3.0）。
- 发布产物：`cargo xtask windows-bundle --release` → 把整个 `build\ntsc-rs-windows-standalone\` 压成 zip 传 Releases（与上游 release 产物形态一致）。
