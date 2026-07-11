//! 残疾人羽毛球规则
//!
//! 残疾人羽毛球是残奥会正式项目。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 残疾人羽毛球规则
pub struct ParaBadmintonRules {
    metadata: RuleMetadata,
}

impl ParaBadmintonRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("残疾人羽毛球规则", "残疾人羽毛球比赛规则")
                .with_origin("BWF/IPC")
                .with_tags(vec!["体育".into(), "羽毛球".into(), "残奥".into()]),
        }
    }

    /// 运动分级
    pub fn classification(&self) -> Vec<&'static str> {
        vec![
            "WH1: 轮椅较严重残疾",
            "WH2: 轮椅较轻残疾",
            "SL3: 站立下肢较重残疾",
            "SL4: 站立下肢较轻残疾",
            "SU5: 站立上肢残疾",
            "SH6: 身材矮小",
        ]
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "残奥会: 14个小项",
            "世界锦标赛",
            "洲际锦标赛",
            "国际系列赛",
            "项目: 单打、双打、混合",
            "比赛制: 小组+淘汰",
        ]
    }

    /// 场地规格
    pub fn court(&self) -> Vec<&'static str> {
        vec![
            "轮椅组: 半场（6.1×6.7米）",
            "站立组: 标准场地（6.1×13.4米）",
            "网高: 1.55米（边）1.524米（中）",
            "发球线: 根据分级调整",
            "地面: 木地板或塑胶",
            "灯光: 标准照明",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "球拍: BWF认证",
            "羽毛球: 标准速度",
            "轮椅: 竞技轮椅",
            "义肢: 允许使用",
            "护具: 允许",
            "服装: BWF认证",
        ]
    }

    /// 技术规则
    pub fn technique(&self) -> Vec<&'static str> {
        vec![
            "得分制: 21分三局两胜",
            "发球: 下手发球",
            "换边: 每局结束",
            "休息: 60秒/局",
            "暂停: 11分休息",
            "轮椅组: 半场移动",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "发球违规",
            "过网击球",
            "触网",
            "连击",
            "持球",
            "轮椅越线（轮椅组）",
            "界外球",
        ]
    }

    /// 适应性规则
    pub fn adaptations(&self) -> Vec<&'static str> {
        vec![
            "轮椅组: 半场比赛",
            "站立组: 全场比赛",
            "发球线调整",
            "网高调整",
            "轮椅固定装置",
            "义肢使用允许",
        ]
    }

    /// 参赛资格
    pub fn eligibility(&self) -> Vec<&'static str> {
        vec![
            "BWF分级认证",
            "最低残疾标准",
            "国际羽联注册",
            "积分排名",
            "体检合格证明",
        ]
    }
}

impl Default for ParaBadmintonRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ParaBadmintonRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("para_badminton")
    }

    fn explain(&self) -> String {
        format!(
            "【残疾人羽毛球规则】\n\n\
            运动分级:\n{}\n\n\
            比赛项目:\n{}\n\n\
            场地规格:\n{}\n\n\
            技术规则:\n{}",
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
            self.court()
                .iter()
                .map(|c| format!("  • {}", c))
                .collect::<Vec<_>>()
                .join("\n"),
            self.technique()
                .iter()
                .map(|t| format!("  • {}", t))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_para_badminton_rules_basic() {
        let rules = ParaBadmintonRules::new();
        assert_eq!(rules.metadata().name, "残疾人羽毛球规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_para_badminton_classification() {
        let rules = ParaBadmintonRules::new();
        let classification = rules.classification();
        assert!(classification.iter().any(|c| c.contains("WH1")));
        assert!(classification.iter().any(|c| c.contains("WH2")));
        assert!(classification.iter().any(|c| c.contains("站立")));
        assert!(classification.len() >= 4);
    }

    #[test]
    fn test_para_badminton_court() {
        let rules = ParaBadmintonRules::new();
        let court = rules.court();
        assert!(court.iter().any(|c| c.contains("轮椅")));
        assert!(court.iter().any(|c| c.contains("网高")));
        assert!(court.len() >= 4);
    }

    #[test]
    fn test_para_badminton_technique() {
        let rules = ParaBadmintonRules::new();
        let technique = rules.technique();
        assert!(technique.iter().any(|t| t.contains("21分")));
        assert!(technique.iter().any(|t| t.contains("发球")));
        assert!(technique.len() >= 4);
    }

    #[test]
    fn test_para_badminton_category() {
        let rules = ParaBadmintonRules::new();
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }
}
