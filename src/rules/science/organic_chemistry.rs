//! 有机化学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 有机化学定律集合
pub struct OrganicChemistryLaws {
    metadata: RuleMetadata,
}

impl OrganicChemistryLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "有机化学定律",
                "有机化学基本定律"
            )
            .with_origin("化学")
            .with_tags(vec!["科学".into(), "化学".into(), "有机".into()]),
        }
    }

    /// 有机反应定律
    pub fn reaction_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("取代反应定律", "原子替换", "一个原子替换另一个"),
            ("加成反应定律", "双键打开", "双键加成新原子"),
            ("消除反应定律", "去除原子", "去除相邻原子形成双键"),
            ("氧化还原定律", "电子转移", "有机化合物氧化还原"),
            ("酯化定律", "酸醇反应", "羧酸与醇生成酯"),
            ("水解定律", "水分子参与", "化合物与水反应"),
            ("聚合定律", "单体聚合", "单体形成聚合物"),
            ("裂解定律", "分子断裂", "大分子断裂成小分子"),
            ("重排定律", "结构变化", "分子内原子重排"),
        ]
    }

    /// 有机反应机理定律
    pub fn mechanism_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("SN1机理定律", "两步取代", "先形成碳正离子"),
            ("SN2机理定律", "一步取代", "亲核试剂直接攻击"),
            ("E1机理定律", "两步消除", "先形成碳正离子"),
            ("E2机理定律", "一步消除", "碱直接攻击"),
            ("亲电加成定律", "亲电试剂", "亲电试剂攻击双键"),
            ("亲核加成定律", "亲核试剂", "亲核试剂攻击"),
            ("自由基机理定律", "自由基链", "自由基链反应"),
            ("协同机理定律", "一步完成", "反应一步完成"),
        ]
    }

    /// 立体化学定律
    pub fn stereochemistry_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("手性定律", "镜像不对称", "手性分子镜像不重合"),
            ("构型定律", "R/S命名", "手性中心构型命名"),
            ("旋光定律", "旋光性", "手性化合物旋光"),
            ("外消旋定律", "等量对映体", "外消旋混合物"),
            ("内消旋定律", "内部对称", "内消旋化合物"),
            ("立体异构定律", "空间排列", "立体异构体"),
            ("顺反异构定律", "双键异构", "顺反异构体"),
            ("对映异构定律", "镜像异构", "对映异构体"),
        ]
    }

    /// 芳香化学定律
    pub fn aromatic_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("芳香性定律", "休克尔规则", "4n+2π电子芳香性"),
            ("亲电芳香取代定律", "芳香取代", "芳香环亲电取代"),
            ("苯环定律", "六碳环", "苯环结构稳定性"),
            ("定位定律", "取代位置", "已有基团影响位置"),
            ("共振定律", "共振结构", "芳香共振稳定"),
            ("芳香亲核取代定律", "特殊条件", "芳香亲核取代"),
        ]
    }

    /// 官能团分类
    pub fn functional_groups(&self) -> Vec<&'static str> {
        vec![
            "烷烃",
            "烯烃",
            "炔烃",
            "醇",
            "醛",
            "酮",
            "羧酸",
            "酯",
            "胺",
            "酰胺",
            "卤代烃",
            "酚",
            "醚",
            "硫醇",
            "硝基化合物",
        ]
    }

    /// 有机化合物类型
    pub fn compound_types(&self) -> Vec<&'static str> {
        vec![
            "脂肪族化合物",
            "芳香族化合物",
            "杂环化合物",
            "碳水化合物",
            "蛋白质",
            "脂质",
            "核酸",
            "氨基酸",
            "维生素",
            "激素",
        ]
    }
}

impl Default for OrganicChemistryLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for OrganicChemistryLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("organic_chemistry")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【有机化学定律】\n\n反应定律:\n{}\n\n机理定律:\n{}\n\n立体化学定律:\n{}\n",
            self.reaction_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.mechanism_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.stereochemistry_laws().iter()
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
    fn test_organic_chemistry_laws() {
        let laws = OrganicChemistryLaws::new();
        assert!(!laws.reaction_laws().is_empty());
        assert!(!laws.functional_groups().is_empty());
    }
}