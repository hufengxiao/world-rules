//! 食物中毒应对
//!
//! 食物中毒症状识别、家庭应对与就医判断规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: FoodPoisoningCareRules,
    name: "食物中毒应对",
    desc: "食物中毒症状识别、家庭应对与就医判断规则",
    origin: "医学",
    tags: ["健康", "食物", "中毒", "安全", "急症"]
}

impl FoodPoisoningCareRules {
    /// 症状识别
    pub fn symptoms(&self) -> Vec<&'static str> {
        vec![
            "识别恶心呕吐、腹痛腹泻",
            "注意发热与脱水迹象",
            "多人同食后相似症状需警惕",
            "症状严重或持续需就医",
        ]
    }

    /// 家庭护理
    pub fn care(&self) -> Vec<&'static str> {
        vec![
            "补充水分与电解质防脱水",
            "呕吐后暂缓进食逐渐试喂",
            "清淡易消化饮食",
            "避免乱用止泄药掩盖病情",
        ]
    }

    /// 就医判断
    pub fn seek_care(&self) -> Vec<&'static str> {
        vec![
            "持续剧烈呕吐无法进水时",
            "高热寒战或血便需及时就医",
            "老年人孕妇儿童更需警惕",
            "疑似食用毒菇等立即急诊",
        ]
    }

    /// 食品安全
    pub fn prevention(&self) -> Vec<&'static str> {
        vec![
            "生熟分开存放充分加热",
            "不食过期变质食物",
            "注意凉拌与隔夜菜品",
            "食品购买选择正规渠道",
        ]
    }
}

impl Rule for FoodPoisoningCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("food_poisoning")
    }

    fn explain(&self) -> String {
        format!(
            "【食物中毒应对】\n{}",
            [
                format!(
                    "症状识别：\\n{}",
                    self.symptoms()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "家庭护理：\\n{}",
                    self.care()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医判断：\\n{}",
                    self.seek_care()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "食品安全：\\n{}",
                    self.prevention()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
            ]
            .join("\n\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_foodpoisoningcarerules_basic() {
        let rules = FoodPoisoningCareRules::new();
        assert_eq!(rules.metadata().name, "食物中毒应对");
        assert!(!rules.symptoms().is_empty());
        assert!(!rules.care().is_empty());
        assert!(!rules.seek_care().is_empty());
        assert!(!rules.prevention().is_empty());
    }

    #[test]
    fn test_foodpoisoningcarerules_validation() {
        let rules = FoodPoisoningCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("food_poisoning"));
    }

    #[test]
    fn test_foodpoisoningcarerules_explain() {
        let rules = FoodPoisoningCareRules::new();
        let e = rules.explain();
        assert!(e.contains("症状识别"));
        assert!(e.contains("家庭护理"));
        assert!(e.contains("就医判断"));
    }
}
