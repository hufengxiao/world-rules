//! 残疾人射箭规则
//!
//! 残疾人射箭比赛包括反曲弓和复合弓项目。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 残疾人射箭规则
pub struct ParaArcheryRules {
    metadata: RuleMetadata,
}

impl ParaArcheryRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("残疾人射箭规则", "残疾人射箭比赛规则")
                .with_origin("World Archery/IPC")
                .with_tags(vec!["体育".into(), "射箭".into(), "残奥".into()]),
        }
    }

    /// 运动分级
    pub fn classification(&self) -> Vec<&'static str> {
        vec![
            "W1级: 严重肢体残疾（轮椅）",
            "W2级: 轮椅使用但上肢功能正常",
            "ST级: 站立或坐姿稳定",
            "VI级: 视力残疾（使用触觉瞄准）",
            "复合弓开放级",
            "反曲弓开放级",
        ]
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "反曲弓: 70米排名赛、淘汰赛",
            "复合弓: 50米排名赛、淘汰赛",
            "W1复合弓: 50米",
            "混合团体赛",
            "团体赛",
            "室内赛: 18米",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "弓: 反曲弓或复合弓",
            "箭: 符合WA规格",
            "瞄准器: 允许（某些级别）",
            "释放器: 允许（复合弓）",
            "支架: 允许（身体支撑）",
            "轮椅: 固定在射击位置",
            "绑带: 允许固定弓具",
        ]
    }

    /// 适应性规则
    pub fn adaptations(&self) -> Vec<&'static str> {
        vec![
            "允许使用身体支撑装置",
            "视力残疾可使用触觉瞄准辅助",
            "轮椅固定在地面",
            "绑带固定弓具（上肢残疾）",
            "口咬释放装置允许",
            "助射支架允许",
        ]
    }

    /// 计分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "靶面: 10环制",
            "箭值: 1-10分",
            "X10: 10环内环（决胜用）",
            "排名赛: 72箭",
            "淘汰赛: 局胜制",
            "最高分: 720分（排名赛）",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "超时射箭",
            "越线犯规",
            "装备违规",
            "接受非法指导",
            "危险行为",
            "违反分级规定",
        ]
    }

    /// 参赛资格
    pub fn eligibility(&self) -> Vec<&'static str> {
        vec![
            "必须通过分级评估",
            "World Archery注册",
            "最低残疾标准",
            "国际分级认证",
            "达标成绩要求",
        ]
    }
}

impl Default for ParaArcheryRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ParaArcheryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("para_archery")
    }

    fn explain(&self) -> String {
        format!(
            "【残疾人射箭规则】\n\n\
            运动分级:\n{}\n\n\
            比赛项目:\n{}\n\n\
            装备要求:\n{}\n\n\
            计分规则:\n{}",
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
            self.scoring()
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
    fn test_para_archery_rules_basic() {
        let rules = ParaArcheryRules::new();
        assert_eq!(rules.metadata().name, "残疾人射箭规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_para_archery_classification() {
        let rules = ParaArcheryRules::new();
        let classification = rules.classification();
        assert!(classification.iter().any(|c| c.contains("W1")));
        assert!(classification.iter().any(|c| c.contains("W2")));
        assert!(classification.iter().any(|c| c.contains("VI")));
        assert!(classification.len() >= 4);
    }

    #[test]
    fn test_para_archery_events() {
        let rules = ParaArcheryRules::new();
        let events = rules.events();
        assert!(events.iter().any(|e| e.contains("反曲弓")));
        assert!(events.iter().any(|e| e.contains("复合弓")));
        assert!(events.len() >= 4);
    }

    #[test]
    fn test_para_archery_scoring() {
        let rules = ParaArcheryRules::new();
        let scoring = rules.scoring();
        assert!(scoring.iter().any(|s| s.contains("10环")));
        assert!(scoring.len() >= 4);
    }

    #[test]
    fn test_para_archery_category() {
        let rules = ParaArcheryRules::new();
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }
}
