//! 语言学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 语言学定律集合
pub struct LinguisticsLaws {
    metadata: RuleMetadata,
}

impl LinguisticsLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("语言学定律", "语言学基本定律")
                .with_origin("人文科学")
                .with_tags(vec!["科学".into(), "语言".into()]),
        }
    }

    /// 语言结构定律
    pub fn structure_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("语言层级定律", "多层结构", "语言层级结构"),
            ("语音定律", "语音系统", "语音规律"),
            ("词汇定律", "词汇系统", "词汇规律"),
            ("语法定律", "语法规则", "语法规则体系"),
            ("语义定律", "意义系统", "语义规则"),
            ("语用定律", "使用规则", "语用规则"),
            ("句法定律", "句子结构", "句子结构规则"),
        ]
    }

    /// 语言演变定律
    pub fn evolution_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("语音演变定律", "语音变化", "语音演变规律"),
            ("词汇演变定律", "词汇变化", "词汇演变规律"),
            ("语法演变定律", "语法变化", "语法演变规律"),
            ("语义演变定律", "意义变化", "语义演变规律"),
            ("语言分化定律", "语言分裂", "语言分化规律"),
            ("语言融合定律", "语言混合", "语言融合规律"),
            ("语言接触定律", "接触影响", "语言接触影响"),
        ]
    }

    /// 语言认知定律
    pub fn cognition_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("语言习得定律", "习得过程", "语言习得规律"),
            ("语言理解定律", "理解机制", "语言理解机制"),
            ("语言产出定律", "产出过程", "语言产出规律"),
            ("语言记忆定律", "记忆存储", "语言记忆存储"),
            ("语言思维定律", "思维关系", "语言与思维关系"),
            ("语言意识定律", "意识参与", "语言意识参与"),
        ]
    }

    /// 语言社会定律
    pub fn social_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("语言社会定律", "社会因素", "语言社会因素"),
            ("语言方言定律", "地域差异", "方言差异规律"),
            ("语言阶层定律", "阶层差异", "语言阶层差异"),
            ("语言性别定律", "性别差异", "语言性别差异"),
            ("语言年龄定律", "年龄差异", "语言年龄差异"),
            ("语言政策定律", "政策影响", "语言政策影响"),
        ]
    }

    /// 语言类型
    pub fn language_types(&self) -> Vec<&'static str> {
        vec![
            "分析语",
            "综合语",
            "屈折语",
            "黏着语",
            "孤立语",
            "多式综合语",
            "人造语言",
            "自然语言",
        ]
    }

    /// 语言学分支
    pub fn branches(&self) -> Vec<&'static str> {
        vec![
            "语音学",
            "音系学",
            "形态学",
            "句法学",
            "语义学",
            "语用学",
            "社会语言学",
            "认知语言学",
        ]
    }

    /// 音系学
    pub fn phonology(&self) -> Vec<&'static str> {
        vec![
            "音位: 能区别意义的最小语音单位",
            "音位变体: 同一音位的不同语音实现",
            "区别特征: 区分音位的最小语音特征",
            "音节结构: 声母+韵母韵母含韵头韵腹韵尾",
            "声调: 利用音高区别意义的超音段特征",
            "语流音变: 连续语音中发生的语音变化",
        ]
    }

    /// 句法与语义
    pub fn syntax_semantics(&self) -> Vec<&'static str> {
        vec![
            "短语结构规则: S→NP VP等句法生成规则",
            "转换生成语法: 深层结构通过转换规则生成表层结构",
            "格语法: 动词与名词之间的语义角色关系",
            "语义场: 具有共同语义特征的词汇聚合",
            "隐喻: 用一个概念域理解另一个概念域",
            "会话含义: 说话者通过违反合作原则传达的隐含意义",
            "言语行为理论: 说话即做事包括言内行为言外行为和言后行为",
        ]
    }

    /// 心理语言学
    pub fn psycholinguistics(&self) -> Vec<&'static str> {
        vec![
            "语言习得: 儿童获得母语能力的过程",
            "关键期假说: 语言学习存在最佳的生物学时期",
            "词汇提取: 从心理词库中检索词的过程",
            "句子理解: 对句法和语义信息的实时加工",
            "双语加工: 双语者两种语言的激活和控制",
            "语言产出: 从思想到言语表达的过程",
        ]
    }
}

impl Default for LinguisticsLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for LinguisticsLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("linguistics")
    }

    fn explain(&self) -> String {
        format!(
            "【语言学定律】\n\n结构定律:\n{}\n\n演变定律:\n{}\n\n认知定律:\n{}\n",
            self.structure_laws()
                .iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.evolution_laws()
                .iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cognition_laws()
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
    fn test_linguistics_laws() {
        let laws = LinguisticsLaws::new();
        assert!(!laws.structure_laws().is_empty());
        assert!(!laws.evolution_laws().is_empty());
    }
}
