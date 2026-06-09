//! 阿瓦隆规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: MafiaRules,
    name: "阿瓦隆规则",
    desc: "阿瓦隆桌游规则",
    origin: "美国",
    tags: ["游戏", "桌游"],
    category: RuleCategory::games("mafia"),
    sections: [("角色", section_0), ("流程", section_1), ("胜负", section_2)]
}

impl MafiaRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["梅林/派西维尔/忠臣", "莫德雷德/刺客/莫甘娜"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["组队投票", "执行任务", "任务成功或失败"]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec!["3个任务成功好人胜", "3个任务失败坏人胜"]
    }
}
