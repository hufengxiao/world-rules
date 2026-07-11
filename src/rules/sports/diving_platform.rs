//! 跳台跳水规则
//!
//! 跳台跳水是奥运会正式比赛项目，
//! 从固定高度跳台上进行各种跳水动作。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 跳台跳水规则
pub struct DivingPlatformRules {
    metadata: RuleMetadata,
}

impl DivingPlatformRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("跳台跳水规则", "世界泳联跳台跳水竞赛规则")
                .with_origin("World Aquatics")
                .with_tags(vec!["体育".into(), "水上".into(), "跳水".into()]),
        }
    }

    /// 跳台高度
    pub fn platform_heights(&self) -> Vec<&'static str> {
        vec![
            "标准高度: 10米",
            "训练高度: 5米",
            "训练高度: 7.5米",
            "奥运会比赛: 10米",
            "青少年比赛: 5米或7.5米",
            "跳台宽度: 至少0.6米",
        ]
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "女子10米跳台单人",
            "男子10米跳台单人",
            "女子10米跳台双人",
            "男子10米跳台双人",
            "混合10米跳台双人",
        ]
    }

    /// 跳水动作组别
    pub fn dive_groups(&self) -> Vec<&'static str> {
        vec![
            "第1组: 向前跳水",
            "第2组: 向后跳水",
            "第3组: 向前反身跳水",
            "第4组: 向后反身跳水",
            "第5组: 转体跳水",
            "第6组: 臂立跳水(仅跳台)",
        ]
    }

    /// 评分标准
    pub fn scoring_criteria(&self) -> Vec<&'static str> {
        vec![
            "起跳: 高度、角度和距离",
            "腾空: 动作姿态和旋转速度",
            "入水: 垂直度和水花大小",
            "最高分: 10分(完美)",
            "7位裁判评分，去掉最高和最低分",
            "双人同步: 动作一致性评分",
        ]
    }

    /// 评分等级
    pub fn scoring_levels(&self) -> Vec<&'static str> {
        vec![
            "10分: 完美",
            "8.5-9.5分: 优秀",
            "7.0-8.0分: 良好",
            "5.0-6.5分: 一般",
            "3.0-4.5分: 较差",
            "0.5-2.5分: 失败",
        ]
    }

    /// 难度系数范围
    pub fn difficulty_range(&self) -> Vec<&'static str> {
        vec![
            "最低难度: 1.2",
            "一般难度: 2.0-2.5",
            "中等难度: 2.6-3.0",
            "高难度: 3.1-3.5",
            "超难动作: 3.6-4.1",
            "难度由动作组成决定",
        ]
    }

    /// 比赛轮次
    pub fn competition_rounds(&self) -> Vec<&'static str> {
        vec![
            "预赛: 所有选手参加",
            "半决赛: 前18名晋级",
            "决赛: 前12名争夺奖牌",
            "单人: 每轮5跳(女)或6跳(男)",
            "双人: 每轮5跳",
            "总分累计制",
        ]
    }

    /// 犯规与扣分
    pub fn penalties(&self) -> Vec<&'static str> {
        vec![
            "重跳: 动作干扰或设备故障",
            "零分: 动作与申报不符",
            "零分: 跳台违规起跳",
            "扣分: 超时(90秒)",
            "扣分: 入水角度过大",
            "双人扣分: 不同步",
        ]
    }

    /// 安全要求
    pub fn safety_requirements(&self) -> Vec<&'static str> {
        vec![
            "水深: 至少5米",
            "水下气泡系统: 缓冲入水冲击",
            "裁判可视范围要求",
            "医疗人员现场待命",
            "热身时间保障",
            "跳台防滑处理",
        ]
    }
}

impl Default for DivingPlatformRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for DivingPlatformRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("diving_platform")
    }

    fn explain(&self) -> String {
        format!(
            "【跳台跳水规则】\n\n\
            跳台高度:\n{}\n\n\
            比赛项目:\n{}\n\n\
            评分标准:\n{}\n",
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
            self.scoring_criteria()
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
    fn diving_platform_rules_basic() {
        let rules = DivingPlatformRules::new();
        assert_eq!(rules.metadata().name, "跳台跳水规则");
        assert!(!rules.explain().is_empty());
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }

    #[test]
    fn diving_platform_heights() {
        let rules = DivingPlatformRules::new();
        let heights = rules.platform_heights();
        assert!(heights.iter().any(|h| h.contains("10米")));
        assert!(heights.len() >= 6);
    }

    #[test]
    fn diving_platform_scoring() {
        let rules = DivingPlatformRules::new();
        let scoring = rules.scoring_criteria();
        assert!(scoring.iter().any(|s| s.contains("起跳")));
        assert!(scoring.iter().any(|s| s.contains("入水")));
        assert!(scoring.len() >= 6);
    }
}