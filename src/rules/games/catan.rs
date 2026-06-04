//! 卡坦岛规则

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: CatanRules,
    name: "卡坦岛规则",
    desc: "卡坦岛桌游规则",
    origin: "德国",
    tags: ["游戏", "桌游"]
}

impl CatanRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["木材/砖块/羊毛/麦子/矿石"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["道路/村庄/城市/发展卡"]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec!["玩家间交易", "港口交易", "银行4:1交易"]
    }
}

impl Rule for CatanRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("catan")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "卡坦岛规则",
            &[
                ("资源", &self.section_0()),
                ("建设", &self.section_1()),
                ("交易", &self.section_2()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_catan_rules() {
        let r = CatanRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
