#!/usr/bin/env node
// 平台感知的打包入口：tauri CLI 的 --bundles 合法值随构建平台编译生成
// （Linux: deb/rpm/appimage；Windows: nsis/msi；macOS: app/dmg），
// 在错误平台上执行会直接报 clap 错误，这里提前拦截并给出可操作提示。
import { spawnSync } from 'node:child_process';

const targets = process.argv.slice(2);
const platform = process.platform; // win32 | linux | darwin

const supported = {
  win32: ['nsis', 'msi'],
  linux: ['deb', 'rpm', 'appimage'],
  darwin: ['app', 'dmg'],
};
const platformNames = { win32: 'Windows', linux: 'Linux', darwin: 'macOS' };

if (targets.length === 0) {
  console.error(`用法: node scripts/build.mjs <类型...>，本平台(${platformNames[platform]})可选: ${supported[platform].join(', ')}`);
  process.exit(1);
}

const invalid = targets.filter((t) => !supported[platform].includes(t));
if (invalid.length > 0) {
  console.error(`✘ 当前平台是 ${platformNames[platform]}，无法在这里打包: ${invalid.join(', ')}`);
  console.error(`  tauri 不支持交叉打包；本平台可打包: ${supported[platform].join(', ')}`);
  if (platform !== 'win32') {
    console.error('  Windows 安装包的获取方式:');
    console.error('    1. 在 Windows 机器上运行 npm run build:win');
    console.error('    2. 推送 v* 标签或手动触发 .github/workflows/windows-build.yml，由 GitHub Actions 云端构建');
  }
  process.exit(1);
}

const result = spawnSync('npx', ['tauri', 'build', '--bundles', ...targets], {
  stdio: 'inherit',
  shell: process.platform === 'win32',
});
process.exit(result.status ?? 1);
