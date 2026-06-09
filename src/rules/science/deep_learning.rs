//! 深度学习理论
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: DeepLearningRules,
    name: "深度学习理论",
    desc: "深度学习理论定律",
    origin: "国际",
    tags: ["科学", "计算机"],
    category: RuleCategory::science("deep_learning"),
    sections: [("网络架构", section_0), ("训练技巧", section_1)]
}

impl DeepLearningRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["卷积神经网络", "循环神经网络", "Transformer注意力机制"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["反向传播", "批归一化", "Dropout正则化"]
    }
}
