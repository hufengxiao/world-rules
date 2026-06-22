//! 商务礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 商务礼仪规则
pub struct BusinessEtiquette {
    metadata: RuleMetadata,
    region: String,
}

impl BusinessEtiquette {
    pub fn new(region: impl Into<String>) -> Self {
        let region = region.into();
        Self {
            metadata: RuleMetadata::new(format!("{}商务礼仪", region), "商务场合礼仪规范")
                .with_origin(region.clone()),
            region,
        }
    }

    /// 获取会议礼仪
    pub fn meeting_etiquette(&self) -> Vec<&'static str> {
        vec![
            "准时参会",
            "手机静音",
            "认真聆听",
            "适时发言",
            "尊重他人观点",
        ]
    }

    /// 获取握手礼仪
    pub fn handshake_rules(&self) -> Vec<&'static str> {
        vec![
            "握手力度适中",
            "眼神交流",
            "保持微笑",
            "握手时间2-3秒",
            "长辈/上级先伸手",
        ]
    }

    /// 获取名片礼仪
    pub fn business_card_rules(&self) -> Vec<&'static str> {
        vec![
            "双手递接名片",
            "认真查看名片",
            "妥善收好名片",
            "不要在名片上写字",
        ]
    }
}

impl Default for BusinessEtiquette {
    fn default() -> Self {
        Self::new("通用")
    }
}

impl Rule for BusinessEtiquette {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("business")
    }

    fn explain(&self) -> String {
        format!(
            "【{}商务礼仪】\n\n\
            会议礼仪:\n{}\n\n\
            握手礼仪:\n{}\n\n\
            名片礼仪:\n{}\n",
            self.region,
            self.meeting_etiquette()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.handshake_rules()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.business_card_rules()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_business_etiquette_china() {
        let rules = BusinessEtiquette::new("中国");
        assert!(!rules.metadata().name.is_empty());
        assert!(!rules.meeting_etiquette().is_empty());
        assert!(!rules.handshake_rules().is_empty());
        assert!(!rules.business_card_rules().is_empty());
    }

    #[test]
    fn test_business_etiquette_different_regions() {
        let china = BusinessEtiquette::new("中国");
        let japan = BusinessEtiquette::new("日本");
        // 不同地区的规则描述应不同
        assert_ne!(china.metadata().name, japan.metadata().name);
    }

    #[test]
    fn test_business_rule_trait() {
        let rules = BusinessEtiquette::new("中国");
        assert_eq!(rules.category(), RuleCategory::social("business"));
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert!(!rules.explain().is_empty());
    }
}
