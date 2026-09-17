//! 腰部腰椎保养
//!
//! 正确搬抬、坐姿与家常护腰锻炼

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: LumbarSpineProtectionRules,
    name: "腰部腰椎保养",
    desc: "正确搬抬、坐姿与家常护腰锻炼",
    origin: "医学",
    tags: ["健康", "腰椎", "护腰", "坐姿"]
}

impl LumbarSpineProtectionRules {
    /// 正确搬抬
    pub fn lift(&self) -> Vec<&'static str> {
        vec![
            "屈膝弯腰用腿力",
            "重物贴近身体",
            "不扭腰猛然起",
            "搬不动找人帮",
        ]
    }

    /// 久坐护腰
    pub fn sitting(&self) -> Vec<&'static str> {
        vec!["挺直腰背坐姿", "常起身活动", "腰后加靠垫", "不塌坐软沙发"]
    }

    /// 锻炼强腰
    pub fn strong(&self) -> Vec<&'static str> {
        vec!["练核心腰腹", "腰肌轻柔拉伸", "游泳骑车护腰", "量力而行"]
    }

    /// 警示就医
    pub fn care(&self) -> Vec<&'static str> {
        vec![
            "腰痛伴下肢麻就医",
            "闪腰缓养休息",
            "不擅自猛力复位",
            "持续腰痛检查",
        ]
    }
}

impl Rule for LumbarSpineProtectionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("lumbar")
    }

    fn explain(&self) -> String {
        format!(
            "【腰部腰椎保养】\n{}",
            [
                format!(
                    "正确搬抬：\\n{}",
                    self.lift()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "久坐护腰：\\n{}",
                    self.sitting()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "锻炼强腰：\\n{}",
                    self.strong()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "警示就医：\\n{}",
                    self.care()
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
    fn test_lumbarspineprotectionrules_basic() {
        let rules = LumbarSpineProtectionRules::new();
        assert_eq!(rules.metadata().name, "腰部腰椎保养");
        assert!(!rules.lift().is_empty());
        assert!(!rules.sitting().is_empty());
        assert!(!rules.strong().is_empty());
        assert!(!rules.care().is_empty());
    }

    #[test]
    fn test_lumbarspineprotectionrules_validation() {
        let rules = LumbarSpineProtectionRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("lumbar"));
    }

    #[test]
    fn test_lumbarspineprotectionrules_explain() {
        let rules = LumbarSpineProtectionRules::new();
        let e = rules.explain();
        assert!(e.contains("正确搬抬"));
        assert!(e.contains("久坐护腰"));
        assert!(e.contains("锻炼强腰"));
    }
}
