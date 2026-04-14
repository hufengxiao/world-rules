//! 环境法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 环境法规则
pub struct EnvironmentalLawRules {
    metadata: RuleMetadata,
}

impl EnvironmentalLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "环境法规则",
                "中国环境法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "环境法".into()]),
        }
    }

    /// 环境保护原则
    pub fn environmental_principles(&self) -> Vec<&'static str> {
        vec![
            "预防为主原则: 预防环境污染",
            "综合治理原则: 综合防治污染",
            "公众参与原则: 公众参与环保",
            "损害担责原则: 污染者承担责任",
            "可持续发展原则: 生态环境保护",
            "保护优先原则: 生态优先保护",
            "污染者付费原则: 污染成本承担",
            "协同治理原则: 多部门协同",
        ]
    }

    /// 环境污染防治
    pub fn pollution_control(&self) -> Vec<&'static str> {
        vec![
            "大气污染防治: 控制空气污染",
            "水污染防治: 保护水资源",
            "土壤污染防治: 保护土壤环境",
            "固体废物污染防治: 垃圾处理",
            "噪声污染防治: 噪音控制",
            "放射性污染防治: 核安全",
            "电磁辐射污染防治",
            "光污染防治",
        ]
    }

    /// 环境许可制度
    pub fn environmental_permit(&self) -> Vec<&'static str> {
        vec![
            "环境影响评价: 项目环评",
            "排污许可: 排污许可证",
            "危险废物经营许可",
            "放射性同位素许可",
            "海洋倾废许可",
            "建设项目环保审批",
            "环保竣工验收",
            "排污总量控制",
        ]
    }

    /// 环境监测制度
    pub fn environmental_monitoring(&self) -> Vec<&'static str> {
        vec![
            "环境质量监测: 空气水质土壤",
            "污染源监测: 排污监测",
            "应急监测: 突发污染监测",
            "自动监测: 在线监控",
            "监测信息公开",
            "监测数据真实性",
            "监测网络建设",
            "第三方监测服务",
        ]
    }

    /// 环境标准体系
    pub fn environmental_standards(&self) -> Vec<&'static str> {
        vec![
            "环境质量标准: 空气水质标准",
            "污染物排放标准: 排放限值",
            "环境监测方法标准",
            "环境标准样品标准",
            "环境基础标准",
            "国家环境标准",
            "地方环境标准",
            "行业环境标准",
        ]
    }

    /// 环境法律责任
    pub fn environmental_liability(&self) -> Vec<&'static str> {
        vec![
            "环境行政处罚: 警告罚款责令停产",
            "环境民事责任: 污染损害赔偿",
            "环境刑事责任: 重大污染入刑",
            "环境公益诉讼: 公益诉讼主体",
            "环境侵权责任: 无过错责任",
            "连带责任: 多污染者连带",
            "惩罚性赔偿: 故意污染",
            "生态环境损害赔偿",
        ]
    }

    /// 生态保护制度
    pub fn ecological_protection(&self) -> Vec<&'static str> {
        vec![
            "自然保护区制度",
            "生态红线划定",
            "生物多样性保护",
            "湿地保护",
            "森林保护",
            "草原保护",
            "海洋生态保护",
            "水土保持",
        ]
    }

    /// 资源保护制度
    pub fn resource_protection(&self) -> Vec<&'static str> {
        vec![
            "水资源保护: 水法规定",
            "土地资源保护: 土地管理",
            "矿产资源保护: 矿产开采",
            "森林资源保护: 林业法规",
            "草原资源保护: 草原管理",
            "渔业资源保护: 渔业法规",
            "野生动物保护: 野生动物法",
            "能源资源保护: 能源法规",
        ]
    }
}

impl Default for EnvironmentalLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for EnvironmentalLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("environmental")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【环境法规则】\n\n环保原则:\n{}\n\n污染防治:\n{}\n\n环境许可:\n{}\n",
            self.environmental_principles().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.pollution_control().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.environmental_permit().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environmental_law_rules() {
        let rules = EnvironmentalLawRules::new();
        assert!(!rules.environmental_principles().is_empty());
        assert!(!rules.pollution_control().is_empty());
    }
}