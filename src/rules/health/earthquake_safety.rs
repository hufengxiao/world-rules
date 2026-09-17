//! 地震避险与逃生
//!
//! 地震时的避险、自我保护与震后安全要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: EarthquakeSafetyRules,
    name: "地震避险与逃生",
    desc: "地震时的避险、自我保护与震后安全要点",
    origin: "安全",
    tags: ["健康", "地震", "避险", "逃生"]
}

impl EarthquakeSafetyRules {
    /// 地震应急
    pub fn during(&self) -> Vec<&'static str> {
        vec![
            "闻到强震信号立即避险",
            "采取伏地遮挡抓牢姿势",
            "就近躲在桌下或结实屋角",
            "避开玻璃窗悬挂物重家具",
        ]
    }

    /// 室内避险
    pub fn indoor(&self) -> Vec<&'static str> {
        vec![
            "不贸然冲出门或乘电梯",
            "护住头部保持镇定",
            "等震动减轻再评估撤离",
            "远离外墙与门窗",
        ]
    }

    /// 室外注意
    pub fn outdoor(&self) -> Vec<&'static str> {
        vec![
            "远离建筑物电线杆树木",
            "避开山体滑坡与危树危石",
            "停车远离高压设施",
            "海边警惕海啸风险",
        ]
    }

    /// 震后处置
    pub fn after(&self) -> Vec<&'static str> {
        vec![
            "检查燃气水电受损",
            "关闭危险管线",
            "预防余震保持警觉",
            "听从应急通知与救援引导",
        ]
    }
}

impl Rule for EarthquakeSafetyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("earthquake_safety")
    }

    fn explain(&self) -> String {
        format!(
            "【地震避险与逃生】\n{}",
            [
                format!(
                    "地震应急：\\n{}",
                    self.during()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "室内避险：\\n{}",
                    self.indoor()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "室外注意：\\n{}",
                    self.outdoor()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "震后处置：\\n{}",
                    self.after()
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
    fn test_earthquakesafetyrules_basic() {
        let rules = EarthquakeSafetyRules::new();
        assert_eq!(rules.metadata().name, "地震避险与逃生");
        assert!(!rules.during().is_empty());
        assert!(!rules.indoor().is_empty());
        assert!(!rules.outdoor().is_empty());
        assert!(!rules.after().is_empty());
    }

    #[test]
    fn test_earthquakesafetyrules_validation() {
        let rules = EarthquakeSafetyRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("earthquake_safety"));
    }

    #[test]
    fn test_earthquakesafetyrules_explain() {
        let rules = EarthquakeSafetyRules::new();
        let e = rules.explain();
        assert!(e.contains("地震应急"));
        assert!(e.contains("室内避险"));
        assert!(e.contains("室外注意"));
    }
}
