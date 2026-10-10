//! 脂肪肝调理
//!
//! 脂肪肝的饮食运动干预与生活管理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: FattyLiverCareRules,
    name: "脂肪肝调理",
    desc: "脂肪肝的饮食运动干预与生活管理",
    origin: "中国",
    tags: ["慢性病", "肝脏", "调理"]
}

impl FattyLiverCareRules {
    /// 饮食调整
    pub fn diet(&self) -> Vec<&'static str> {
        vec![
            "控制总热量摄入",
            "减少高糖高脂食物",
            "增加蔬菜与粗粮",
            "戒酒极为关键",
        ]
    }

    /// 运动干预
    pub fn exercise(&self) -> Vec<&'static str> {
        vec![
            "坚持有氧运动",
            "每周规律锻炼",
            "循序渐进减重",
            "控制体重为核心",
        ]
    }

    /// 生活管理
    pub fn lifestyle(&self) -> Vec<&'static str> {
        vec!["规律作息不熬夜", "避免久坐", "控制血糖血脂", "慎用伤肝药物"]
    }

    /// 就医提示
    pub fn seeking_help(&self) -> Vec<&'static str> {
        vec![
            "定期查肝功能",
            "转氨酶升高应就诊",
            "减重后复查",
            "重度脂肪肝遵医嘱",
        ]
    }
}

impl Rule for FattyLiverCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("fatty_liver_care")
    }

    fn explain(&self) -> String {
        format!(
            "【脂肪肝调理】\n{}",
            [
                format!(
                    "饮食调整：\\n{}",
                    self.diet()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "运动干预：\\n{}",
                    self.exercise()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "生活管理：\\n{}",
                    self.lifestyle()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医提示：\\n{}",
                    self.seeking_help()
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
    fn test_fattylivercarerules_basic() {
        let rules = FattyLiverCareRules::new();
        assert_eq!(rules.metadata().name, "脂肪肝调理");
        assert!(!rules.diet().is_empty());
        assert!(!rules.exercise().is_empty());
        assert!(!rules.lifestyle().is_empty());
        assert!(!rules.seeking_help().is_empty());
    }

    #[test]
    fn test_fattylivercarerules_validation() {
        let rules = FattyLiverCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("fatty_liver_care"));
    }

    #[test]
    fn test_fattylivercarerules_explain() {
        let rules = FattyLiverCareRules::new();
        let e = rules.explain();
        assert!(e.contains("饮食调整"));
        assert!(e.contains("运动干预"));
        assert!(e.contains("生活管理"));
    }
}
