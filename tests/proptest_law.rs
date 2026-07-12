//! 法律规则属性测试
//!
//! 使用 proptest 对法律规则进行属性测试，
//! 确保在各种输入条件下不会 panic 并保持正确性。

use proptest::prelude::*;
use world_rules::rules::core::{Rule, RuleCategory, ValidateContext};
use world_rules::rules::law::{
    CivilLawRules, CompanyLawRules, ConstitutionRules, ConsumerLawRules, CriminalLawRules,
    LaborLawRules, TaxLawRules,
};

/// 生成随机中文字符串（用于测试法律文本）
prop_compose! {
    fn chinese_text()(text in "[\u{4e00}-\u{9fff}]{0,100}") -> String {
        text
    }
}

/// 生成随机法律案例描述
prop_compose! {
    fn legal_case_text()(case in "合同|侵权|刑事|民事|商事|行政|劳动|税务|宪法") -> String {
        case
    }
}

/// 生成随机数字范围
prop_compose! {
    fn age_value()(age in 0u8..=120) -> u8 {
        age
    }
}

/// 生成随机金额
prop_compose! {
    fn amount_value()(amount in 0u64..=1000000000) -> u64 {
        amount
    }
}

// ==================== 规则创建测试 ====================

proptest! {
    /// 测试民法规则创建不会 panic
    #[test]
    fn test_civil_law_creation_no_panic() {
        let result = std::panic::catch_unwind(|| {
            CivilLawRules::new()
        });
        prop_assert!(result.is_ok());
    }

    /// 测试刑法规则创建不会 panic
    #[test]
    fn test_criminal_law_creation_no_panic() {
        let result = std::panic::catch_unwind(|| {
            CriminalLawRules::new()
        });
        prop_assert!(result.is_ok());
    }

    /// 测试宪法规则创建不会 panic
    #[test]
    fn test_constitution_creation_no_panic() {
        let result = std::panic::catch_unwind(|| {
            ConstitutionRules::new()
        });
        prop_assert!(result.is_ok());
    }

    /// 测试公司法规则创建不会 panic
    #[test]
    fn test_company_law_creation_no_panic() {
        let result = std::panic::catch_unwind(|| {
            CompanyLawRules::new()
        });
        prop_assert!(result.is_ok());
    }

    /// 测试消费者保护法规则创建不会 panic
    #[test]
    fn test_consumer_law_creation_no_panic() {
        let result = std::panic::catch_unwind(|| {
            ConsumerLawRules::new()
        });
        prop_assert!(result.is_ok());
    }

    /// 测试劳动法规则创建不会 panic
    #[test]
    fn test_labor_law_creation_no_panic() {
        let result = std::panic::catch_unwind(|| {
            LaborLawRules::new()
        });
        prop_assert!(result.is_ok());
    }

    /// 测试税法规则创建不会 panic
    #[test]
    fn test_tax_law_creation_no_panic() {
        let result = std::panic::catch_unwind(|| {
            TaxLawRules::new()
        });
        prop_assert!(result.is_ok());
    }
}

// ==================== 规则元数据测试 ====================

proptest! {
    /// 测试民法规则元数据属性
    #[test]
    fn test_civil_law_metadata_properties() {
        let rules = CivilLawRules::new();
        let meta = rules.metadata();

        // 元数据名称不为空
        prop_assert!(!meta.name.is_empty());
        // 元数据描述不为空
        prop_assert!(!meta.description.is_empty());
        // 版本号格式正确
        prop_assert!(meta.version.contains('.') || meta.version == "1.0.0");
    }

    /// 测试刑法规则元数据属性
    #[test]
    fn test_criminal_law_metadata_properties() {
        let rules = CriminalLawRules::new();
        let meta = rules.metadata();

        prop_assert!(!meta.name.is_empty());
        prop_assert!(!meta.description.is_empty());
    }

    /// 测试宪法规则元数据属性
    #[test]
    fn test_constitution_metadata_properties() {
        let rules = ConstitutionRules::new();
        let meta = rules.metadata();

        prop_assert!(!meta.name.is_empty());
        prop_assert!(!meta.description.is_empty());
    }
}

// ==================== 规则分类测试 ====================

