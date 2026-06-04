//! 太空法

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: SpaceLawRules,
    name: "太空法",
    desc: "外层空间法律规则",
    origin: "国际",
    tags: ["法律", "航空"]
}

impl SpaceLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["不得主权宣示", "自由探索", "和平利用"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["空间物体损害责任", "宇航员救助义务", "空间碎片减缓"]
    }
}

impl Rule for SpaceLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("space_law")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "太空法",
            &[("基本原则", &self.section_0()), ("责任", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_space_law_rules() {
        let r = SpaceLawRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
