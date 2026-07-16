//! 民事主体深度规则
//!
//! 实现民事主体的详细规则验证，包括：
//! - 自然人规则（权利能力、行为能力、监护）
//! - 法人规则（设立、能力、机关、责任）
//! - 非法人组织规则（设立、能力、责任）

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
use serde::{Deserialize, Serialize};

/// 自然人民事行为能力类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapacityType {
    /// 完全民事行为能力
    Full,
    /// 限制民事行为能力
    Limited,
    /// 无民事行为能力
    None,
}

/// 自然人年龄分类
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgeCategory {
    /// 不满8周岁
    UnderEight,
    /// 8周岁以上不满18周岁
    EightToEighteen,
    /// 16周岁以上以劳动收入为主要生活来源
    WorkingMinor,
    /// 18周岁以上
    Adult,
}

/// 法人类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegalPersonType {
    /// 营利法人
    ForProfit,
    /// 非营利法人
    NonProfit,
    /// 特别法人
    Special,
}

/// 自然人规则验证结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NaturalPersonValidation {
    /// 是否具有民事权利能力
    pub has_capacity: bool,
    /// 行为能力类型
    pub capacity_type: CapacityType,
    /// 是否需要监护人
    pub needs_guardian: bool,
    /// 监护人顺序
    pub guardian_order: Vec<String>,
    /// 验证消息
    pub message: String,
}

/// 法人规则验证结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalPersonValidation {
    /// 法人类型
    pub person_type: LegalPersonType,
    /// 是否有效设立
    pub valid_establishment: bool,
    /// 法定代表人是否合法
    pub valid_representative: bool,
    /// 验证消息
    pub message: String,
}

simple_rule! {
    struct: CivilSubjectDeepRules,
    name: "民事主体深度规则",
    desc: "民事主体的详细规则验证",
    origin: "中国",
    tags: ["法律", "民法", "民事主体"]
}

impl CivilSubjectDeepRules {
    /// 判断自然人的民事行为能力类型
    ///
    /// # Arguments
    /// * `age` - 年龄（周岁）
    /// * `is_working` - 是否以劳动收入为主要生活来源
    /// * `can_identify` - 是否能辨认自己的行为
    ///
    /// # Returns
    /// 返回民事行为能力类型
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::law::civil_subject_deep::{CivilSubjectDeepRules, CapacityType};
    ///
    /// let rules = CivilSubjectDeepRules::new();
    /// let capacity = rules.determine_capacity(25, false, true);
    /// assert_eq!(capacity, CapacityType::Full);
    ///
    /// let limited = rules.determine_capacity(10, false, true);
    /// assert_eq!(limited, CapacityType::Limited);
    /// ```
    pub fn determine_capacity(&self, age: u32, is_working: bool, can_identify: bool) -> CapacityType {
        if age >= 18 {
            if can_identify {
                CapacityType::Full
            } else {
                CapacityType::Limited
            }
        } else if age >= 16 && is_working {
            CapacityType::Full
        } else if age >= 8 {
            if can_identify {
                CapacityType::Limited
            } else {
                CapacityType::None
            }
        } else {
            CapacityType::None
        }
    }

    /// 获取年龄分类
    ///
    /// # Arguments
    /// * `age` - 年龄（周岁）
    /// * `is_working` - 是否以劳动收入为主要生活来源
    ///
    /// # Returns
    /// 返回年龄分类
    pub fn get_age_category(&self, age: u32, is_working: bool) -> AgeCategory {
        if age >= 18 {
            AgeCategory::Adult
        } else if age >= 16 && is_working {
            AgeCategory::WorkingMinor
        } else if age >= 8 {
            AgeCategory::EightToEighteen
        } else {
            AgeCategory::UnderEight
        }
    }

