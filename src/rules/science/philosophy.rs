//! 哲学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 哲学定律集合
pub struct PhilosophyLaws {
    metadata: RuleMetadata,
}

impl PhilosophyLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("哲学定律", "哲学基本定律")
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

    /// 美学定律
    pub fn aesthetics_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("审美本质定律", "审美本质", "美的本质探求"),
            ("审美经验定律", "经验结构", "审美经验分析"),
            ("审美范畴定律", "优美崇高", "审美范畴体系"),
            ("艺术哲学定律", "艺术本质", "艺术哲学思考"),
            ("审美判断力定律", "判断力", "康德审美判断力批判"),
            ("审美无功利定律", "无功利", "审美态度无功利性"),
            ("审美趣味定律", "趣味标准", "审美趣味的普遍性"),
            ("审美理念定律", "理念显现", "审美理念感性显现"),
        ]
    }

    /// 政治哲学定律
    pub fn political_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("社会契约定律", "契约论", "社会契约论基础"),
            ("自然权利定律", "天赋人权", "自然权利理论"),
            ("公共理性定律", "公共理性", "公共理性与正义"),
            ("自由意志定律", "意志自由", "自由意志哲学基础"),
            ("分配正义定律", "分配原则", "分配正义原则"),
            ("程序正义定律", "程序公正", "程序正义理论"),
            ("社群主义定律", "社群价值", "社群主义政治哲学"),
            ("世界主义定律", "全球正义", "世界主义伦理观"),
        ]
    }

    /// 科学哲学定律
    pub fn philosophy_of_science_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("科学方法论定律", "方法论", "科学方法哲学反思"),
            ("科学实在论定律", "实在论", "科学理论实在性"),
            ("范式转换定律", "范式", "库恩科学革命结构"),
            ("证伪主义定律", "可证伪", "波普尔证伪主义"),
            ("科学划界定律", "划界标准", "科学与非科学划界"),
            ("科学革命定律", "科学革命", "科学革命的结构"),
            ("研究纲领定律", "研究纲领", "拉卡托斯研究纲领"),
            ("科学社会建构定律", "社会建构", "科学知识社会建构"),
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
            "【哲学定律】\n\n本体论定律:\n{}\n\n认识论定律:\n{}\n\n逻辑定律:\n{}\n\n美学定律:\n{}\n\n政治哲学定律:\n{}\n\n科学哲学定律:\n{}\n",
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
                .join("\n"),
            self.aesthetics_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.political_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.philosophy_of_science_laws().iter()
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
