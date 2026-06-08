//! 知识产权详解2
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: IpDetailed2Rules, name: "知识产权详解2", desc: "知识产权法详解2", origin: "中国", tags: ["法律", "知识产权"] }
impl IpDetailed2Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["申请流程", "无效宣告"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["注册条件", "异议程序"]
    }
}
impl Rule for IpDetailed2Rules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("ip_detailed2")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "知识产权详解2",
            &[("专利", &self.section_0()), ("商标", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = IpDetailed2Rules::new();
        assert!(!r.explain().is_empty());
    }
}
