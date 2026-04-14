//! 政治学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 政治学定律集合
pub struct PoliticalScienceLaws {
    metadata: RuleMetadata,
}

impl PoliticalScienceLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "政治学定律",
                "政治学基本定律"
            )
            .with_origin("社会科学")
            .with_tags(vec!["科学".into(), "政治".into()]),
        }
    }

    /// 权力定律
    pub fn power_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("权力来源定律", "权力来源", "权力来源于人民"),
            ("权力分配定律", "权力配置", "权力配置与分配"),
            ("权力制衡定律", "相互制约", "权力相互制约"),
            ("权力委托定律", "委托代理", "权力委托代理关系"),
            ("权力腐败定律", "权力失控", "绝对权力导致腐败"),
            ("权力合法性定律", "合法基础", "权力合法性基础"),
            ("权力传承定律", "权力转移", "权力交接机制"),
        ]
    }

    /// 政治制度定律
    pub fn institution_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("民主定律", "人民主权", "人民享有主权"),
            ("法治定律", "法律至上", "法律高于一切"),
            ("分权定律", "三权分立", "权力分立制衡"),
            ("代议定律", "代表制度", "人民代表制度"),
            ("选举定律", "选举机制", "选举产生代表"),
            ("宪法定律", "宪法至上", "宪法根本法律"),
            ("政党定律", "政党制度", "政党政治制度"),
        ]
    }

    /// 国际关系定律
    pub fn international_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("主权定律", "国家主权", "国家主权平等"),
            ("国家利益定律", "利益优先", "国家利益优先"),
            ("国际法定律", "国际规范", "国际法规范"),
            ("外交定律", "外交原则", "外交基本原则"),
            ("冲突定律", "国际冲突", "国际冲突管理"),
            ("合作定律", "国际合作", "国际合作机制"),
            ("条约定律", "条约效力", "条约法律效力"),
        ]
    }

    /// 政治行为定律
    pub fn behavior_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("政治参与定律", "公民参与", "公民政治参与"),
            ("投票定律", "投票行为", "投票行为规律"),
            ("政治态度定律", "态度形成", "政治态度形成"),
            ("政治认同定律", "认同形成", "政治认同机制"),
            ("政治动员定律", "动员机制", "政治动员规律"),
            ("政治传播定律", "传播规律", "政治传播规律"),
            ("政治社会化定律", "社会化过程", "政治社会化"),
        ]
    }

    /// 政治制度类型
    pub fn system_types(&self) -> Vec<&'static str> {
        vec![
            "民主制度",
            "专制制度",
            "共和制度",
            "君主制度",
            "议会制度",
            "总统制度",
            "混合制度",
            "联邦制度",
        ]
    }

    /// 政治学理论
    pub fn theories(&self) -> Vec<&'static str> {
        vec![
            "自由主义理论",
            "保守主义理论",
            "社会主义理论",
            "现实主义理论",
            "建构主义理论",
            "制度主义理论",
            "功能主义理论",
            "精英主义理论",
        ]
    }
}

impl Default for PoliticalScienceLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for PoliticalScienceLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("political_science")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【政治学定律】\n\n权力定律:\n{}\n\n制度定律:\n{}\n\n国际关系定律:\n{}\n",
            self.power_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.institution_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.international_laws().iter()
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
    fn test_political_science_laws() {
        let laws = PoliticalScienceLaws::new();
        assert!(!laws.power_laws().is_empty());
        assert!(!laws.institution_laws().is_empty());
    }
}