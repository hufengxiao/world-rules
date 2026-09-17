//! 机场出行礼仪
//!
//! 机场值机、安检、候机与登机的秩序礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: AirportEtiquetteRules,
    name: "机场出行礼仪",
    desc: "机场值机、安检、候机与登机的秩序礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "机场", "候机", "登机"]
}

impl AirportEtiquetteRules {
    /// 值机与安检
    pub fn checkin(&self) -> Vec<&'static str> {
        vec![
            "预留足够时间到机场",
            "按提示有序办理值机",
            "配合安检顺序",
            "取回行李不落失",
        ]
    }

    /// 候机大厅
    pub fn lounge(&self) -> Vec<&'static str> {
        vec![
            "不大声喧哗影响他人",
            "不占座躺卧",
            "手机保持静音接听",
            "整理随行物品不散落",
        ]
    }

    /// 登机秩序
    pub fn boarding(&self) -> Vec<&'static str> {
        vec![
            "按舱位与顺序登机",
            "登机时有序不挤",
            "就座后放好随身行李",
            "不霸占头顶空间",
        ]
    }

    /// 同行乘客
    pub fn passenger(&self) -> Vec<&'static str> {
        vec![
            "遵守工作人员引导",
            "尊重同机旅客",
            "飞行中守纪律听从指示",
            "机上礼貌沟通",
        ]
    }
}

impl Rule for AirportEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("airport")
    }

    fn explain(&self) -> String {
        format!(
            "【机场出行礼仪】\n{}",
            [
                format!(
                    "值机与安检：\\n{}",
                    self.checkin()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "候机大厅：\\n{}",
                    self.lounge()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "登机秩序：\\n{}",
                    self.boarding()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "同行乘客：\\n{}",
                    self.passenger()
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
    fn test_airportetiquetterules_basic() {
        let rules = AirportEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "机场出行礼仪");
        assert!(!rules.checkin().is_empty());
        assert!(!rules.lounge().is_empty());
        assert!(!rules.boarding().is_empty());
        assert!(!rules.passenger().is_empty());
    }

    #[test]
    fn test_airportetiquetterules_validation() {
        let rules = AirportEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("airport"));
    }

    #[test]
    fn test_airportetiquetterules_explain() {
        let rules = AirportEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("值机与安检"));
        assert!(e.contains("候机大厅"));
        assert!(e.contains("登机秩序"));
    }
}
