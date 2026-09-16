//! 地铁搭乘与乘车礼仪
//!
//! 地铁、轨道交通等出入站与车厢内的礼仪与公共规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SubwayEtiquetteRules,
    name: "地铁搭乘与乘车礼仪",
    desc: "地铁、轨道交通等出入站与车厢内的礼仪与公共规则",
    origin: "公共交通",
    tags: ["社交", "礼仪", "地铁", "公共交通", "乘车"]
}

impl SubwayEtiquetteRules {
    /// 候车与上下车
    pub fn boarding(&self) -> Vec<&'static str> {
        vec![
            "站在站台黄线外顺序排队",
            "遵循先下后上原则",
            "不抢门不推挤",
            "让急于下车者先行通过",
        ]
    }

    /// 车厢内礼仪
    pub fn carriage(&self) -> Vec<&'static str> {
        vec![
            "主动为老幼病残孕让座",
            "不倚靠或独占扶手栏杆",
            "随身包物不占邻座",
            "不长时间接听大声电话",
        ]
    }

    /// 公共卫生
    pub fn hygiene(&self) -> Vec<&'static str> {
        vec![
            "不在车厢内饮食",
            "不随地吐痰丢物",
            "咳嗽喷嚏用手肘遮挡",
            "异味食物不带入车厢",
        ]
    }

    /// 安全常识
    pub fn safety(&self) -> Vec<&'static str> {
        vec![
            "站台候车不越过安全线",
            "车门警示时不强挤",
            "不倚靠护栏与车门",
            "紧急时听从乘务人员引导",
        ]
    }
}

impl Rule for SubwayEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("subway")
    }

    fn explain(&self) -> String {
        format!(
            "【地铁搭乘与乘车礼仪】\n{}",
            [
                format!(
                    "候车与上下车：\\n{}",
                    self.boarding()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "车厢内礼仪：\\n{}",
                    self.carriage()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "公共卫生：\\n{}",
                    self.hygiene()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安全常识：\\n{}",
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
    fn test_subwayetiquetterules_basic() {
        let rules = SubwayEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "地铁搭乘与乘车礼仪");
        assert!(!rules.boarding().is_empty());
        assert!(!rules.carriage().is_empty());
        assert!(!rules.hygiene().is_empty());
        assert!(!rules.safety().is_empty());
    }

    #[test]
    fn test_subwayetiquetterules_validation() {
        let rules = SubwayEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("subway"));
    }

    #[test]
    fn test_subwayetiquetterules_explain() {
        let rules = SubwayEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("候车与上下车"));
        assert!(e.contains("车厢内礼仪"));
        assert!(e.contains("公共卫生"));
    }
}
