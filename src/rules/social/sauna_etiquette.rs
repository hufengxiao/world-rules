//! 桑拿礼仪
//!
//! 公共桑拿房的共用规范与安全礼节

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SaunaEtiquetteRules,
    name: "桑拿礼仪",
    desc: "公共桑拿房的共用规范与安全礼节",
    origin: "北欧",
    tags: ["公共场所", "桑拿", "礼仪"]
}

impl SaunaEtiquetteRules {
    /// 入房准备
    pub fn preparation(&self) -> Vec<&'static str> {
        vec!["入房前先淋浴", "用毛巾垫坐", "备好替换衣物", "依规定着装"]
    }

    /// 共用礼节
    pub fn sharing(&self) -> Vec<&'static str> {
        vec![
            "保持安静不喧哗",
            "礼让空间",
            "不长时间占用座位",
            "浇水量适度",
        ]
    }

    /// 安全注意
    pub fn safety(&self) -> Vec<&'static str> {
        vec!["控制停留时间", "及时补水", "头晕即离开", "心脏病患者慎入"]
    }

    /// 前后护理
    pub fn care(&self) -> Vec<&'static str> {
        vec![
            "出房后温水冲洗",
            "避免骤冷刺激",
            "充分休息再离开",
            "清洁座位区域",
        ]
    }
}

impl Rule for SaunaEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("sauna_etiquette")
    }

    fn explain(&self) -> String {
        format!(
            "【桑拿礼仪】\n{}",
            [
                format!(
                    "入房准备：\\n{}",
                    self.preparation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "共用礼节：\\n{}",
                    self.sharing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安全注意：\\n{}",
                    self.safety()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "前后护理：\\n{}",
                    self.care()
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
    fn test_saunaetiquetterules_basic() {
        let rules = SaunaEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "桑拿礼仪");
        assert!(!rules.preparation().is_empty());
        assert!(!rules.sharing().is_empty());
        assert!(!rules.safety().is_empty());
        assert!(!rules.care().is_empty());
    }

    #[test]
    fn test_saunaetiquetterules_validation() {
        let rules = SaunaEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("sauna_etiquette"));
    }

    #[test]
    fn test_saunaetiquetterules_explain() {
        let rules = SaunaEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("入房准备"));
        assert!(e.contains("共用礼节"));
        assert!(e.contains("安全注意"));
    }
}
