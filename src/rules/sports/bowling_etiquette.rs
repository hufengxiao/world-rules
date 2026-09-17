//! 保龄球礼仪与规则
//!
//! 保龄球记分、轮流与场地礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BowlingEtiquetteRules,
    name: "保龄球礼仪与规则",
    desc: "保龄球记分、轮流与场地礼仪",
    origin: "国际",
    tags: ["体育", "保龄球", "礼仪"]
}

impl BowlingEtiquetteRules {
    /// 轮流投球
    pub fn taking(&self) -> Vec<&'static str> {
        vec![
            "按球道顺序依次投球",
            "前一人投完再上",
            "每轮记录两次机会",
            "尊重他人节奏",
        ]
    }

    /// 投球要领
    pub fn throw(&self) -> Vec<&'static str> {
        vec![
            "从发球线助跑投球",
            "瞄准球瓶稳准发力",
            "不越线不干扰",
            "控制球速适顺",
        ]
    }

    /// 计分常识
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "每次击倒的瓶记分",
            "补中加下一投奖励",
            "全倒加后两次奖励",
            "总分竞高者胜",
        ]
    }

    /// 场地礼仪
    pub fn manner(&self) -> Vec<&'static str> {
        vec![
            "换专用球鞋上场",
            "不占他人球道",
            "投后令退避让",
            "尊重他人发挥",
        ]
    }
}

impl Rule for BowlingEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("bowling")
    }

    fn explain(&self) -> String {
        format!(
            "【保龄球礼仪与规则】\n{}",
            [
                format!(
                    "轮流投球：\\n{}",
                    self.taking()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "投球要领：\\n{}",
                    self.throw()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "计分常识：\\n{}",
                    self.scoring()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "场地礼仪：\\n{}",
                    self.manner()
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
    fn test_bowlingetiquetterules_basic() {
        let rules = BowlingEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "保龄球礼仪与规则");
        assert!(!rules.taking().is_empty());
        assert!(!rules.throw().is_empty());
        assert!(!rules.scoring().is_empty());
        assert!(!rules.manner().is_empty());
    }

    #[test]
    fn test_bowlingetiquetterules_validation() {
        let rules = BowlingEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::sports("bowling"));
    }

    #[test]
    fn test_bowlingetiquetterules_explain() {
        let rules = BowlingEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("轮流投球"));
        assert!(e.contains("投球要领"));
        assert!(e.contains("计分常识"));
    }
}
