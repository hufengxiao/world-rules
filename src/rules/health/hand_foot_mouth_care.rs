//! 手足口病护理
//!
//! 儿童手足口病的家庭护理与隔离防护规则，包括口腔与皮疹护理、饮食调整、消毒隔离及重症预警识别。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: HandFootMouthCareRules,
    name: "手足口病护理",
    desc: "儿童手足口病的家庭护理与隔离防护规则，包括口腔与皮疹护理、饮食调整、消毒隔离及重症预警识别。",
    origin: "儿科与疾控传染病防控建议",
    tags: ["健康", "儿童", "传染病", "护理"]
}

impl HandFootMouthCareRules {
    /// 隔离防护
    pub fn isolation(&self) -> Vec<&'static str> {
        vec![
            "发病期居家隔离避免接触他人",
            "玩具餐具单独使用并消毒",
            "照护者勤洗手",
            "症状消失后仍需观察数日",
        ]
    }

    /// 口腔护理
    pub fn oral(&self) -> Vec<&'static str> {
        vec![
            "饭后温水漱口保持清洁",
            "口腔溃疡可涂护创药物",
            "避免酸辣烫等刺激食物",
            "少量多次喂温水",
        ]
    }

    /// 皮疹护理
    pub fn skin_care(&self) -> Vec<&'static str> {
        vec![
            "保持皮肤清洁干燥",
            "剪短指甲防止抓破",
            "穿柔软宽松衣物",
            "疱疹破溃处防感染",
        ]
    }

    /// 饮食与预警
    pub fn diet(&self) -> Vec<&'static str> {
        vec![
            "给温凉流质或软食",
            "避免坚硬粗糙食物",
            "持续高热精神差须就医",
            "出现惊跳肢体抖动立即急诊",
        ]
    }
}

impl Rule for HandFootMouthCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("hfmd")
    }

    fn explain(&self) -> String {
        format!(
            "【手足口病护理】\n{}",
            [
                format!(
                    "隔离防护：\\n{}",
                    self.isolation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "口腔护理：\\n{}",
                    self.oral()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "皮疹护理：\\n{}",
                    self.skin_care()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "饮食与预警：\\n{}",
                    self.diet()
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
    fn test_handfootmouthcarerules_basic() {
        let rules = HandFootMouthCareRules::new();
        assert_eq!(rules.metadata().name, "手足口病护理");
        assert!(!rules.isolation().is_empty());
        assert!(!rules.oral().is_empty());
        assert!(!rules.skin_care().is_empty());
        assert!(!rules.diet().is_empty());
    }

    #[test]
    fn test_handfootmouthcarerules_validation() {
        let rules = HandFootMouthCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("hfmd"));
    }

    #[test]
    fn test_handfootmouthcarerules_explain() {
        let rules = HandFootMouthCareRules::new();
        let e = rules.explain();
        assert!(e.contains("隔离防护"));
        assert!(e.contains("口腔护理"));
        assert!(e.contains("皮疹护理"));
    }
}
