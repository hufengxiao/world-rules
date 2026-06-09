//! 银行法详解2
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: BankingDetailed2Rules,
    name: "银行法详解2",
    desc: "银行法详解2",
    origin: "中国",
    tags: ["法律", "金融"],
    category: RuleCategory::law("banking_detailed2"),
    sections: [("业务", section_0), ("风控", section_1)]
}

impl BankingDetailed2Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["存款保险", "贷款管理"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["资本充足率", "流动性"]
    }
}
