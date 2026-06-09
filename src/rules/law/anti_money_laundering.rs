//! 反洗钱法
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: AntiMoneyLaunderingRules,
    name: "反洗钱法",
    desc: "反洗钱法律规则",
    origin: "中国",
    tags: ["法律", "金融"],
    category: RuleCategory::law("anti_money_laundering"),
    sections: [("义务主体", section_0), ("措施", section_1)]
}

impl AntiMoneyLaunderingRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["金融机构", "特定非金融机构"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["客户身份识别", "大额交易报告", "可疑交易报告"]
    }
}
