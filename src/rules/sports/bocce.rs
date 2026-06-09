//! 滚球规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: BocceRules,
    name: "滚球规则",
    desc: "意大利滚球运动规则",
    origin: "意大利",
    tags: ["体育", "休闲"],
    category: RuleCategory::sports("bocce"),
    sections: [("场地规格", court_specifications), ("技术动作", techniques), ("得分规则", scoring), ("装备要求", equipment)]
}

impl BocceRules {
    pub fn court_specifications(&self) -> Vec<&'static str> {
        vec![
            "场地尺寸: 27.5×4.5米",
            "目标球位置",
            "投掷区域",
            "场地边界",
            "表面要求",
        ]
    }

    pub fn techniques(&self) -> Vec<&'static str> {
        vec!["投掷技术", "滚球技术", "瞄准技术", "击球技术", "控制技术"]
    }

    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "距离目标球最近得分",
            "每局最多4分",
            "得分测量",
            "比分记录",
            "比赛胜负",
        ]
    }

    pub fn equipment(&self) -> Vec<&'static str> {
        vec!["滚球", "目标球", "测量工具", "场地装备", "比赛服装"]
    }
}
