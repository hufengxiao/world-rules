//! 插话与接话
//!
//! 有话要插时先礼后言不打断他人的分寸

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: InterruptingPolitelyRules,
    name: "插话与接话",
    desc: "有话要插时先礼后言不打断他人的分寸",
    origin: "中国",
    tags: ["社交", "插话", "礼貌", "沟通"]
}

impl InterruptingPolitelyRules {
    /// 先等时机
    pub fn wait(&self) -> Vec<&'static str> {
        vec![
            "等对方告一段落",
            "别抢话打断",
            "认真听完再开口",
            "留意停顿空隙",
        ]
    }

    /// 示意插言
    pub fn signal(&self) -> Vec<&'static str> {
        vec!["说打扰一下", "不好意思插句嘴", "举手示意", "简短说明来意"]
    }

    /// 插言方式
    pub fn manner(&self) -> Vec<&'static str> {
        vec!["简明扼要", "不拉扯话题", "不抢对方台词", "说完回归原题"]
    }

    /// 被插话时
    pub fn receive(&self) -> Vec<&'static str> {
        vec![
            "对方插话先停下来",
            "礼貌接应",
            "稍后再续原话",
            "不发火不争抢",
        ]
    }
}

impl Rule for InterruptingPolitelyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("interrupt")
    }

    fn explain(&self) -> String {
        format!(
            "【插话与接话】\n{}",
            [
                format!(
                    "先等时机：\\n{}",
                    self.wait()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "示意插言：\\n{}",
                    self.signal()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "插言方式：\\n{}",
                    self.manner()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "被插话时：\\n{}",
                    self.receive()
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
    fn test_interruptingpolitelyrules_basic() {
        let rules = InterruptingPolitelyRules::new();
        assert_eq!(rules.metadata().name, "插话与接话");
        assert!(!rules.wait().is_empty());
        assert!(!rules.signal().is_empty());
        assert!(!rules.manner().is_empty());
        assert!(!rules.receive().is_empty());
    }

    #[test]
    fn test_interruptingpolitelyrules_validation() {
        let rules = InterruptingPolitelyRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("interrupt"));
    }

    #[test]
    fn test_interruptingpolitelyrules_explain() {
        let rules = InterruptingPolitelyRules::new();
        let e = rules.explain();
        assert!(e.contains("先等时机"));
        assert!(e.contains("示意插言"));
        assert!(e.contains("插言方式"));
    }
}
