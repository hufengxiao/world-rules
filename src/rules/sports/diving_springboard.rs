//! 跳板跳水规则
//!
//! 跳板跳水是奥运会正式比赛项目，
//! 使用弹性跳板进行各种跳水动作。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 跳板跳水规则
pub struct DivingSpringboardRules {
    metadata: RuleMetadata,
}

impl DivingSpringboardRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("跳板跳水规则", "世界泳联跳板跳水竞赛规则")
                .with_origin("World Aquatics")
                .with_tags(vec!["体育".into(), "水上".into(), "跳水".into()]),
        }
    }

    /// 跳板规格
    pub fn springboard_specs(&self) -> Vec<&'static str> {
        vec![
            "标准高度: 1米和3米",
            "跳板长度: 4.8米",
            "跳板宽度: 0.5米",
            "弹性调整: 可调节支点",
            "防滑表面处理",
            "允许使用助跑",
        ]
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "女子3米跳板单人",
            "男子3米跳板单人",
            "女子3米跳板双人",
            "男子3米跳板双人",
            "女子1米跳板单人",
            "男子1米跳板单人",
            "混合3米跳板双人",
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
            "仅限跳台: 第6组臂立跳水",
        ]
    }

    /// 跳板技术特点
    pub fn springboard_technique(&self) -> Vec<&'static str> {
        vec![
            "利用跳板弹性获得高度",
            "助跑起跳技术",
            "压板动作要平稳",
            "弹跳时机把握",
            "腾空高度比跳台低",
            "更适合翻腾动作",
        ]
    }

    /// 评分标准
    pub fn scoring_criteria(&self) -> Vec<&'static str> {
        vec![
            "助跑: 平稳、流畅",
            "起跳: 利用弹板高度",
            "腾空: 动作姿态优美",
            "入水: 垂直、水花小",
            "双人: 同步性评分",
            "7位裁判评分制",
        ]
    }

    /// 难度系数
    pub fn difficulty_factors(&self) -> Vec<&'static str> {
        vec![
            "翻腾周数: 每周增加0.4-0.6",
            "转体周数: 每周增加0.2-0.4",
            "入水方向: 向前/向后影响",
            "动作组合: 连续动作难度叠加",
            "跳板高度: 3米比1米难度略低",
            "最高难度可达3.9",
        ]
    }

    /// 比赛轮次
    pub fn competition_rounds(&self) -> Vec<&'static str> {
        vec![
            "预赛: 所有选手参加",
            "半决赛: 前18名晋级",
            "决赛: 前12名争夺奖牌",
            "女子: 5轮动作",
            "男子: 6轮动作",
            "总分累计排名",
        ]
    }

    /// 犯规与处罚
    pub fn penalties(&self) -> Vec<&'static str> {
        vec![
            "零分: 动作与申报不符",
            "零分: 跳板违规使用",
            "扣分: 超时(90秒内起跳)",
            "扣分: 入水角度过大",
            "重跳: 设备故障",
            "双人不同步扣分",
        ]
    }

    /// 场地要求
    pub fn venue_requirements(&self) -> Vec<&'static str> {
        vec![
            "水深: 至少5米",
            "跳板安装稳固",
            "水下气泡系统",
            "裁判席位设置",
            "医疗救护设施",
            "热身场地保障",
        ]
    }
}

impl Default for DivingSpringboardRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for DivingSpringboardRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("diving_springboard")
    }

    fn explain(&self) -> String {
        format!(
            "【跳板跳水规则】\n\n\
            跳板规格:\n{}\n\n\
            比赛项目:\n{}\n\n\
            评分标准:\n{}\n",
            self.springboard_specs()
                .iter()
                .map(|s| format!("  • {}", s))
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
    fn diving_springboard_rules_basic() {
        let rules = DivingSpringboardRules::new();
        assert_eq!(rules.metadata().name, "跳板跳水规则");
        assert!(!rules.explain().is_empty());
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }

    #[test]
    fn diving_springboard_specs() {
        let rules = DivingSpringboardRules::new();
        let specs = rules.springboard_specs();
        assert!(specs.iter().any(|s| s.contains("1米")));
        assert!(specs.iter().any(|s| s.contains("3米")));
        assert!(specs.len() >= 6);
    }

    #[test]
    fn diving_springboard_technique() {
        let rules = DivingSpringboardRules::new();
        let tech = rules.springboard_technique();
        assert!(tech.iter().any(|t| t.contains("弹性")));
        assert!(tech.len() >= 6);
    }
}
