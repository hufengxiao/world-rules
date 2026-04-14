//! 老年人权益保障法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 老年人权益保障法规则
pub struct ElderlyProtectionLawRules {
    metadata: RuleMetadata,
}

impl ElderlyProtectionLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "老年人权益保障法规则",
                "中国老年人权益保障法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "老年人权益".into()]),
        }
    }

    /// 老年人权利
    pub fn elderly_rights(&self) -> Vec<&'static str> {
        vec![
            "生活赡养权利",
            "财产保护权利",
            "继承权利保障",
            "婚姻自由权利",
            "受教育权利",
            "文化娱乐权利",
            "参与发展权利",
            "获得救助权利",
        ]
    }

    /// 家庭赡养义务
    pub fn family_support(&self) -> Vec<&'static str> {
        vec![
            "经济供养义务",
            "生活照料义务",
            "精神慰藉义务",
            "医疗护理义务",
            "住房保障义务",
            "探望问候义务",
            "赡养协议签订",
            "赡养纠纷调解",
        ]
    }

    /// 社会保障
    pub fn social_security(&self) -> Vec<&'static str> {
        vec![
            "养老保险保障",
            "医疗保险保障",
            "长期护理保险",
            "社会救助保障",
            "社会福利保障",
            "养老服务补贴",
            "高龄津贴制度",
            "残疾老人补贴",
        ]
    }

    /// 社会服务
    pub fn social_services(&self) -> Vec<&'static str> {
        vec![
            "居家养老服务",
            "社区养老服务",
            "机构养老服务",
            "医养结合服务",
            "日间照料服务",
            "上门养老服务",
            "康复护理服务",
            "精神慰藉服务",
        ]
    }

    /// 社会优待
    pub fn social_privileges(&self) -> Vec<&'static str> {
        vec![
            "交通优待服务",
            "医疗优待服务",
            "文化优待服务",
            "公共服务优待",
            "法律援助优待",
            "紧急救助服务",
            "办事便利服务",
            "消费优惠服务",
        ]
    }

    ///宜居环境
    pub fn livable_environment(&self) -> Vec<&'static str> {
        vec![
            "无障碍设施建设",
            "适老化改造支持",
            "老年友好社区",
            "老年活动场所",
            "老年健身设施",
            "安全出行保障",
            "居住环境改善",
            "公共设施适配",
        ]
    }

    /// 参与社会发展
    pub fn social_participation(&self) -> Vec<&'static str> {
        vec![
            "志愿服务参与",
            "文化传承参与",
            "社区治理参与",
            "关心下一代工作",
            "老年教育参与",
            "老年体育活动",
            "老年文化活动",
            "社会公益活动",
        ]
    }

    /// 违法行为
    pub fn protection_violations(&self) -> Vec<&'static str> {
        vec![
            "拒绝赡养行为",
            "虐待老年人行为",
            "遗弃老年人行为",
            "侵害财产权益",
            "侵害人身权益",
            "歧视老年人行为",
            "骗取老年人财物",
            "侵害养老服务权益",
        ]
    }
}

impl Default for ElderlyProtectionLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ElderlyProtectionLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("elderly_protection")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【老年人权益保障法规则】\n\n老年人权利:\n{}\n\n家庭赡养:\n{}\n\n社会保障:\n{}\n",
            self.elderly_rights().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.family_support().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.social_security().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elderly_protection_law_rules() {
        let rules = ElderlyProtectionLawRules::new();
        assert!(!rules.elderly_rights().is_empty());
        assert!(!rules.family_support().is_empty());
    }
}