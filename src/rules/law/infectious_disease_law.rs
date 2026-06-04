//! 传染病防治法

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: InfectiousDiseaseLawRules,
    name: "传染病防治法",
    desc: "传染病防治法律规则",
    origin: "中国",
    tags: ["法律", "医疗"]
}

impl InfectiousDiseaseLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["甲类强制隔离", "乙类严格管控", "丙类监测管理"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["疫苗接种", "监测预警", "隔离封锁"]
    }
}

impl Rule for InfectiousDiseaseLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("infectious_disease_law")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "传染病防治法",
            &[
                ("分类管理", &self.section_0()),
                ("防控措施", &self.section_1()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_infectious_disease_law_rules() {
        let r = InfectiousDiseaseLawRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
