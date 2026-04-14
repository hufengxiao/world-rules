//! 人类学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 人类学定律集合
pub struct AnthropologyLaws {
    metadata: RuleMetadata,
}

impl AnthropologyLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "人类学定律",
                "人类学基本定律"
            )
            .with_origin("社会科学")
            .with_tags(vec!["科学".into(), "人类".into()]),
        }
    }

    /// 人类进化定律
    pub fn evolution_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("人猿分化定律", "共同祖先", "人类与猿类分化"),
            ("直立行走定律", "直立进化", "人类直立行走进化"),
            ("脑容量定律", "大脑增大", "人类脑容量增大"),
            ("工具使用定律", "工具进化", "人类工具使用进化"),
            ("语言进化定律", "语言发展", "人类语言进化"),
            ("社会进化定律", "社会发展", "人类社会进化"),
            ("文化进化定律", "文化发展", "人类文化进化"),
        ]
    }

    /// 文化定律
    pub fn culture_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("文化相对定律", "文化相对性", "文化无绝对优劣"),
            ("文化传播定律", "文化传播", "文化跨地域传播"),
            ("文化适应定律", "环境适应", "文化适应环境"),
            ("文化变迁定律", "文化变化", "文化变迁规律"),
            ("文化传承定律", "代际传递", "文化代际传承"),
            ("文化多元定律", "多元共存", "文化多样性"),
            ("文化融合定律", "文化整合", "不同文化融合"),
        ]
    }

    /// 社会组织定律
    pub fn organization_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("亲属制度定律", "亲属关系", "亲属关系组织"),
            ("婚姻制度定律", "婚姻形式", "婚姻制度多样性"),
            ("家庭制度定律", "家庭结构", "家庭组织形式"),
            ("部落定律", "部落组织", "部落社会组织"),
            ("氏族定律", "氏族结构", "氏族组织结构"),
            ("等级制度定律", "等级分化", "社会等级制度"),
        ]
    }

    /// 语言定律
    pub fn language_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("语言分化定律", "语言分化", "语言分化规律"),
            ("语言融合定律", "语言混合", "语言融合现象"),
            ("语言演变定律", "语言变化", "语言演变规律"),
            ("语系定律", "语系分类", "语言分类"),
            ("语言接触定律", "语言接触", "语言接触影响"),
            ("语言替换定律", "语言替代", "语言替换现象"),
        ]
    }

    /// 人类学分支
    pub fn branches(&self) -> Vec<&'static str> {
        vec![
            "体质人类学",
            "文化人类学",
            "社会人类学",
            "考古人类学",
            "语言人类学",
            "心理人类学",
            "应用人类学",
            "医学人类学",
        ]
    }

    /// 研究方法
    pub fn methods(&self) -> Vec<&'static str> {
        vec![
            "田野调查",
            "参与观察",
            "深度访谈",
            "文献研究",
            "比较研究",
            "历史研究",
            "统计分析",
            "生物测量",
        ]
    }
}

impl Default for AnthropologyLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AnthropologyLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("anthropology")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【人类学定律】\n\n进化定律:\n{}\n\n文化定律:\n{}\n\n组织定律:\n{}\n",
            self.evolution_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.culture_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.organization_laws().iter()
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
    fn test_anthropology_laws() {
        let laws = AnthropologyLaws::new();
        assert!(!laws.evolution_laws().is_empty());
        assert!(!laws.culture_laws().is_empty());
    }
}