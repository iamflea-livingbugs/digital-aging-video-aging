<p align="center">
    <a href="https://ntsc.rs">
        <picture>
            <source media="(prefers-color-scheme: dark)" srcset="./docs/img/logo-darkmode.svg">
            <img alt="ntsc-rs logo" src="./docs/img/logo-lightmode.svg" width="1216">
        </picture>
    </a>
</p>

---

> **Fork notice / 分支说明**
>
> This is an independent downstream build of [ntsc-rs/ntsc-rs](https://github.com/ntsc-rs/ntsc-rs) (upstream `0.9.6`) with two additions:
> 1. **Simplified Chinese GUI localization** (display layer only; presets, setting keys and save formats stay byte-compatible with upstream).
> 2. **PAL video-standard simulation** (new "Video standard" setting; NTSC remains the default, old presets load unchanged).
>
> 中文说明见 [README-zh-Hans.md](./README-zh-Hans.md)。本仓库不隶属于上游项目，原项目名称与许可证归原作者所有。

---

**ntsc-rs** is a video effect which emulates NTSC and VHS video artifacts. It can be used as an After Effects, Premiere, or OpenFX plugin, or as a standalone application.

![Screenshot of the ntsc-rs standalone application](./docs/img/appdemo.png)

## Download and Install

The latest version of ntsc-rs can be downloaded from [the releases page](https://github.com/valadaptive/ntsc-rs/releases).

After downloading, [read the documentation for how to run it](https://ntsc.rs/docs/standalone-installation/). In particular, ntsc-rs will not work properly on Linux unless you install all of the GStreamer packages listed in the documentation.

## More information

ntsc-rs is a rough Rust port of [ntscqt](https://github.com/JargeZ/ntscqt), a PyQt-based GUI for [ntsc](https://github.com/zhuker/ntsc), itself a Python port of [composite-video-simulator](https://github.com/joncampbell123/composite-video-simulator). Reimplementing the image processing in multithreaded Rust allows it to run at (mostly) real-time speeds.

It's not an exact port--some processing passes have visibly different results, and some new ones have been added.
