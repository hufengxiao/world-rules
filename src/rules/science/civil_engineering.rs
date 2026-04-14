//! 土木工程定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 土木工程定律集合
pub struct CivilEngineeringLaws {
    metadata: RuleMetadata,
}

impl CivilEngineeringLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "土木工程定律",
                "土木工程基本定律"
            )
            .with_origin("工程")
            .with_tags(vec!["科学".into(), "工程".into(), "土木".into()]),
        }
    }

    /// 结构工程定律
    pub fn structure_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("结构设计定律", "安全设计", "结构安全设计"),
            ("承载力定律", "承载能力", "结构承载力计算"),
            ("稳定定律", "稳定性分析", "结构稳定性"),
            ("刚度定律", "刚度要求", "结构刚度设计"),
            ("抗震定律", "抗震设计", "抗震结构设计"),
            ("疲劳定律", "疲劳分析", "结构疲劳寿命"),
            ("连接定律", "连接设计", "构件连接设计"),
            ("预应力定律", "预应力原理", "预应力结构"),
        ]
    }

    /// 岩土工程定律
    pub fn geotechnical_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("土力学定律", "土体性质", "土力学原理"),
            ("地基定律", "地基设计", "地基承载力"),
            ("边坡定律", "边坡稳定", "边坡稳定性分析"),
            ("挡土墙定律", "挡土设计", "挡土墙设计"),
            ("桩基定律", "桩基础", "桩基础设计"),
            ("渗流定律", "渗流分析", "土体渗流规律"),
            ("沉降定律", "沉降计算", "地基沉降预测"),
            ("液化定律", "液化判别", "土体液化分析"),
        ]
    }

    /// 混凝土工程定律
    pub fn concrete_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("混凝土强度定律", "强度等级", "混凝土强度设计"),
            ("配合比定律", "配合设计", "混凝土配合比"),
            ("养护定律", "养护条件", "混凝土养护"),
            ("耐久性定律", "耐久要求", "混凝土耐久性"),
            ("收缩定律", "收缩控制", "混凝土收缩"),
            ("开裂定律", "裂缝控制", "裂缝控制措施"),
            ("钢筋定律", "配筋设计", "钢筋配置原则"),
        ]
    }

    /// 施工工程定律
    pub fn construction_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("施工定律", "施工组织", "施工组织设计"),
            ("进度定律", "进度控制", "施工进度管理"),
            ("质量定律", "质量控制", "施工质量管理"),
            ("安全定律", "安全施工", "施工安全管理"),
            ("造价定律", "造价控制", "工程造价管理"),
            ("环境定律", "环境影响", "施工环境保护"),
            ("验收定律", "验收标准", "工程验收规范"),
        ]
    }

    /// 结构类型
    pub fn structure_types(&self) -> Vec<&'static str> {
        vec![
            "框架结构",
            "剪力墙结构",
            "钢结构",
            "混凝土结构",
            "砌体结构",
            "木结构",
            "组合结构",
            "预应力结构",
        ]
    }

    /// 工程类型
    pub fn project_types(&self) -> Vec<&'static str> {
        vec![
            "建筑工程",
            "道路工程",
            "桥梁工程",
            "隧道工程",
            "水利工程",
            "港口工程",
            "市政工程",
            "地下工程",
        ]
    }
}

impl Default for CivilEngineeringLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CivilEngineeringLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("civil_engineering")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【土木工程定律】\n\n结构工程定律:\n{}\n\n岩土工程定律:\n{}\n\n混凝土定律:\n{}\n",
            self.structure_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.geotechnical_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.concrete_laws().iter()
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
    fn test_civil_engineering_laws() {
        let laws = CivilEngineeringLaws::new();
        assert!(!laws.structure_laws().is_empty());
        assert!(!laws.geotechnical_laws().is_empty());
    }
}