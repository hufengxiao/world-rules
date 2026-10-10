//! 骨折急救
//!
//! 疑似骨折时的固定与搬运急救方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: FractureFirstAidRules,
    name: "骨折急救",
    desc: "疑似骨折时的固定与搬运急救方法",
    origin: "中国",
    tags: ["急救", "骨折", "应急"]
}

impl FractureFirstAidRules {
    /// 识别判断
    pub fn recognition(&self) -> Vec<&'static str> {
        vec![
            "局部剧痛肿胀",
            "肢体畸形异常活动",
            "活动受限骨擦音",
            "开放性伤口见骨",
        ]
    }

    /// 固定处理
    pub fn immobilize(&self) -> Vec<&'static str> {
        vec![
            "就地固定不随意搬动",
            "用夹板固定上下关节",
            "无夹板可用硬物替代",
            "固定松紧适度",
        ]
    }

    /// 开放骨折
    pub fn open_fracture(&self) -> Vec<&'static str> {
        vec!["不将外露骨复位", "覆盖清洁敷料", "压迫止血包扎", "尽快送医"]
    }

    /// 搬运注意
    pub fn transport(&self) -> Vec<&'static str> {
        vec![
            "脊柱伤者整体平托",
            "避免扭折伤肢",
            "保暖防休克",
            "尽快送医治疗",
        ]
    }
}

impl Rule for FractureFirstAidRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("fracture_first_aid")
    }

    fn explain(&self) -> String {
        format!(
            "【骨折急救】\n{}",
            [
                format!(
                    "识别判断：\\n{}",
                    self.recognition()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "固定处理：\\n{}",
                    self.immobilize()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "开放骨折：\\n{}",
                    self.open_fracture()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "搬运注意：\\n{}",
                    self.transport()
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
    fn test_fracturefirstaidrules_basic() {
        let rules = FractureFirstAidRules::new();
        assert_eq!(rules.metadata().name, "骨折急救");
        assert!(!rules.recognition().is_empty());
        assert!(!rules.immobilize().is_empty());
        assert!(!rules.open_fracture().is_empty());
        assert!(!rules.transport().is_empty());
    }

    #[test]
    fn test_fracturefirstaidrules_validation() {
        let rules = FractureFirstAidRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("fracture_first_aid"));
    }

    #[test]
    fn test_fracturefirstaidrules_explain() {
        let rules = FractureFirstAidRules::new();
        let e = rules.explain();
        assert!(e.contains("识别判断"));
        assert!(e.contains("固定处理"));
        assert!(e.contains("开放骨折"));
    }
}
