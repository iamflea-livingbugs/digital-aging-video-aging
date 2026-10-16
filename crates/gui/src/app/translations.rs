/// 显示层翻译表（Display-layer localization table）。
///
/// 设计原则：
/// - 只在 GUI 渲染处调用，不动数据层 / 预设 JSON / 配置键名，保证与上游预设完全兼容。
/// - 查不到的词条原样返回，用户自定义内容（预设名、文件名、路径、错误信息）绝不会被改写。
/// - 技术术语（NTSC / VHS / H.264 / FFV1 / PNG 等）保留英文，符合影像工作流习惯。
pub fn tr(text: &str) -> &str {
    match text {
        // ========== 顶部菜单 ==========
        "File" => "文件",
        "Open" => "打开",
        "Quit" => "退出",
        "Edit" => "编辑",
        "Undo" => "撤销",
        "Redo" => "重做",
        "View" => "视图",
        "Help" => "帮助",
        "License" => "许可证",
        "Third-Party Licenses" => "第三方许可证",
        "About + Credits" => "关于与致谢",
        "Check for Updates..." => "检查更新…",
        "Online Documentation ⤴" => "在线文档 ⤴",

        // ========== 主题 / 缩放菜单 ==========
        "Theme" => "主题",
        "System" => "跟随系统",
        "Light theme" => "浅色",
        "Dark" => "深色",
        "Follow system color theme" => "跟随系统颜色主题",
        "Use light mode" => "使用浅色模式",
        "Use dark mode" => "使用深色模式",
        "Zoom" => "缩放",

        // ========== 左侧面板标签页 ==========
        "Effect" => "效果",
        "Render" => "渲染",
        "Easy mode" => "简易模式",

        // ========== 预设操作 ==========
        "Presets" => "预设",
        "Save to..." => "保存到…",
        "Load from..." => "从…加载",
        "📋 Copy" => "📋 复制",
        "📄 Paste" => "📄 粘贴",
        "Paste JSON" => "粘贴 JSON",
        "Load" => "加载",
        "Reset" => "重置",
        "Save" => "保存",
        "Save as" => "另存为",
        "Overwrite" => "覆盖",
        "Reload" => "重新加载",
        "Open folder" => "打开文件夹",
        "Delete" => "删除",
        "Rename" => "重命名",
        "Drop to install presets" => "拖放预设文件以安装",
        "Drop to load preset" => "拖放预设以加载",
        "Drop to load media" => "拖放媒体文件以加载",
        "Error loading presets directory" => "加载预设目录时出错",

        // ========== 渲染设置 ==========
        "Codec" => "编码器",
        "H.264" => "H.264",
        "FFV1 (Lossless)" => "FFV1（无损）",
        "PNG Sequence" => "PNG 序列",
        "Bit depth" => "位深",
        "8-bit" => "8 位",
        "10-bit" => "10 位",
        "12-bit" => "12 位",
        "Quality" => "质量",
        "Encoding speed" => "编码速度",
        "Compression level" => "压缩级别",
        "4:2:0 chroma subsampling" => "4:2:0 色度子采样",
        "Destination file:" => "输出文件：",
        "Browse for a path" => "浏览路径",
        "Duration:" => "时长：",
        "Interlaced output" => "隔行输出",
        "Video quality factor, where 0 is the worst quality and 50 is the best. Higher quality \
         videos take up more space." =>
            "视频质量系数，0 为最差，50 为最好。质量越高，视频占用空间越大。",
        "Encoding speed preset. Higher encoding speeds provide a worse compression ratio, \
         resulting in larger videos at a given quality." =>
            "编码速度预设。编码速度越高，压缩比越差，在相同质量下视频文件越大。",
        "Subsample the chrominance planes to half the resolution of the luminance plane. \
         Increases playback compatibility." =>
            "将色度平面子采样为亮度平面分辨率的一半。提高播放兼容性。",
        "Subsample the chrominance planes to half the resolution of the luminance plane. \
         Results in smaller files." =>
            "将色度平面子采样为亮度平面分辨率的一半。文件体积更小。",
        "Compression level for PNG encoding. Higher compression levels produce smaller files \
         but take longer to render." =>
            "PNG 编码的压缩级别。压缩级别越高，文件越小，但渲染时间越长。",
        "Image sequences do not support interlaced output." => "图像序列不支持隔行输出。",
        "To enable interlaced output, set the \"Use field\" setting to \"Interleaved\"." =>
            "要启用隔行输出，请将\"使用场\"设置设为\"交错\"。",

        // ========== 视频面板 ==========
        "Save frame" => "保存帧",
        "Copy frame" => "复制帧",
        "fps" => "帧/秒",
        " (progressive)" => "（逐行）",
        " (interlaced)" => "（隔行）",
        "No media loaded" => "未加载媒体",
        "Zoom preview" => "缩放预览",
        "Fit" => "适应窗口",
        "Unmute" => "取消静音",
        "Mute" => "静音",
        "Effect preview" => "效果预览",
        "Enable" => "启用",
        "Disable" => "禁用",
        "Split" => "分屏",
        "Scale to" => "缩放至",
        "lines" => "行",
        "Resizing filter" => "缩放滤镜",
        "Nearest" => "最近邻",
        "Bilinear" => "双线性",
        "Bicubic" => "双三次",
        "Nearest-neighbor (pixelated) scaling. Note that this is still slower than Bilinear" =>
            "最近邻（像素化）缩放。注意：仍比双线性慢",
        "Lower-quality but faster scaling filter" => "质量较低但速度更快的缩放滤镜",
        "Sharper scaling filter; ~20% slower than Bilinear" =>
            "更锐利的缩放滤镜；比双线性慢约 20%",
        "Scale the video prior to applying the effect. Real NTSC footage is 480 lines tall. \
         This applies to both the preview and the final render, and is not saved as part of \
         presets." =>
            "在应用效果前缩放视频。真实 NTSC 影像为 480 行。此设置同时作用于预览和最终渲染，且不会随预设保存。",

        // ========== 对话框 ==========
        "OK" => "确定",
        "Cancel" => "取消",
        "Check for Updates" => "检查更新",
        "Checking for updates... " => "正在检查更新… ",
        "✓ Up to date" => "✓ 已是最新版本",
        "Download from GitHub ⤴" => "从 GitHub 下载 ⤴",
        "Error checking for updates:" => "检查更新时出错：",
        "Latest version:" => "最新版本：",
        "Used by:" => "使用方：",
        "Output directory is not empty" => "输出目录不为空",
        "You're rendering an image sequence into a directory that isn't empty. This will \
         output many individual image files into that directory." =>
            "你正在向一个非空目录渲染图像序列，这会向该目录输出大量单独的图像文件。",
        "An error occurred while loading" => "加载过程中发生错误",
        "Randomize seed" => "随机化种子",
        "by " => "作者 ",
        "...loosely based on " => "……松散基于 ",
        "...which is a GUI for " => "……它是以下项目的 GUI：",
        "...which is a port of " => "……它是以下项目的移植：",

        // ========== 渲染任务卡片 ==========
        "Waiting..." => "等待中…",
        "Rendering..." => "渲染中…",
        "Paused" => "已暂停",
        "Completed in" => "已完成，用时",
        "Error:" => "错误：",
        "Time remaining: " => "剩余时间：",
        "Open containing folder" => "打开所在文件夹",
        " hour" => " 小时",
        " hours" => " 小时",
        " minute" => " 分钟",
        " minutes" => " 分钟",
        " second" => " 秒",
        " seconds" => " 秒",

        // ========== 效果设置：通用参数 ==========
        "Random seed" => "随机种子",
        "Use field" => "使用场",
        "Alternating" => "交替",
        "Upper only" => "仅上场",
        "Lower only" => "仅下场",
        "Interleaved (upper first)" => "交错（上场优先）",
        "Interleaved (lower first)" => "交错（下场优先）",
        "Both" => "全部",
        "Lowpass filter type" => "低通滤波器类型",
        "Constant K (blurry)" => "恒定 K（模糊）",
        "Butterworth (sharper)" => "巴特沃斯（更锐利）",
        "Input luma filter" => "输入亮度滤波器",
        "Notch" => "陷波",
        "Box" => "方框",
        "None" => "无",
        "Chroma low-pass in" => "色度输入低通",
        "Chroma low-pass out" => "色度输出低通",
        "Full" => "全强度",
        "Light" => "轻度",
        "Composite signal sharpening" => "复合信号锐化",
        "Composite signal noise" => "复合信号噪声",
        "Intensity" => "强度",
        "Frequency" => "频率",
        "Detail" => "细节",
        "Snow" => "雪花噪点",
        "Snow anisotropy" => "雪花各向异性",
        "Snow intensity" => "雪花强度",
        "Chroma demodulation filter" => "色度解调滤波器",
        "2D" => "2D",
        "2D adaptive" => "2D 自适应",
        "1-line comb" => "1 行梳状",
        "2-line comb" => "2 行梳状",
        "Luma smear" => "亮度拖影",
        "Head switching" => "磁头切换",
        "Height" => "高度",
        "Offset" => "偏移",
        "Horizontal shift" => "水平偏移",
        "Start mid-line" => "从扫描线中间开始",
        "Position" => "位置",
        "Jitter" => "抖动",
        "Tracking noise" => "跟踪噪声",
        "Wave intensity" => "波形强度",
        "Noise intensity" => "噪声强度",
        "Ringing" => "振铃",
        "Power" => "功率",
        "Scale" => "缩放",
        "Luma noise" => "亮度噪声",
        "Chroma noise" => "色度噪声",
        "Chroma phase error" => "色度相位误差",
        "Chroma phase noise" => "色度相位噪声",
        "Chroma delay (horizontal)" => "色度延迟（水平）",
        "Chroma delay (vertical)" => "色度延迟（垂直）",
        "VHS emulation" => "VHS 仿真",
        "Tape speed" => "磁带速度",
        "SP (Standard Play)" => "SP（标准播放）",
        "LP (Long Play)" => "LP（长时间播放）",
        "EP (Extended Play)" => "EP（扩展播放）",
        "Chroma loss" => "色度丢失",
        "Sharpen" => "锐化",
        "Edge wave" => "边缘抖动",
        "Speed" => "速度",
        "Vertically blend chroma" => "垂直混合色度",
        "Scale with video size" => "按视频尺寸缩放",
        "Horizontal scale" => "水平缩放",
        "Vertical scale" => "垂直缩放",
        "Saturation" => "饱和度",

        // ========== 效果设置：描述（悬停提示） ==========
        "Choose which rows (\"fields\" in NTSC parlance) of the source image will be used." =>
            "选择使用源图像的哪些行（NTSC 术语中的\"场\"）。",
        "Skip every other row, alternating between skipping even and odd rows." =>
            "每隔一行跳过一次，偶数行与奇数行交替跳过。",
        "Skip every lower row, keeping the upper ones." => "跳过所有下场行，保留上场行。",
        "Skip every upper row, keeping the lower ones." => "跳过所有上场行，保留下场行。",
        "Treat the video as interlaced, with the upper field as the earlier frame." =>
            "将视频视为隔行，上场作为较早的帧。",
        "Treat the video as interlaced, with the lower field as the earlier frame." =>
            "将视频视为隔行，下场作为较早的帧。",
        "Use all rows; don't skip any." => "使用所有行，不跳过任何行。",
        "The low-pass filter to use throughout the effect." => "效果全程使用的低通滤波器。",
        "Simple constant-k filter. Produces longer, blurry results." =>
            "简单的恒定 K 滤波器。产生较长、模糊的结果。",
        "Filter with a sharper falloff. Produces sharpened, less blurry results." =>
            "具有更锐利衰减的滤波器。产生更锐利、更少模糊的结果。",
        "Filter the input luminance to decrease rainbow artifacts." =>
            "对输入亮度进行滤波以减少彩虹伪影。",
        "Apply a notch filter to the input luminance signal. Sharp, but has ringing artifacts." =>
            "对输入亮度信号应用陷波滤波器。锐利，但存在振铃伪影。",
        "Apply a simple box filter to the input luminance signal." =>
            "对输入亮度信号应用简单的方框滤波器。",
        "Do not filter the luminance signal. Adds rainbow artifacts." =>
            "不对亮度信号进行滤波。会增加彩虹伪影。",
        "Apply a low-pass filter to the input chrominance (color) signal." =>
            "对输入色度（颜色）信号应用低通滤波器。",
        "Full-intensity low-pass filter." => "全强度低通滤波器。",
        "Less intense low-pass filter." => "较低强度的低通滤波器。",
        "No low-pass filter." => "无低通滤波器。",
        "Boost high frequencies in the NTSC signal, sharpening the image and intensifying \
         colors." => "增强 NTSC 信号中的高频，锐化图像并强化颜色。",
        "Noise applied to the composite NTSC signal." => "应用于复合 NTSC 信号的噪声。",
        "Intensity of the noise." => "噪声强度。",
        "Base wavelength, in pixels, of the noise." => "噪声的基础波长（像素）。",
        "Octaves of noise." => "噪声的倍频程数。",
        "Frequency of random speckles in the image." => "图像中随机斑点（雪花）的频率。",
        "Determines whether the speckles are placed truly randomly or concentrated in certain \
         rows." => "决定斑点是完全随机分布还是集中在特定行。",
        "Filter used to modulate the chrominance (color) data out of the composite NTSC \
         signal." => "用于从复合 NTSC 信号中解调出色度（颜色）数据的滤波器。",
        "Simple horizontal box blur." => "简单的水平方框模糊。",
        "Notch filter. Sharper than a box blur, but with ringing artifacts." =>
            "陷波滤波器。比方框模糊更锐利，但存在振铃伪影。",
        "2D \"digital-style\" filter. Highest all-around quality." =>
            "2D\"数字风格\"滤波器。综合质量最高。",
        "2D edge-following filter. Avoids \"dot crawl\" artifacts at all costs, but results in \
         less detailed output and is likely to cause rainbow artifacts." =>
            "2D 边缘跟踪滤波器。不惜一切代价避免\"点爬行\"伪影，但输出细节较少，且可能产生彩虹伪影。",
        "Average the current row with the previous one, phase-cancelling the chrominance \
         (color) signals." => "将当前行与上一行求平均，以相位抵消色度（颜色）信号。",
        "Average the current row with the previous and next ones, phase-cancelling the \
         chrominance (color) signals." => "将当前行与上一行和下一行求平均，以相位抵消色度（颜色）信号。",
        "Average the current row with the previous one, phase-cancelling the chrominance \
         signals. Only works if the scanline phase shift is 180 degrees." =>
            "将当前行与上一行求平均，以相位抵消色度信号。仅当扫描线相位偏移为 180 度时有效。",
        "Average the current row with the previous and next ones, phase-cancelling the \
         chrominance signals. Only works if the scanline phase shift is 180 degrees." =>
            "将当前行与上一行和下一行求平均，以相位抵消色度信号。仅当扫描线相位偏移为 180 度时有效。",
        "Emulate VHS head-switching artifacts at the bottom of the image." =>
            "在图像底部模拟 VHS 磁头切换伪影。",
        "Total height of the head-switching artifact." => "磁头切换伪影的总高度。",
        "How much of the head-switching artifact is off-screen." =>
            "磁头切换伪影在屏幕外有多少。",
        "How much the head-switching artifact shifts rows horizontally." =>
            "磁头切换伪影在水平方向移动行的程度。",
        "Start the head-switching artifact mid-scanline, with some static where it begins." =>
            "在扫描线中间开始磁头切换伪影，起始处带有一些静噪。",
        "Horizontal position at which the head-switching starts." =>
            "磁头切换开始处的水平位置。",
        "How much the head-switching artifact \"jitters\" horizontally." =>
            "磁头切换伪影在水平方向\"抖动\"的程度。",
        "Emulate noise from VHS tracking error." => "模拟 VHS 跟踪误差产生的噪声。",
        "Total height of the tracking artifacts." => "跟踪伪影的总高度。",
        "How much the affected scanlines \"wave\" back and forth." =>
            "受影响的扫描线前后\"摆动\"的程度。",
        "Frequency of speckle-type noise in the artifacts." => "伪影中斑点型噪声的频率。",
        "How much the speckles are clustered by scanline." => "斑点按扫描线聚集的程度。",
        "Intensity of non-speckle noise." => "非斑点噪声的强度。",
        "Additional ringing artifacts, simulated with a notch filter." =>
            "使用陷波滤波器模拟的附加振铃伪影。",
        "Frequency/period of the ringing, in \"rings per pixel\"." =>
            "振铃的频率/周期，单位\"每像素环数\"。",
        "The power of the notch filter / how far out the ringing extends." =>
            "陷波滤波器的功率 / 振铃延伸的距离。",
        "Intensity of the ringing." => "振铃强度。",
        "Noise applied to the luminance signal. Useful for higher-frequency noise than the \
         \"Composite noise\" setting can provide." =>
            "应用于亮度信号的噪声。适用于比\"复合噪声\"设置所能提供的更高频噪声。",
        "Noise applied to the chrominance (color) signal." => "应用于色度（颜色）信号的噪声。",
        "Noise applied to the chrominance signal." => "应用于色度信号的噪声。",
        "Phase error for the chrominance (color) signal." => "色度（颜色）信号的相位误差。",
        "Noise applied per-scanline to the phase of the chrominance (color) signal." =>
            "逐扫描线应用于色度（颜色）信号相位的噪声。",
        "Noise applied per-scanline to the phase of the chrominance signal." =>
            "逐扫描线应用于色度信号相位的噪声。",
        "Horizontal offset of the chrominance (color) signal." =>
            "色度（颜色）信号的水平偏移。",
        "Vertical offset of the chrominance (color) signal. Usually increases with VHS \
         generation loss." => "色度（颜色）信号的垂直偏移。通常随 VHS 翻录代损而增加。",
        "Emulate cutoff of high-frequency data at various VHS recording speeds." =>
            "模拟不同 VHS 录制速度下的高频数据截止。",
        "Chance that the chrominance (color) signal is completely lost in each scanline." =>
            "每条扫描线中色度（颜色）信号完全丢失的概率。",
        "Sharpening of the image, as done by some VHS decks." =>
            "类似某些 VHS 录像机进行的图像锐化。",
        "Amount of sharpening to apply." => "应用的锐化量。",
        "Frequency / radius of the sharpening, relative to the tape speed's cutoff frequency." =>
            "锐化的频率 / 半径，相对于磁带速度的截止频率。",
        "Horizontal waving of the image." => "图像的水平晃动。",
        "Horizontal waving of the image, in pixels." => "图像的水平晃动幅度（像素）。",
        "Speed at which the horizontal waving occurs." => "水平晃动发生的速度。",
        "Base wavelength for the horizontal waving." => "水平晃动的基础波长。",
        "Octaves of noise for the waves." => "晃动的噪声倍频程数。",
        "Vertically blend each scanline's chrominance with the scanline above it." =>
            "将每条扫描线的色度与上方的扫描线垂直混合。",
        "Apply a low-pass filter to the output chroma signal." =>
            "对输出色度信号应用低通滤波器。",
        "Scale the effect by these factors." => "按这些因子缩放效果。",
        "Horizontally scale the effect by this amount. For 480p video, leave this at 1.0 for \
         the most physically-accurate result." =>
            "按此数值水平缩放效果。对于 480p 视频，请保留为 1.0 以获得最物理准确的结果。",
        "Vertically scale the effect by this amount. You should probably leave this at 1.0." =>
            "按此数值垂直缩放效果。建议保留为 1.0。",
        "Multiply the scaling factors by the video's height. Prefer scaling the input video to \
         480p instead, which gives much more accurate-looking results." =>
            "将缩放因子乘以视频高度。建议改为将输入视频缩放到 480p，结果会准确得多。",

        // 未收录的词条原样返回（预设名、文件名、错误信息等一律不受影响）
        _ => text,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translates_gui_and_parameter_labels() {
        assert_eq!(tr("Render"), "渲染");
        assert_eq!(tr("Random seed"), "随机种子");
        assert_eq!(tr("Presets"), "预设");
        assert_eq!(tr("Save as"), "另存为");
        assert_eq!(tr("VHS emulation"), "VHS 仿真");
        assert_eq!(tr("Chroma demodulation filter"), "色度解调滤波器");
        assert_eq!(tr("Interlaced output"), "隔行输出");
    }

    #[test]
    fn preserves_unknown_text_and_user_content() {
        let text = String::from("用户 preset_42.json");
        assert_eq!(tr(&text), text);
        assert_eq!(tr(""), "");
        assert_eq!(tr("H.264"), "H.264");
        assert_eq!(tr("D:\\videos\\my_footage.mov"), "D:\\videos\\my_footage.mov");
    }

    #[test]
    fn settings_descriptors_are_covered() {
        // ntsc_rs 标准模式全部顶层/子级标签都应命中翻译表
        for label in [
            "Random seed",
            "Use field",
            "Lowpass filter type",
            "Input luma filter",
            "Chroma low-pass in",
            "Composite signal sharpening",
            "Composite signal noise",
            "Snow",
            "Snow anisotropy",
            "Chroma demodulation filter",
            "Luma smear",
            "Head switching",
            "Tracking noise",
            "Ringing",
            "Luma noise",
            "Chroma noise",
            "Chroma phase error",
            "Chroma phase noise",
            "Chroma delay (horizontal)",
            "Chroma delay (vertical)",
            "VHS emulation",
            "Vertically blend chroma",
            "Chroma low-pass out",
            "Scale",
        ] {
            assert_ne!(tr(label), label, "missing translation for {label:?}");
        }
    }
}
