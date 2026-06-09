//! 电梯详细礼仪
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: ElevatorDetailedRules,
    name: "电梯详细礼仪",
    desc: "电梯乘坐详细礼仪",
    origin: "国际",
    tags: ["社交", "公共"],
    category: RuleCategory::social("elevator_detailed"),
    sections: [("乘坐", section_0), ("礼让", section_1)]
}

impl ElevatorDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["先下后上", "主动按键", "帮他人按楼层"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["让老人小孩先进", "帮按开门键", "电梯满时等下一趟"]
    }
}
