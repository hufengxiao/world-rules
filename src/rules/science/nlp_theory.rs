//! 自然语言处理理论

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: NlpTheoryRules,
    name: "自然语言处理理论",
    desc: "NLP理论定律",
    origin: "国际",
    tags: ["科学", "计算机"]
}

impl NlpTheoryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["词嵌入", "序列到序列模型", "注意力机制"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["机器翻译", "情感分析", "文本生成"]
    }
}

impl Rule for NlpTheoryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("nlp_theory")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "自然语言处理理论",
            &[("基础", &self.section_0()), ("应用", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_nlp_theory_rules() {
        let r = NlpTheoryRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