    /// 验证自然人是否具有民事权利能力
    ///
    /// # Arguments
    /// * `is_alive` - 是否存活
    /// * `is_born_alive` - 是否活着出生（针对胎儿）
    ///
    /// # Returns
    /// 返回是否具有民事权利能力
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::law::civil_subject_deep::CivilSubjectDeepRules;
    ///
    /// let rules = CivilSubjectDeepRules::new();
    /// assert!(rules.has_civil_capacity(true, true));
    /// assert!(!rules.has_civil_capacity(false, false));
    /// ```
    pub fn has_civil_capacity(&self, is_alive: bool, is_born_alive: bool) -> bool {
        is_alive || is_born_alive
    }

    /// 获取监护人顺序（针对未成年人）
    ///
    /// # Returns
    /// 返回监护人顺序列表
    pub fn get_guardian_order_minor(&self) -> Vec<String> {
        vec![
            "父母".to_string(),
            "祖父母、外祖父母".to_string(),
            "兄、姐".to_string(),
            "其他愿意担任监护人的个人或组织".to_string(),
        ]
    }

    /// 获取监护人顺序（针对成年人）
    ///
    /// # Returns
    /// 返回监护人顺序列表
    pub fn get_guardian_order_adult(&self) -> Vec<String> {
        vec![
            "配偶".to_string(),
            "父母、子女".to_string(),
            "其他近亲属".to_string(),
            "其他愿意担任监护人的个人或组织".to_string(),
        ]
    }

    /// 验证自然人主体资格
    ///
    /// # Arguments
    /// * `age` - 年龄
    /// * `is_working` - 是否工作
    /// * `can_identify` - 是否能辨认行为
    /// * `is_alive` - 是否存活
    ///
    /// # Returns
    /// 返回验证结果
    pub fn validate_natural_person(
        &self,
        age: u32,
        is_working: bool,
        can_identify: bool,
        is_alive: bool,
    ) -> NaturalPersonValidation {
        let has_capacity = self.has_civil_capacity(is_alive, true);
        let capacity_type = self.determine_capacity(age, is_working, can_identify);
        let needs_guardian = capacity_type != CapacityType::Full;
        
        let guardian_order = if age < 18 {
            self.get_guardian_order_minor()
        } else {
            self.get_guardian_order_adult()
        };

        let message = match capacity_type {
            CapacityType::Full => "具有完全民事行为能力".to_string(),
            CapacityType::Limited => "具有限制民事行为能力，需要监护人".to_string(),
            CapacityType::None => "无民事行为能力，需要监护人代理".to_string(),
        };

        NaturalPersonValidation {
            has_capacity,
            capacity_type,
            needs_guardian,
            guardian_order,
            message,
        }
    }

    /// 验证法人设立条件
    ///
    /// # Arguments
    /// * `has_name` - 是否有名称
    /// * `has_org_structure` - 是否有组织机构
    /// * `has_address` - 是否有住所
    /// * `has_property` - 是否有财产或经费
    ///
    /// # Returns
    /// 返回是否满足设立条件
    ///
    /// # Examples
    /// ```
    /// use world_rules::rules::law::civil_subject_deep::CivilSubjectDeepRules;
    ///
    /// let rules = CivilSubjectDeepRules::new();
    /// assert!(rules.validate_legal_person_setup(true, true, true, true));
    /// assert!(!rules.validate_legal_person_setup(false, true, true, true));
    /// ```
    pub fn validate_legal_person_setup(
        &self,
        has_name: bool,
        has_org_structure: bool,
        has_address: bool,
        has_property: bool,
    ) -> bool {
        has_name && has_org_structure && has_address && has_property
    }

    /// 获取法人类型说明
    ///
    /// # Arguments
    /// * `person_type` - 法人类型
    ///
    /// # Returns
    /// 返回法人类型说明
    pub fn describe_legal_person_type(&self, person_type: LegalPersonType) -> Vec<String> {
        match person_type {
            LegalPersonType::ForProfit => vec![
                "营利法人：以取得利润并分配给股东等出资人为目的成立的法人".to_string(),
                "包括：有限责任公司、股份有限公司等".to_string(),
                "特点：以营利为目的，利润分配给出资人".to_string(),
            ],
            LegalPersonType::NonProfit => vec![
                "非营利法人：为公益目的或其他非营利目的成立的法人".to_string(),
                "包括：事业单位、社会团体、基金会、社会服务机构等".to_string(),
                "特点：不向出资人分配利润，收入用于公益目的".to_string(),
            ],
            LegalPersonType::Special => vec![
                "特别法人：具有特殊职能的法人".to_string(),
                "包括：机关法人、农村集体经济组织法人、城镇农村合作经济组织法人、基层群众性自治组织法人".to_string(),
                "特点：履行特定公共职能或实现特定社会目的".to_string(),
            ],
        }
    }

