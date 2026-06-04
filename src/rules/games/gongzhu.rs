//! 拱猪规则

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: GongzhuRules,
    name: "拱猪规则",
    desc: "拱猪卡牌游戏规则",
    origin: "中国",
    tags: ["游戏", "卡牌"]
}

impl GongzhuRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "猪(黑桃Q)负100分",
            "羊(方块J)正100分",
            "变压器(梅花10)翻倍",
            "红心每张负分",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["梅花2先出", "同花跟牌", "无同花可垫牌"]
    }
}

impl Rule for GongzhuRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("gongzhu")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "拱猪规则",
            &[
                ("特殊牌", &self.section_0()),
                ("出牌规则", &self.section_1()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gongzhu_rules() {
        let r = GongzhuRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
