//! 呼吸系统基础
//!
//! 肺呼吸、气体交换与呼吸道健康的基础常识

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: RespiratorySystemBasicsRules,
    name: "呼吸系统基础",
    desc: "肺呼吸、气体交换与呼吸道健康的基础常识",
    origin: "国际",
    tags: ["科学", "呼吸", "肺", "生理"]
}

impl RespiratorySystemBasicsRules {
    /// 呼吸机制
    pub fn mechanism(&self) -> Vec<&'static str> {
        vec![
            "吸气送氧呼气排碳",
            "膈肌运动助呼吸",
            "肺泡进行气体交换",
            "呼吸深浅可调节",
        ]
    }

    /// 呼吸道防护
    pub fn protect(&self) -> Vec<&'static str> {
        vec![
            "保持呼吸道清洁",
            "远离烟尘污染",
            "避免长时间深呼吸有害气体",
            "锻炼利好肺功能",
        ]
    }

    /// 健康促进
    pub fn health(&self) -> Vec<&'static str> {
        vec![
            "规律运动助呼吸",
            "充足睡眠修复",
            "良好姿势利呼吸",
            "保持湿润通风",
        ]
    }

    /// 注意信号
    pub fn care(&self) -> Vec<&'static str> {
        vec![
            "持续咳嗽气促就医",
            "胸闷喘重检查",
            "儿童呼吸异常重视",
            "规范护肺不吸烟",
        ]
    }
}

impl Rule for RespiratorySystemBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("respiratory")
    }

    fn explain(&self) -> String {
        format!(
            "【呼吸系统基础】\n{}",
            [
                format!(
                    "呼吸机制：\\n{}",
                    self.mechanism()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "呼吸道防护：\\n{}",
                    self.protect()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "健康促进：\\n{}",
                    self.health()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "注意信号：\\n{}",
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
    fn test_respiratorysystembasicsrules_basic() {
        let rules = RespiratorySystemBasicsRules::new();
        assert_eq!(rules.metadata().name, "呼吸系统基础");
        assert!(!rules.mechanism().is_empty());
        assert!(!rules.protect().is_empty());
        assert!(!rules.health().is_empty());
        assert!(!rules.care().is_empty());
    }

    #[test]
    fn test_respiratorysystembasicsrules_validation() {
        let rules = RespiratorySystemBasicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("respiratory"));
    }

    #[test]
    fn test_respiratorysystembasicsrules_explain() {
        let rules = RespiratorySystemBasicsRules::new();
        let e = rules.explain();
        assert!(e.contains("呼吸机制"));
        assert!(e.contains("呼吸道防护"));
        assert!(e.contains("健康促进"));
    }
}
