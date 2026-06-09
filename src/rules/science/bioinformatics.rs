//! 生物信息学定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: BioinformaticsRules,
    name: "生物信息学定律",
    desc: "生物信息学定律",
    origin: "国际",
    tags: ["科学", "生物"],
    category: RuleCategory::science("bioinformatics"),
    sections: [("序列分析", section_0), ("基因组", section_1)]
}

impl BioinformaticsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["序列比对算法", "BLAST搜索", "多序列比对"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["基因预测", "基因组组装", "比较基因组学"]
    }
}
