//! 礼物礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 送礼礼仪规则
pub struct GiftEtiquette {
    metadata: RuleMetadata,
    culture: String,
}

impl GiftEtiquette {
    pub fn new(culture: impl Into<String>) -> Self {
        let culture = culture.into();
        Self {
            metadata: RuleMetadata::new(
                format!("{}送礼礼仪", culture),
                "送礼习俗与禁忌"
            )
            .with_origin(culture.clone()),
            culture,
        }
    }

    /// 获取不宜赠送的物品
    pub fn inappropriate_gifts(&self) -> Vec<&'static str> {
        match self.culture.as_str() {
            "中国" => vec![
                "钟 (送终)",
                "伞 (散)",
                "鞋 (邪)",
                "梨 (离)",
                "蜡烛",
                "刀剪",
            ],
            "西方" => vec![
                "空钱包",
                "尖锐物品",
                "二手物品",
            ],
            _ => vec![
                "过于廉价的物品",
                "过于昂贵的物品",
                "有争议的物品",
            ],
        }
    }

    /// 获取送礼时机
    pub fn gift_timing(&self) -> Vec<&'static str> {
        match self.culture.as_str() {
            "中国" => vec![
                "春节",
                "中秋",
                "生日",
                "婚礼",
                "升职",
            ],
            "西方" => vec![
                "Christmas (圣诞节)",
                "Birthday (生日)",
                "Wedding (婚礼)",
                "Anniversary (纪念日)",
            ],
            _ => vec![
                "节日",
                "生日",
                "特殊纪念日",
            ],
        }
    }
}

impl Rule for GiftEtiquette {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("gift")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【{}送礼礼仪】\n\n\
            不宜赠送:\n{}\n\n\
            送礼时机:\n{}\n",
            self.culture,
            self.inappropriate_gifts().iter().map(|s| format!("  • {}", s)).collect::<Vec<_>>().join("\n"),
            self.gift_timing().iter().map(|s| format!("  • {}", s)).collect::<Vec<_>>().join("\n")
        )
    }
}