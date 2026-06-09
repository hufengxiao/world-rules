//! 海洋法公约
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: MaritimeLawIntlRules,
    name: "海洋法公约",
    desc: "联合国海洋法公约规则",
    origin: "国际",
    tags: ["法律", "国际"],
    category: RuleCategory::law("maritime_law_intl"),
    sections: [("海域划分", section_0), ("航行权", section_1)]
}

impl MaritimeLawIntlRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["领海12海里", "专属经济区200海里", "大陆架", "公海"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["无害通过权", "过境通行", "群岛海道通过"]
    }
}