    /// 验证法人主体资格
    ///
    /// # Arguments
    /// * `person_type` - 法人类型
    /// * `has_name` - 是否有名称
    /// * `has_org_structure` - 是否有组织机构
    /// * `has_address` - 是否有住所
    /// * `has_property` - 是否有财产或经费
    /// * `has_representative` - 是否有法定代表人
    ///
    /// # Returns
    /// 返回验证结果
    pub fn validate_legal_person(
        &self,
        person_type: LegalPersonType,
        has_name: bool,
        has_org_structure: bool,
        has_address: bool,
        has_property: bool,
        has_representative: bool,
    ) -> LegalPersonValidation {
        let valid_establishment = self.validate_legal_person_setup(
            has_name,
            has_org_structure,
            has_address,
            has_property,
        );
        
        let message = if valid_establishment && has_representative {
            "法人设立合法，具有民事主体资格".to_string()
        } else if !valid_establishment {
            "法人设立条件不满足，不具有民事主体资格".to_string()
        } else {
            "法人未确定法定代表人，主体资格不完整".to_string()
        };

        LegalPersonValidation {
            person_type,
            valid_establishment,
            valid_representative: has_representative,
            message,
        }
    }

    /// 获取非法人组织类型
    ///
    /// # Returns
    /// 返回非法人组织类型列表
    pub fn get_non_legal_person_types(&self) -> Vec<String> {
        vec![
            "个人独资企业".to_string(),
            "合伙企业".to_string(),
            "不具有法人资格的专业服务机构".to_string(),
            "法人的分支机构".to_string(),
        ]
    }

    /// 验证胎儿利益保护
    ///
    /// # Arguments
    /// * `is_born_alive` - 是否活着出生
    ///
    /// # Returns
    /// 返回是否享有民事权利能力
    pub fn validate_fetus_protection(&self, is_born_alive: bool) -> (bool, String) {
        if is_born_alive {
            (true, "胎儿活着出生，自出生时起具有民事权利能力".to_string())
        } else {
            (false, "胎儿出生为死体，不具有民事权利能力".to_string())
        }
    }

    /// 验证宣告失踪条件
    ///
    /// # Arguments
    /// * `missing_years` - 下落不明年数
    ///
    /// # Returns
    /// 返回是否满足宣告失踪条件
    pub fn validate_missing_declaration(&self, missing_years: u32) -> (bool, String) {
        if missing_years >= 2 {
            (true, format!("下落不明满{}年，可以申请宣告失踪", missing_years))
        } else {
            (false, format!("下落不明未满2年，不符合宣告失踪条件（当前{}年）", missing_years))
        }
    }

    /// 验证宣告死亡条件
    ///
    /// # Arguments
    /// * `missing_years` - 下落不明年数
    /// * `is_accident` - 是否因意外事件下落不明
    ///
    /// # Returns
    /// 返回是否满足宣告死亡条件
    pub fn validate_death_declaration(&self, missing_years: u32, is_accident: bool) -> (bool, String) {
        if is_accident {
            if missing_years >= 2 {
                (true, "因意外事件下落不明满2年，可以申请宣告死亡".to_string())
            } else {
                (false, format!("意外事件下落不明未满2年（当前{}年）", missing_years))
            }
        } else if missing_years >= 4 {
            (true, format!("下落不明满{}年，可以申请宣告死亡", missing_years))
        } else {
            (false, format!("下落不明未满4年（当前{}年）", missing_years))
        }
    }

