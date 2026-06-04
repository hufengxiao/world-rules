//! 编译器理论
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: CompilerTheoryRules, name: "编译器理论", desc: "编译器设计理论", origin: "国际", tags: ["科学", "计算机"] }
impl CompilerTheoryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["词法分析", "语法分析", "语义分析"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["代码优化", "目标代码生成"]
    }
}
impl Rule for CompilerTheoryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("compiler_theory")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "编译器理论",
            &[("前端", &self.section_0()), ("后端", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CompilerTheoryRules::new();
        assert!(!r.explain().is_empty());
    }
}
