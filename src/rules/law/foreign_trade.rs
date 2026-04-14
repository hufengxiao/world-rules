//! 对外贸易法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 对外贸易法规则
pub struct ForeignTradeLawRules {
    metadata: RuleMetadata,
}

impl ForeignTradeLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "对外贸易法规则",
                "中国对外贸易法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "对外贸易".into()]),
        }
    }

    /// 贸易方式
    pub fn trade_methods(&self) -> Vec<&'static str> {
        vec![
            "货物进出口: 一般货物贸易",
            "技术进出口: 技术贸易管理",
            "服务贸易: 国际服务贸易",
            "加工贸易: 加工装配贸易",
            "易货贸易: 对等交换贸易",
            "补偿贸易: 产品补偿贸易",
            "边境贸易: 边境小额贸易",
            "电子商务贸易: 网络贸易",
        ]
    }

    /// 贸易经营者
    pub fn trade_operators(&self) -> Vec<&'static str> {
        vec![
            "外贸经营者备案: 经营者登记",
            "外贸经营权: 经营权利取得",
            "国有外贸企业: 国营贸易企业",
            "民营外贸企业: 民营贸易主体",
            "外资外贸企业: 外商投资贸易",
            "外贸代理: 外贸代理制度",
            "经营者资质: 资质条件要求",
            "经营者义务: 贸易合规义务",
        ]
    }

    /// 进出口管理
    pub fn trade_management(&self) -> Vec<&'static str> {
        vec![
            "进出口许可: 许可审批制度",
            "配额管理: 配额分配制度",
            "许可证管理: 许可证申请",
            "自动许可: 自动许可登记",
            "禁止进出口: 禁止贸易商品",
            "限制进出口: 限制贸易管理",
            "关税配额: 关税配额管理",
            "贸易目录: 贸易商品目录",
        ]
    }

    /// 贸易救济措施
    pub fn trade_remedy(&self) -> Vec<&'static str> {
        vec![
            "反倾销措施: 反倾销调查征税",
            "反补贴措施: 反补贴调查征税",
            "保障措施: 保障措施调查",
            "调查程序: 救济调查程序",
            "临时措施: 救济临时措施",
            "征税措施: 最终征税措施",
            "复审制度: 措施复审程序",
            "争端解决: WTO争端解决",
        ]
    }

    /// 关税制度
    pub fn tariff_system(&self) -> Vec<&'static str> {
        vec![
            "进口关税: 进口关税征收",
            "出口关税: 出口关税征收",
            "关税税率: 税率设置规定",
            "关税减免: 减免税优惠政策",
            "保税制度: 保税区保税仓库",
            "关税配额: 关税配额税率",
            "关税估价: 完税价格确定",
            "关税征收: 征收程序规定",
        ]
    }

    /// 贸易促进
    pub fn trade_promotion(&self) -> Vec<&'static str> {
        vec![
            "出口退税: 出口税收退还",
            "出口信贷: 出口信用贷款",
            "出口信用保险: 出口保险支持",
            "贸易促进机构: 促进服务机构",
            "国际展会: 展览促销活动",
            "信息服务: 贸易信息支持",
            "培训服务: 贸易培训服务",
            "法律服务: 贸易法律支持",
        ]
    }

    /// 原产地规则
    pub fn origin_rules(&self) -> Vec<&'static str> {
        vec![
            "原产地确定: 商品产地认定",
            "原产地证书: 原产证明文件",
            "优惠原产地: 特惠原产待遇",
            "非优惠原产地: 一般原产规则",
            "实质性改变标准",
            "增值标准: 增值百分比",
            "加工工序标准: 工序要求",
            "原产地签证: 原产证签发",
        ]
    }

    /// 贸易管制
    pub fn trade_control(&self) -> Vec<&'static str> {
        vec![
            "商品检验: 进出口检验",
            "质量认证: 质量认证制度",
            "动植物检疫: 检疫检验制度",
            "卫生检疫: 卫生安全检验",
            "知识产权保护: 边境保护措施",
            "国家安全审查: 安全审查制度",
            "环保标准: 环境保护要求",
            "技术标准: 技术标准规范",
        ]
    }
}

impl Default for ForeignTradeLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ForeignTradeLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("foreign_trade")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【对外贸易法规则】\n\n贸易方式:\n{}\n\n进出口管理:\n{}\n\n贸易救济:\n{}\n",
            self.trade_methods().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.trade_management().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.trade_remedy().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foreign_trade_law_rules() {
        let rules = ForeignTradeLawRules::new();
        assert!(!rules.trade_methods().is_empty());
        assert!(!rules.trade_management().is_empty());
    }
}