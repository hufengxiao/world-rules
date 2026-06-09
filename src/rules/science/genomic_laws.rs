//! 基因组学定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: GenomicLawsRules,
    name: "基因组学定律",
    desc: "基因组学定律",
    origin: "国际",
    tags: ["科学", "生物"],
    category: RuleCategory::science("genomic_laws"),
    sections: [("测序", section_0), ("分析", section_1)]
}

impl GenomicLawsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["二代测序", "三代测序"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["变异检测", "全基因组关联"]
    }
}