    /// 获取民事主体深度规则列表
    pub fn get_deep_rules(&self) -> Vec<&'static str> {
        vec![
            // 自然人规则
            "民事权利能力：自然人从出生时起到死亡时止，具有民事权利能力",
            "胎儿利益保护：胎儿视为具有民事权利能力，但娩出为死体的除外",
            "完全民事行为能力：年满18周岁；或16周岁以上以劳动收入为主要生活来源",
            "限制民事行为能力：8周岁以上的未成年人；或不能完全辨认自己行为的成年人",
            "无民事行为能力：不满8周岁的未成年人；或不能辨认自己行为的成年人",
            "监护顺序（未成年）：父母→祖父母外祖父母→兄姐→其他愿意监护的个人或组织",
            "监护顺序（成年）：配偶→父母子女→其他近亲属→其他愿意监护的个人或组织",
            "宣告失踪：下落不明满2年，利害关系人可申请宣告失踪",
            "宣告死亡：下落不明满4年，或意外事件下落不明满2年",
            
            // 法人规则
            "法人设立条件：有名称、组织机构、住所、财产或经费",
            "营利法人：以取得利润并分配给出资人为目的，如公司",
            "非营利法人：为公益目的成立，不分配利润，如事业单位、社会团体",
            "特别法人：机关法人、农村集体经济组织、基层群众性自治组织",
            "法定代表人：代表法人从事民事活动的负责人",
            "法人分支机构：以法人名义从事活动，责任由法人承担",
            
            // 非法人组织规则
            "个人独资企业：投资人以其个人财产对企业债务承担无限责任",
            "合伙企业：合伙人对合伙企业债务承担无限连带责任",
            "专业服务机构：不具有法人资格的专业服务机构，合伙人承担无限责任",
        ]
    }
}

