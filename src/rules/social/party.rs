//! 派对礼仪

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: PartyRules,
    name: "派对礼仪",
    desc: "派对社交礼仪",
    origin: "国际",
    tags: ["社交", "礼仪"]
}

impl PartyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["了解Dress Code", "正式派对着正装", "便装派对休闲装"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["主动自我介绍", "适度饮酒", "尊重主人安排"]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec!["适时告辞", "向主人道谢", "不过早也不过晚离开"]
    }
}

impl Rule for PartyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("party")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "派对礼仪",
            &[
                ("着装", &self.section_0()),
                ("社交", &self.section_1()),
                ("离场", &self.section_2()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_party_rules() {
        let r = PartyRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
