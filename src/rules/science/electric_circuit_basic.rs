//! 电路基础知识
//!
//! 电路回路、串联并联与安全用电

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ElectricCircuitBasicRules,
    name: "电路基础知识",
    desc: "电路回路、串联并联与安全用电",
    origin: "物理",
    tags: ["科学", "电路", "物理", "安全"]
}

impl ElectricCircuitBasicRules {
    /// 回路组成
    pub fn parts(&self) -> Vec<&'static str> {
        vec![
            "电源导线用电器",
            "闭合回路电流通",
            "断开即断流",
            "构成完整电路",
        ]
    }

    /// 串并联
    pub fn config(&self) -> Vec<&'static str> {
        vec!["串联依次连接", "一处断全部停", "并联各自独立", "互不影响"]
    }

    /// 安全用电
    pub fn safety(&self) -> Vec<&'static str> {
        vec!["不湿手碰电", "不超载插座", "不用破损线", "谨防触电"]
    }

    /// 实际应用
    pub fn example(&self) -> Vec<&'static str> {
        vec!["家中的灯并联", "开关串在支路", "短路易发热", "注意防护"]
    }
}

impl Rule for ElectricCircuitBasicRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("circuit")
    }

    fn explain(&self) -> String {
        format!(
            "【电路基础知识】\n{}",
            [
                format!(
                    "回路组成：\\n{}",
                    self.parts()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "串并联：\\n{}",
                    self.config()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安全用电：\\n{}",
                    self.safety()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "实际应用：\\n{}",
                    self.example()
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
    fn test_electriccircuitbasicrules_basic() {
        let rules = ElectricCircuitBasicRules::new();
        assert_eq!(rules.metadata().name, "电路基础知识");
        assert!(!rules.parts().is_empty());
        assert!(!rules.config().is_empty());
        assert!(!rules.safety().is_empty());
        assert!(!rules.example().is_empty());
    }

    #[test]
    fn test_electriccircuitbasicrules_validation() {
        let rules = ElectricCircuitBasicRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("circuit"));
    }

    #[test]
    fn test_electriccircuitbasicrules_explain() {
        let rules = ElectricCircuitBasicRules::new();
        let e = rules.explain();
        assert!(e.contains("回路组成"));
        assert!(e.contains("串并联"));
        assert!(e.contains("安全用电"));
    }
}
