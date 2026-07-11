//! 残疾人乒乓球规则
//!
//! 残疾人乒乓球包括站立和轮椅级别。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 残疾人乒乓球规则
pub struct ParaTableTennisRules {
    metadata: RuleMetadata,
}

impl ParaTableTennisRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("残疾人乒乓球规则", "残疾人乒乓球比赛规则")
                .with_origin("ITTF/IPC")
                .with_tags(vec!["体育".into(), "乒乓球".into(), "残奥".into()]),
        }
    }

    /// 运动分级
    pub fn classification(&self) -> Vec<&'static str> {
        vec![
            "TT1-TT5: 轮椅级别",
            "TT6-TT10: 站立级别",
            "TT11: 智力残疾",
            "数字越小=残疾程度越重",
            "TT1: 最严重肢体残疾",
            "TT10: 最轻度肢体残疾",
        ]
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "单打: 按级别",
            "团体: 按级别分组",
            "混合团体",
            "奥运会项目: 11个小项",
            "世界锦标赛: 全级别",
            "比赛制: 11分制",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "球拍: ITTF认证",
            "球: 标准乒乓球",
            "球台: 标准高度",
            "轮椅: 标准（轮椅级）",
            "假肢: 允许",
            "网: 标准高度",
            "禁止: 非认证器材",
        ]
    }

    /// 适应性规则
    pub fn adaptations(&self) -> Vec<&'static str> {
        vec![
            "轮椅级别: 球台高度适配",
            "轮椅固定装置允许",
            "假肢辅助允许",
            "站立级别: 可选支撑",
            "发球规则适配",
            "边界判定调整",
        ]
    }

    /// 发球规则
    pub fn serving(&self) -> Vec<&'static str> {
        vec![
            "轮椅级: 发球可从后方",
            "抛球: 16厘米以上",
            "抛球困难: 可不抛球",
            "轮换发球: 每2分",
            "决胜: 每1分轮换",
            "违例: 发球犯规",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "轮椅触网",
            "球拍触球台",
            "非持拍手触球台",
            "发球违例",
            "接受非法指导",
            "分级违规",
        ]
    }

    /// 参赛资格
    pub fn eligibility(&self) -> Vec<&'static str> {
        vec![
            "ITTF分级认证",
            "最低残疾标准",
            "国际分级证书",
            "ITTF注册",
            "达标成绩",
        ]
    }
}

impl Default for ParaTableTennisRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ParaTableTennisRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("para_table_tennis")
    }

    fn explain(&self) -> String {
        format!(
            "【残疾人乒乓球规则】\n\n\
            运动分级:\n{}\n\n\
            比赛项目:\n{}\n\n\
            装备要求:\n{}\n\n\
            发球规则:\n{}",
            self.classification()
                .iter()
                .map(|c| format!("  • {}", c))
                .collect::<Vec<_>>()
                .join("\n"),
            self.events()
                .iter()
                .map(|e| format!("  • {}", e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.equipment()
                .iter()
                .map(|eq| format!("  • {}", eq))
                .collect::<Vec<_>>()
                .join("\n"),
            self.serving()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_para_table_tennis_rules_basic() {
        let rules = ParaTableTennisRules::new();
        assert_eq!(rules.metadata().name, "残疾人乒乓球规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_para_table_tennis_classification() {
        let rules = ParaTableTennisRules::new();
        let classification = rules.classification();
        assert!(classification.iter().any(|c| c.contains("TT")));
        assert!(classification.iter().any(|c| c.contains("轮椅")));
        assert!(classification.iter().any(|c| c.contains("站立")));
        assert!(classification.len() >= 4);
    }

    #[test]
    fn test_para_table_tennis_events() {
        let rules = ParaTableTennisRules::new();
        let events = rules.events();
        assert!(events.iter().any(|e| e.contains("单打")));
        assert!(events.iter().any(|e| e.contains("团体")));
        assert!(events.len() >= 4);
    }

    #[test]
    fn test_para_table_tennis_serving() {
        let rules = ParaTableTennisRules::new();
        let serving = rules.serving();
        assert!(serving.iter().any(|s| s.contains("发球")));
        assert!(serving.len() >= 4);
    }

    #[test]
    fn test_para_table_tennis_category() {
        let rules = ParaTableTennisRules::new();
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }
}
