//! 认知心理学定律

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: CognitivePsychologyRules,
    name: "认知心理学定律",
    desc: "认知心理学定律",
    origin: "国际",
    tags: ["科学", "心理学"]
}

impl CognitivePsychologyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["选择性注意", "注意资源有限", "非注意盲视"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["工作记忆模型", "遗忘曲线", "编码特异性"]
    }
}

impl Rule for CognitivePsychologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("cognitive_psychology")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "认知心理学定律",
            &[("注意", &self.section_0()), ("记忆", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cognitive_psychology_rules() {
        let r = CognitivePsychologyRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
