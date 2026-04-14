//! 刑事诉讼法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 刑事诉讼法规则
pub struct CriminalProcedureLawRules {
    metadata: RuleMetadata,
}

impl CriminalProcedureLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "刑事诉讼法规则",
                "中国刑事诉讼法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "刑事诉讼".into()]),
        }
    }

    /// 刑事诉讼原则
    pub fn basic_principles(&self) -> Vec<&'static str> {
        vec![
            "无罪推定原则",
            "证据裁判原则",
            "程序公正原则",
            "公开审判原则",
            "辩护权保障原则",
            "禁止双重追诉原则",
            "疑罪从无原则",
            "人权保障原则",
        ]
    }

    /// 管辖规则
    pub fn jurisdiction(&self) -> Vec<&'static str> {
        vec![
            "级别管辖: 法院层级管辖",
            "地域管辖: 地区管辖规则",
            "专门管辖: 专门法院管辖",
            "移送管辖: 案件移送规则",
            "指定管辖: 上级指定管辖",
            "管辖权争议处理",
            "优先管辖规则",
            "合并管辖规则",
        ]
    }

    /// 强制措施
    pub fn compulsory_measures(&self) -> Vec<&'static str> {
        vec![
            "拘传: 强制传唤",
            "取保候审: 保证金保证",
            "监视居住: 监视居住措施",
            "拘留: 刑事拘留",
            "逮捕: 正式逮捕",
            "强制措施期限",
            "强制措施变更",
            "强制措施解除",
        ]
    }

    /// 刑事侦查
    pub fn criminal_investigation(&self) -> Vec<&'static str> {
        vec![
            "立案程序: 案件立案",
            "侦查措施: 侦查手段",
            "讯问犯罪嫌疑人",
            "询问证人被害人",
            "勘验检查: 现场勘查",
            "搜查扣押: 证据搜查",
            "鉴定程序: 司法鉴定",
            "侦查终结: 侦查结束",
        ]
    }

    /// 刑事起诉
    pub fn criminal_prosecution(&self) -> Vec<&'static str> {
        vec![
            "审查起诉程序",
            "提起公诉: 公诉提起",
            "不起诉决定",
            "起诉书制作",
            "证据移送: 证据提交",
            "补充侦查: 侦查补充",
            "撤回起诉: 起诉撤回",
            "起诉期限规定",
        ]
    }

    /// 刑事审判
    pub fn criminal_trial(&self) -> Vec<&'static str> {
        vec![
            "一审程序: 第一审审判",
            "二审程序: 第二审审判",
            "开庭审理程序",
            "法庭调查程序",
            "法庭辩论程序",
            "被告人最后陈述",
            "判决宣告程序",
            "审判期限规定",
        ]
    }

    /// 刑事证据
    pub fn criminal_evidence(&self) -> Vec<&'static str> {
        vec![
            "证据种类: 证据类型",
            "证据收集: 证据获取",
            "证据审查: 证据审核",
            "证据认定: 证据采信",
            "非法证据排除",
            "证据举证责任",
            "证据证明标准",
            "证据保全措施",
        ]
    }

    /// 刑事辩护
    pub fn criminal_defense(&self) -> Vec<&'static str> {
        vec![
            "辩护权保障",
            "辩护人权利",
            "辩护人义务",
            "自行辩护权利",
            "委托辩护权利",
            "指定辩护情形",
            "辩护意见表达",
            "辩护证据提交",
        ]
    }
}

impl Default for CriminalProcedureLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CriminalProcedureLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("criminal_procedure")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【刑事诉讼法规则】\n\n基本原则:\n{}\n\n强制措施:\n{}\n\n刑事侦查:\n{}\n",
            self.basic_principles().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.compulsory_measures().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.criminal_investigation().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_criminal_procedure_law_rules() {
        let rules = CriminalProcedureLawRules::new();
        assert!(!rules.basic_principles().is_empty());
        assert!(!rules.compulsory_measures().is_empty());
    }
}