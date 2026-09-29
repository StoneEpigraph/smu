# SMU

轻量级桌面工具箱（Tauri 2 + Vue 3），常驻托盘，全局快捷入口。

## 功能

- 🔢 计算器、⏰ 时间转换、🎨 取色器
- 🔐 编码工具（MD5 / SHA / SM3 / Base64 / URL / Hex / ROT13）、SM2 国密加解密与签名
- 📅 日历待办（系统通知提醒）、📝 快捷笔记
- 🎫 身份证号校验与随机生成（GB 11643-1999）

## 开发

```bash
npm install
npm run tauri dev
```

Linux 构建依赖见下文各发行版小节；Windows 需 VS Build Tools（C++ 工作负载）+ Rust (msvc) + Node 20+。

## 打包

统一入口是 `tauri build`，通过 `--bundles` 参数指定打包类型（空格或逗号分隔均可）。
项目已提供常用组合的 npm 脚本：

| 命令 | 运行平台 | 产物 |
| --- | --- | --- |
| `npm run build:win` | Windows | NSIS 安装包 `.exe` + `.msi` |
| `npm run build:linux` | Linux | `.deb` + `.rpm` + `.AppImage` |
| `npm run build:deb` | Linux | `.deb`（Ubuntu / Debian 系） |
| `npm run build:rpm` | Linux | `.rpm`（Fedora / CentOS 系） |
| `npm run build:appimage` | Linux | `.AppImage`（免安装、全发行版通用） |
| `npm run build:arch` | Arch | `.pkg.tar.zst`（走 PKGBUILD 源码构建） |
| `npm run arch:src` | Arch | AUR 源码包 `smu-<ver>.tar.gz` + 刷新 `.SRCINFO` |
| `npm run tauri build` | 当前平台 | 当前平台全部类型（`targets: "all"`） |
| `npm run tauri build -- --bundles deb rpm` | Linux | 自定义组合，任意目标随意搭配 |

产物输出目录：`<target-dir>/release/bundle/{deb,rpm,appimage,nsis,msi}/`
（默认 `src-tauri/target/`；若在 cargo 配置中设置了 `CARGO_TARGET_DIR`/`build.target-dir`，则在其下，例如 `~/.target/release/bundle/`。）

> **平台限制**：tauri CLI 的 `--bundles` 合法值随构建平台编译生成（Linux 上只有 `deb`/`rpm`/`appimage`，Windows 上只有 `nsis`/`msi`），tauri 不支持交叉打包。上面的 npm 脚本已做平台守卫，在错误平台上运行会给出提示。Windows 安装包请在 Windows 机器本地构建，或用下文 GitHub Actions 云端构建；任意类型组合也可以直接 `node scripts/build.mjs deb rpm`。

### 版本号

发布前需同步修改四处：
`package.json`、`src-tauri/Cargo.toml`、`src-tauri/tauri.conf.json` 的 `version`，以及 `PKGBUILD` 的 `pkgver`（`pkgrel` 归 1）。

---

### Ubuntu / Debian 系

```bash
# 构建依赖
sudo apt update
sudo apt install -y libwebkit2gtk-4.1-dev build-essential curl wget file \
  libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev patchelf

# 打包 deb
npm run build:deb
# 产物: src-tauri/target/release/bundle/deb/SMU_0.1.1_amd64.deb

# 安装
sudo apt install ./SMU_0.1.1_amd64.deb
```

### CentOS / Fedora 系

```bash
# 构建依赖（Fedora / CentOS Stream 10）
sudo dnf install -y webkit2gtk4.1-devel gtk3-devel libappindicator-gtk3-devel \
  librsvg2-devel libxdo-devel openssl-devel patchelf gcc gcc-c++ make curl wget file

# 打包 rpm
npm run build:rpm
# 产物: src-tauri/target/release/bundle/rpm/SMU-0.1.1-1.x86_64.rpm

# 安装
sudo dnf install ./SMU-0.1.1-1.x86_64.rpm
```

> 注意：Tauri 2 依赖 webkit2gtk-4.1（libsoup3）。Fedora 36+ / CentOS Stream 10 原生可用；
> CentOS Stream 9 若仓库中没有 `webkit2gtk4.1`，建议改用下文 AppImage 免依赖运行。

### AppImage（全发行版通用）

