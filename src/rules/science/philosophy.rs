//! 哲学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 哲学定律集合
pub struct PhilosophyLaws {
    metadata: RuleMetadata,
}

impl PhilosophyLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "哲学定律",
                "哲学基本定律"
            )
            .with_origin("人文科学")
            .with_tags(vec!["科学".into(), "哲学".into()]),
        }
    }

    /// 本体论定律
    pub fn ontology_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("存在定律", "存在优先", "存在先于本质"),
            ("本质定律", "本质探寻", "事物本质探求"),
            ("实体定律", "实体存在", "实体作为存在基础"),
            ("现象定律", "现象显现", "事物现象显现"),
            ("因果定律", "因果联系", "因果必然联系"),
            ("同一性定律", "事物同一", "事物自身同一"),
            ("矛盾定律", "矛盾存在", "矛盾普遍存在"),
        ]
    }

    /// 认识论定律
    pub fn epistemology_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("知识定律", "知识来源", "知识来源与本质"),
            ("真理定律", "真理标准", "真理判定标准"),
            ("理性定律", "理性认知", "理性认识能力"),
            ("经验定律", "经验来源", "经验知识来源"),
            ("怀疑定律", "怀疑精神", "怀疑作为方法"),
            ("确定性定律", "确定追求", "知识的确定性"),
            ("理解定律", "理解方式", "理解的本质"),
        ]
    }

    /// 逻辑定律
    pub fn logic_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("同一律", "A是A", "同一思维过程概念同一"),
            ("矛盾律", "A与非A不能同真", "矛盾命题不能同真"),
            ("排中律", "A或非A必有一真", "矛盾命题必有一真"),
            ("充足理由律", "有充足理由", "事物必有充足理由"),
            ("演绎定律", "演绎推理", "演绎推理有效性"),
            ("归纳定律", "归纳推理", "归纳推理规律"),
            ("类比定律", "类比推理", "类比推理规则"),
        ]
    }

    /// 伦理学定律
    pub fn ethics_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("善定律", "善的追求", "善的价值追求"),
            ("道德定律", "道德规范", "道德规范体系"),
            ("正义定律", "正义原则", "正义分配原则"),
            ("责任定律", "道德责任", "道德责任承担"),
            ("自由定律", "道德自由", "道德选择自由"),
            ("义务定律", "道德义务", "道德义务要求"),
            ("功利定律", "功利原则", "功利最大原则"),
        ]
    }

    /// 哲学分支
    pub fn branches(&self) -> Vec<&'static str> {
        vec![
            "形而上学",
            "本体论",
            "认识论",
            "伦理学",
            "美学",
            "逻辑学",
            "政治哲学",
            "宗教哲学",
        ]
    }

    /// 哲学流派
    pub fn schools(&self) -> Vec<&'static str> {
        vec![
            "唯心主义",
            "唯物主义",
            "理性主义",
            "经验主义",
            "实用主义",
            "存在主义",
            "结构主义",
            "后现代主义",
        ]
    }
}

impl Default for PhilosophyLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for PhilosophyLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("philosophy")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【哲学定律】\n\n本体论定律:\n{}\n\n认识论定律:\n{}\n\n逻辑定律:\n{}\n",
            self.ontology_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.epistemology_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.logic_laws().iter()
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
    fn test_philosophy_laws() {
        let laws = PhilosophyLaws::new();
        assert!(!laws.ontology_laws().is_empty());
        assert!(!laws.logic_laws().is_empty());
    }
}