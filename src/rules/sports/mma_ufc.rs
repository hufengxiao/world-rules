//! UFC综合格斗规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MmaUfcRules, name: "UFC综合格斗规则", desc: "UFC综合格斗规则", origin: "美国", tags: ["体育", "格斗"] }
impl MmaUfcRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "比赛3回合每回合5分钟",
            "冠军赛5回合每回合5分钟",
            "回合间休息1分钟",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "10分制:赢方10分输方9分或更少",
            "有效打击:拳腿膝",
            "摔跤:成功摔倒对手",
            "控制:地面控制时间",
            "降服:绞技关节技",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "插眼击裆咬人",
            "击打后脑勺脊椎",
            "抓扯头发抓笼网",
            "12点到6点肘击(向下肘击)",
            "处罚:扣分或取消资格",
        ]
    }

    pub fn section_3(&self) -> Vec<&'static str> {
        vec![
            "蝇量级125磅",
            "雏量级135磅",
            "羽量级145磅",
            "轻量级155磅",
            "次中量级170磅",
            "中量级185磅",
            "轻重量级205磅",
            "重量级265磅",
        ]
    }
}
impl Rule for MmaUfcRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("mma_ufc")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "UFC综合格斗规则",
            &[
                ("比赛规则", &self.section_0()),
                ("得分规则", &self.section_1()),
                ("犯规", &self.section_2()),
                ("体重级别", &self.section_3()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MmaUfcRules::new();
        assert!(!r.explain().is_empty());
    }
}
