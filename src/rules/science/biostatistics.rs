//! 生物统计学定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: BiostatisticsRules,
    name: "生物统计学定律",
    desc: "生物统计学定律",
    origin: "国际",
    tags: ["科学", "生物"],
    category: RuleCategory::science("biostatistics"),
    sections: [("方法", section_0), ("设计", section_1)]
}

impl BiostatisticsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["t检验", "卡方检验", "ANOVA"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["随机对照", "队列研究", "病例对照"]
    }
}
