//! 土壤污染防治法
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: SoilPollutionLawRules,
    name: "土壤污染防治法",
    desc: "土壤污染防治法律规则",
    origin: "中国",
    tags: ["法律", "环境"],
    category: RuleCategory::law("soil_pollution_law"),
    sections: [("预防", section_0), ("修复", section_1)]
}

impl SoilPollutionLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["源头防控", "重点监管单位", "灌溉水质"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["修复责任", "修复标准", "效果评估"]
    }
}
