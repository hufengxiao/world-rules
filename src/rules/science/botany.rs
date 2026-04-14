//! 植物学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 植物学定律集合
pub struct BotanyLaws {
    metadata: RuleMetadata,
}

impl BotanyLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "植物学定律",
                "植物学基本定律"
            )
            .with_origin("生物学")
            .with_tags(vec!["科学".into(), "生物".into(), "植物".into()]),
        }
    }

    /// 植物形态定律
    pub fn morphology_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("形态定律", "形态特征", "植物形态多样性"),
            ("根定律", "根系结构", "根系形态特征"),
            ("茎定律", "茎结构", "茎形态类型"),
            ("叶定律", "叶形态", "叶形态多样性"),
            ("花定律", "花结构", "花器官结构"),
            ("果定律", "果实类型", "果实形态分类"),
            ("种子定律", "种子结构", "种子形态特征"),
        ]
    }

    /// 植物生理定律
    pub fn physiology_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("光合定律", "光合作用", "光能转换为化学能"),
            ("呼吸定律", "植物呼吸", "植物呼吸代谢"),
            ("蒸腾定律", "水分蒸发", "水分蒸腾作用"),
            ("运输定律", "物质运输", "维管束运输"),
            ("生长定律", "植物生长", "植物生长规律"),
            ("分化定律", "细胞分化", "植物细胞分化"),
            ("激素定律", "激素调节", "植物激素作用"),
            ("开花定律", "开花调控", "开花诱导机制"),
        ]
    }

    /// 植物分类定律
    pub fn classification_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("分类定律", "分类系统", "植物分类方法"),
            ("进化定律", "进化关系", "植物进化系统"),
            ("多样性定律", "物种多样", "植物多样性"),
            ("分布定律", "地理分布", "植物地理分布"),
            ("特化定律", "特化适应", "植物特化规律"),
            ("杂交定律", "杂交现象", "植物杂交规律"),
        ]
    }

    /// 植物生态定律
    pub fn ecology_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("生态定律", "生态适应", "植物生态适应"),
            ("竞争定律", "植物竞争", "植物资源竞争"),
            ("共生定律", "共生关系", "植物共生现象"),
            ("适应定律", "环境适应", "植物环境适应"),
            ("群落定律", "群落结构", "植物群落组成"),
            ("演替定律", "群落演替", "植物群落演替"),
        ]
    }

    /// 植物类群
    pub fn plant_groups(&self) -> Vec<&'static str> {
        vec![
            "藻类植物",
            "苔藓植物",
            "蕨类植物",
            "裸子植物",
            "被子植物",
            "单子叶植物",
            "双子叶植物",
            "草本植物",
        ]
    }

    /// 植物生长环境
    pub fn environments(&self) -> Vec<&'static str> {
        vec![
            "热带",
            "温带",
            "寒带",
            "沙漠",
            "湿地",
            "高山",
            "海洋",
            "淡水",
        ]
    }
}

impl Default for BotanyLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BotanyLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("botany")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【植物学定律】\n\n形态定律:\n{}\n\n生理定律:\n{}\n\n分类定律:\n{}\n",
            self.morphology_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.physiology_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.classification_laws().iter()
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
    fn test_botany_laws() {
        let laws = BotanyLaws::new();
        assert!(!laws.morphology_laws().is_empty());
        assert!(!laws.physiology_laws().is_empty());
    }
}