proptest! {
    /// 测试民法规则分类正确
    #[test]
    fn test_civil_law_category() {
        let rules = CivilLawRules::new();
        let category = rules.category();

        prop_assert!(matches!(category, RuleCategory::Law(_)));
        let category_str = category.to_string();
        prop_assert!(category_str.starts_with("Law/"));
    }

    /// 测试刑法规则分类正确
    #[test]
    fn test_criminal_law_category() {
        let rules = CriminalLawRules::new();
        let category = rules.category();

        prop_assert!(matches!(category, RuleCategory::Law(_)));
    }

    /// 测试宪法规则分类正确
    #[test]
    fn test_constitution_category() {
        let rules = ConstitutionRules::new();
        let category = rules.category();

        prop_assert!(matches!(category, RuleCategory::Law(_)));
    }

    /// 测试公司法规则分类正确
    #[test]
    fn test_company_law_category() {
        let rules = CompanyLawRules::new();
        let category = rules.category();

        prop_assert!(matches!(category, RuleCategory::Law(_)));
    }
}

// ==================== 规则验证测试 ====================

proptest! {
    /// 测试民法规则验证不会 panic（任何输入）
    #[test]
    fn test_civil_law_validate_no_panic(text in chinese_text()) {
        let rules = CivilLawRules::new();
        let ctx = ValidateContext::Generic(text);

        let result = std::panic::catch_unwind(|| {
            rules.validate(&ctx)
        });
        prop_assert!(result.is_ok());

        // 验证结果应该是 Ok
        if let Ok(validate_result) = result {
            prop_assert!(validate_result.is_ok());
        }
    }

    /// 测试刑法规则验证不会 panic
    #[test]
    fn test_criminal_law_validate_no_panic(text in chinese_text()) {
        let rules = CriminalLawRules::new();
        let ctx = ValidateContext::Generic(text);

        let result = std::panic::catch_unwind(|| {
            rules.validate(&ctx)
        });
        prop_assert!(result.is_ok());
    }

    /// 测试宪法规则验证不会 panic
    #[test]
    fn test_constitution_validate_no_panic(text in chinese_text()) {
        let rules = ConstitutionRules::new();
        let ctx = ValidateContext::Generic(text);

        let result = std::panic::catch_unwind(|| {
            rules.validate(&ctx)
        });
        prop_assert!(result.is_ok());
    }

    /// 测试公司法规则验证不会 panic
    #[test]
    fn test_company_law_validate_no_panic(text in chinese_text()) {
        let rules = CompanyLawRules::new();
        let ctx = ValidateContext::Generic(text);

        let result = std::panic::catch_unwind(|| {
            rules.validate(&ctx)
        });
        prop_assert!(result.is_ok());
    }

    /// 测试消费者法规则验证不会 panic
    #[test]
    fn test_consumer_law_validate_no_panic(text in chinese_text()) {
        let rules = ConsumerLawRules::new();
        let ctx = ValidateContext::Generic(text);

        let result = std::panic::catch_unwind(|| {
            rules.validate(&ctx)
        });
        prop_assert!(result.is_ok());
    }

    /// 测试劳动法规则验证不会 panic
    #[test]
    fn test_labor_law_validate_no_panic(text in chinese_text()) {
        let rules = LaborLawRules::new();
        let ctx = ValidateContext::Generic(text);

        let result = std::panic::catch_unwind(|| {
            rules.validate(&ctx)
        });
        prop_assert!(result.is_ok());
    }

    /// 测试税法规则验证不会 panic
    #[test]
    fn test_tax_law_validate_no_panic(text in chinese_text()) {
        let rules = TaxLawRules::new();
        let ctx = ValidateContext::Generic(text);

        let result = std::panic::catch_unwind(|| {
            rules.validate(&ctx)
        });
        prop_assert!(result.is_ok());
    }
}

// ==================== 规则说明测试 ====================

proptest! {
    /// 测试民法规则说明生成不会 panic
    #[test]
    fn test_civil_law_explain_no_panic() {
        let rules = CivilLawRules::new();

        let result = std::panic::catch_unwind(|| {
            rules.explain()
        });
        prop_assert!(result.is_ok());

        if let Ok(explanation) = result {
            prop_assert!(!explanation.is_empty());
        }
    }

    /// 测试刑法规则说明生成不会 panic
    #[test]
    fn test_criminal_law_explain_no_panic() {
        let rules = CriminalLawRules::new();

        let result = std::panic::catch_unwind(|| {
            rules.explain()
        });
        prop_assert!(result.is_ok());

        if let Ok(explanation) = result {
            prop_assert!(!explanation.is_empty());
        }
    }

    /// 测试宪法规则说明生成不会 panic
    #[test]
    fn test_constitution_explain_no_panic() {
        let rules = ConstitutionRules::new();

        let result = std::panic::catch_unwind(|| {
            rules.explain()
        });
        prop_assert!(result.is_ok());

        if let Ok(explanation) = result {
            prop_assert!(!explanation.is_empty());
        }
    }

    /// 测试公司法规则说明生成不会 panic
    #[test]
    fn test_company_law_explain_no_panic() {
        let rules = CompanyLawRules::new();

        let result = std::panic::catch_unwind(|| {
            rules.explain()
        });
        prop_assert!(result.is_ok());
    }
}

