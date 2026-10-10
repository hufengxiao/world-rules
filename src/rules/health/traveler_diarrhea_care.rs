//! 旅行者腹泻应对
//!
//! 外出旅行时腹泻的预防与应对规则，包括饮食饮水安全、补液、用药分寸与就医判断，保障旅途健康。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: TravelerDiarrheaCareRules,
    name: "旅行者腹泻应对",
    desc: "外出旅行时腹泻的预防与应对规则，包括饮食饮水安全、补液、用药分寸与就医判断，保障旅途健康。",
    origin: "旅行医学与感染科防护建议",
    tags: ["健康", "旅行", "腹泻", "防护"]
}

impl TravelerDiarrheaCareRules {
    /// 预防要点
    pub fn prevention(&self) -> Vec<&'static str> {
        vec![
            "饮用瓶装或煮沸的水",
            "不吃生冷未熟食物",
            "水果去皮后食用",
            "饭前便后洗手",
        ]
    }

    /// 补液护理
    pub fn hydration(&self) -> Vec<&'static str> {
        vec![
            "及时补充口服补液盐",
            "少量多次饮水",
            "避免含糖过高饮料",
            "注意电解质平衡",
        ]
    }

    /// 饮食调整
    pub fn diet(&self) -> Vec<&'static str> {
        vec![
            "暂食清淡易消化食物",
            "避免油腻乳制品",
            "少量多餐减轻肠胃负担",
            "症状缓解后逐步恢复",
        ]
    }

    /// 用药与就医
    pub fn medical(&self) -> Vec<&'static str> {
        vec![
            "备好常用止泻药物",
            "按说明用药不滥用抗生素",
            "高热血便脱水须就医",
            "症状持续超三天及时就诊",
        ]
    }
}

impl Rule for TravelerDiarrheaCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("traveler_diarrhea")
    }

    fn explain(&self) -> String {
        format!(
            "【旅行者腹泻应对】\n{}",
            [
                format!(
                    "预防要点：\\n{}",
                    self.prevention()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "补液护理：\\n{}",
                    self.hydration()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "饮食调整：\\n{}",
                    self.diet()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "用药与就医：\\n{}",
                    self.medical()
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
    fn test_travelerdiarrheacarerules_basic() {
        let rules = TravelerDiarrheaCareRules::new();
        assert_eq!(rules.metadata().name, "旅行者腹泻应对");
        assert!(!rules.prevention().is_empty());
        assert!(!rules.hydration().is_empty());
        assert!(!rules.diet().is_empty());
        assert!(!rules.medical().is_empty());
    }

    #[test]
    fn test_travelerdiarrheacarerules_validation() {
        let rules = TravelerDiarrheaCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("traveler_diarrhea"));
    }

    #[test]
    fn test_travelerdiarrheacarerules_explain() {
        let rules = TravelerDiarrheaCareRules::new();
        let e = rules.explain();
        assert!(e.contains("预防要点"));
        assert!(e.contains("补液护理"));
        assert!(e.contains("饮食调整"));
    }
}
