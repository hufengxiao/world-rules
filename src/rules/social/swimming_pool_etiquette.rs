//! 游泳馆礼仪
//!
//! 游泳馆、泳池内锻炼时的公共礼仪与安全规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SwimmingPoolEtiquetteRules,
    name: "游泳馆礼仪",
    desc: "游泳馆、泳池内锻炼时的公共礼仪与安全规则",
    origin: "国际",
    tags: ["社交", "礼仪", "游泳", "泳池"]
}

impl SwimmingPoolEtiquetteRules {
    /// 入场与更衣
    pub fn dress(&self) -> Vec<&'static str> {
        vec![
            "入水前先冲洗身体",
            "穿合适泳装进入泳区",
            "不在泳池边进食饮水",
            "遵守馆内人数与时段安排",
        ]
    }

    /// 泳道秩序
    pub fn lane(&self) -> Vec<&'static str> {
        vec![
            "按方向循环游动不倒行",
            "变速前先减速靠边",
            "不在泳道中央停歇占道",
            "泳速相差大时礼让快速泳道",
        ]
    }

    /// 安全规则
    pub fn safety(&self) -> Vec<&'static str> {
        vec![
            "不会游泳者佩戴浮具靠池边",
            "不进行危险跳水",
            "不干扰救生员工作",
            "深水区遵守深浅分界",
        ]
    }

    /// 卫生礼仪
    pub fn hygiene(&self) -> Vec<&'static str> {
        vec!["咳嗽喷嚏远离人群", "不在水中解便吐痰", "离开前冲洗干净"]
    }
}

impl Rule for SwimmingPoolEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("pool")
    }

    fn explain(&self) -> String {
        format!(
            "【游泳馆礼仪】\n{}",
            [
                format!(
                    "入场与更衣：\\n{}",
                    self.dress()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "泳道秩序：\\n{}",
                    self.lane()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安全规则：\\n{}",
                    self.safety()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "卫生礼仪：\\n{}",
                    self.hygiene()
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
    fn test_swimmingpooletiquetterules_basic() {
        let rules = SwimmingPoolEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "游泳馆礼仪");
        assert!(!rules.dress().is_empty());
        assert!(!rules.lane().is_empty());
        assert!(!rules.safety().is_empty());
        assert!(!rules.hygiene().is_empty());
    }

    #[test]
    fn test_swimmingpooletiquetterules_validation() {
        let rules = SwimmingPoolEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("pool"));
    }

    #[test]
    fn test_swimmingpooletiquetterules_explain() {
        let rules = SwimmingPoolEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("入场与更衣"));
        assert!(e.contains("泳道秩序"));
        assert!(e.contains("安全规则"));
    }
}
