//! 跳绳健身规范
//!
//! 跳绳热身、动作要领与安全量力锻炼规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: JumpRopeFitnessRules,
    name: "跳绳健身规范",
    desc: "跳绳热身、动作要领与安全量力锻炼规则",
    origin: "国际",
    tags: ["体育", "跳绳", "健身"]
}

impl JumpRopeFitnessRules {
    /// 动作要领
    pub fn form(&self) -> Vec<&'static str> {
        vec!["手腕转动摇绳", "蹬踏轻跳着地", "身体挺直微屈膝", "节奏均匀"]
    }

    /// 合理规划
    pub fn plan(&self) -> Vec<&'static str> {
        vec![
            "循序渐进增加时长",
            "穿着合适的鞋",
            "地面平整避震荡",
            "量力而行",
        ]
    }

    /// 热身放松
    pub fn warmup(&self) -> Vec<&'static str> {
        vec![
            "运动前充分热身",
            "踝膝腕活动开",
            "结束后拉伸放松",
            "身体不适停止",
        ]
    }

    /// 场地礼仪
    pub fn etiquette(&self) -> Vec<&'static str> {
        vec![
            "宽敞空间防甩绳",
            "避开他人衣物",
            "不扰户安静区",
            "爱护公共场地",
        ]
    }
}

impl Rule for JumpRopeFitnessRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("jump_rope")
    }

    fn explain(&self) -> String {
        format!(
            "【跳绳健身规范】\n{}",
            [
                format!(
                    "动作要领：\\n{}",
                    self.form()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "合理规划：\\n{}",
                    self.plan()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "热身放松：\\n{}",
                    self.warmup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "场地礼仪：\\n{}",
                    self.etiquette()
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
    fn test_jumpropefitnessrules_basic() {
        let rules = JumpRopeFitnessRules::new();
        assert_eq!(rules.metadata().name, "跳绳健身规范");
        assert!(!rules.form().is_empty());
        assert!(!rules.plan().is_empty());
        assert!(!rules.warmup().is_empty());
        assert!(!rules.etiquette().is_empty());
    }

    #[test]
    fn test_jumpropefitnessrules_validation() {
        let rules = JumpRopeFitnessRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::sports("jump_rope"));
    }

    #[test]
    fn test_jumpropefitnessrules_explain() {
        let rules = JumpRopeFitnessRules::new();
        let e = rules.explain();
        assert!(e.contains("动作要领"));
        assert!(e.contains("合理规划"));
        assert!(e.contains("热身放松"));
    }
}
