//! 久坐健康防护
//!
//! 减少久坐危害的间歇活动与体态调整规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ProlongedSittingSafetyRules,
    name: "久坐健康防护",
    desc: "减少久坐危害的间歇活动与体态调整规则",
    origin: "医学",
    tags: ["健康", "久坐", "体态", "活动"]
}

impl ProlongedSittingSafetyRules {
    /// 定时起身
    pub fn break_time(&self) -> Vec<&'static str> {
        vec![
            "每约一小时起身活动",
            "短暂站立伸伸腰背",
            "走动促循环",
            "避免连续久坐数小时",
        ]
    }

    /// 体态调整
    pub fn posture(&self) -> Vec<&'static str> {
        vec![
            "保持腰背挺直坐姿",
            "屏幕与视线平齐",
            "双手自然承托",
            "适时站立工作",
        ]
    }

    /// 活动锻炼
    pub fn activity(&self) -> Vec<&'static str> {
        vec![
            "每日坚持适度运动",
            "拉伸放松肩颈腰",
            "穿插站立与走动",
            "善用肢体锻炼",
        ]
    }

    /// 风险关注
    pub fn risk(&self) -> Vec<&'static str> {
        vec![
            "留意下肢肿胀不适",
            "重视腰椎久痛",
            "久坐人群定期活动",
            "不良反应就医评估",
        ]
    }
}

impl Rule for ProlongedSittingSafetyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("prolonged_sitting")
    }

    fn explain(&self) -> String {
        format!(
            "【久坐健康防护】\n{}",
            [
                format!(
                    "定时起身：\\n{}",
                    self.break_time()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "体态调整：\\n{}",
                    self.posture()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "活动锻炼：\\n{}",
                    self.activity()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "风险关注：\\n{}",
                    self.risk()
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
    fn test_prolongedsittingsafetyrules_basic() {
        let rules = ProlongedSittingSafetyRules::new();
        assert_eq!(rules.metadata().name, "久坐健康防护");
        assert!(!rules.break_time().is_empty());
        assert!(!rules.posture().is_empty());
        assert!(!rules.activity().is_empty());
        assert!(!rules.risk().is_empty());
    }

    #[test]
    fn test_prolongedsittingsafetyrules_validation() {
        let rules = ProlongedSittingSafetyRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("prolonged_sitting"));
    }

    #[test]
    fn test_prolongedsittingsafetyrules_explain() {
        let rules = ProlongedSittingSafetyRules::new();
        let e = rules.explain();
        assert!(e.contains("定时起身"));
        assert!(e.contains("体态调整"));
        assert!(e.contains("活动锻炼"));
    }
}
