//! 区块链理论

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: BlockchainTheoryRules,
    name: "区块链理论",
    desc: "区块链技术理论定律",
    origin: "国际",
    tags: ["科学", "计算机"]
}

impl BlockchainTheoryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["工作量证明", "权益证明", "拜占庭容错"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["哈希函数", "默克尔树", "数字签名"]
    }
}

impl Rule for BlockchainTheoryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("blockchain_theory")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "区块链理论",
            &[
                ("共识机制", &self.section_0()),
                ("密码学", &self.section_1()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_blockchain_theory_rules() {
        let r = BlockchainTheoryRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