```bash
npm run build:appimage
# 产物: src-tauri/target/release/bundle/appimage/SMU_0.1.1_amd64.AppImage
```

运行：

```bash
chmod +x SMU_0.1.1_amd64.AppImage
./SMU_0.1.1_amd64.AppImage
```

- 运行时需要 FUSE（Ubuntu：`sudo apt install libfuse2`）。
- 无 FUSE 的环境（容器/服务器）可解包运行：`./SMU_0.1.1_amd64.AppImage --appimage-extract-and-run`。
- 打包时需要联网下载 linuxdeploy 工具；GitHub 访问慢可用镜像加速：

  ```bash
  TAURI_BUNDLER_TOOLS_GITHUB_MIRROR=https://gh-proxy.com/ npm run build:appimage
  ```

### Arch Linux / AUR（源码包）

仓库根目录的 `PKGBUILD` 即 Arch 打包配置（源码构建，依赖 `webkit2gtk-4.1`、`gtk3`、`libappindicator`）。

```bash
# 方式一：直接构建本机安装包（构建目录隔离在 .makepkg/，不污染仓库）
npm run build:arch
# 产物: smu-0.1.1-1-x86_64.pkg.tar.zst
sudo pacman -U smu-0.1.1-1-x86_64.pkg.tar.zst

# 方式二：生成 AUR 发布所需的源码包 + .SRCINFO（基于 git HEAD，先提交代码）
npm run arch:src
# 产物: smu-0.1.1.tar.gz（与 PKGBUILD 的 source= 对应）、.SRCINFO
```

发布到 AUR：

```bash
git clone ssh://aur@aur.archlinux.org/smu.git aur-smu
cp PKGBUILD .SRCINFO smu-0.1.1.tar.gz aur-smu/
cd aur-smu && git add -A && git commit -m "update to 0.1.1" && git push
```

> `arch:src` 用 `git archive` 打包，只包含已提交内容；生成前请先提交改动。
> 版本更新时同步改 `PKGBUILD` 的 `pkgver`，`pkgrel` 归 1，再重新执行 `npm run arch:src`。

### Windows

> 只能在 Windows 上构建（tauri 不支持从 Linux/macOS 交叉打 Windows 安装包）。在 Linux/macOS 上可改用 GitHub Actions：推送 `v*` 标签或手动触发 `windows-build.yml`，产物在该次运行的 Artifacts 里。

```bash
npm run build:win
# 产物:
#   bundle/nsis/SMU_0.1.1_x64-setup.exe   中文安装向导（按用户安装，免管理员）
#   bundle/msi/SMU_0.1.1_x64_zh-CN.msi    企业部署用 MSI
#   target/release/SMU.exe                独立可执行文件（需系统已有 WebView2）
```

Windows 10/11 自带 WebView2；缺失时安装器会自动下载（`webviewInstallMode: downloadBootstrapper`）。
安装器/程序代码签名暂未配置，需要时在 `tauri.conf.json > bundle > windows` 增加
`certificateThumbprint`，并在构建环境提供证书。

### CI 自动构建

| Workflow | Runner | 产物 | 触发 |
| --- | --- | --- | --- |
| `.github/workflows/linux-build.yml` | ubuntu-latest | deb + rpm + AppImage | 手动（workflow_dispatch）或推送 `v*` 标签 |
| `.github/workflows/windows-build.yml` | windows-latest | NSIS exe + MSI | 手动或推送 `v*` 标签 |

手动触发：GitHub 仓库 → Actions → 选择工作流 → Run workflow，构建结果在该次运行的 Artifacts 里。
推送 `v*` 标签（如 `git tag v0.1.1 && git push origin v0.1.1`）会额外自动创建 GitHub Release 并附上全部安装包。

### 发布 Checklist

1. 同步版本号：`package.json`、`src-tauri/Cargo.toml`、`src-tauri/tauri.conf.json`、`PKGBUILD`（`pkgrel` 归 1）
2. `npm run build` 确认类型检查与前端构建通过
3. 提交代码，`npm run arch:src` 刷新 AUR 源码包与 `.SRCINFO`，一并提交
4. 打标签 `git tag v<x.y.z> && git push origin main --tags`，等待两个 CI 工作流产出安装包
5. 更新 AUR（见上）
