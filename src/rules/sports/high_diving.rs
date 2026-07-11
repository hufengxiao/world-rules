//! 高台跳水规则
//!
//! 高台跳水是极限水上运动，
//! 从20-27米高度跳下，是极限运动的一种。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 高台跳水规则
pub struct HighDivingRules {
    metadata: RuleMetadata,
}

impl HighDivingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("高台跳水规则", "世界泳联高台跳水竞赛规则")
                .with_origin("World Aquatics")
                .with_tags(vec!["体育".into(), "水上".into(), "极限运动".into()]),
        }
    }

    /// 跳台高度
    pub fn platform_heights(&self) -> Vec<&'static str> {
        vec![
            "女子标准高度: 20米",
            "男子标准高度: 27米",
            "训练高度: 10-15米",
            "平台宽度: 至少1.5米",
            "平台延伸: 至少2米",
            "入水速度可达85公里/小时",
        ]
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "女子20米跳台单人",
            "男子27米跳台单人",
            "世界锦标赛项目",
            "世界杯项目",
            "悬崖跳水世界系列赛",
        ]
    }

    /// 跳水动作要求
    pub fn dive_requirements(&self) -> Vec<&'static str> {
        vec![
            "必须完成翻腾动作",
            "必须包含转体动作",
            "入水角度必须垂直",
            "必须脚先入水",
            "禁止头朝下入水(危险)",
            "动作时长: 2-3秒",
        ]
    }

    /// 评分标准
    pub fn scoring_criteria(&self) -> Vec<&'static str> {
        vec![
            "起跳质量: 高度和角度",
            "空中姿态: 动作优美度",
            "翻腾转体: 动作完成度",
            "入水质量: 水花控制",
            "整体印象: 创意和艺术性",
            "5位裁判评分",
        ]
    }

    /// 难度系数
    pub fn difficulty_factors(&self) -> Vec<&'static str> {
        vec![
            "翻腾周数: 每周0.4-0.6",
            "转体周数: 每周0.2-0.4",
            "起跳方式: 增加0.1-0.3",
            "动作组合难度",
            "DD范围: 2.0-6.0",
            "超难动作可达DD 6.0",
        ]
    }

    /// 安全要求
    pub fn safety_requirements(&self) -> Vec<&'static str> {
        vec![
            "水深: 至少5米",
            "水下救援队待命",
            "医疗直升机待命",
            "选手需具备专业资格",
            "赛前体检要求",
            "禁止头朝下入水",
        ]
    }

    /// 比赛轮次
    pub fn competition_rounds(&self) -> Vec<&'static str> {
        vec![
            "预赛: 每人3跳",
            "半决赛: 前12名晋级",
            "决赛: 前8名争夺奖牌",
            "总分累计制",
            "每轮间隔至少10分钟",
            "恶劣天气可延期",
        ]
    }

    /// 犯规与处罚
    pub fn penalties(&self) -> Vec<&'static str> {
        vec![
            "零分: 头朝下入水",
            "零分: 动作与申报不符",
            "扣分: 入水角度不佳",
            "扣分: 超时",
            "取消资格: 安全违规",
            "重跳: 设备故障",
        ]
    }

    /// 选手资格
    pub fn athlete_requirements(&self) -> Vec<&'static str> {
        vec![
            "年龄要求: 18岁以上",
            "专业跳水训练经验",
            "通过高台跳水资格认证",
            "体检合格证明",
            "签署免责声明",
            "购买专项保险",
        ]
    }
}

impl Default for HighDivingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for HighDivingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("high_diving")
    }

    fn explain(&self) -> String {
        format!(
            "【高台跳水规则】\n\n\
            跳台高度:\n{}\n\n\
            比赛项目:\n{}\n\n\
            安全要求:\n{}\n",
            self.platform_heights()
                .iter()
                .map(|h| format!("  • {}", h))
                .collect::<Vec<_>>()
                .join("\n"),
            self.events()
                .iter()
                .map(|e| format!("  • {}", e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.safety_requirements()
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
    fn high_diving_rules_basic() {
        let rules = HighDivingRules::new();
        assert_eq!(rules.metadata().name, "高台跳水规则");
        assert!(!rules.explain().is_empty());
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }

    #[test]
    fn high_diving_heights() {
        let rules = HighDivingRules::new();
        let heights = rules.platform_heights();
        assert!(heights.iter().any(|h| h.contains("20米")));
        assert!(heights.iter().any(|h| h.contains("27米")));
        assert!(heights.len() >= 6);
    }

    #[test]
    fn high_diving_safety() {
        let rules = HighDivingRules::new();
        let safety = rules.safety_requirements();
        assert!(safety.iter().any(|s| s.contains("救援")));
        assert!(safety.iter().any(|s| s.contains("医疗")));
        assert!(safety.len() >= 6);
    }
}
