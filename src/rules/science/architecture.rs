//! 建筑学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 建筑学定律集合
pub struct ArchitectureLaws {
    metadata: RuleMetadata,
}

impl ArchitectureLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "建筑学定律",
                "建筑学基本定律"
            )
            .with_origin("工程科学")
            .with_tags(vec!["科学".into(), "建筑".into()]),
        }
    }

    /// 建筑设计定律
    pub fn design_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("功能定律", "功能优先", "建筑功能优先"),
            ("形式定律", "形式表达", "建筑形式表达"),
            ("空间定律", "空间组织", "空间组织规律"),
            ("比例定律", "比例关系", "建筑比例关系"),
            ("尺度定律", "尺度控制", "尺度与人体关系"),
            ("均衡定律", "视觉平衡", "建筑视觉平衡"),
            ("韵律定律", "节奏排列", "建筑韵律排列"),
            ("对比定律", "对比效果", "建筑对比效果"),
        ]
    }

    /// 建筑结构定律
    pub fn structure_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("荷载定律", "荷载传递", "荷载传递路径"),
            ("支撑定律", "支撑系统", "支撑结构系统"),
            ("稳定定律", "稳定条件", "结构稳定条件"),
            ("刚度定律", "刚度要求", "结构刚度要求"),
            ("强度定律", "强度条件", "材料强度条件"),
            ("抗震定律", "抗震设计", "抗震设计要求"),
            ("风载定律", "风荷载", "风荷载计算"),
        ]
    }

    /// 建筑物理定律
    pub fn physics_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("采光定律", "光线利用", "自然采光规律"),
            ("通风定律", "气流组织", "通风气流组织"),
            ("保温定律", "热量保持", "保温隔热规律"),
            ("隔音定律", "声波隔离", "隔音声波控制"),
            ("防火定律", "防火设计", "防火设计要求"),
            ("防水定律", "防水设计", "防水设计要求"),
            ("热工定律", "热工性能", "建筑热工性能"),
        ]
    }

    /// 建筑环境定律
    pub fn environment_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("环境适应定律", "环境响应", "建筑环境响应"),
            ("气候定律", "气候设计", "气候适应性设计"),
            ("地形定律", "地形利用", "地形利用规律"),
            ("景观定律", "景观融合", "建筑景观融合"),
            ("生态定律", "生态建筑", "生态建筑设计"),
            ("可持续定律", "可持续发展", "可持续建筑"),
        ]
    }

    /// 建筑类型
    pub fn building_types(&self) -> Vec<&'static str> {
        vec![
            "住宅建筑",
            "公共建筑",
            "商业建筑",
            "工业建筑",
            "教育建筑",
            "医疗建筑",
            "文化建筑",
            "体育建筑",
        ]
    }

    /// 建筑风格
    pub fn styles(&self) -> Vec<&'static str> {
        vec![
            "古典风格",
            "现代风格",
            "后现代风格",
            "民族风格",
            "地域风格",
            "生态风格",
            "高科技风格",
            "解构风格",
        ]
    }
}

impl Default for ArchitectureLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ArchitectureLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("architecture")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【建筑学定律】\n\n设计定律:\n{}\n\n结构定律:\n{}\n\n物理定律:\n{}\n",
            self.design_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.structure_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.physics_laws().iter()
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
    fn test_architecture_laws() {
        let laws = ArchitectureLaws::new();
        assert!(!laws.design_laws().is_empty());
        assert!(!laws.structure_laws().is_empty());
    }
}