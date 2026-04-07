//! 继承法规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 继承法规则
pub struct InheritanceLawRules {
    metadata: RuleMetadata,
}

impl InheritanceLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "继承法规则",
                "中国继承法基本规则"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "继承".into()]),
        }
    }

    /// 继承开始时间
    pub fn inheritance_start(&self) -> Vec<&'static str> {
        vec![
            "被继承人死亡时开始",
            "包括自然死亡和宣告死亡",
            "相互有继承关系的几人同时死亡推定长辈先死",
        ]
    }

    /// 法定继承人
    pub fn legal_heirs(&self) -> Vec<&'static str> {
        vec![
            "第一顺序: 配偶、子女、父母",
            "第二顺序: 兄弟姐妹、祖父母、外祖父母",
            "有第一顺序继承人时，第二顺序不继承",
            "丧偶儿媳/女婿对公婆/岳父母尽了赡养义务的作为第一顺序",
        ]
    }

    /// 遗嘱形式
    pub fn will_forms(&self) -> Vec<&'static str> {
        vec![
            "自书遗嘱: 亲笔书写、签名、注明年月日",
            "代书遗嘱: 两人以上见证，一人代书",
            "打印遗嘱: 两人以上见证，每页签名",
            "录音录像遗嘱: 两人以上见证",
            "口头遗嘱: 紧急情况，两人以上见证",
            "公证遗嘱: 经公证机构办理",
        ]
    }

    /// 遗嘱见证人限制
    pub fn witness_restrictions(&self) -> Vec<&'static str> {
        vec![
            "无民事行为能力人",
            "限制民事行为能力人",
            "继承人、受遗赠人",
            "与继承人、受遗赠人有利害关系的人",
        ]
    }

    /// 继承份额
    pub fn inheritance_shares(&self) -> Vec<&'static str> {
        vec![
            "同一顺序继承人一般均等",
            "生活有特殊困难又缺乏劳动能力的应予照顾",
            "尽了主要扶养义务的可多分",
            "有扶养能力和条件不尽扶养义务的应少分或不分",
        ]
    }

    /// 放弃继承
    pub fn renouncing_inheritance(&self) -> Vec<&'static str> {
        vec![
            "必须在遗产处理前明确表示放弃",
            "没有表示的视为接受继承",
            "放弃继承不得附带条件",
            "放弃继承后遗产归其他继承人",
        ]
    }
}

impl Default for InheritanceLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for InheritanceLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("inheritance")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【继承法规则】\n\n\
            法定继承人:\n{}\n\n\
            遗嘱形式:\n{}\n\n\
            继承份额:\n{}\n",
            self.legal_heirs().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.will_forms().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.inheritance_shares().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inheritance_law_rules() {
        let rules = InheritanceLawRules::new();
        assert!(!rules.legal_heirs().is_empty());
    }
}