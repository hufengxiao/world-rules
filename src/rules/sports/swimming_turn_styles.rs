//! 游泳转身要领
//!
//! 自由泳蛙泳翻滚转身与规则技术

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SwimmingTurnStylesRules,
    name: "游泳转身要领",
    desc: "自由泳蛙泳翻滚转身与规则技术",
    origin: "国际",
    tags: ["体育", "游泳", "转身"]
}

impl SwimmingTurnStylesRules {
    /// 自由泳滚翻
    pub fn flip(&self) -> Vec<&'static str> {
        vec!["接近池壁收身", "前滚翻转身", "双脚蹬壁", "流线滑出"]
    }

    /// 蛙泳转身
    pub fn breast(&self) -> Vec<&'static str> {
        vec!["双手触壁", "转身蹬离", "水面转头", "保持节奏"]
    }

    /// 蹬壁出发
    pub fn push(&self) -> Vec<&'static str> {
        vec!["蹬壁用力", "身体伸直滑行", "双臂前伸", "减少阻力"]
    }

    /// 比赛规则
    pub fn regulation(&self) -> Vec<&'static str> {
        vec!["必须触池壁转身", "泳姿规范", "不得越线借力", "遵守泳规"]
    }
}

impl Rule for SwimmingTurnStylesRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("swim_turn")
    }

    fn explain(&self) -> String {
        format!(
            "【游泳转身要领】\n{}",
            [
                format!(
                    "自由泳滚翻：\\n{}",
                    self.flip()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "蛙泳转身：\\n{}",
                    self.breast()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "蹬壁出发：\\n{}",
                    self.push()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "比赛规则：\\n{}",
                    self.regulation()
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
    fn test_swimmingturnstylesrules_basic() {
        let rules = SwimmingTurnStylesRules::new();
        assert_eq!(rules.metadata().name, "游泳转身要领");
        assert!(!rules.flip().is_empty());
        assert!(!rules.breast().is_empty());
        assert!(!rules.push().is_empty());
        assert!(!rules.regulation().is_empty());
    }

    #[test]
    fn test_swimmingturnstylesrules_validation() {
        let rules = SwimmingTurnStylesRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::sports("swim_turn"));
    }

    #[test]
    fn test_swimmingturnstylesrules_explain() {
        let rules = SwimmingTurnStylesRules::new();
        let e = rules.explain();
        assert!(e.contains("自由泳滚翻"));
        assert!(e.contains("蛙泳转身"));
        assert!(e.contains("蹬壁出发"));
    }
}
