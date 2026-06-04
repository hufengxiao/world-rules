//! 口腔健康规则

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: OralHealthRules,
    name: "口腔健康规则",
    desc: "口腔健康护理规则",
    origin: "国际",
    tags: ["健康", "口腔"]
}

impl OralHealthRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["早晚刷牙每次3分钟", "正确刷牙方法", "定期换牙刷"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["半年洗牙一次", "定期口腔检查", "发现问题及时治疗"]
    }
}

impl Rule for OralHealthRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("oral_health")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "口腔健康规则",
            &[("刷牙", &self.section_0()), ("检查", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_oral_health_rules() {
        let r = OralHealthRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
