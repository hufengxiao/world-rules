//! 游泳泳姿基础
//!
//! 四种泳姿要领、呼吸与泳池安全的基本要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SwimmingStrokeBasicsRules,
    name: "游泳泳姿基础",
    desc: "四种泳姿要领、呼吸与泳池安全的基本要点",
    origin: "国际",
    tags: ["体育", "游泳", "泳姿", "安全"]
}

impl SwimmingStrokeBasicsRules {
    /// 基本泳姿
    pub fn strokes(&self) -> Vec<&'static str> {
        vec![
            "蛙泳重腿蹬夹与划",
            "自由泳打腿与换气配合",
            "仰泳仰面划手",
            "蝶泳双臂同步打腿",
        ]
    }

    /// 呼吸技巧
    pub fn breath(&self) -> Vec<&'static str> {
        vec![
            "协调换气避免呛水",
            "呼气入水吸气出水",
            "自由泳侧向转头换气",
            "长距离规律呼吸",
        ]
    }

    /// 场地安全
    pub fn safety(&self) -> Vec<&'static str> {
        vec![
            "在安全泳区游泳",
            "不独自无救生下水",
            "热身充分再入水",
            "身体不适停止",
        ]
    }

    /// 礼仪共享
    pub fn manner(&self) -> Vec<&'static str> {
        vec![
            "各泳道各行其道",
            "不让道冲撞他人",
            "慢泳靠边礼让",
            "不在池边奔跑",
        ]
    }
}

impl Rule for SwimmingStrokeBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("swimming_stroke")
    }

    fn explain(&self) -> String {
        format!(
            "【游泳泳姿基础】\n{}",
            [
                format!(
                    "基本泳姿：\\n{}",
                    self.strokes()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "呼吸技巧：\\n{}",
                    self.breath()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "场地安全：\\n{}",
                    self.safety()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "礼仪共享：\\n{}",
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
    fn test_swimmingstrokebasicsrules_basic() {
        let rules = SwimmingStrokeBasicsRules::new();
        assert_eq!(rules.metadata().name, "游泳泳姿基础");
        assert!(!rules.strokes().is_empty());
        assert!(!rules.breath().is_empty());
        assert!(!rules.safety().is_empty());
        assert!(!rules.manner().is_empty());
    }

    #[test]
    fn test_swimmingstrokebasicsrules_validation() {
        let rules = SwimmingStrokeBasicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::sports("swimming_stroke"));
    }

    #[test]
    fn test_swimmingstrokebasicsrules_explain() {
        let rules = SwimmingStrokeBasicsRules::new();
        let e = rules.explain();
        assert!(e.contains("基本泳姿"));
        assert!(e.contains("呼吸技巧"));
        assert!(e.contains("场地安全"));
    }
}
