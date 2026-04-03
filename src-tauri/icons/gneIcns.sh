#!/bin/bash

# 检查 512x512.png 是否存在
if [ ! -f "512x512.png" ]; then
  echo "错误: 512x512.png 文件不存在"
  exit 1
fi

# 检查 ImageMagick 是否安装
if ! command -v convert &> /dev/null; then
  echo "错误: ImageMagick 未安装，请运行: sudo pacman -S imagemagick"
  exit 1
fi

# 创建临时目录
TMP_DIR=$(mktemp -d)
echo "创建临时目录: $TMP_DIR"

# 生成不同尺寸的 PNG 文件
sizes=("16" "32" "64" "128" "256" "512")

for size in "${sizes[@]}"; do
  output="$TMP_DIR/icon_${size}x${size}.png"
  echo "生成 ${size}x${size} 图标"
  convert 512x512.png -resize ${size}x${size} $output
  if [ $? -eq 0 ]; then
    echo "  成功"
  else
    echo "  失败"
  fi
done

# 检查是否有 iconutil 命令（macOS 工具）
if command -v iconutil &> /dev/null; then
  echo "使用 iconutil 生成 icns 文件"
  # 创建 .iconset 目录
  ICONSET_DIR="$TMP_DIR/icon.iconset"
  mkdir -p $ICONSET_DIR
  
  # 复制文件到 iconset 目录
  cp "$TMP_DIR/icon_16x16.png" "$ICONSET_DIR/icon_16x16.png"
  cp "$TMP_DIR/icon_32x32.png" "$ICONSET_DIR/icon_32x32.png"
  cp "$TMP_DIR/icon_64x64.png" "$ICONSET_DIR/icon_64x64.png"
  cp "$TMP_DIR/icon_128x128.png" "$ICONSET_DIR/icon_128x128.png"
  cp "$TMP_DIR/icon_256x256.png" "$ICONSET_DIR/icon_256x256.png"
  cp "$TMP_DIR/icon_512x512.png" "$ICONSET_DIR/icon_512x512.png"
  
  # 生成 icns 文件
  iconutil -c icns $ICONSET_DIR -o icon.icns
  
  if [ $? -eq 0 ]; then
    echo "icns 文件生成成功: icon.icns"
  else
    echo "icns 文件生成失败"
  fi
else
  echo "警告: iconutil 命令未找到（这是 macOS 工具）"
  echo "请使用在线工具生成 icns 文件，或在 macOS 系统上运行此脚本"
  echo "推荐使用: https://www.icoconverter.com/"
  
  # 尝试使用 convert 直接生成 icns
  echo "尝试使用 ImageMagick 直接生成 icns..."
  convert 512x512.png -resize 512x512 icon.icns
  
  if [ $? -eq 0 ]; then
    echo "icns 文件生成成功: icon.icns"
  else
    echo "icns 文件生成失败"
  fi
fi

# 清理临时目录
echo "清理临时目录"
rm -rf $TMP_DIR

echo "脚本执行完成！"
