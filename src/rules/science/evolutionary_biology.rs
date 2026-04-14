//! 进化生物学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 进化生物学定律集合
pub struct EvolutionaryBiologyLaws {
    metadata: RuleMetadata,
}

impl EvolutionaryBiologyLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "进化生物学定律",
                "进化生物学基本定律"
            )
            .with_origin("生物学")
            .with_tags(vec!["科学".into(), "生物".into(), "进化".into()]),
        }
    }

    /// 自然选择定律
    pub fn natural_selection_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("自然选择定律", "适者生存", "有利性状被保留"),
            ("适应性定律", "环境适应", "生物适应环境"),
            ("变异定律", "个体差异", "种群存在变异"),
            ("遗传定律", "性状遗传", "有利性状遗传"),
            ("过度繁殖定律", "后代过剩", "产生过多后代"),
            ("生存斗争定律", "资源竞争", "个体间竞争"),
            ("适者生存定律", "有利存活", "有利个体存活"),
        ]
    }

    /// 进化机制定律
    pub fn mechanism_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("突变定律", "遗传变异来源", "突变产生新变异"),
            ("基因重组定律", "组合变异", "基因重组产生变异"),
            ("基因流动定律", "种群交流", "基因在不同种群流动"),
            ("遗传漂变定律", "随机变化", "基因频率随机变化"),
            ("选择定律", "定向改变", "选择改变基因频率"),
            ("非随机交配定律", "配偶选择", "非随机配偶选择"),
            ("协同进化定律", "相互影响", "物种相互影响进化"),
        ]
    }

    /// 进化模式定律
    pub fn pattern_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("趋同进化定律", "相似性状", "不同物种相似性状"),
            ("趋异进化定律", "性状分化", "同源物种性状分化"),
            ("平行进化定律", "独立相似", "独立进化相似性状"),
            ("协同进化定律", "相互适应", "物种相互适应"),
            ("适应性辐射定律", "多样分化", "一个物种分化多种"),
            ("进化停滞定律", "形态稳定", "形态长期稳定"),
            ("灭绝定律", "物种消失", "物种灭绝"),
        ]
    }

    /// 物种定律
    pub fn species_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("物种概念定律", "物种定义", "物种定义标准"),
            ("物种形成定律", "新物种形成", "新物种形成过程"),
            ("隔离定律", "生殖隔离", "生殖隔离形成物种"),
            ("地理隔离定律", "空间分离", "地理隔离导致分化"),
            ("生态隔离定律", "生态分离", "生态隔离"),
            ("行为隔离定律", "行为差异", "行为差异隔离"),
            ("杂交定律", "种间杂交", "物种间杂交"),
        ]
    }

    /// 进化证据
    pub fn evidence(&self) -> Vec<&'static str> {
        vec![
            "化石证据",
            "解剖证据",
            "胚胎证据",
            "分子证据",
            "行为证据",
            "地理分布证据",
            "古生物证据",
        ]
    }

    /// 进化时间尺度
    pub fn time_scales(&self) -> Vec<&'static str> {
        vec![
            "瞬时进化",
            "短期进化",
            "长期进化",
            "地质时间尺度",
            "进化爆发",
            "进化停滞",
            "周期进化",
        ]
    }
}

impl Default for EvolutionaryBiologyLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for EvolutionaryBiologyLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("evolutionary_biology")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【进化生物学定律】\n\n自然选择定律:\n{}\n\n进化机制定律:\n{}\n\n物种定律:\n{}\n",
            self.natural_selection_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.mechanism_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.species_laws().iter()
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
    fn test_evolutionary_biology_laws() {
        let laws = EvolutionaryBiologyLaws::new();
        assert!(!laws.natural_selection_laws().is_empty());
        assert!(!laws.mechanism_laws().is_empty());
    }
}