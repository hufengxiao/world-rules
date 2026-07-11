//! 轮椅击剑规则
//!
//! 轮椅击剑是固定轮椅进行的击剑比赛。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 轮椅击剑规则
pub struct WheelchairFencingRules {
    metadata: RuleMetadata,
}

impl WheelchairFencingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("轮椅击剑规则", "轮椅击剑比赛规则")
                .with_origin("IWAS/IPC")
                .with_tags(vec![
                    "体育".into(),
                    "轮椅".into(),
                    "击剑".into(),
                    "残奥".into(),
                ]),
        }
    }

    /// 运动分级
    pub fn classification(&self) -> Vec<&'static str> {
        vec![
            "A级: 功能较好（坐姿平衡）",
            "B级: 功能中等",
            "C级: 功能较差",
            "剑种分类: 花剑/重剑/佩剑",
            "分级评估: 功能测试",
            "性别混合: 部分级别",
        ]
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "花剑: 男子/女子各级别",
            "重剑: 男子/女子各级别",
            "佩剑: 男子（A级和B级）",
            "团体赛: 剑种团体",
            "残奥会: 男女各10个小项",
            "世界锦标赛: 全级别",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "剑: FIE认证（花剑/重剑/佩剑）",
            "轮椅: 固定框架",
            "击剑服: FIE认证",
            "面罩: FIE认证",
            "手套: 标准击剑手套",
            "固定装置: 轮椅固定",
            "禁止: 非认证器材",
        ]
    }

    /// 比赛规则
    pub fn gameplay(&self) -> Vec<&'static str> {
        vec![
            "轮椅固定: 固定框架",
            "比赛距离: 根据剑种",
            "回合: 限制时间",
            "得分: 电子计分",
            "接触: 剑尖有效",
            "比赛制: 直接淘汰",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "轮椅移动",
            "非法触碰",
            "危险动作",
            "延误比赛",
            "接受非法指导",
            "装备违规",
        ]
    }

    /// 计分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "花剑: 有效区域躯干",
            "重剑: 全身有效",
            "佩剑: 上半身有效",
            "先击中: 得分",
            "同时击: 花剑无效",
            "决赛: 15分制",
        ]
    }

    /// 参赛资格
    pub fn eligibility(&self) -> Vec<&'static str> {
        vec![
            "IWAS分级认证",
            "FIE执照",
            "最低残疾标准",
            "国际分级证书",
            "达标成绩",
        ]
    }
}

impl Default for WheelchairFencingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for WheelchairFencingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("wheelchair_fencing")
    }

    fn explain(&self) -> String {
        format!(
            "【轮椅击剑规则】\n\n\
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
    fn test_wheelchair_fencing_rules_basic() {
        let rules = WheelchairFencingRules::new();
        assert_eq!(rules.metadata().name, "轮椅击剑规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_wheelchair_fencing_classification() {
        let rules = WheelchairFencingRules::new();
        let classification = rules.classification();
        assert!(classification.iter().any(|c| c.contains("A级")));
        assert!(classification.iter().any(|c| c.contains("B级")));
        assert!(classification.iter().any(|c| c.contains("C级")));
        assert!(classification.len() >= 4);
    }

    #[test]
    fn test_wheelchair_fencing_events() {
        let rules = WheelchairFencingRules::new();
        let events = rules.events();
        assert!(events.iter().any(|e| e.contains("花剑")));
        assert!(events.iter().any(|e| e.contains("重剑")));
        assert!(events.iter().any(|e| e.contains("佩剑")));
        assert!(events.len() >= 4);
    }

    #[test]
    fn test_wheelchair_fencing_scoring() {
        let rules = WheelchairFencingRules::new();
        let scoring = rules.scoring();
        assert!(scoring.iter().any(|s| s.contains("花剑")));
        assert!(scoring.iter().any(|s| s.contains("重剑")));
        assert!(scoring.len() >= 4);
    }

    #[test]
    fn test_wheelchair_fencing_category() {
        let rules = WheelchairFencingRules::new();
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }
}
