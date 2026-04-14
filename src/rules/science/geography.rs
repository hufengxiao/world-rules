//! 地理学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 地理学定律集合
pub struct GeographyLaws {
    metadata: RuleMetadata,
}

impl GeographyLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "地理学定律",
                "地理学基本定律"
            )
            .with_origin("自然科学")
            .with_tags(vec!["科学".into(), "地理".into()]),
        }
    }

    /// 自然地理定律
    pub fn physical_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("地带性定律", "地域分异", "地理环境地带分布"),
            ("垂直地带定律", "高度变化", "垂直地理带"),
            ("经度地带定律", "经度变化", "经度地带分布"),
            ("纬度地带定律", "纬度变化", "纬度地带分布"),
            ("地理循环定律", "循环过程", "地理循环演化"),
            ("地形定律", "地貌形成", "地貌形成规律"),
            ("水文定律", "水循环", "水文循环规律"),
            ("气候定律", "气候分布", "气候分布规律"),
        ]
    }

    /// 人文地理定律
    pub fn human_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("人口定律", "人口分布", "人口分布规律"),
            ("城市定律", "城市分布", "城市发展规律"),
            ("经济定律", "经济布局", "经济地理布局"),
            ("文化定律", "文化地域", "文化地域分布"),
            ("交通定律", "交通网络", "交通网络布局"),
            ("聚落定律", "聚落分布", "聚落分布规律"),
            ("农业定律", "农业区位", "农业区位规律"),
            ("工业定律", "工业区位", "工业区位规律"),
        ]
    }

    /// 地理空间定律
    pub fn spatial_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("空间分布定律", "分布规律", "地理空间分布"),
            ("空间结构定律", "结构组织", "空间结构组织"),
            ("空间互动定律", "相互作用", "空间相互作用"),
            ("空间扩散定律", "扩散规律", "地理扩散规律"),
            ("空间等级定律", "等级体系", "空间等级体系"),
            ("距离衰减定律", "距离影响", "距离衰减效应"),
            ("区位定律", "区位选择", "区位选择规律"),
        ]
    }

    /// 地理环境定律
    pub fn environment_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("环境容量定律", "承载力", "环境承载力"),
            ("环境演化定律", "环境变化", "环境演化规律"),
            ("环境脆弱定律", "脆弱区域", "环境脆弱性"),
            ("生态地理定律", "生态分布", "生态地理分布"),
            ("资源定律", "资源分布", "资源地理分布"),
            ("灾害定律", "灾害分布", "灾害地理分布"),
        ]
    }

    /// 地理分支
    pub fn branches(&self) -> Vec<&'static str> {
        vec![
            "自然地理",
            "人文地理",
            "经济地理",
            "城市地理",
            "文化地理",
            "政治地理",
            "历史地理",
            "区域地理",
        ]
    }

    /// 地理要素
    pub fn elements(&self) -> Vec<&'static str> {
        vec![
            "地形",
            "气候",
            "水文",
            "土壤",
            "植被",
            "人口",
            "城市",
            "交通",
        ]
    }
}

impl Default for GeographyLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for GeographyLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("geography")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【地理学定律】\n\n自然地理定律:\n{}\n\n人文地理定律:\n{}\n\n空间定律:\n{}\n",
            self.physical_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.human_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.spatial_laws().iter()
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
    fn test_geography_laws() {
        let laws = GeographyLaws::new();
        assert!(!laws.physical_laws().is_empty());
        assert!(!laws.human_laws().is_empty());
    }
}