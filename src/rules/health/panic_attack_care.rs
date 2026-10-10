//! 惊恐发作应对
//!
//! 突发惊恐发作时的自我平复与长期应对

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PanicAttackCareRules,
    name: "惊恐发作应对",
    desc: "突发惊恐发作时的自我平复与长期应对",
    origin: "中国",
    tags: ["心理", "焦虑", "应对"]
}

impl PanicAttackCareRules {
    /// 发作表现
    pub fn symptoms(&self) -> Vec<&'static str> {
        vec![
            "突发心悸胸闷",
            "呼吸急促濒死感",
            "手脚发麻头晕",
            "多在十分钟内达峰",
        ]
    }

    /// 即时应对
    pub fn immediate(&self) -> Vec<&'static str> {
        vec![
            "提醒自己会过去",
            "缓慢深呼吸调息",
            "找安静处坐下",
            "专注数数或触感",
        ]
    }

    /// 呼吸调节
    pub fn breathing(&self) -> Vec<&'static str> {
        vec![
            "用腹式呼吸缓慢",
            "吸气四秒屏息片刻",
            "缓慢呼气六至八秒",
            "避免过度换气",
        ]
    }

    /// 长期管理
    pub fn long_term(&self) -> Vec<&'static str> {
        vec![
            "规律作息减少诱因",
            "限制咖啡因摄入",
            "学习放松训练",
            "反复发作应就医",
        ]
    }
}

impl Rule for PanicAttackCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("panic_attack_care")
    }

    fn explain(&self) -> String {
        format!(
            "【惊恐发作应对】\n{}",
            [
                format!(
                    "发作表现：\\n{}",
                    self.symptoms()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "即时应对：\\n{}",
                    self.immediate()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "呼吸调节：\\n{}",
                    self.breathing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "长期管理：\\n{}",
                    self.long_term()
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
    fn test_panicattackcarerules_basic() {
        let rules = PanicAttackCareRules::new();
        assert_eq!(rules.metadata().name, "惊恐发作应对");
        assert!(!rules.symptoms().is_empty());
        assert!(!rules.immediate().is_empty());
        assert!(!rules.breathing().is_empty());
        assert!(!rules.long_term().is_empty());
    }

    #[test]
    fn test_panicattackcarerules_validation() {
        let rules = PanicAttackCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("panic_attack_care"));
    }

    #[test]
    fn test_panicattackcarerules_explain() {
        let rules = PanicAttackCareRules::new();
        let e = rules.explain();
        assert!(e.contains("发作表现"));
        assert!(e.contains("即时应对"));
        assert!(e.contains("呼吸调节"));
    }
}
