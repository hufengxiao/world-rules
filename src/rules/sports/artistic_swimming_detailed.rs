//! 艺术游泳详细规则
//!
//! 艺术游泳（前称花样游泳）是奥运会正式比赛项目，
//! 结合游泳、舞蹈和体操元素。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 艺术游泳详细规则
pub struct ArtisticSwimmingDetailedRules {
    metadata: RuleMetadata,
}

impl ArtisticSwimmingDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("艺术游泳详细规则", "世界泳联艺术游泳竞赛规则")
                .with_origin("World Aquatics")
                .with_tags(vec!["体育".into(), "水上".into(), "艺术游泳".into()]),
        }
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "女子双人技术自选",
            "女子双人自由自选",
            "混合双人技术自选",
            "混合双人自由自选",
            "集体技术自选",
            "集体自由自选",
            "集体技巧自选",
            "单人技术自选",
            "单人自由自选",
        ]
    }

    /// 技术自选要求
    pub fn technical_requirements(&self) -> Vec<&'static str> {
        vec![
            "必须完成规定动作元素",
            "技术自选时长: 2分15秒-2分50秒",
            "动作元素必须在指定时间内完成",
            "至少包含3个推进动作",
            "至少包含1个慢速旋转",
            "必须展示水面和水下动作",
        ]
    }

    /// 自由自选要求
    pub fn free_requirements(&self) -> Vec<&'static str> {
        vec![
            "自由编排，无强制动作",
            "双人自由自选: 2分30秒-3分30秒",
            "集体自由自选: 3分30秒-4分30秒",
            "单人自由自选: 2分15秒-3分30秒",
            "音乐选择自由",
            "允许使用道具(需批准)",
        ]
    }

    /// 评分系统
    pub fn scoring_system(&self) -> Vec<&'static str> {
        vec![
            "执行分: 完成质量(最高10分)",
            "艺术印象分: 编排与艺术表现(最高10分)",
            "难度分: 动作难度系数",
            "总分 = (执行分 + 艺术印象分) × 难度分",
            "每组裁判评分后去掉最高分和最低分",
            "技术裁判和艺术裁判独立评分",
        ]
    }

    /// 执行评分要素
    pub fn execution_elements(&self) -> Vec<&'static str> {
        vec![
            "精确度: 动作执行的准确性",
            "控制力: 水中位置的稳定性",
            "同步性: 与队友和音乐的同步",
            "流畅性: 动作衔接的自然程度",
            "推进技术: 各种推进动作的质量",
            "延展性: 肢体线条的美感",
        ]
    }

    /// 艺术印象评分要素
    pub fn artistic_elements(&self) -> Vec<&'static str> {
        vec![
            "编排创意: 动作组合的创新性",
            "音乐表达: 动作与音乐的契合",
            "队形变化: 集体项目的队形",
            "过渡衔接: 动作之间的流畅度",
            "艺术表现力: 情感传达",
            "空间运用: 场地利用效果",
        ]
    }

    /// 难度要素
    pub fn difficulty_elements(&self) -> Vec<&'static str> {
        vec![
            "推进动作: 托举、抛跳等",
            "旋转动作: 各类旋转技巧",
            "平衡动作: 倒立、静止等",
            "变换动作: 方向和位置转换",
            "组合难度: 连续动作的复杂度",
            "水下动作: 憋气时间与复杂度",
        ]
    }

    /// 犯规与扣分
    pub fn penalties(&self) -> Vec<&'static str> {
        vec![
            "触碰池底: 每次扣0.5-2分",
            "超时/时间不足: 扣2分",
            "队员掉队: 扣0.5-1分",
            "音乐停止: 扣2分",
            "禁止动作: 扣2分",
            "时间违规: 扣2分",
            "技术动作缺失: 每个扣0.5分",
        ]
    }

    /// 场地要求
    pub fn venue_requirements(&self) -> Vec<&'static str> {
        vec![
            "泳池尺寸: 至少20米×30米",
            "水深: 至少3米",
            "水温: 26°C ± 1°C",
            "水下音响系统",
            "裁判席视野要求",
            "观众席设置要求",
        ]
    }

    /// 参赛要求
    pub fn participation_requirements(&self) -> Vec<&'static str> {
        vec![
            "集体项目: 最少4人，最多8人",
            "年龄要求: 15岁以上参加奥运会",
            "允许混合性别组合",
            "比赛服装规定",
            "禁止使用悬浮装置",
            "音乐时长限制",
        ]
    }
}

impl Default for ArtisticSwimmingDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ArtisticSwimmingDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("artistic_swimming_detailed")
    }

    fn explain(&self) -> String {
        format!(
            "【艺术游泳详细规则】\n\n\
            比赛项目:\n{}\n\n\
            技术自选要求:\n{}\n\n\
            自由自选要求:\n{}\n\n\
            评分系统:\n{}\n",
            self.events()
                .iter()
                .map(|e| format!("  • {}", e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.technical_requirements()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.free_requirements()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.scoring_system()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artistic_swimming_detailed_rules_basic() {
        let rules = ArtisticSwimmingDetailedRules::new();
        assert_eq!(rules.metadata().name, "艺术游泳详细规则");
        assert!(!rules.explain().is_empty());
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }

    #[test]
    fn artistic_swimming_events() {
        let rules = ArtisticSwimmingDetailedRules::new();
        let events = rules.events();
        assert!(events.iter().any(|e| e.contains("双人")));
        assert!(events.iter().any(|e| e.contains("集体")));
        assert!(events.len() >= 9);
    }

    #[test]
    fn artistic_swimming_scoring() {
        let rules = ArtisticSwimmingDetailedRules::new();
        let scoring = rules.scoring_system();
        assert!(scoring.iter().any(|s| s.contains("执行分")));
        assert!(scoring.iter().any(|s| s.contains("艺术印象")));
        assert!(scoring.iter().any(|s| s.contains("难度")));
        assert!(scoring.len() >= 6);
    }
}
