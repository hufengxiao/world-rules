//! 经前综合征调理
//!
//! 经前综合征的饮食、情绪与症状缓解调理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PremenstrualSyndromeCareRules,
    name: "经前综合征调理",
    desc: "经前综合征的饮食、情绪与症状缓解调理",
    origin: "医学",
    tags: ["健康", "经前综合征", "女性", "调理"]
}

impl PremenstrualSyndromeCareRules {
    /// 饮食调整
    pub fn diet(&self) -> Vec<&'static str> {
        vec![
            "摄入充足复合碳水",
            "限盐限咖啡因降肿",
            "富含钙镁蔬果",
            "少甜食与精致",
        ]
    }

    /// 情绪关照
    pub fn mood(&self) -> Vec<&'static str> {
        vec![
            "承认情绪波动正常",
            "适度倾诉与人分享",
            "练习放松与深呼吸",
            "充足睡眠稳定情绪",
        ]
    }

    /// 运动舒缓
    pub fn exercise(&self) -> Vec<&'static str> {
        vec![
            "中低强度规律运动",
            "舒缓拉伸减轻不适",
            "散步有氧改善疲劳",
            "量力而行不过度",
        ]
    }

    /// 求助就医
    pub fn seek_care(&self) -> Vec<&'static str> {
        vec![
            "症状影响生活就医",
            "疼痛剧烈及时评估",
            "情绪异常重就医",
            "遵医嘱规范调理",
        ]
    }
}

impl Rule for PremenstrualSyndromeCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("pms_care")
    }

    fn explain(&self) -> String {
        format!(
            "【经前综合征调理】\n{}",
            [
                format!(
                    "饮食调整：\\n{}",
                    self.diet()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "情绪关照：\\n{}",
                    self.mood()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "运动舒缓：\\n{}",
                    self.exercise()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "求助就医：\\n{}",
                    self.seek_care()
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
    fn test_premenstrualsyndromecarerules_basic() {
        let rules = PremenstrualSyndromeCareRules::new();
        assert_eq!(rules.metadata().name, "经前综合征调理");
        assert!(!rules.diet().is_empty());
        assert!(!rules.mood().is_empty());
        assert!(!rules.exercise().is_empty());
        assert!(!rules.seek_care().is_empty());
    }

    #[test]
    fn test_premenstrualsyndromecarerules_validation() {
        let rules = PremenstrualSyndromeCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("pms_care"));
    }

    #[test]
    fn test_premenstrualsyndromecarerules_explain() {
        let rules = PremenstrualSyndromeCareRules::new();
        let e = rules.explain();
        assert!(e.contains("饮食调整"));
        assert!(e.contains("情绪关照"));
        assert!(e.contains("运动舒缓"));
    }
}
