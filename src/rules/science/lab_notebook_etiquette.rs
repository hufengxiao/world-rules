//! 实验记录规范
//!
//! 科学实验记录本规范记录、可复现与诚信原则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: LabNotebookEtiquetteRules,
    name: "实验记录规范",
    desc: "科学实验记录本规范记录、可复现与诚信原则",
    origin: "国际",
    tags: ["科学", "实验", "记录", "可复现", "诚信"]
}

impl LabNotebookEtiquetteRules {
    /// 及时记录
    pub fn timely(&self) -> Vec<&'static str> {
        vec![
            "实验当日即记录",
            "按时间顺序详实记载",
            "记录条件与步骤",
            "不依赖事后回忆补记",
        ]
    }

    /// 内容完整
    pub fn detail(&self) -> Vec<&'static str> {
        vec![
            "写明原料仪器与用量",
            "记录环境温度湿度",
            "标注观察现象与数据",
            "记录异常与偏差",
        ]
    }

    /// 可复现性
    pub fn reproducible(&self) -> Vec<&'static str> {
        vec![
            "写得他人可重复操作",
            "标注每次改动处",
            "保留原始数据的来源",
            "图文并茂清楚易读",
        ]
    }

    /// 诚信原则
    pub fn honesty(&self) -> Vec<&'static str> {
        vec![
            "不编造或篡改数据",
            "如实记录失败实验",
            "标注他人贡献与引用",
            "不把参考当自己的结果",
        ]
    }
}

impl Rule for LabNotebookEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("lab_notebook")
    }

    fn explain(&self) -> String {
        format!(
            "【实验记录规范】\n{}",
            [
                format!(
                    "及时记录：\\n{}",
                    self.timely()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "内容完整：\\n{}",
                    self.detail()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "可复现性：\\n{}",
                    self.reproducible()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "诚信原则：\\n{}",
                    self.honesty()
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
    fn test_labnotebooketiquetterules_basic() {
        let rules = LabNotebookEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "实验记录规范");
        assert!(!rules.timely().is_empty());
        assert!(!rules.detail().is_empty());
        assert!(!rules.reproducible().is_empty());
        assert!(!rules.honesty().is_empty());
    }

    #[test]
    fn test_labnotebooketiquetterules_validation() {
        let rules = LabNotebookEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("lab_notebook"));
    }

    #[test]
    fn test_labnotebooketiquetterules_explain() {
        let rules = LabNotebookEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("及时记录"));
        assert!(e.contains("内容完整"));
        assert!(e.contains("可复现性"));
    }
}
