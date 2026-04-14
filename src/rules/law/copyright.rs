//! 著作权法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 著作权法规则
pub struct CopyrightLawRules {
    metadata: RuleMetadata,
}

impl CopyrightLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "著作权法规则",
                "中国著作权法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "著作权".into()]),
        }
    }

    /// 著作权客体
    pub fn copyright_objects(&self) -> Vec<&'static str> {
        vec![
            "文字作品: 文学艺术创作",
            "口述作品: 演讲授课内容",
            "音乐作品: 歌曲乐曲创作",
            "戏剧作品: 戏剧曲艺作品",
            "舞蹈作品: 舞蹈编排创作",
            "美术作品: 绘画雕塑作品",
            "摄影作品: 摄影艺术作品",
            "建筑作品: 建筑设计作品",
        ]
    }

    /// 著作权权利
    pub fn copyright_rights(&self) -> Vec<&'static str> {
        vec![
            "发表权: 公开发表权利",
            "署名权: 作者署名权利",
            "修改权: 作品修改权利",
            "保护作品完整权",
            "复制权: 作品复制权利",
            "发行权: 作品发行权利",
            "出租权: 作品出租权利",
            "展览权: 作品展览权利",
            "表演权: 作品表演权利",
            "放映权: 作品放映权利",
            "广播权: 作品广播权利",
            "信息网络传播权",
            "摄制权: 作品摄制权利",
            "改编权: 作品改编权利",
            "翻译权: 作品翻译权利",
            "汇编权: 作品汇编权利",
        ]
    }

    /// 著作权保护期限
    pub fn protection_period(&self) -> Vec<&'static str> {
        vec![
            "作者终生及死后50年",
            "合作作品保护50年",
            "法人作品保护50年",
            "电影作品保护50年",
            "摄影作品保护50年",
            "发表权保护50年",
            "署名权永久保护",
            "修改权永久保护",
        ]
    }

    /// 著作权归属
    pub fn copyright_ownership(&self) -> Vec<&'static str> {
        vec![
            "作者享有著作权",
            "合作作品共有著作权",
            "职务作品著作权归属",
            "委托作品著作权约定",
            "法人作品著作权归属",
            "汇编作品著作权归属",
            "演绎作品著作权归属",
            "合作作品权利行使",
        ]
    }

    /// 著作权许可
    pub fn copyright_license(&self) -> Vec<&'static str> {
        vec![
            "许可合同: 许可协议签订",
            "许可方式: 许可种类选择",
            "许可期限: 许可期限约定",
            "许可地域: 许可范围限定",
            "许可费用: 许可报酬约定",
            "独占许可: 独家许可使用",
            "排他许可: 排他许可使用",
            "普通许可: 一般许可使用",
        ]
    }

    /// 著作权侵权
    pub fn copyright_infringement(&self) -> Vec<&'static str> {
        vec![
            "抄袭侵权: 抄袭他人作品",
            "复制侵权: 未经许可复制",
            "发行侵权: 未经许可发行",
            "传播侵权: 未经许可传播",
            "改编侵权: 未经许可改编",
            "翻译侵权: 未经许可翻译",
            "署名侵权: 署名权侵犯",
            "网络侵权: 网络传播侵权",
        ]
    }

    /// 合理使用
    pub fn fair_use(&self) -> Vec<&'static str> {
        vec![
            "个人学习使用",
            "介绍评论引用",
            "新闻报道使用",
            "课堂教学使用",
            "科学研究使用",
            "图书馆馆藏使用",
            "免费表演使用",
            "公务使用: 国家机关使用",
        ]
    }

    /// 著作权保护措施
    pub fn protection_measures(&self) -> Vec<&'static str> {
        vec![
            "侵权诉讼: 著作权诉讼",
            "行政保护: 行政执法保护",
            "证据保全: 诉前证据保全",
            "临时禁令: 诉前禁令措施",
            "赔偿计算: 损害赔偿计算",
            "惩罚性赔偿: 惩罚性赔偿",
            "技术措施保护",
            "权利管理信息保护",
        ]
    }
}

impl Default for CopyrightLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CopyrightLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("copyright")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【著作权法规则】\n\n著作权客体:\n{}\n\n著作权权利:\n{}\n\n保护期限:\n{}\n",
            self.copyright_objects().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.copyright_rights().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.protection_period().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copyright_law_rules() {
        let rules = CopyrightLawRules::new();
        assert!(!rules.copyright_objects().is_empty());
        assert!(!rules.copyright_rights().is_empty());
    }
}