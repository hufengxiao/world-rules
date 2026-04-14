//! 反垄断法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 反垄断法规则
pub struct AntimonopolyLawRules {
    metadata: RuleMetadata,
}

impl AntimonopolyLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "反垄断法规则",
                "中国反垄断法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "反垄断".into()]),
        }
    }

    /// 垄断行为类型
    pub fn monopoly_types(&self) -> Vec<&'static str> {
        vec![
            "垄断协议: 经营者联合垄断",
            "滥用市场支配地位: 单独垄断行为",
            "经营者集中: 企业并购集中",
            "行政性垄断: 行政权力滥用",
            "横向垄断协议: 同业竞争者协议",
            "纵向垄断协议: 上下游协议",
            "轴辐协议: 轴辐式垄断",
            "共同滥用: 共同市场支配",
        ]
    }

    /// 垄断协议规则
    pub fn monopoly_agreements(&self) -> Vec<&'static str> {
        vec![
            "固定价格协议: 禁止价格垄断",
            "限制产量协议: 禁止产量限制",
            "分割市场协议: 禁止市场分割",
            "联合抵制协议: 禁止联合抵制",
            "转售价格维持: 禁止限价",
            "排他性协议: 禁止排他交易",
            "豁免情形: 特定协议豁免",
            "宽大制度: 自首宽大处理",
        ]
    }

    /// 滥用市场支配地位
    pub fn market_dominance_abuse(&self) -> Vec<&'static str> {
        vec![
            "市场支配地位认定: 市场份额标准",
            "不公平高价: 禁止过高定价",
            "不公平低价: 禁止掠夺定价",
            "拒绝交易: 禁止拒绝交易",
            "限定交易: 禁止限定交易",
            "搭售行为: 禁止搭售捆绑",
            "差别待遇: 禁止歧视对待",
            "正当理由例外: 合理行为例外",
        ]
    }

    /// 经营者集中规则
    pub fn business_concentration(&self) -> Vec<&'static str> {
        vec![
            "经营者合并: 企业合并行为",
            "经营者收购: 企业收购行为",
            "经营者控制: 企业控制行为",
            "申报标准: 申报营业额标准",
            "申报程序: 集中申报程序",
            "审查程序: 集中审查程序",
            "禁止集中: 禁止反竞争集中",
            "附加条件集中: 条件批准集中",
        ]
    }

    /// 行政性垄断规则
    pub fn administrative_monopoly(&self) -> Vec<&'static str> {
        vec![
            "禁止地区封锁: 禁止地方保护",
            "禁止行业垄断: 禁止行业壁垒",
            "禁止强制交易: 灭绝强制买卖",
            "禁止限制竞争: 禁止限制经营",
            "禁止歧视待遇: 禁止差别对待",
            "禁止排他性规定: 禁止排他条款",
            "公平竞争审查: 政策公平审查",
            "行政垄断监督: 监督处理机制",
        ]
    }

    /// 反垄断执法机构
    pub fn enforcement_authorities(&self) -> Vec<&'static str> {
        vec![
            "市场监管总局执法",
            "省级执法机构",
            "行业监管配合",
            "执法协作机制",
            "调查权限: 调查取证",
            "处罚权限: 行政处罚",
            "裁决权限: 案件裁决",
            "复议权限: 行政复议",
        ]
    }

    /// 反垄断法律责任
    pub fn antimonopoly_liability(&self) -> Vec<&'static str> {
        vec![
            "行政责任: 罚款责令停止",
            "民事责任: 损害赔偿责任",
            "刑事责任: 情节严重入刑",
            "罚款标准: 上一年销售额1%-10%",
            "没收违法所得: 违法所得没收",
            "责令停止违法行为",
            "责令改正违法行为",
            "解除垄断协议",
        ]
    }

    /// 反垄断豁免
    pub fn antimonopoly_exemptions(&self) -> Vec<&'static str> {
        vec![
            "技术进步豁免: 技术改进协议",
            "质量效率豁免: 提高质量协议",
            "中小企业豁免: 小企业合作",
            "公共利益豁免: 公共利益协议",
            "出口豁免: 出口贸易协议",
            "安全生产豁免: 安全生产协议",
            "环保豁免: 环境保护协议",
            "豁免条件: 证明豁免条件",
        ]
    }
}

impl Default for AntimonopolyLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AntimonopolyLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("antimonopoly")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【反垄断法规则】\n\n垄断类型:\n{}\n\n滥用支配地位:\n{}\n\n经营者集中:\n{}\n",
            self.monopoly_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.market_dominance_abuse().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.business_concentration().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_antimonopoly_law_rules() {
        let rules = AntimonopolyLawRules::new();
        assert!(!rules.monopoly_types().is_empty());
        assert!(!rules.monopoly_agreements().is_empty());
    }
}