//! 人体器官系统
//!
//! 循环、呼吸、消化等主要器官系统的基础常识

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: HumanOrganSystemsRules,
    name: "人体器官系统",
    desc: "循环、呼吸、消化等主要器官系统的基础常识",
    origin: "国际",
    tags: ["科学", "人体", "器官", "生理"]
}

impl HumanOrganSystemsRules {
    /// 循环系统
    pub fn circulatory(&self) -> Vec<&'static str> {
        vec![
            "心脏泵血推动循环",
            "血管输送血液",
            "血液输送氧与养分",
            "心肺劳逸皆关联",
        ]
    }

    /// 呼吸消化
    pub fn respira_digest(&self) -> Vec<&'static str> {
        vec![
            "肺进行气体交换",
            "胃肠消化吸收营养",
            "各系统相互配合",
            "肝肾代谢过滤",
        ]
    }

    /// 内分泌与免疫
    pub fn endo_immun(&self) -> Vec<&'static str> {
        vec![
            "内分泌调节身体活动",
            "免疫防御抵抗病原",
            "系统失衡影响健康",
            "睡眠营养支持机能",
        ]
    }

    /// 保养与就医
    pub fn care(&self) -> Vec<&'static str> {
        vec![
            "规律运动强健器官",
            "均衡营养护系统",
            "异常症状及时就医",
            "定期体检了解机能",
        ]
    }
}

impl Rule for HumanOrganSystemsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("human_organs")
    }

    fn explain(&self) -> String {
        format!(
            "【人体器官系统】\n{}",
            [
                format!(
                    "循环系统：\\n{}",
                    self.circulatory()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "呼吸消化：\\n{}",
                    self.respira_digest()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "内分泌与免疫：\\n{}",
                    self.endo_immun()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "保养与就医：\\n{}",
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
    fn test_humanorgansystemsrules_basic() {
        let rules = HumanOrganSystemsRules::new();
        assert_eq!(rules.metadata().name, "人体器官系统");
        assert!(!rules.circulatory().is_empty());
        assert!(!rules.respira_digest().is_empty());
        assert!(!rules.endo_immun().is_empty());
        assert!(!rules.care().is_empty());
    }

    #[test]
    fn test_humanorgansystemsrules_validation() {
        let rules = HumanOrganSystemsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("human_organs"));
    }

    #[test]
    fn test_humanorgansystemsrules_explain() {
        let rules = HumanOrganSystemsRules::new();
        let e = rules.explain();
        assert!(e.contains("循环系统"));
        assert!(e.contains("呼吸消化"));
        assert!(e.contains("内分泌与免疫"));
    }
}
