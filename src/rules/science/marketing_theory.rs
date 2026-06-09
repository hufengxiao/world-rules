//! 市场营销定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: MarketingTheoryRules,
    name: "市场营销定律",
    desc: "市场营销定律",
    origin: "国际",
    tags: ["科学", "管理"],
    category: RuleCategory::science("marketing_theory"),
    sections: [("策略", section_0), ("数字营销", section_1)]
}

impl MarketingTheoryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["STP市场细分战略", "4P营销组合", "SWOT分析"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["社交媒体营销", "内容营销", "SEO搜索引擎优化"]
    }
}
