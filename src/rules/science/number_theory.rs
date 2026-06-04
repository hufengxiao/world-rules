//! 数论定律

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: NumberTheoryRules,
    name: "数论定律",
    desc: "数论定律",
    origin: "国际",
    tags: ["科学", "数学"]
}

impl NumberTheoryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["算术基本定理", "费马小定理", "欧拉定理"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["黎曼假设", "孪生素数猜想", "哥德巴赫猜想"]
    }
}

impl Rule for NumberTheoryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("number_theory")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "数论定律",
            &[("基本定理", &self.section_0()), ("猜想", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number_theory_rules() {
        let r = NumberTheoryRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
