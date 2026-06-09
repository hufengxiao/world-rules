//! 税法详解2
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: TaxDetailed2Rules,
    name: "税法详解2",
    desc: "税法详解2",
    origin: "中国",
    tags: ["法律", "税法"],
    category: RuleCategory::law("tax_detailed2"),
    sections: [("增值税", section_0), ("所得税", section_1)]
}

impl TaxDetailed2Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["税率", "进项抵扣", "发票管理"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["应纳税所得额", "优惠政策"]
    }
}
