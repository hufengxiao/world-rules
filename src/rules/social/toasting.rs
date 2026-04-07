//! 敬酒礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 敬酒礼仪
pub struct ToastingEtiquette {
    metadata: RuleMetadata,
}

impl ToastingEtiquette {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "敬酒礼仪",
                "中国传统敬酒礼仪规范"
            )
            .with_origin("中国")
            .with_tags(vec!["社交".into(), "餐饮".into()]),
        }
    }

    /// 敬酒顺序
    pub fn toasting_order(&self) -> Vec<&'static str> {
        vec![
            "主人先向主宾敬酒",
            "按年龄、职位、身份顺序",
            "长辈先敬晚辈后敬",
            "主宾敬酒后再互相敬酒",
            "平辈之间可自由敬酒",
        ]
    }

    /// 敬酒姿势
    pub fn posture(&self) -> Vec<&'static str> {
        vec![
            "双手端杯",
            "起身站立(被敬者可坐)",
            "面对被敬者",
            "杯口略低于对方杯口",
            "眼睛注视对方",
        ]
    }

    /// 敬酒语言
    pub fn toasting_words(&self) -> Vec<&'static str> {
        vec![
            "先说祝福语",
            "感谢对方的帮助或支持",
            "表达敬意",
            "语言真诚简洁",
            "避免过于正式或随意",
        ]
    }

    /// 接受敬酒
    pub fn accepting_toast(&self) -> Vec<&'static str> {
        vec![
            "表示感谢",
            "可起身或坐着接受",
            "举杯示意",
            "说回应语",
            "适量饮酒",
        ]
    }

    /// 拒绝敬酒
    pub fn declining_toast(&self) -> Vec<&'static str> {
        vec![
            "礼貌解释原因",
            "可用茶水代替",
            "表示敬意但不饮酒",
            "避免直接说\"不喝\"",
            "敬酒人应尊重对方选择",
        ]
    }

    /// 酒量控制
    pub fn drinking_guidelines(&self) -> Vec<&'static str> {
        vec![
            "适量饮酒，不可过量",
            "醉酒失态是大忌",
            "可提前说明酒量",
            "代驾安排",
            "未成年人和孕妇不应饮酒",
        ]
    }

    /// 碰杯规则
    pub fn clinking_rules(&self) -> Vec<&'static str> {
        vec![
            "双方同时举杯",
            "杯口碰撞",
            "发出清脆声音",
            "同时饮酒",
            "注视对方眼睛",
        ]
    }
}

impl Default for ToastingEtiquette {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ToastingEtiquette {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("toasting")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【敬酒礼仪】\n\n\
            敬酒顺序:\n{}\n\n\
            敬酒姿势:\n{}\n\n\
            接受敬酒:\n{}\n\n\
            酒量控制:\n{}\n",
            self.toasting_order().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.posture().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.accepting_toast().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.drinking_guidelines().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toasting_etiquette() {
        let rules = ToastingEtiquette::new();
        assert!(!rules.toasting_order().is_empty());
    }
}