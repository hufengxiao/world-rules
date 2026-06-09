//! 反不正当竞争法
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: AntiUnfairCompetitionRules,
    name: "反不正当竞争法",
    desc: "反不正当竞争法律规则",
    origin: "中国",
    tags: ["法律", "商业"],
    category: RuleCategory::law("anti_unfair_competition"),
    sections: [("不正当行为", section_0), ("互联网专条", section_1)]
}

impl AntiUnfairCompetitionRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["混淆行为", "商业贿赂", "虚假宣传", "侵犯商业秘密"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["流量劫持", "恶意不兼容", "数据爬取"]
    }
}
