//! 急救规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: FirstAidRules, name: "急救规则", desc: "基本急救规则", origin: "国际", tags: ["健康", "急救"] }
impl FirstAidRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "确认安全环境",
            "判断意识:轻拍重唤",
            "拨打120急救电话",
            "胸外按压:双手交叠掌根按压胸骨下半段",
            "按压深度5-6cm频率100-120次/分钟",
            "30次按压后2次人工呼吸",
            "持续直到急救人员到达",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "直接压迫:用干净布料直接按压伤口",
            "抬高肢体:受伤肢体抬高于心脏",
            "止血带:四肢大出血时使用(每小时放松1次)",
            "填塞:深部伤口用干净布料填塞",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "烫伤:冲脱泡盖送(冷水冲洗15分钟以上)",
            "骨折:固定制动不要复位",
            "中毒:拨打120保留毒物样本",
            "溺水:先确保自身安全再施救",
            "触电:先断电源再施救",
        ]
    }
}
impl Rule for FirstAidRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("first_aid")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "急救规则",
            &[
                ("心肺复苏CPR", &self.section_0()),
                ("止血", &self.section_1()),
                ("常见急救", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = FirstAidRules::new();
        assert!(!r.explain().is_empty());
    }
}
