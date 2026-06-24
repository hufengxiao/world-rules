//! 区块链详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BlockchainDetailedRules, name: "区块链详细定律", desc: "区块链定律", origin: "国际", tags: ["科学", "计算机"] }
impl BlockchainDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["PoW PoS"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["智能合约"]
    }
}
impl Rule for BlockchainDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("blockchain_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "区块链详细定律",
            &[("共识", &self.section_0()), ("应用", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BlockchainDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
