#!/bin/bash

# 检查 512x512.png 是否存在
if [ ! -f "512x512.png" ]; then
  echo "错误: 512x512.png 文件不存在"
  exit 1
fi

# 生成 Square 开头的各个规格的图片
sizes=("30x30" "44x44" "71x71" "89x89" "107x107" "142x142" "150x150" "284x284" "310x310")

for size in "${sizes[@]}"; do
  output="Square${size}Logo.png"
  echo "生成 $output"
  convert 512x512.png -resize ${size} $output
  if [ $? -eq 0 ]; then
    echo "  成功"
  else
    echo "  失败"
  fi
done

# 生成 128x128 开头的两个图片
echo "生成 128x128.png"
convert 512x512.png -resize 128x128 128x128.png
echo "生成 128x128@2x.png"
convert 512x512.png -resize 256x256 128x128@2x.png

echo "图标生成完成！"
