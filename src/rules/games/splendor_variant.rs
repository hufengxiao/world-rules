//! 璀璨宝石
//!
//! 收集宝石筹码购买卡牌的引擎构筑桌游

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SplendorVariantRules,
    name: "璀璨宝石",
    desc: "收集宝石筹码购买卡牌的引擎构筑桌游",
    origin: "法国",
    tags: ["桌游", "策略", "引擎"]
}

impl SplendorVariantRules {
    /// 开局布置
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "按人数摆放宝石筹码",
            "三档发展卡分列展示",
            "贵族卡置于一旁",
            "确定起始玩家",
        ]
    }

    /// 回合行动
    pub fn turn(&self) -> Vec<&'static str> {
        vec![
            "可选三种不同宝石各一枚",
            "或取同色两枚需满四枚",
            "或购买一张展示卡",
            "或预定一张卡并取黄金",
        ]
    }

    /// 购买发展
    pub fn buying(&self) -> Vec<&'static str> {
        vec![
            "用宝石购买发展卡",
            "卡上宝石可永久抵扣",
            "黄金可代任意宝石",
            "筹码上限十枚",
        ]
    }

    /// 贵族拜访
    pub fn noble(&self) -> Vec<&'static str> {
        vec![
            "发展卡满足贵族要求即访",
            "贵族提供额外分数",
            "每次仅可获一位",
            "自动获得无需行动",
        ]
    }

    /// 胜负结束
    pub fn ending(&self) -> Vec<&'static str> {
        vec![
            "先达十五分者获胜",
            "同分者购卡少者胜",
            "发展卡与贵族计分",
            "结束当轮决出胜负",
        ]
    }
}

impl Rule for SplendorVariantRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("splendor_variant")
    }

    fn explain(&self) -> String {
        format!(
            "【璀璨宝石】\n{}",
            [
                format!(
                    "开局布置：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "回合行动：\\n{}",
                    self.turn()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "购买发展：\\n{}",
                    self.buying()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "贵族拜访：\\n{}",
                    self.noble()
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
    fn test_splendorvariantrules_basic() {
        let rules = SplendorVariantRules::new();
        assert_eq!(rules.metadata().name, "璀璨宝石");
        assert!(!rules.setup().is_empty());
        assert!(!rules.turn().is_empty());
        assert!(!rules.buying().is_empty());
        assert!(!rules.noble().is_empty());
        assert!(!rules.ending().is_empty());
    }

    #[test]
    fn test_splendorvariantrules_validation() {
        let rules = SplendorVariantRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("splendor_variant"));
    }

    #[test]
    fn test_splendorvariantrules_explain() {
        let rules = SplendorVariantRules::new();
        let e = rules.explain();
        assert!(e.contains("开局布置"));
        assert!(e.contains("回合行动"));
        assert!(e.contains("购买发展"));
    }
}
