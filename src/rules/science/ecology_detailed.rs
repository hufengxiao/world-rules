//! 生态学详细定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: EcologyDetailedRules,
    name: "生态学详细定律",
    desc: "生态学详细定律",
    origin: "国际",
    tags: ["科学", "生物"],
    category: RuleCategory::science("ecology_detailed"),
    sections: [("种群", section_0), ("生态系统", section_1)]
}

impl EcologyDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["逻辑斯谛增长", "竞争排斥"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["能量流动", "物质循环"]
    }
}
