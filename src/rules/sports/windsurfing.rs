//! 帆板运动规则
//!
//! 帆板（Windsurfing）是奥运会正式比赛项目，
//! 结合了冲浪和帆船的元素。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 帆板运动规则
pub struct WindsurfingRules {
    metadata: RuleMetadata,
}

impl WindsurfingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("帆板规则", "世界帆联帆板竞赛规则")
                .with_origin("World Sailing")
                .with_tags(vec!["体育".into(), "水上".into(), "帆板".into()]),
        }
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "男子帆板RS:X级",
            "女子帆板RS:X级",
            "男子帆板IQFoil级",
            "女子帆板IQFoil级",
            "帆板马拉松赛",
            "帆板绕标赛",
            "帆板速度赛",
        ]
    }

    /// 板型分类
    pub fn board_types(&self) -> Vec<&'static str> {
        vec![
            "RS:X级: 奥运会标准板",
            "IQFoil级: 翼型帆板(2024奥运)",
            "Formula级别: 专业竞赛板",
            "Slalom级别: 速度竞赛板",
            "Wave级别: 波浪帆板",
            "Freestyle级别: 自由式帆板",
        ]
    }

    /// 竞赛规则
    pub fn racing_rules(&self) -> Vec<&'static str> {
        vec![
            "起航信号: 5分钟预告",
            "航线绕标: 按规定方向",
            "风向变化: 航线可调整",
            "碰撞规则: 避让优先权",
            "终点判定: 线上完整通过",
            "计时精确到秒",
        ]
    }

    /// 评分系统
    pub fn scoring_system(&self) -> Vec<&'static str> {
        vec![
            "积分制: 第1名得1分",
            "积分制: 第2名得2分",
            "总分: 各轮积分之和",
            "低分制排名",
            "可丢弃一轮最差成绩",
            "金牌轮: 双倍积分",
        ]
    }

    /// 装备要求
    pub fn equipment_requirements(&self) -> Vec<&'static str> {
        vec![
            "帆板长度: 2.5-4米",
            "帆面积: 根据级别规定",
            "桅杆: 符合级别标准",
            "脚踏板: 固定牢固",
            "安全绳: 必备装备",
            "救生衣: 强制穿戴",
        ]
    }

    /// 气象条件
    pub fn weather_conditions(&self) -> Vec<&'static str> {
        vec![
            "最低风速: 3节",
            "最大风速: 25节",
            "风向要求: 稳定风向",
            "浪高限制: 2米以下",
            "恶劣天气可延期",
            "安全委员会决定",
        ]
    }

    /// 禁赛与处罚
    pub fn penalties(&self) -> Vec<&'static str> {
        vec![
            "违规绕标: DSQ取消成绩",
            "抢航: DSQ或评分惩罚",
            "碰撞: 评分惩罚或DSQ",
            "违反避让规则: 评分惩罚",
            "使用禁用装备: DSQ",
            "抗议程序规定",
        ]
    }

    /// 安全要求
    pub fn safety_requirements(&self) -> Vec<&'static str> {
        vec![
            "救生衣强制穿戴",
            "安全绳连接帆板",
            "水上救援艇待命",
            "通信设备要求",
            "恶劣天气预案",
            "医疗救护设施",
        ]
    }

    /// 参赛资格
    pub fn participation_requirements(&self) -> Vec<&'static str> {
        vec![
            "通过资格赛选拔",
            "国家级帆板协会认证",
            "年龄限制: 16岁以上",
            "体检合格证明",
            "保险证明",
            "级别认证",
        ]
    }
}

impl Default for WindsurfingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for WindsurfingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("windsurfing")
    }

    fn explain(&self) -> String {
        format!(
            "【帆板运动规则】\n\n\
            比赛项目:\n{}\n\n\
            板型分类:\n{}\n\n\
            竞赛规则:\n{}\n",
            self.events()
                .iter()
                .map(|e| format!("  • {}", e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.board_types()
                .iter()
                .map(|b| format!("  • {}", b))
                .collect::<Vec<_>>()
                .join("\n"),
            self.racing_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn windsurfing_rules_basic() {
        let rules = WindsurfingRules::new();
        assert_eq!(rules.metadata().name, "帆板运动规则");
        assert!(!rules.explain().is_empty());
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }

    #[test]
    fn windsurfing_events() {
        let rules = WindsurfingRules::new();
        let events = rules.events();
        assert!(events.iter().any(|e| e.contains("RS:X")));
        assert!(events.iter().any(|e| e.contains("IQFoil")));
        assert!(events.len() >= 7);
    }

    #[test]
    fn windsurfing_equipment() {
        let rules = WindsurfingRules::new();
        let equip = rules.equipment_requirements();
        assert!(equip.iter().any(|e| e.contains("帆")));
        assert!(equip.iter().any(|e| e.contains("救生衣")));
        assert!(equip.len() >= 6);
    }
}
