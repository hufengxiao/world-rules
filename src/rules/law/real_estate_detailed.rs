//! 房地产法详解
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: RealEstateDetailedRules,
    name: "房地产法详解",
    desc: "房地产法详解",
    origin: "中国",
    tags: ["法律", "房产"],
    category: RuleCategory::law("real_estate_detailed"),
    sections: [("开发", section_0), ("交易", section_1)]
}

impl RealEstateDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["建设用地", "规划许可"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["商品房预售", "产权登记"]
    }
}
