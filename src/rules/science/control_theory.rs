//! 控制理论定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: ControlTheoryRules,
    name: "控制理论定律",
    desc: "控制理论定律",
    origin: "国际",
    tags: ["科学", "工程"],
    category: RuleCategory::science("control_theory"),
    sections: [("经典", section_0), ("现代", section_1)]
}

impl ControlTheoryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["PID控制", "根轨迹"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["状态空间", "最优控制"]
    }
}
