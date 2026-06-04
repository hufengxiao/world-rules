//! 人工智能法规

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: AiRegulationRules,
    name: "人工智能法规",
    desc: "人工智能法律规则",
    origin: "国际",
    tags: ["法律", "科技"]
}

impl AiRegulationRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["透明可解释", "公平无歧视", "安全可控"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["人脸识别限制", "自动化决策审查", "深度伪造监管"]
    }
}

impl Rule for AiRegulationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("ai_regulation")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "人工智能法规",
            &[
                ("基本原则", &self.section_0()),
                ("应用限制", &self.section_1()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ai_regulation_rules() {
        let r = AiRegulationRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
