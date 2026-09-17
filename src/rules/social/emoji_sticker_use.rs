//! 表情包与表情符号使用礼仪
//!
//! 使用表情、颜文字和贴图的分寸与场合

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: EmojiStickerUseRules,
    name: "表情包与表情符号使用礼仪",
    desc: "使用表情、颜文字和贴图的分寸与场合",
    origin: "国际",
    tags: ["社交", "礼仪", "表情", "贴图"]
}

impl EmojiStickerUseRules {
    /// 场合分寸
    pub fn occasion(&self) -> Vec<&'static str> {
        vec![
            "正式场合少用表情",
            "轻松聊天随意用",
            "配合语气不曲解",
            "尊重对方接受度",
        ]
    }

    /// 适度表达
    pub fn moderation(&self) -> Vec<&'static str> {
        vec![
            "表情辅助不替代",
            "不连发大量表情",
            "不随意用争吵性图标",
            "含义清楚不误读",
        ]
    }

    /// 贴图使用
    pub fn sticker(&self) -> Vec<&'static str> {
        vec![
            "用符合语境的贴图",
            "避免冒犯性内容",
            "不发含敏感图",
            "不随意转发他人表情",
        ]
    }

    /// 善用沟通
    pub fn communicate(&self) -> Vec<&'static str> {
        vec![
            "重要信息不靠表情",
            "需说明当用文字",
            "多表达真诚",
            "表情增进温度",
        ]
    }
}

impl Rule for EmojiStickerUseRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("emoji")
    }

    fn explain(&self) -> String {
        format!(
            "【表情包与表情符号使用礼仪】\n{}",
            [
                format!(
                    "场合分寸：\\n{}",
                    self.occasion()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "适度表达：\\n{}",
                    self.moderation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "贴图使用：\\n{}",
                    self.sticker()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "善用沟通：\\n{}",
                    self.communicate()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
            ]
            .join("\n\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_emojistickeruserules_basic() {
        let rules = EmojiStickerUseRules::new();
        assert_eq!(rules.metadata().name, "表情包与表情符号使用礼仪");
        assert!(!rules.occasion().is_empty());
        assert!(!rules.moderation().is_empty());
        assert!(!rules.sticker().is_empty());
        assert!(!rules.communicate().is_empty());
    }

    #[test]
    fn test_emojistickeruserules_validation() {
        let rules = EmojiStickerUseRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("emoji"));
    }

    #[test]
    fn test_emojistickeruserules_explain() {
        let rules = EmojiStickerUseRules::new();
        let e = rules.explain();
        assert!(e.contains("场合分寸"));
        assert!(e.contains("适度表达"));
        assert!(e.contains("贴图使用"));
    }
}
