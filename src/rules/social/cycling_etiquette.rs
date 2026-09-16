//! 骑行礼仪与安全
//!
//! 城市与郊野骑行时的道路交通礼仪与安全规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CyclingEtiquetteRules,
    name: "骑行礼仪与安全",
    desc: "城市与郊野骑行时的道路交通礼仪与安全规则",
    origin: "国际",
    tags: ["社交", "礼仪", "骑行", "自行车", "安全"]
}

impl CyclingEtiquetteRules {
    /// 上路准备
    pub fn preparation(&self) -> Vec<&'static str> {
        vec![
            "骑行前检查刹车轮胎链条",
            "规范佩戴头盔",
            "夜间装置反光与车灯",
            "了解本地骑行交通规则",
        ]
    }

    /// 道路行为
    pub fn on_road(&self) -> Vec<&'static str> {
        vec![
            "靠右骑行不逆行",
            "遵守信号灯与交通标线",
            "变道转向提前打手势",
            "不并排占道或追逐竞驶",
        ]
    }

    /// 与人相处
    pub fn with_others(&self) -> Vec<&'static str> {
        vec![
            "经过行人减速并保持距离",
            "超车前清晰提示",
            "路口减速礼让",
            "不随意穿行机动车道",
        ]
    }

    /// 停放
    pub fn parking(&self) -> Vec<&'static str> {
        vec!["在指定区域规范停放", "不影响他人通行", "留意停放处并上锁"]
    }
}

impl Rule for CyclingEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("cycling")
    }

    fn explain(&self) -> String {
        format!(
            "【骑行礼仪与安全】\n{}",
            [
                format!(
                    "上路准备：\\n{}",
                    self.preparation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "道路行为：\\n{}",
                    self.on_road()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "与人相处：\\n{}",
                    self.with_others()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "停放：\\n{}",
                    self.parking()
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
    fn test_cyclingetiquetterules_basic() {
        let rules = CyclingEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "骑行礼仪与安全");
        assert!(!rules.preparation().is_empty());
        assert!(!rules.on_road().is_empty());
        assert!(!rules.with_others().is_empty());
        assert!(!rules.parking().is_empty());
    }

    #[test]
    fn test_cyclingetiquetterules_validation() {
        let rules = CyclingEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("cycling"));
    }

    #[test]
    fn test_cyclingetiquetterules_explain() {
        let rules = CyclingEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("上路准备"));
        assert!(e.contains("道路行为"));
        assert!(e.contains("与人相处"));
    }
}
