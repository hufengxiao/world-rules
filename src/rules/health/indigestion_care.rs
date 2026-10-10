//! 消化不良调理
//!
//! 餐后上腹饱胀不适的饮食与生活习惯调理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: IndigestionCareRules,
    name: "消化不良调理",
    desc: "餐后上腹饱胀不适的饮食与生活习惯调理",
    origin: "中国",
    tags: ["消化", "饮食", "调理"]
}

impl IndigestionCareRules {
    /// 饮食调整
    pub fn diet(&self) -> Vec<&'static str> {
        vec![
            "少食多餐减轻胃负担",
            "细嚼慢咽每口多嚼",
            "避免油腻辛辣生冷",
            "饭后不宜立即躺卧",
        ]
    }

    /// 进食习惯
    pub fn habits(&self) -> Vec<&'static str> {
        vec![
            "定时定量规律三餐",
            "进餐时心情放松",
            "勿边走边吃边说话",
            "戒烟限酒减少刺激",
        ]
    }

    /// 生活调理
    pub fn lifestyle(&self) -> Vec<&'static str> {
        vec![
            "餐后缓步走动助消化",
            "避免餐后剧烈运动",
            "规律作息减轻压力",
            "腹部保暖避免受凉",
        ]
    }

    /// 何时就医
    pub fn seeking_help(&self) -> Vec<&'static str> {
        vec![
            "持续数周不缓解就医",
            "体重下降须警惕",
            "黑便呕血立即就诊",
            "夜间痛醒应查因",
        ]
    }
}

impl Rule for IndigestionCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("indigestion_care")
    }

    fn explain(&self) -> String {
        format!(
            "【消化不良调理】\n{}",
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
                    "进食习惯：\\n{}",
                    self.habits()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "生活调理：\\n{}",
                    self.lifestyle()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "何时就医：\\n{}",
                    self.seeking_help()
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
    fn test_indigestioncarerules_basic() {
        let rules = IndigestionCareRules::new();
        assert_eq!(rules.metadata().name, "消化不良调理");
        assert!(!rules.diet().is_empty());
        assert!(!rules.habits().is_empty());
        assert!(!rules.lifestyle().is_empty());
        assert!(!rules.seeking_help().is_empty());
    }

    #[test]
    fn test_indigestioncarerules_validation() {
        let rules = IndigestionCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("indigestion_care"));
    }

    #[test]
    fn test_indigestioncarerules_explain() {
        let rules = IndigestionCareRules::new();
        let e = rules.explain();
        assert!(e.contains("饮食调整"));
        assert!(e.contains("进食习惯"));
        assert!(e.contains("生活调理"));
    }
}
