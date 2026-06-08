//! 银行法详解2
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: BankingDetailed2Rules, name: "银行法详解2", desc: "银行法详解2", origin: "中国", tags: ["法律", "金融"] }
impl BankingDetailed2Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["存款保险", "贷款管理"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["资本充足率", "流动性"]
    }
}
impl Rule for BankingDetailed2Rules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("banking_detailed2")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "银行法详解2",
            &[("业务", &self.section_0()), ("风控", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BankingDetailed2Rules::new();
        assert!(!r.explain().is_empty());
    }
}
