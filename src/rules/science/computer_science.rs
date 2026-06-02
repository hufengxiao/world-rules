//! 计算机科学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 计算机科学定律集合
pub struct ComputerScienceLaws {
    metadata: RuleMetadata,
}

impl ComputerScienceLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "计算机科学定律",
                "计算机科学基本定律和定理"
            )
            .with_origin("计算机科学")
            .with_tags(vec!["科学".into(), "计算机".into()]),
        }
    }

    /// 理论计算机科学定律
    pub fn theory_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("图灵机理论", "可计算性", "图灵机定义可计算函数"),
            ("邱奇-图灵论题", "可计算性等价", "所有有效计算方法等价于图灵机"),
            ("停机问题", "不可判定", "不存在判定程序是否会停机的算法"),
            ("P与NP问题", "复杂性类", "多项式时间内可判定与可验证"),
            ("NP完全理论", "最难NP问题", "NP完全问题是NP问题中最难的"),
            ("哥德尔不完备定理", "形式系统局限", "任何足够强的形式系统都不完备"),
            ("递归定理", "自引用", "程序可以引用自己的代码"),
            ("莱斯定理", "程序性质不可判定", "非平凡程序性质不可判定"),
        ]
    }

    /// 算法复杂度定律
    pub fn complexity_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("大O表示法", "O(n)增长率", "算法时间复杂度上界"),
            ("排序算法下界", "O(n log n)", "比较排序最优复杂度"),
            ("哈希表碰撞", "O(1)平均", "哈希表期望O(1)查找"),
            ("二分搜索", "O(log n)", "有序数组搜索复杂度"),
            ("搜索空间爆炸", "NP-hard", "组合搜索问题复杂"),
            ("摊还分析", "平均复杂度", "多次操作的平均复杂度"),
        ]
    }

    /// 信息理论定律
    pub fn information_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("香农信息熵", "H = -Σp log p", "信息不确定性度量"),
            ("信源编码定理", "压缩极限", "无损压缩的理论极限"),
            ("信道编码定理", "传输极限", "可靠通信的理论极限"),
            ("互信息", "I(X;Y)", "两个变量的信息关联"),
            ("柯尔莫哥洛夫复杂度", "最短描述", "对象的最短程序描述"),
        ]
    }

    /// 编程定律
    pub fn programming_laws(&self) -> Vec<&'static str> {
        vec![
            "摩尔定律: 晶体管数量每两年翻倍",
            "阿姆达尔定律: 并行加速极限",
            "布鲁克斯定律: 增加人力延迟软件",
            "康威定律: 软件结构反映组织结构",
            "霍夫曼定律: 软件维护成本",
            "零一定律: 二进制逻辑基础",
            "Wirth定律: 软件变慢比硬件变快",
            "祖尔定律: 复杂性增长",
        ]
    }

    /// 计算机原理
    pub fn principles(&self) -> Vec<&'static str> {
        vec![
            "冯·诺依曼架构",
            "存储程序概念",
            "二进制运算",
            "指令周期",
            "输入输出",
            "存储层次",
            "缓存原理",
            "虚拟化技术",
        ]
    }

    /// 算法复杂度
    pub fn algorithm_complexity(&self) -> Vec<&'static str> {
        vec![
            "大O表示法: 描述算法时间或空间复杂度渐近上界",
            "P vs NP: P类问题是否等于NP类问题是计算机科学核心开放问题",
            "NP完全问题: NP类中最难的一类问题",
            "分治法: 将问题分解为更小子问题递归求解再合并",
            "动态规划: 利用重叠子问题和最优子结构高效求解",
            "贪心算法: 每步选择局部最优解期望得到全局最优",
            "回溯法: 系统搜索解空间树并在不满足条件时回退",
        ]
    }

    /// 计算理论
    pub fn computation_theory(&self) -> Vec<&'static str> {
        vec![
            "图灵机: 理想化的抽象计算模型",
            "停机问题: 不存在通用算法判定任意程序是否会终止",
            "丘奇图灵论题: 可计算函数等价于图灵可计算函数",
            "正则语言: 有限自动机识别的语言类",
            "上下文无关语言: 下推自动机识别的语言类",
            "时间层次定理: 更多计算时间能解决更多问题",
            "空间层次定理: 更多存储空间能解决更多问题",
        ]
    }


    /// 分布式系统
    pub fn distributed_systems(&self) -> Vec<&'static str> {
        vec![
            "CAP定理: 一致性可用性和分区容错性不可兼得",
            "一致性哈希: 分布式系统中数据均匀分布的方法",
            "Raft共识: 易于理解的分布式共识算法",
            "Paxos: 经典的分布式共识算法",
            "分布式事务: 跨多个节点的原子事务",
            "微服务: 将应用拆分为独立部署的小服务",
            "消息队列: 异步通信的中间件",
        ]
    }

}

impl Default for ComputerScienceLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ComputerScienceLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("computer_science")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【计算机科学定律】\n\n理论定律:\n{}\n\n复杂度定律:\n{}\n\n信息理论:\n{}\n\n编程定律:\n{}\n",
            self.theory_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.complexity_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.information_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.programming_laws().iter()
                .map(|law| format!("▶ {}", law))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_computer_science_laws() {
        let laws = ComputerScienceLaws::new();
        assert!(!laws.theory_laws().is_empty());
    }
}