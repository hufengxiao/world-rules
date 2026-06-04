//! 市场营销定律

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: MarketingTheoryRules,
    name: "市场营销定律",
    desc: "市场营销定律",
    origin: "国际",
    tags: ["科学", "管理"]
}

impl MarketingTheoryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["STP市场细分战略", "4P营销组合", "SWOT分析"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["社交媒体营销", "内容营销", "SEO搜索引擎优化"]
    }
}

impl Rule for MarketingTheoryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("marketing_theory")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "市场营销定律",
            &[("策略", &self.section_0()), ("数字营销", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_marketing_theory_rules() {
        let r = MarketingTheoryRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
