//! 交通事故处理
//!
//! 发生交通事故时的现场处置、取证与责任处理要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: TrafficAccidentHandlingRules,
    name: "交通事故处理",
    desc: "发生交通事故时的现场处置、取证与责任处理要点",
    origin: "中国",
    tags: ["法律", "交通", "事故", "处理", "维权"]
}

impl TrafficAccidentHandlingRules {
    /// 现场处置
    pub fn on_scene(&self) -> Vec<&'static str> {
        vec![
            "立即靠边停车开启警示",
            "有人受伤先救助并报警",
            "轻微事故可拍照后撤离",
            "设置警示标志防二次事故",
        ]
    }

    /// 固定证据
    pub fn evidence(&self) -> Vec<&'static str> {
        vec![
            "拍摄现场与车辆位置",
            "记录对方车辆与信息",
            "留存行车记录仪内容",
            "保留责任认定书与单据",
        ]
    }

    /// 责任与理赔
    pub fn claims(&self) -> Vec<&'static str> {
        vec![
            "配合交警责任认定",
            "联系保险公司报备",
            "如实陈述事故经过",
            "按程序提交理赔材料",
        ]
    }

    /// 法律途径
    pub fn law(&self) -> Vec<&'static str> {
        vec![
            "协商达不成责任一致的再依法处理",
            "责任认定不符可申请复核",
            "损失主张应留有票据证据",
            "拒绝肇事逃逸等违法行为",
        ]
    }
}

impl Rule for TrafficAccidentHandlingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("traffic_accident")
    }

    fn explain(&self) -> String {
        format!(
            "【交通事故处理】\n{}",
            [
                format!(
                    "现场处置：\\n{}",
                    self.on_scene()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "固定证据：\\n{}",
                    self.evidence()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "责任与理赔：\\n{}",
                    self.claims()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "法律途径：\\n{}",
                    self.law()
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
    fn test_trafficaccidenthandlingrules_basic() {
        let rules = TrafficAccidentHandlingRules::new();
        assert_eq!(rules.metadata().name, "交通事故处理");
        assert!(!rules.on_scene().is_empty());
        assert!(!rules.evidence().is_empty());
        assert!(!rules.claims().is_empty());
        assert!(!rules.law().is_empty());
    }

    #[test]
    fn test_trafficaccidenthandlingrules_validation() {
        let rules = TrafficAccidentHandlingRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("traffic_accident"));
    }

    #[test]
    fn test_trafficaccidenthandlingrules_explain() {
        let rules = TrafficAccidentHandlingRules::new();
        let e = rules.explain();
        assert!(e.contains("现场处置"));
        assert!(e.contains("固定证据"));
        assert!(e.contains("责任与理赔"));
    }
}
