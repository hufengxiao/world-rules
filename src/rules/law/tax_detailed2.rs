//! 税法详解2
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: TaxDetailed2Rules, name: "税法详解2", desc: "税法详解2", origin: "中国", tags: ["法律", "税法"] }
impl TaxDetailed2Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["税率", "进项抵扣", "发票管理"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["应纳税所得额", "优惠政策"]
    }
}
impl Rule for TaxDetailed2Rules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("tax_detailed2")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "税法详解2",
            &[("增值税", &self.section_0()), ("所得税", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = TaxDetailed2Rules::new();
        assert!(!r.explain().is_empty());
    }
}
