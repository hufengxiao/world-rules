//! 电子商务法

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: EcommerceLawRules,
    name: "电子商务法",
    desc: "电子商务法律规则",
    origin: "中国",
    tags: ["法律", "电商"]
}

impl EcommerceLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["市场主体登记", "纳税义务", "信息公示"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["平台治理责任", "知识产权保护", "消费者权益"]
    }
}

impl Rule for EcommerceLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("ecommerce_law")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "电子商务法",
            &[
                ("经营者义务", &self.section_0()),
                ("平台责任", &self.section_1()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ecommerce_law_rules() {
        let r = EcommerceLawRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
