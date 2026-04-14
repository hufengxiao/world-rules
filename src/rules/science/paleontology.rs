//! 古生物学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 古生物学定律集合
pub struct PaleontologyLaws {
    metadata: RuleMetadata,
}

impl PaleontologyLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "古生物学定律",
                "古生物学基本定律"
            )
            .with_origin("生物学")
            .with_tags(vec!["科学".into(), "生物".into(), "古生物".into()]),
        }
    }

    /// 化石定律
    pub fn fossil_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("化石定律", "化石形成", "化石形成条件"),
            ("保存定律", "化石保存", "化石保存方式"),
            ("石化定律", "石化过程", "石化过程机制"),
            ("埋藏定律", "埋藏规律", "化石埋藏规律"),
            ("分布定律", "地理分布", "化石地理分布"),
            ("密度定律", "化石密度", "化石丰度变化"),
            ("层位定律", "地层位置", "化石地层位置"),
        ]
    }

    /// 进化定律
    pub fn evolution_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("进化定律", "进化记录", "化石进化记录"),
            ("演化定律", "演化序列", "演化序列重建"),
            ("起源定律", "起源追溯", "物种起源追溯"),
            ("灭绝定律", "灭绝事件", "大灭绝事件"),
            ("辐射定律", "适应辐射", "辐射演化事件"),
            ("过渡定律", "过渡类型", "过渡化石类型"),
            ("速率定律", "进化速率", "进化速率变化"),
        ]
    }

    /// 生物地层定律
    pub fn biostratigraphy_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("生物地层定律", "化石地层", "生物地层划分"),
            ("标志定律", "标志化石", "标志化石应用"),
            ("带定律", "化石带", "化石带划分"),
            ("对比定律", "地层对比", "生物地层对比"),
            ("年代定律", "年代确定", "化石年代确定"),
            ("顺序定律", "出现顺序", "化石出现顺序"),
        ]
    }

    /// 古生态定律
    pub fn paleoecology_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("古生态定律", "生态重建", "古生态环境重建"),
            ("群落定律", "古群落", "古生物群落"),
            ("环境定律", "古环境", "古环境推断"),
            ("气候定律", "古气候", "古气候变化"),
            ("海陆定律", "海陆分布", "古海陆分布"),
            ("食物定律", "食物链", "古食物链重建"),
        ]
    }

    /// 重要化石类型
    pub fn fossil_types(&self) -> Vec<&'static str> {
        vec![
            "三叶虫",
            "恐龙",
            "始祖鸟",
            "猛犸象",
            "菊石",
            "古鱼类",
            "古植物",
            "古人类",
        ]
    }

    /// 地质年代
    pub fn geological_periods(&self) -> Vec<&'static str> {
        vec![
            "寒武纪",
            "奥陶纪",
            "志留纪",
            "泥盆纪",
            "石炭纪",
            "二叠纪",
            "三叠纪",
            "侏罗纪",
        ]
    }
}

impl Default for PaleontologyLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for PaleontologyLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("paleontology")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【古生物学定律】\n\n化石定律:\n{}\n\n进化定律:\n{}\n\n生物地层定律:\n{}\n",
            self.fossil_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.evolution_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.biostratigraphy_laws().iter()
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
    fn test_paleontology_laws() {
        let laws = PaleontologyLaws::new();
        assert!(!laws.fossil_laws().is_empty());
        assert!(!laws.evolution_laws().is_empty());
    }
}