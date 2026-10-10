//! 温泉礼仪
//!
//! 泡温泉的入浴规范与公共场所礼节

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: HotSpringEtiquetteRules,
    name: "温泉礼仪",
    desc: "泡温泉的入浴规范与公共场所礼节",
    origin: "日本",
    tags: ["公共场所", "入浴", "礼仪"]
}

impl HotSpringEtiquetteRules {
    /// 入浴准备
    pub fn preparation(&self) -> Vec<&'static str> {
        vec![
            "入池前先淋浴洗净",
            "长发盘起不入水",
            "随身物品放入储物柜",
            "换洗衣物放指定处",
        ]
    }

    /// 入池礼节
    pub fn bathing(&self) -> Vec<&'static str> {
        vec![
            "毛巾不浸入池中",
            "不在池内搓洗身体",
            "保持安静勿喧哗",
            "礼让空间不挤占",
        ]
    }

    /// 公共秩序
    pub fn order(&self) -> Vec<&'static str> {
        vec![
            "按指示区分男女区",
            "纹身者依规定处理",
            "不携带食物入浴",
            "不在池边奔跑",
        ]
    }

    /// 安全健康
    pub fn safety(&self) -> Vec<&'static str> {
        vec![
            "泡浴时间不宜过久",
            "及时补充水分",
            "酒后不宜入浴",
            "身体不适立即离开",
        ]
    }
}

impl Rule for HotSpringEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("hot_spring_etiquette")
    }

    fn explain(&self) -> String {
        format!(
            "【温泉礼仪】\n{}",
            [
                format!(
                    "入浴准备：\\n{}",
                    self.preparation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "入池礼节：\\n{}",
                    self.bathing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "公共秩序：\\n{}",
                    self.order()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安全健康：\\n{}",
                    self.safety()
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
    fn test_hotspringetiquetterules_basic() {
        let rules = HotSpringEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "温泉礼仪");
        assert!(!rules.preparation().is_empty());
        assert!(!rules.bathing().is_empty());
        assert!(!rules.order().is_empty());
        assert!(!rules.safety().is_empty());
    }

    #[test]
    fn test_hotspringetiquetterules_validation() {
        let rules = HotSpringEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::social("hot_spring_etiquette")
        );
    }

    #[test]
    fn test_hotspringetiquetterules_explain() {
        let rules = HotSpringEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("入浴准备"));
        assert!(e.contains("入池礼节"));
        assert!(e.contains("公共秩序"));
    }
}
