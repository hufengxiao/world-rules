//! 残疾人赛艇规则
//!
//! 残疾人赛艇包括各种适应性赛艇项目。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 残疾人赛艇规则
pub struct ParaRowingRules {
    metadata: RuleMetadata,
}

impl ParaRowingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("残疾人赛艇规则", "残疾人赛艇比赛规则")
                .with_origin("FISA/IPC")
                .with_tags(vec!["体育".into(), "赛艇".into(), "残奥".into()]),
        }
    }

    /// 运动分级
    pub fn classification(&self) -> Vec<&'static str> {
        vec![
            "PR1级: 躯干功能丧失（单桨）",
            "PR2级: 躯干和腿功能受限",
            "PR3级: 腿功能正常或部分限制",
            "PR3-Mix: 混合艇（男女混合）",
            "视觉残疾: PR3级别",
            "智力残疾: PR3级别",
        ]
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "PR1单人双桨: 1000米",
            "PR2双人双桨: 1000米",
            "PR3混合四人双桨: 2000米",
            "PR3混合双人双桨: 2000米",
            "残奥会项目: 4个小项",
            "世界锦标赛: 更多级别",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "赛艇: FISA认证规格",
            "固定座椅: PR1必需",
            "滑座: PR2/PR3使用",
            "桨: 标准赛艇桨",
            "浮标: 安全要求",
            "船号: 必须标明",
            "禁止: 电子辅助设备",
        ]
    }

    /// 适应性规则
    pub fn adaptations(&self) -> Vec<&'static str> {
        vec![
            "PR1: 固定座椅+背部支撑",
            "PR2: 固定座椅可选",
            "手绑装置允许",
            "足部固定允许",
            "视力残疾声音信号",
            "出发辅助允许",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "距离: 1000米/2000米",
            "航道: 6-8条",
            "预赛: 淘汰制",
            "决赛: A/B决赛",
            "时间限制规则",
            "重新出发规定",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "越出航道",
            "干扰他人",
            "装备违规",
            "接受非法援助",
            "出发犯规",
            "终点犯规",
        ]
    }

    /// 参赛资格
    pub fn eligibility(&self) -> Vec<&'static str> {
        vec![
            "必须通过分级评估",
            "FISA注册",
            "最低残疾标准",
            "游泳能力测试",
            "达标成绩",
        ]
    }
}

impl Default for ParaRowingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ParaRowingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("para_rowing")
    }

    fn explain(&self) -> String {
        format!(
            "【残疾人赛艇规则】\n\n\
            运动分级:\n{}\n\n\
            比赛项目:\n{}\n\n\
            装备要求:\n{}\n\n\
            适应性规则:\n{}",
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
            self.adaptations()
                .iter()
                .map(|a| format!("  • {}", a))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_para_rowing_rules_basic() {
        let rules = ParaRowingRules::new();
        assert_eq!(rules.metadata().name, "残疾人赛艇规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_para_rowing_classification() {
        let rules = ParaRowingRules::new();
        let classification = rules.classification();
        assert!(classification.iter().any(|c| c.contains("PR1")));
        assert!(classification.iter().any(|c| c.contains("PR2")));
        assert!(classification.iter().any(|c| c.contains("PR3")));
        assert!(classification.len() >= 4);
    }

    #[test]
    fn test_para_rowing_events() {
        let rules = ParaRowingRules::new();
        let events = rules.events();
        assert!(events.iter().any(|e| e.contains("PR1")));
        assert!(events.iter().any(|e| e.contains("1000米")));
        assert!(events.len() >= 4);
    }

    #[test]
    fn test_para_rowing_equipment() {
        let rules = ParaRowingRules::new();
        let equipment = rules.equipment();
        assert!(equipment.iter().any(|e| e.contains("赛艇")));
        assert!(equipment.iter().any(|e| e.contains("座椅")));
        assert!(equipment.len() >= 4);
    }

    #[test]
    fn test_para_rowing_category() {
        let rules = ParaRowingRules::new();
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }
}