// ==================== 民法规则方法测试 ====================

proptest! {
    /// 测试民法基本原则获取不会 panic
    #[test]
    fn test_civil_basic_principles_no_panic() {
        let rules = CivilLawRules::new();

        let result = std::panic::catch_unwind(|| {
            rules.basic_principles()
        });
        prop_assert!(result.is_ok());

        if let Ok(principles) = result {
            prop_assert!(!principles.is_empty());
        }
    }

    /// 测试民事主体获取不会 panic
    #[test]
    fn test_civil_subjects_no_panic() {
        let rules = CivilLawRules::new();

        let result = std::panic::catch_unwind(|| {
            rules.civil_subjects()
        });
        prop_assert!(result.is_ok());
    }

    /// 测试自然人行为能力获取不会 panic
    #[test]
    fn test_capacity_of_person_no_panic() {
        let rules = CivilLawRules::new();

        let result = std::panic::catch_unwind(|| {
            rules.capacity_of_person()
        });
        prop_assert!(result.is_ok());
    }

    /// 测试民事权利获取不会 panic
    #[test]
    fn test_civil_rights_no_panic() {
        let rules = CivilLawRules::new();

        let result = std::panic::catch_unwind(|| {
            rules.civil_rights()
        });
        prop_assert!(result.is_ok());
    }

    /// 测试民事法律行为获取不会 panic
    #[test]
    fn test_civil_acts_no_panic() {
        let rules = CivilLawRules::new();

        let result = std::panic::catch_unwind(|| {
            rules.civil_acts()
        });
        prop_assert!(result.is_ok());
    }

    /// 测试代理制度获取不会 panic
    #[test]
    fn test_agency_system_no_panic() {
        let rules = CivilLawRules::new();

        let result = std::panic::catch_unwind(|| {
            rules.agency_system()
        });
        prop_assert!(result.is_ok());
    }

    /// 测试诉讼时效获取不会 panic
    #[test]
    fn test_limitation_of_action_no_panic() {
        let rules = CivilLawRules::new();

        let result = std::panic::catch_unwind(|| {
            rules.limitation_of_action()
        });
        prop_assert!(result.is_ok());
    }
}

// ==================== 特定法律场景测试 ====================

proptest! {
    /// 测试合同相关文本验证
    #[test]
    fn test_contract_text_validate(contract_type in legal_case_text()) {
        let rules = CivilLawRules::new();
        let ctx = ValidateContext::Generic(format!("{}纠纷", contract_type));

        let result = std::panic::catch_unwind(|| {
            rules.validate(&ctx)
        });
        prop_assert!(result.is_ok());
    }

    /// 测试年龄相关法律规则（民法行为能力）
    #[test]
    fn test_age_related_law(age in age_value()) {
        let rules = CivilLawRules::new();

        // 根据年龄生成不同的法律场景
        let scenario = if age < 8 {
            "无民事行为能力人"
        } else if age < 18 {
            "限制民事行为能力人"
        } else {
            "完全民事行为能力人"
        };

        let ctx = ValidateContext::Generic(format!("年龄{}岁，{}", age, scenario));

        let result = std::panic::catch_unwind(|| {
            rules.validate(&ctx)
        });
        prop_assert!(result.is_ok());
    }
}

// ==================== 多规则批量测试 ====================

proptest! {
    /// 测试多个法律规则同时验证不会 panic
    #[test]
    fn test_multiple_rules_validate(text in chinese_text()) {
        let rules: Vec<Box<dyn Rule>> = vec![
            Box::new(CivilLawRules::new()),
            Box::new(CriminalLawRules::new()),
            Box::new(ConstitutionRules::new()),
        ];

        let ctx = ValidateContext::Generic(text);

        for rule in &rules {
            let result = std::panic::catch_unwind(|| {
                rule.validate(&ctx)
            });
            prop_assert!(result.is_ok());
        }
    }
}

#[cfg(test)]
mod additional_tests {
    use super::*;

    #[test]
    fn test_proptest_config_basic() {
        // 基本功能验证
        proptest!(|(text in "合同|侵权")| {
            let rules = CivilLawRules::new();
            let ctx = ValidateContext::Generic(text);
            assert!(rules.validate(&ctx).is_ok());
        });
    }
}