impl Rule for CivilSubjectDeepRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("civil_subject_deep")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        let rules = self.get_deep_rules();
        let mut result = String::from("民事主体深度规则：\n\n");
        
        result.push_str("【自然人规则】\n");
        for rule in rules.iter().take(9) {
            result.push_str(&format!("• {}\n", rule));
        }
        
        result.push_str("\n【法人规则】\n");
        for rule in rules.iter().skip(9).take(6) {
            result.push_str(&format!("• {}\n", rule));
        }
        
        result.push_str("\n【非法人组织规则】\n");
        for rule in rules.iter().skip(15) {
            result.push_str(&format!("• {}\n", rule));
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_civil_subject_deep_rules() {
        let rules = CivilSubjectDeepRules::new();
        assert_eq!(rules.metadata().name, "民事主体深度规则");
        assert!(!rules.get_deep_rules().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_determine_capacity_adult() {
        let rules = CivilSubjectDeepRules::new();
        assert_eq!(rules.determine_capacity(25, false, true), CapacityType::Full);
        assert_eq!(rules.determine_capacity(18, false, true), CapacityType::Full);
    }

    #[test]
    fn test_determine_capacity_working_minor() {
        let rules = CivilSubjectDeepRules::new();
        assert_eq!(rules.determine_capacity(16, true, true), CapacityType::Full);
        assert_eq!(rules.determine_capacity(17, true, true), CapacityType::Full);
    }

    #[test]
    fn test_determine_capacity_limited() {
        let rules = CivilSubjectDeepRules::new();
        assert_eq!(rules.determine_capacity(10, false, true), CapacityType::Limited);
        assert_eq!(rules.determine_capacity(15, false, true), CapacityType::Limited);
    }

    #[test]
    fn test_determine_capacity_none() {
        let rules = CivilSubjectDeepRules::new();
        assert_eq!(rules.determine_capacity(5, false, true), CapacityType::None);
        assert_eq!(rules.determine_capacity(7, false, true), CapacityType::None);
    }

    #[test]
    fn test_has_civil_capacity() {
        let rules = CivilSubjectDeepRules::new();
        assert!(rules.has_civil_capacity(true, true));
        assert!(rules.has_civil_capacity(true, false));
        assert!(!rules.has_civil_capacity(false, false));
    }

    #[test]
    fn test_guardian_order_minor() {
        let rules = CivilSubjectDeepRules::new();
        let order = rules.get_guardian_order_minor();
        assert_eq!(order[0], "父母");
        assert_eq!(order.len(), 4);
    }

    #[test]
    fn test_guardian_order_adult() {
        let rules = CivilSubjectDeepRules::new();
        let order = rules.get_guardian_order_adult();
        assert_eq!(order[0], "配偶");
        assert_eq!(order.len(), 4);
    }

    #[test]
    fn test_validate_natural_person() {
        let rules = CivilSubjectDeepRules::new();
        let result = rules.validate_natural_person(25, false, true, true);
        assert!(result.has_capacity);
        assert_eq!(result.capacity_type, CapacityType::Full);
        assert!(!result.needs_guardian);
    }

    #[test]
    fn test_validate_natural_person_minor() {
        let rules = CivilSubjectDeepRules::new();
        let result = rules.validate_natural_person(10, false, true, true);
        assert!(result.has_capacity);
        assert_eq!(result.capacity_type, CapacityType::Limited);
        assert!(result.needs_guardian);
    }

    #[test]
    fn test_validate_legal_person_setup() {
        let rules = CivilSubjectDeepRules::new();
        assert!(rules.validate_legal_person_setup(true, true, true, true));
        assert!(!rules.validate_legal_person_setup(false, true, true, true));
    }

    #[test]
    fn test_describe_legal_person_type() {
        let rules = CivilSubjectDeepRules::new();
        let desc = rules.describe_legal_person_type(LegalPersonType::ForProfit);
        assert!(!desc.is_empty());
        assert!(desc[0].contains("营利法人"));
    }

    #[test]
    fn test_validate_legal_person() {
        let rules = CivilSubjectDeepRules::new();
        let result = rules.validate_legal_person(
            LegalPersonType::ForProfit,
            true,
            true,
            true,
            true,
            true,
        );
        assert!(result.valid_establishment);
        assert!(result.valid_representative);
    }

    #[test]
    fn test_fetus_protection() {
        let rules = CivilSubjectDeepRules::new();
        let (has_cap, msg) = rules.validate_fetus_protection(true);
        assert!(has_cap);
        assert!(msg.contains("活着出生"));
        
        let (no_cap, msg2) = rules.validate_fetus_protection(false);
        assert!(!no_cap);
        assert!(msg2.contains("死体"));
    }

    #[test]
    fn test_missing_declaration() {
        let rules = CivilSubjectDeepRules::new();
        let (valid, _) = rules.validate_missing_declaration(2);
        assert!(valid);
        
        let (invalid, _) = rules.validate_missing_declaration(1);
        assert!(!invalid);
    }

    #[test]
    fn test_death_declaration() {
        let rules = CivilSubjectDeepRules::new();
        let (valid, _) = rules.validate_death_declaration(4, false);
        assert!(valid);
        
        let (valid_accident, _) = rules.validate_death_declaration(2, true);
        assert!(valid_accident);
        
        let (invalid, _) = rules.validate_death_declaration(3, false);
        assert!(!invalid);
    }

    #[test]
    fn test_category() {
        let rules = CivilSubjectDeepRules::new();
        assert_eq!(rules.category(), RuleCategory::law("civil_subject_deep"));
    }

    #[test]
    fn test_get_non_legal_person_types() {
        let rules = CivilSubjectDeepRules::new();
        let types = rules.get_non_legal_person_types();
        assert!(!types.is_empty());
        assert!(types.contains(&"合伙企业".to_string()));
    }

    #[test]
    fn test_get_age_category() {
        let rules = CivilSubjectDeepRules::new();
        assert_eq!(rules.get_age_category(25, false), AgeCategory::Adult);
        assert_eq!(rules.get_age_category(16, true), AgeCategory::WorkingMinor);
        assert_eq!(rules.get_age_category(10, false), AgeCategory::EightToEighteen);
        assert_eq!(rules.get_age_category(5, false), AgeCategory::UnderEight);
    }
}