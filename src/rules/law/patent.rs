//! 专利法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 专利法规则
pub struct PatentLawRules {
    metadata: RuleMetadata,
}

impl PatentLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "专利法规则",
                "中国专利法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "专利".into()]),
        }
    }

    /// 专利类型
    pub fn patent_types(&self) -> Vec<&'static str> {
        vec![
            "发明专利: 技术方案保护",
            "实用新型专利: 小发明保护",
            "外观设计专利: 产品外观保护",
            "发明专利保护20年",
            "实用新型保护10年",
            "外观设计保护15年",
            "保护期限自申请日起算",
            "年费缴纳维持专利权",
        ]
    }

    /// 专利授予条件
    pub fn patent_requirements(&self) -> Vec<&'static str> {
        vec![
            "新颖性: 未公开的技术",
            "创造性: 有实质性特点",
            "实用性: 可制造使用",
            "不属于现有技术",
            "不属于公知技术",
            "具有技术进步性",
            "能够产生积极效果",
            "符合产业政策",
        ]
    }

    /// 专利申请程序
    pub fn application_procedure(&self) -> Vec<&'static str> {
        vec![
            "提交申请: 提交申请文件",
            "初步审查: 形式审查",
            "公布申请: 发明专利公布",
            "实质审查: 发明实质审查",
            "请求实审: 申请实审请求",
            "授权公告: 专利授权公布",
            "申请优先权: 优先权制度",
            "申请驳回: 驳回处理程序",
        ]
    }

    /// 专利权内容
    pub fn patent_rights(&self) -> Vec<&'static str> {
        vec![
            "制造权: 制造专利产品",
            "使用权: 使用专利方法",
            "销售权: 销售专利产品",
            "许诺销售权: 销售承诺",
            "进口权: 进口专利产品",
            "许可权: 许可他人实施",
            "转让权: 转让专利权",
            "标记权: 专利标记权利",
        ]
    }

    /// 专利实施许可
    pub fn patent_license(&self) -> Vec<&'static str> {
        vec![
            "独占许可: 独家实施许可",
            "排他许可: 排他性许可",
            "普通许可: 一般实施许可",
            "强制许可: 强制许可制度",
            "许可合同: 许可协议签订",
            "许可登记: 许可合同备案",
            "许可费: 许可费用约定",
            "许可范围: 许可范围限定",
        ]
    }

    /// 专利侵权行为
    pub fn patent_infringement(&self) -> Vec<&'static str> {
        vec![
            "制造侵权: 侵权制造产品",
            "使用侵权: 侵权使用专利",
            "销售侵权: 侵权销售产品",
            "许诺销售侵权: 侵权销售承诺",
            "进口侵权: 侵权进口产品",
            "方法侵权: 侵权使用方法",
            "间接侵权: 辅助侵权行为",
            "等同侵权: 等同技术侵权",
        ]
    }

    /// 专利保护措施
    pub fn patent_protection(&self) -> Vec<&'static str> {
        vec![
            "侵权诉讼: 专利侵权诉讼",
            "行政处理: 海关边境保护",
            "临时禁令: 诉前禁令",
            "证据保全: 诉前证据保全",
            "赔偿计算: 损害赔偿计算",
            "惩罚性赔偿: 惩罚性赔偿",
            "举证责任: 举证责任分配",
            "无效宣告: 专利权无效",
        ]
    }

    /// 专利无效宣告
    pub fn invalidation_declaration(&self) -> Vec<&'static str> {
        vec![
            "无效请求: 无效申请程序",
            "无效理由: 不符合授权条件",
            "无效审查: 无效审查程序",
            "无效决定: 无效决定作出",
            "无效上诉: 无效决定上诉",
            "无效效力: 无效追溯效力",
            "无效反诉: 侵权诉讼中无效",
            "无效证据: 无效证据规则",
        ]
    }
}

impl Default for PatentLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for PatentLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("patent")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【专利法规则】\n\n专利类型:\n{}\n\n授予条件:\n{}\n\n专利权内容:\n{}\n",
            self.patent_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.patent_requirements().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.patent_rights().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patent_law_rules() {
        let rules = PatentLawRules::new();
        assert!(!rules.patent_types().is_empty());
        assert!(!rules.patent_requirements().is_empty());
    }
}