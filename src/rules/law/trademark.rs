//! 商标法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 商标法规则
pub struct TrademarkLawRules {
    metadata: RuleMetadata,
}

impl TrademarkLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "商标法规则",
                "中国商标法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "商标".into()]),
        }
    }

    /// 商标类型
    pub fn trademark_types(&self) -> Vec<&'static str> {
        vec![
            "商品商标: 商品标记",
            "服务商标: 服务标记",
            "集体商标: 团体成员商标",
            "证明商标: 证明品质商标",
            "注册商标: 注册保护商标",
            "未注册商标: 未注册使用",
            "驰名商标: 知名商标保护",
            "地理标志商标: 地域产品商标",
        ]
    }

    /// 商标注册条件
    pub fn registration_conditions(&self) -> Vec<&'static str> {
        vec![
            "显著性: 商标识别性",
            "合法性: 符合法律规定",
            "非功能性: 不为功能设计",
            "不与在先权利冲突",
            "不与他人商标相同近似",
            "不违反禁用条款",
            "不属于通用名称",
            "不欺骗误导消费者",
        ]
    }

    /// 商标注册程序
    pub fn registration_procedure(&self) -> Vec<&'static str> {
        vec![
            "商标申请: 提交注册申请",
            "形式审查: 申请形式审查",
            "实质审查: 商标实质审查",
            "初审公告: 初审公告程序",
            "异议程序: 商标异议程序",
            "核准注册: 注册核准发证",
            "注册公告: 注册公告公布",
            "驳回复审: 驳回申请复审",
        ]
    }

    /// 商标权内容
    pub fn trademark_rights(&self) -> Vec<&'static str> {
        vec![
            "专用权: 独占使用权利",
            "许可权: 许可他人使用",
            "转让权: 转让商标权",
            "续展权: 注册续展权利",
            "禁止权: 禁止侵权使用",
            "标记权: 商标标记权利",
            "质押权: 商标质押融资",
            "诉讼权: 侵权诉讼权利",
        ]
    }

    /// 商标保护期限
    pub fn protection_period(&self) -> Vec<&'static str> {
        vec![
            "注册有效期10年",
            "续展注册有效期10年",
            "续展申请期限: 有效期前12个月内",
            "宽展期: 有效期后6个月",
            "无限续展次数",
            "未续展商标注销",
            "使用证据保存",
            "注册商标管理",
        ]
    }

    /// 商标侵权行为
    pub fn trademark_infringement(&self) -> Vec<&'static str> {
        vec![
            "使用侵权: 侵权使用商标",
            "销售侵权: 销售侵权商品",
            "伪造商标: 伪造注册商标",
            "销售伪造商标标识",
            "反向假冒: 撤换商标标识",
            "帮助侵权: 辅助侵权行为",
            "驰名商标淡化侵权",
            "网络侵权: 网络商标侵权",
        ]
    }

    /// 驰名商标保护
    pub fn well_known_protection(&self) -> Vec<&'static str> {
        vec![
            "驰名商标认定: 驰名认定程序",
            "跨类保护: 类别保护扩展",
            "禁止混淆: 防止商标混淆",
            "禁止淡化: 防止商标淡化",
            "认定标准: 驰名认定因素",
            "认定效力: 驰名保护效力",
            "国际保护: 驰名国际保护",
            "证据要求: 驰名证明材料",
        ]
    }

    /// 商标使用义务
    pub fn trademark_use_obligation(&self) -> Vec<&'static str> {
        vec![
            "注册后使用义务",
            "三年不使用撤销制度",
            "使用证据保存要求",
            "正当使用义务",
            "不改变商标标识",
            "不超出核定范围",
            "许可使用备案",
            "转让使用证明",
        ]
    }
}

impl Default for TrademarkLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for TrademarkLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("trademark")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【商标法规则】\n\n商标类型:\n{}\n\n注册条件:\n{}\n\n商标权内容:\n{}\n",
            self.trademark_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.registration_conditions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.trademark_rights().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trademark_law_rules() {
        let rules = TrademarkLawRules::new();
        assert!(!rules.trademark_types().is_empty());
        assert!(!rules.registration_conditions().is_empty());
    }
}