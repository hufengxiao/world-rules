//! F1 FIA详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: F1FiaDetailedRules, name: "F1 FIA详细规则", desc: "FIA一级方程式规则", origin: "国际", tags: ["体育", "赛车"] }
impl F1FiaDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "正赛积分:第1名25分/第2名18分/第3名15分",
            "第4-10名:12/10/8/6/4/2/1分",
            "最快圈速:额外1分(需进入前10)",
            "冲刺赛积分:8/7/6/5/4/3/2/1",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "动力单元:1.6升V6涡轮增压+能量回收",
            "最高转速:15000rpm",
            "轮胎供应商:Pirelli(5种配方)",
            "燃油限制:110kg/比赛",
            "DRS:可调尾翼系统(减少空气阻力)",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "排位赛:Q1/Q2/Q3三节淘汰制",
            "正赛:最短305公里或2小时",
            "安全车:事故时安全车带领",
            "红旗:严重事故时比赛暂停",
            "进站策略:至少使用两种不同配方轮胎",
        ]
    }
}
impl Rule for F1FiaDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("f1_fia_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "F1 FIA详细规则",
            &[
                ("积分系统", &self.section_0()),
                ("技术规则", &self.section_1()),
                ("比赛规则", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = F1FiaDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
