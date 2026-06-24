//! 武术IWUF规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: WushuIwufRules, name: "武术IWUF规则", desc: "国际武术联合会规则", origin: "中国", tags: ["体育", "格斗"] }
impl WushuIwufRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "长拳:快速灵活多跳跃旋转",
            "南拳:刚猛有力发声助力",
            "太极:缓慢柔和连绵不断",
            "刀剑枪棍:器械套路",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "拳腿摔三种技术",
            "得分:拳1分腿2分摔3分",
            "每局2分钟共3局",
            "禁止:击打后脑裆部咽喉",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "套路:动作质量演练水平难度分",
            "散打:有效打击得分",
            "裁判组:3-5名裁判打分",
        ]
    }
}
impl Rule for WushuIwufRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("wushu_iwuf")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "武术IWUF规则",
            &[
                ("套路比赛", &self.section_0()),
                ("散打比赛", &self.section_1()),
                ("评分规则", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = WushuIwufRules::new();
        assert!(!r.explain().is_empty());
    }
}
