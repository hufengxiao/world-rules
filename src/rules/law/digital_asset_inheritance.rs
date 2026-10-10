//! 数字遗产继承
//!
//! 网络账号、虚拟财产等数字资产的继承规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: DigitalAssetInheritanceRules,
    name: "数字遗产继承",
    desc: "网络账号、虚拟财产等数字资产的继承规则",
    origin: "中国",
    tags: ["法律", "继承", "数字"]
}

impl DigitalAssetInheritanceRules {
    /// 资产范围
    pub fn scope(&self) -> Vec<&'static str> {
        vec![
            "社交账号与内容",
            "虚拟货币游戏装备",
            "数字钱包与余额",
            "云端照片文件",
        ]
    }

    /// 法律属性
    pub fn nature(&self) -> Vec<&'static str> {
        vec![
            "具财产价值者可继承",
            "人身属性账号难继承",
            "受平台服务协议约束",
            "依民法典继承规则",
        ]
    }

    /// 继承方式
    pub fn methods(&self) -> Vec<&'static str> {
        vec![
            "法定继承或遗嘱指定",
            "办理继承公证",
            "向平台提交继承材料",
            "有争议可诉讼解决",
        ]
    }

    /// 实践建议
    pub fn advice(&self) -> Vec<&'static str> {
        vec![
            "生前整理数字资产清单",
            "重要账号做好安排",
            "妥善保存密钥凭证",
            "及时更新平台政策",
        ]
    }
}

impl Rule for DigitalAssetInheritanceRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("digital_asset_inheritance")
    }

    fn explain(&self) -> String {
        format!(
            "【数字遗产继承】\n{}",
            [
                format!(
                    "资产范围：\\n{}",
                    self.scope()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "法律属性：\\n{}",
                    self.nature()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "继承方式：\\n{}",
                    self.methods()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "实践建议：\\n{}",
                    self.advice()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
            ]
            .join("\n\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_digitalassetinheritancerules_basic() {
        let rules = DigitalAssetInheritanceRules::new();
        assert_eq!(rules.metadata().name, "数字遗产继承");
        assert!(!rules.scope().is_empty());
        assert!(!rules.nature().is_empty());
        assert!(!rules.methods().is_empty());
        assert!(!rules.advice().is_empty());
    }

    #[test]
    fn test_digitalassetinheritancerules_validation() {
        let rules = DigitalAssetInheritanceRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::law("digital_asset_inheritance")
        );
    }

    #[test]
    fn test_digitalassetinheritancerules_explain() {
        let rules = DigitalAssetInheritanceRules::new();
        let e = rules.explain();
        assert!(e.contains("资产范围"));
        assert!(e.contains("法律属性"));
        assert!(e.contains("继承方式"));
    }
}
