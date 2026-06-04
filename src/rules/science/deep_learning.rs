//! 深度学习理论

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: DeepLearningRules,
    name: "深度学习理论",
    desc: "深度学习理论定律",
    origin: "国际",
    tags: ["科学", "计算机"]
}

impl DeepLearningRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["卷积神经网络", "循环神经网络", "Transformer注意力机制"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["反向传播", "批归一化", "Dropout正则化"]
    }
}

impl Rule for DeepLearningRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("deep_learning")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "深度学习理论",
            &[
                ("网络架构", &self.section_0()),
                ("训练技巧", &self.section_1()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_deep_learning_rules() {
        let r = DeepLearningRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
