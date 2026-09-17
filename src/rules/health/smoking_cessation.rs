//! 戒烟与远离烟草
//!
//! 制定戒烟计划、应对烟瘾与无烟生活规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SmokingCessationRules,
    name: "戒烟与远离烟草",
    desc: "制定戒烟计划、应对烟瘾与无烟生活规则",
    origin: "医学",
    tags: ["健康", "戒烟", "烟草", "肺部", "习惯"]
}

impl SmokingCessationRules {
    /// 戒断准备
    pub fn prepare(&self) -> Vec<&'static str> {
        vec![
            "明确戒烟原因与目标",
            "设定戒烟日期",
            "告知亲友取得支持",
            "清除身边烟具烟草",
        ]
    }

    /// 应对烟瘾
    pub fn craving(&self) -> Vec<&'static str> {
        vec![
            "烟瘾上来时转移注意",
            "嚼无糖口香糖或喝水",
            "短暂散步或深呼吸",
            "避免诱发场景与饮酒",
        ]
    }

    /// 戒后调适
    pub fn adjust(&self) -> Vec<&'static str> {
        vec![
            "管理情绪波动与睡眠",
            "坚持运动释放压力",
            "记录自己的进步",
            "稍有复吸时不必放弃",
        ]
    }

    /// 借助帮助
    pub fn help(&self) -> Vec<&'static str> {
        vec![
            "可寻求戒烟门诊支持",
            "遵医嘱考虑替代疗法",
            "家人互相监督鼓励",
            "戒烟有困难不羞于求助",
        ]
    }
}

impl Rule for SmokingCessationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("smoking_cessation")
    }

    fn explain(&self) -> String {
        format!(
            "【戒烟与远离烟草】\n{}",
            [
                format!(
                    "戒断准备：\\n{}",
                    self.prepare()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "应对烟瘾：\\n{}",
                    self.craving()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "戒后调适：\\n{}",
                    self.adjust()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "借助帮助：\\n{}",
                    self.help()
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
    fn test_smokingcessationrules_basic() {
        let rules = SmokingCessationRules::new();
        assert_eq!(rules.metadata().name, "戒烟与远离烟草");
        assert!(!rules.prepare().is_empty());
        assert!(!rules.craving().is_empty());
        assert!(!rules.adjust().is_empty());
        assert!(!rules.help().is_empty());
    }

    #[test]
    fn test_smokingcessationrules_validation() {
        let rules = SmokingCessationRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("smoking_cessation"));
    }

    #[test]
    fn test_smokingcessationrules_explain() {
        let rules = SmokingCessationRules::new();
        let e = rules.explain();
        assert!(e.contains("戒断准备"));
        assert!(e.contains("应对烟瘾"));
        assert!(e.contains("戒后调适"));
    }
}
