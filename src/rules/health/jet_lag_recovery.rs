//! 倒时差调理
//!
//! 跨时区旅行后生物钟紊乱的快速调整方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: JetLagRecoveryRules,
    name: "倒时差调理",
    desc: "跨时区旅行后生物钟紊乱的快速调整方法",
    origin: "中国",
    tags: ["睡眠", "旅行", "生物钟"]
}

impl JetLagRecoveryRules {
    /// 飞行前准备
    pub fn preparation(&self) -> Vec<&'static str> {
        vec![
            "提前数日逐步调整作息",
            "按目的地时间安排睡眠",
            "飞行中多饮水少饮酒",
            "备好眼罩耳塞",
        ]
    }

    /// 到达后调整
    pub fn adjustment(&self) -> Vec<&'static str> {
        vec![
            "白天多晒太阳",
            "按当地时间用餐作息",
            "白天不打盹坚持清醒",
            "傍晚适度活动",
        ]
    }

    /// 光照利用
    pub fn light(&self) -> Vec<&'static str> {
        vec![
            "晨光有助提前生物钟",
            "傍晚避强光利于延后",
            "依向东向西调整",
            "光照是最有效手段",
        ]
    }

    /// 辅助方法
    pub fn assistance(&self) -> Vec<&'static str> {
        vec![
            "短期可用褪黑素",
            "避免依赖安眠药",
            "规律运动助调节",
            "一般数日可适应",
        ]
    }
}

impl Rule for JetLagRecoveryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("jet_lag_recovery")
    }

    fn explain(&self) -> String {
        format!(
            "【倒时差调理】\n{}",
            [
                format!(
                    "飞行前准备：\\n{}",
                    self.preparation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "到达后调整：\\n{}",
                    self.adjustment()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "光照利用：\\n{}",
                    self.light()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "辅助方法：\\n{}",
                    self.assistance()
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
    fn test_jetlagrecoveryrules_basic() {
        let rules = JetLagRecoveryRules::new();
        assert_eq!(rules.metadata().name, "倒时差调理");
        assert!(!rules.preparation().is_empty());
        assert!(!rules.adjustment().is_empty());
        assert!(!rules.light().is_empty());
        assert!(!rules.assistance().is_empty());
    }

    #[test]
    fn test_jetlagrecoveryrules_validation() {
        let rules = JetLagRecoveryRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("jet_lag_recovery"));
    }

    #[test]
    fn test_jetlagrecoveryrules_explain() {
        let rules = JetLagRecoveryRules::new();
        let e = rules.explain();
        assert!(e.contains("飞行前准备"));
        assert!(e.contains("到达后调整"));
        assert!(e.contains("光照利用"));
    }
}
