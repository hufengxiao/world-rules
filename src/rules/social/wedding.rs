//! 婚礼礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 婚礼文化类型
#[derive(Debug, Clone)]
pub enum WeddingCulture {
    Chinese,
    Western,
    Japanese,
}

/// 婚礼礼仪规则
pub struct WeddingEtiquette {
    metadata: RuleMetadata,
    culture: WeddingCulture,
}

impl WeddingEtiquette {
    pub fn new(culture: WeddingCulture) -> Self {
        Self {
            metadata: RuleMetadata::new(
                format!("{}婚礼礼仪", Self::culture_name(&culture)),
                "婚礼礼仪规范"
            )
            .with_origin(Self::culture_name(&culture)),
            culture,
        }
    }

    fn culture_name(culture: &WeddingCulture) -> &'static str {
        match culture {
            WeddingCulture::Chinese => "中式",
            WeddingCulture::Western => "西式",
            WeddingCulture::Japanese => "日式",
        }
    }

    /// 婚礼流程
    pub fn wedding_process(&self) -> Vec<&'static str> {
        match self.culture {
            WeddingCulture::Chinese => vec![
                "提亲",
                "定亲 (订婚)",
                "过大礼",
                "安床",
                "迎亲",
                "拜堂",
                "敬茶",
                "婚宴",
                "回门",
            ],
            WeddingCulture::Western => vec![
                "订婚",
                "婚礼筹备",
                "婚礼仪式",
                "誓词交换",
                "戒指交换",
                "婚礼祝福",
                "婚宴",
                "蜜月",
            ],
            WeddingCulture::Japanese => vec![
                "见面礼",
                "结纳",
                "婚礼准备",
                "神前婚礼",
                "三三九度杯",
                "誓词",
                "婚宴",
            ],
        }
    }

    /// 新人礼仪
    pub fn bride_groom_etiquette(&self) -> Vec<&'static str> {
        match self.culture {
            WeddingCulture::Chinese => vec![
                "新娘穿红色婚纱",
                "新郎西装或传统服饰",
                "拜堂要恭敬",
                "敬茶要跪拜",
                "婚宴要敬酒",
            ],
            WeddingCulture::Western => vec![
                "新娘穿白色婚纱",
                "新郎穿西装",
                "父亲陪伴新娘入场",
                "交换誓词和戒指",
                "切蛋糕",
            ],
            WeddingCulture::Japanese => vec![
                "新娘穿白无垢",
                "新郎穿传统礼服",
                "三三九度杯仪式",
                "亲属代表发言",
            ],
        }
    }

    /// 客人礼仪
    pub fn guest_etiquette(&self) -> Vec<&'static str> {
        match self.culture {
            WeddingCulture::Chinese => vec![
                "穿喜庆颜色 (避免全白全黑)",
                "送礼金要双数",
                "红包写上祝福语",
                "准时到场",
                "不要提前离场",
            ],
            WeddingCulture::Western => vec![
                "正装出席",
                "送礼物或礼金",
                "婚礼仪式保持安静",
                "祝福新人",
                "参加婚宴",
            ],
            WeddingCulture::Japanese => vec![
                "穿正装",
                "准备祝仪金",
                "签名入场",
                "安静参加仪式",
                "婚宴中互动",
            ],
        }
    }

    /// 礼金标准
    pub fn gift_guidelines(&self) -> Vec<&'static str> {
        match self.culture {
            WeddingCulture::Chinese => vec![
                "一般朋友: 200-500元",
                "好朋友: 500-1000元",
                "亲戚: 1000-2000元或更多",
                "金额要双数 (避4)",
                "用红包包装",
            ],
            WeddingCulture::Western => vec![
                "礼物为主",
                "礼金通常$50-$200",
                "可在婚礼网站登记礼物",
            ],
            WeddingCulture::Japanese => vec![
                "祝仪金通常1万-3万日元",
                "新钞",
                "专用祝仪袋",
                "金额避免偶数",
            ],
        }
    }
}

impl Rule for WeddingEtiquette {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("wedding")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【{}婚礼礼仪】\n\n\
            婚礼流程:\n{}\n\n\
            新人礼仪:\n{}\n\n\
            客人礼仪:\n{}\n\n\
            礼金指南:\n{}\n",
            Self::culture_name(&self.culture),
            self.wedding_process().iter().map(|s| format!("  • {}", s)).collect::<Vec<_>>().join("\n"),
            self.bride_groom_etiquette().iter().map(|s| format!("  • {}", s)).collect::<Vec<_>>().join("\n"),
            self.guest_etiquette().iter().map(|s| format!("  • {}", s)).collect::<Vec<_>>().join("\n"),
            self.gift_guidelines().iter().map(|s| format!("  • {}", s)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chinese_wedding() {
        let wedding = WeddingEtiquette::new(WeddingCulture::Chinese);
        assert!(wedding.wedding_process().contains(&"拜堂"));
    }
}