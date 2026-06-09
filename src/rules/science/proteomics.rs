//! 蛋白质组学定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: ProteomicsRules,
    name: "蛋白质组学定律",
    desc: "蛋白质组学定律",
    origin: "国际",
    tags: ["科学", "生物"],
    category: RuleCategory::science("proteomics"),
    sections: [("技术", section_0), ("分析", section_1)]
}

impl ProteomicsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["质谱分析", "二维电泳"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["蛋白质互作网络", "翻译后修饰"]
    }
}
