//! 机器人学理论
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: RoboticsTheoryRules,
    name: "机器人学理论",
    desc: "机器人学理论定律",
    origin: "国际",
    tags: ["科学", "工程"],
    category: RuleCategory::science("robotics_theory"),
    sections: [("运动", section_0), ("感知", section_1)]
}

impl RoboticsTheoryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["正运动学", "轨迹规划"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["SLAM", "力反馈"]
    }
}
