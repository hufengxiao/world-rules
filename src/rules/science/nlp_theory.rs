//! 自然语言处理理论
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: NlpTheoryRules,
    name: "自然语言处理理论",
    desc: "NLP理论定律",
    origin: "国际",
    tags: ["科学", "计算机"],
    category: RuleCategory::science("nlp_theory"),
    sections: [("基础", section_0), ("应用", section_1)]
}

impl NlpTheoryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["词嵌入", "序列到序列模型", "注意力机制"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["机器翻译", "情感分析", "文本生成"]
    }
}
