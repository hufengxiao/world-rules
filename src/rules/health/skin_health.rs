//! 皮肤健康规则

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: SkinHealthRules,
    name: "皮肤健康规则",
    desc: "皮肤健康护理规则",
    origin: "国际",
    tags: ["健康", "护肤"]
}

impl SkinHealthRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["温和洁面", "不过度清洁", "卸妆彻底"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["日常防晒SPF30+", "补涂防晒", "物理防晒"]
    }
}

impl Rule for SkinHealthRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("skin_health")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "皮肤健康规则",
            &[("清洁", &self.section_0()), ("防晒", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_skin_health_rules() {
        let r = SkinHealthRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
