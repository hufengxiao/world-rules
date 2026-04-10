//! 定向越野规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 定向越野规则
pub struct OrienteeringRules {
    metadata: RuleMetadata,
}

impl OrienteeringRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "定向越野规则",
                "定向越野比赛基本规则"
            )
            .with_origin("瑞典")
            .with_tags(vec!["体育".into(), "越野".into()]),
        }
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "徒步定向: 森林越野",
            "山地自行车定向",
            "滑雪定向",
            "城市定向",
            "长距离定向",
        ]
    }

    /// 比赛距离
    pub fn distances(&self) -> Vec<&'static str> {
        vec![
            "短距离: 2-4公里",
            "中距离: 5-8公里",
            "长距离: 10-15公里",
            "超长距离: 15公里以上",
            "精英距离",
        ]
    }

    /// 检查点规则
    pub fn checkpoint_rules(&self) -> Vec<&'static str> {
        vec![
            "地图标注检查点",
            "检查点旗帜: 白橙三角形",
            "电子打卡系统",
            "打卡顺序必须正确",
            "漏打卡无效",
        ]
    }

    /// 地图使用
    pub fn map_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛专用地图",
            "比例尺: 1:5000-1:15000",
            "起点和终点标注",
            "检查点位置标注",
            "地形特征标注",
        ]
    }

    /// 参赛者要求
    pub fn participant_requirements(&self) -> Vec<&'static str> {
        vec![
            "独立完成比赛",
            "不得接受他人帮助",
            "不得跟随他人",
            "遵守安全规定",
            "装备齐全",
        ]
    }

    /// 禁止行为
    pub fn prohibited_actions(&self) -> Vec<&'static str> {
        vec![
            "接受他人导航帮助",
            "破坏检查点",
            "抄近路",
            "跟随其他选手",
            "危险行为",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "地图: 比赛地图",
            "打卡卡: 电子打卡",
            "指南针: 方向判断",
            "适合的服装",
            "水和补给",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "了解地形",
            "携带通讯设备",
            "医疗站设置",
            "比赛监督",
            "紧急撤离",
        ]
    }
}

impl Default for OrienteeringRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for OrienteeringRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("orienteering")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【定向越野规则】\n\n\
            检查点规则:\n{}\n\n\
            地图使用:\n{}\n\n\
            禁止行为:\n{}\n\n\
            安全规则:\n{}\n",
            self.checkpoint_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.map_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.prohibited_actions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orienteering_rules() {
        let rules = OrienteeringRules::new();
        assert!(!rules.checkpoint_rules().is_empty());
    }
}