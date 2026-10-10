//! 沙包洞
//!
//! 投掷沙包入洞的户外派对游戏，美式后院经典

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CornHoleRules,
    name: "沙包洞",
    desc: "投掷沙包入洞的户外派对游戏，美式后院经典",
    origin: "美国",
    tags: ["户外", "投掷", "派对"]
}

impl CornHoleRules {
    /// 场地布置
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "两块带洞斜板相距数米",
            "每队四人分站两端",
            "每队备四个沙包",
            "板面倾斜便于落洞",
        ]
    }

    /// 投掷规则
    pub fn throwing(&self) -> Vec<&'static str> {
        vec![
            "轮流投掷沙包",
            "从己方板后投掷",
            "越过对方板线",
            "入洞或落板得分",
        ]
    }

    /// 计分方式
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "入洞计三分",
            "落板计一分",
            "双方抵消后净分计",
            "仅净高分方得分",
        ]
    }

    /// 胜负结束
    pub fn ending(&self) -> Vec<&'static str> {
        vec![
            "先达二十一分者胜",
            "须正好达到或超分",
            "轮流交换投掷顺序",
            "每轮两队各投四包",
        ]
    }
}

impl Rule for CornHoleRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("corn_hole")
    }

    fn explain(&self) -> String {
        format!(
            "【沙包洞】\n{}",
            [
                format!(
                    "场地布置：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "投掷规则：\\n{}",
                    self.throwing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "计分方式：\\n{}",
                    self.scoring()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "胜负结束：\\n{}",
                    self.ending()
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
    fn test_cornholerules_basic() {
        let rules = CornHoleRules::new();
        assert_eq!(rules.metadata().name, "沙包洞");
        assert!(!rules.setup().is_empty());
        assert!(!rules.throwing().is_empty());
        assert!(!rules.scoring().is_empty());
        assert!(!rules.ending().is_empty());
    }

    #[test]
    fn test_cornholerules_validation() {
        let rules = CornHoleRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("corn_hole"));
    }

    #[test]
    fn test_cornholerules_explain() {
        let rules = CornHoleRules::new();
        let e = rules.explain();
        assert!(e.contains("场地布置"));
        assert!(e.contains("投掷规则"));
        assert!(e.contains("计分方式"));
    }
}
