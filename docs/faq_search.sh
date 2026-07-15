#!/bin/bash
# FAQ 搜索脚本
# 用法: ./faq_search.sh <关键词>

if [ $# -eq 0 ]; then
    echo "用法: ./faq_search.sh <关键词>"
    echo "示例: ./faq_search.sh 麻将"
    exit 1
fi

KEYWORD="$1"
FAQ_FILE="$(dirname "$0")/FAQ.md"

if [ ! -f "$FAQ_FILE" ]; then
    echo "错误: FAQ.md 文件不存在"
    exit 1
fi

echo "===== FAQ 搜索结果: $KEYWORD ====="
echo ""

# 搜索标题和内容
grep -i -n --color=always -A 5 "### Q:.*$KEYWORD\|$KEYWORD" "$FAQ_FILE" | head -50

echo ""
echo "查看完整 FAQ: docs/FAQ.md"