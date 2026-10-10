//! 神经信号传导
//!
//! 神经冲动以电信号与递质传递的生理机制

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: NerveSignalTransmissionRules,
    name: "神经信号传导",
    desc: "神经冲动以电信号与递质传递的生理机制",
    origin: "科学常识",
    tags: ["生理", "神经", "机制"]
}

impl NerveSignalTransmissionRules {
    /// 静息电位
    pub fn resting(&self) -> Vec<&'static str> {
        vec![
            "神经元内外电位差",
            "膜内负膜外正",
            "由离子浓度差维持",
            "约负七十毫伏",
        ]
    }

    /// 动作电位
    pub fn action_potential(&self) -> Vec<&'static str> {
        vec![
            "刺激使膜电位反转",
            "钠离子内流形成去极化",
            "电位骤升后复极",
            "冲动沿轴突传导",
        ]
    }

    /// 突触传递
    pub fn synapse(&self) -> Vec<&'static str> {
        vec![
            "冲动抵达轴突末梢",
            "释放神经递质",
            "递质跨越突触间隙",
            "结合受体引发下级冲动",
        ]
    }

    /// 信号特点
    pub fn features(&self) -> Vec<&'static str> {
        vec![
            "全或无式发放",
            "单向传导不逆行",
            "有髓纤维传导更快",
            "递质决定兴奋或抑制",
        ]
    }
}

impl Rule for NerveSignalTransmissionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("nerve_signal_transmission")
    }

    fn explain(&self) -> String {
        format!(
            "【神经信号传导】\n{}",
            [
                format!(
                    "静息电位：\\n{}",
                    self.resting()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "动作电位：\\n{}",
                    self.action_potential()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "突触传递：\\n{}",
                    self.synapse()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "信号特点：\\n{}",
                    self.features()
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
    fn test_nervesignaltransmissionrules_basic() {
        let rules = NerveSignalTransmissionRules::new();
        assert_eq!(rules.metadata().name, "神经信号传导");
        assert!(!rules.resting().is_empty());
        assert!(!rules.action_potential().is_empty());
        assert!(!rules.synapse().is_empty());
        assert!(!rules.features().is_empty());
    }

    #[test]
    fn test_nervesignaltransmissionrules_validation() {
        let rules = NerveSignalTransmissionRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::science("nerve_signal_transmission")
        );
    }

    #[test]
    fn test_nervesignaltransmissionrules_explain() {
        let rules = NerveSignalTransmissionRules::new();
        let e = rules.explain();
        assert!(e.contains("静息电位"));
        assert!(e.contains("动作电位"));
        assert!(e.contains("突触传递"));
    }
}
