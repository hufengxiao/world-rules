//! 环境科学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 环境科学定律集合
pub struct EnvironmentalScienceLaws {
    metadata: RuleMetadata,
}

impl EnvironmentalScienceLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "环境科学定律",
                "环境科学基本定律"
            )
            .with_origin("环境科学")
            .with_tags(vec!["科学".into(), "环境".into()]),
        }
    }

    /// 生态系统定律
    pub fn ecosystem_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("能量流动定律", "单向流动", "能量在生态系统中单向流动"),
            ("物质循环定律", "循环利用", "物质在生态系统中循环"),
            ("生态金字塔定律", "能量递减", "营养级越高能量越少"),
            ("生态位定律", "资源分配", "物种占据特定生态位"),
            ("食物链定律", "营养关系", "捕食与被捕食关系"),
            ("生物多样性定律", "稳定性相关", "多样性决定生态系统稳定"),
            ("种群动态定律", "增长平衡", "种群数量动态变化"),
            ("竞争定律", "资源竞争", "物种间竞争资源"),
        ]
    }

    /// 环境污染定律
    pub fn pollution_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("污染物扩散定律", "稀释扩散", "污染物在环境中扩散"),
            ("生物累积定律", "浓度增加", "污染物在生物体内累积"),
            ("生物放大定律", "营养级放大", "污染物沿食物链放大"),
            ("降解定律", "自然分解", "污染物自然降解"),
            ("临界负荷定律", "环境承受", "环境承受污染极限"),
            ("半衰期定律", "衰变时间", "污染物衰减一半时间"),
        ]
    }

    /// 气候变化定律
    pub fn climate_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("温室效应定律", "热量吸收", "温室气体吸收热量"),
            ("辐射平衡定律", "输入输出", "地球辐射能量平衡"),
            ("碳循环定律", "碳交换", "碳在大气和生态系统交换"),
            ("反馈定律", "正负反馈", "气候变化的反馈机制"),
            ("临界点定律", "不可逆转", "气候变化临界点"),
            ("海平面定律", "温度影响", "温度升高导致海平面上升"),
        ]
    }

    /// 可持续发展定律
    pub fn sustainability_laws(&self) -> Vec<&'static str> {
        vec![
            "代际公平: 未来世代权益",
            "代内公平: 现世代公平",
            "预防原则: 预防环境损害",
            "污染者付费: 污染者承担成本",
            "资源效率: 高效利用资源",
            "循环经济: 减量化再利用",
            "生态足迹: 环境承载力",
            "生命周期评估: 全过程分析",
        ]
    }

    /// 环境指标
    pub fn indicators(&self) -> Vec<&'static str> {
        vec![
            "空气质量指数",
            "水质指标",
            "土壤污染指数",
            "碳排放指标",
            "生物多样性指数",
            "森林覆盖率",
            "可再生能源比例",
            "资源回收率",
        ]
    }
}

impl Default for EnvironmentalScienceLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for EnvironmentalScienceLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("environmental_science")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【环境科学定律】\n\n生态系统定律:\n{}\n\n环境污染定律:\n{}\n\n气候变化定律:\n{}\n",
            self.ecosystem_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.pollution_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.climate_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environmental_science_laws() {
        let laws = EnvironmentalScienceLaws::new();
        assert!(!laws.ecosystem_laws().is_empty());
    }
}