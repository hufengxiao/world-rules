//! 足部水泡与磨伤护理
//!
//! 久走或运动导致的足部水泡、磨皮的处理与预防

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: FootBlisterCareRules,
    name: "足部水泡与磨伤护理",
    desc: "久走或运动导致的足部水泡、磨皮的处理与预防",
    origin: "国际",
    tags: ["健康", "足部", "水泡", "护理", "运动"]
}

impl FootBlisterCareRules {
    /// 水泡处理
    pub fn treat(&self) -> Vec<&'static str> {
        vec![
            "水泡小者消毒后贴敷保护",
            "大泡用无菌针头刺破引流",
            "破皮处清洗消毒防感染",
            "不硬撕脱落的皮肤",
        ]
    }

    /// 磨伤护理
    pub fn care(&self) -> Vec<&'static str> {
        vec![
            "磨伤处清洗干爽",
            "涂抗菌软膏后包扎",
            "观察有无红肿化脓",
            "愈合期间减少摩擦",
        ]
    }

    /// 预防
    pub fn prevention(&self) -> Vec<&'static str> {
        vec![
            "穿合脚减震的运动鞋",
            "新鞋先磨合再长走",
            "穿吸湿排汗的袜子",
            "长途前涂抹防磨膏",
        ]
    }

    /// 就医提示
    pub fn seek(&self) -> Vec<&'static str> {
        vec![
            "红肿发热伴疼痛加剧就医",
            "糖尿病患者水泡需专业处理",
            "疑有异物嵌入就医移除",
            "伤口长期不愈及时就诊",
        ]
    }
}

impl Rule for FootBlisterCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("foot_blister")
    }

    fn explain(&self) -> String {
        format!(
            "【足部水泡与磨伤护理】\n{}",
            [
                format!(
                    "水泡处理：\\n{}",
                    self.treat()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "磨伤护理：\\n{}",
                    self.care()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "预防：\\n{}",
                    self.prevention()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医提示：\\n{}",
                    self.seek()
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
    fn test_footblistercarerules_basic() {
        let rules = FootBlisterCareRules::new();
        assert_eq!(rules.metadata().name, "足部水泡与磨伤护理");
        assert!(!rules.treat().is_empty());
        assert!(!rules.care().is_empty());
        assert!(!rules.prevention().is_empty());
        assert!(!rules.seek().is_empty());
    }

    #[test]
    fn test_footblistercarerules_validation() {
        let rules = FootBlisterCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("foot_blister"));
    }

    #[test]
    fn test_footblistercarerules_explain() {
        let rules = FootBlisterCareRules::new();
        let e = rules.explain();
        assert!(e.contains("水泡处理"));
        assert!(e.contains("磨伤护理"));
        assert!(e.contains("预防"));
    }
}
