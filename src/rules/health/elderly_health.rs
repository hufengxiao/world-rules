//! 老年健康规则

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: ElderlyHealthRules,
    name: "老年健康规则",
    desc: "老年健康管理规则",
    origin: "国际",
    tags: ["健康", "老年"]
}

impl ElderlyHealthRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["太极拳", "散步", "平衡训练"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["家居改造", "穿防滑鞋", "定期检查视力"]
    }
}

impl Rule for ElderlyHealthRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("elderly_health")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "老年健康规则",
            &[("运动", &self.section_0()), ("防跌", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_elderly_health_rules() {
        let r = ElderlyHealthRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
