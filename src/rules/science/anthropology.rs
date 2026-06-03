//! 人类学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 人类学定律集合
pub struct AnthropologyLaws {
    metadata: RuleMetadata,
}

impl AnthropologyLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("人类学定律", "人类学基本定律")
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

    /// 文化人类学
    pub fn cultural_anthropology(&self) -> Vec<&'static str> {
        vec![
            "文化相对主义: 每种文化应在其自身语境中理解",
            "功能主义: 文化各要素对维持社会整体起特定功能",
            "结构主义: 表层文化现象下存在普遍的心智结构",
            "象征人类学: 文化是意义之网需要解读和诠释",
            "实践理论: 文化通过日常实践被再生产和改变",
            "全球化: 文化经济政治在全球范围的相互联系",
        ]
    }

    /// 体质人类学
    pub fn physical_anthropology(&self) -> Vec<&'static str> {
        vec![
            "人类进化: 从南方古猿到现代人的演化历程",
            "直立行走: 人类最早的重要适应性特征",
            "脑容量变化: 从约400cc增加到现代人的1400cc",
            "分子人类学: 利用DNA分析追溯人类起源和迁移",
            "夏娃假说: 现代人类共同起源于约20万年前的非洲",
            "适应性特征: 人类体质特征对环境的适应性变化",
        ]
    }

    /// 语言人类学
    pub fn linguistic_anthropology(&self) -> Vec<&'static str> {
        vec![
            "萨丕尔沃尔夫假说: 语言结构影响思维方式",
            "语言相对论: 不同语言对现实的不同分割",
            "语言濒危: 少数语言使用者减少面临消亡",
            "语言复兴: 濒临消亡语言的恢复和保护",
            "语码转换: 说话者在同一对话中切换语言",
            "语言与权力: 语言使用反映和维护社会权力关系",
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
            self.evolution_laws()
                .iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.culture_laws()
                .iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.organization_laws()
                .iter()
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
