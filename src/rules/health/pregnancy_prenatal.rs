//! 孕期保健
//!
//! 孕期产检、营养、运动与安全的生活方式规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PregnancyPrenatalRules,
    name: "孕期保健",
    desc: "孕期产检、营养、运动与安全的生活方式规则",
    origin: "医学",
    tags: ["健康", "孕期", "产检", "营养", "孕妇"]
}

impl PregnancyPrenatalRules {
    /// 规律产检
    pub fn checkup(&self) -> Vec<&'static str> {
        vec![
            "按时进行产前检查",
            "遵循医嘱各期筛查",
            "记录孕期身体状况",
            "异常及时就医",
        ]
    }

    /// 营养饮食
    pub fn nutrition(&self) -> Vec<&'static str> {
        vec![
            "均衡补充叶酸钙铁",
            "多样化饮食保证营养",
            "控糖限盐",
            "避免生食含酒精",
        ]
    }

    /// 适度活动
    pub fn activity(&self) -> Vec<&'static str> {
        vec![
            "遵医嘱适度散步",
            "避免剧烈对抗运动",
            "留意身体信号",
            "多休息不劳累",
        ]
    }

    /// 安全用药
    pub fn medication(&self) -> Vec<&'static str> {
        vec![
            "用药须经医生评估",
            "不擅自吃任何药物",
            "避开有害物质环境",
            "出现见红或剧痛急诊",
        ]
    }
}

impl Rule for PregnancyPrenatalRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("pregnancy")
    }

    fn explain(&self) -> String {
        format!(
            "【孕期保健】\n{}",
            [
                format!(
                    "规律产检：\\n{}",
                    self.checkup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "营养饮食：\\n{}",
                    self.nutrition()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "适度活动：\\n{}",
                    self.activity()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安全用药：\\n{}",
                    self.medication()
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
    fn test_pregnancyprenatalrules_basic() {
        let rules = PregnancyPrenatalRules::new();
        assert_eq!(rules.metadata().name, "孕期保健");
        assert!(!rules.checkup().is_empty());
        assert!(!rules.nutrition().is_empty());
        assert!(!rules.activity().is_empty());
        assert!(!rules.medication().is_empty());
    }

    #[test]
    fn test_pregnancyprenatalrules_validation() {
        let rules = PregnancyPrenatalRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("pregnancy"));
    }

    #[test]
    fn test_pregnancyprenatalrules_explain() {
        let rules = PregnancyPrenatalRules::new();
        let e = rules.explain();
        assert!(e.contains("规律产检"));
        assert!(e.contains("营养饮食"));
        assert!(e.contains("适度活动"));
    }
}
