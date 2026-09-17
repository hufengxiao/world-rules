//! 巴氏刷牙法
//!
//! 巴氏刷牙法、每面牙与时长正确刷牙

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BrushingTechniqueBassRules,
    name: "巴氏刷牙法",
    desc: "巴氏刷牙法、每面牙与时长正确刷牙",
    origin: "牙科",
    tags: ["健康", "刷牙", "口腔", "方法"]
}

impl BrushingTechniqueBassRules {
    /// 巴氏法
    pub fn bass(&self) -> Vec<&'static str> {
        vec!["刷毛斜向龈沟", "小幅度前后颤动", "逐颗清洁", "轻力不刷伤"]
    }

    /// 各面刷到
    pub fn surfaces(&self) -> Vec<&'static str> {
        vec!["外侧面内侧都要", "咬合面横刷", "不遗漏后牙", "全面覆盖"]
    }

    /// 时长频率
    pub fn duration(&self) -> Vec<&'static str> {
        vec!["每次约两分钟", "早晚各一次", "餐后清水漱口", "换软毛刷"]
    }

    /// 定期看牙
    pub fn visit(&self) -> Vec<&'static str> {
        vec!["每半到一年洗牙", "发现蛀牙就诊", "牙疼不拖", "口腔健康"]
    }
}

impl Rule for BrushingTechniqueBassRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("brushing")
    }

    fn explain(&self) -> String {
        format!(
            "【巴氏刷牙法】\n{}",
            [
                format!(
                    "巴氏法：\\n{}",
                    self.bass()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "各面刷到：\\n{}",
                    self.surfaces()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "时长频率：\\n{}",
                    self.duration()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "定期看牙：\\n{}",
                    self.visit()
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
    fn test_brushingtechniquebassrules_basic() {
        let rules = BrushingTechniqueBassRules::new();
        assert_eq!(rules.metadata().name, "巴氏刷牙法");
        assert!(!rules.bass().is_empty());
        assert!(!rules.surfaces().is_empty());
        assert!(!rules.duration().is_empty());
        assert!(!rules.visit().is_empty());
    }

    #[test]
    fn test_brushingtechniquebassrules_validation() {
        let rules = BrushingTechniqueBassRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("brushing"));
    }

    #[test]
    fn test_brushingtechniquebassrules_explain() {
        let rules = BrushingTechniqueBassRules::new();
        let e = rules.explain();
        assert!(e.contains("巴氏法"));
        assert!(e.contains("各面刷到"));
        assert!(e.contains("时长频率"));
    }
}
