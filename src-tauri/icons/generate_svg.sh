#!/bin/bash

# 显示帮助信息
show_help() {
  echo "使用方法: $0 [选项] <输入文件> [输出文件]"
  echo ""
  echo "选项:"
  echo "  -h, --help     显示帮助信息"
  echo "  -s, --size     设置输出 SVG 的尺寸 (例如: 512x512)"
  echo "  -q, --quality  设置输出质量 (0-100)"
  echo ""
  echo "示例:"
  echo "  $0 512x512.png output.svg"
  echo "  $0 -s 256x256 input.png output.svg"
  echo ""
}

# 默认参数
SIZE=""
QUALITY=90

# 解析命令行参数
while [[ $# -gt 0 ]]; do
  case $1 in
    -h|--help)
      show_help
      exit 0
      ;;
    -s|--size)
      SIZE="$2"
      shift # 跳过参数值
      ;;
    -q|--quality)
      QUALITY="$2"
      shift # 跳过参数值
      ;;
    *)
      if [ -z "$INPUT_FILE" ]; then
        INPUT_FILE="$1"
      elif [ -z "$OUTPUT_FILE" ]; then
        OUTPUT_FILE="$1"
      else
        echo "错误: 过多的参数"
        show_help
        exit 1
      fi
      ;;
  esac
  shift
done

# 检查必要参数
if [ -z "$INPUT_FILE" ]; then
  echo "错误: 未指定输入文件"
  show_help
  exit 1
fi

if [ -z "$OUTPUT_FILE" ]; then
  # 默认输出文件名
  OUTPUT_FILE="${INPUT_FILE%.*}.svg"
fi

# 检查输入文件是否存在
if [ ! -f "$INPUT_FILE" ]; then
  echo "错误: 输入文件 '$INPUT_FILE' 不存在"
  exit 1
fi

# 检查 ImageMagick 是否安装
if ! command -v convert &> /dev/null; then
  echo "错误: ImageMagick 未安装，请运行: sudo pacman -S imagemagick"
  exit 1
fi

# 构建转换命令
CONVERT_CMD="convert"

if [ ! -z "$SIZE" ]; then
  CONVERT_CMD="$CONVERT_CMD -resize $SIZE"
fi

CONVERT_CMD="$CONVERT_CMD -quality $QUALITY"
CONVERT_CMD="$CONVERT_CMD '$INPUT_FILE' '$OUTPUT_FILE'"

# 执行转换
echo "正在将 '$INPUT_FILE' 转换为 '$OUTPUT_FILE'"
echo "执行命令: $CONVERT_CMD"

if eval $CONVERT_CMD; then
  echo "成功: SVG 文件已生成: $OUTPUT_FILE"
  echo ""
  echo "注意: 此方法生成的是包含嵌入图像的 SVG，不是矢量 SVG。"
  echo "如需真正的矢量 SVG，请使用专门的矢量编辑软件进行转换。"
else
  echo "错误: 转换失败"
  exit 1
fi
