//! 假牙与义齿护理
//!
//! 假牙清洁保养与戴用舒适的口腔护理方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: DentureCareCleaningRules,
    name: "假牙与义齿护理",
    desc: "假牙清洁保养与戴用舒适的口腔护理方法",
    origin: "中国",
    tags: ["健康", "假牙", "义齿", "口腔"]
}

impl DentureCareCleaningRules {
    /// 日常清洁
    pub fn clean(&self) -> Vec<&'static str> {
        vec![
            "饭后取下冲洗",
            "用软毛刷轻刷",
            "专用清洁剂浸泡",
            "勿用热水烫变形",
        ]
    }

    /// 佩戴卫生
    pub fn wear(&self) -> Vec<&'static str> {
        vec![
            "睡前取下泡水",
            "保持湿润不发干",
            "刷洗口腔余缝",
            "定期复诊调整",
        ]
    }

    /// 舒适调整
    pub fn comfort(&self) -> Vec<&'static str> {
        vec!["初戴磨合有不适", "改动压迫处", "不适明显就医", "勿自行打磨"]
    }

    /// 健康习惯
    pub fn habit(&self) -> Vec<&'static str> {
        vec![
            "进食细嚼慢咽",
            "避免过热过硬",
            "留意口腔黏膜",
            "保持口腔卫生",
        ]
    }
}

impl Rule for DentureCareCleaningRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("denture_care")
    }

    fn explain(&self) -> String {
        format!(
            "【假牙与义齿护理】\n{}",
            [
                format!(
                    "日常清洁：\\n{}",
                    self.clean()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "佩戴卫生：\\n{}",
                    self.wear()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "舒适调整：\\n{}",
                    self.comfort()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "健康习惯：\\n{}",
                    self.habit()
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
    fn test_denturecarecleaningrules_basic() {
        let rules = DentureCareCleaningRules::new();
        assert_eq!(rules.metadata().name, "假牙与义齿护理");
        assert!(!rules.clean().is_empty());
        assert!(!rules.wear().is_empty());
        assert!(!rules.comfort().is_empty());
        assert!(!rules.habit().is_empty());
    }

    #[test]
    fn test_denturecarecleaningrules_validation() {
        let rules = DentureCareCleaningRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("denture_care"));
    }

    #[test]
    fn test_denturecarecleaningrules_explain() {
        let rules = DentureCareCleaningRules::new();
        let e = rules.explain();
        assert!(e.contains("日常清洁"));
        assert!(e.contains("佩戴卫生"));
        assert!(e.contains("舒适调整"));
    }
}
