World Rules - 世界规则库
一个收集各种规则的 Rust 库，包含真实的游戏算法、牌型识别和规则验证。
622 条规则，覆盖 6 大分类（游戏/体育/社交/科学/法律/健康）
831 个测试（788 单元 + 28 集成），clippy 零 warning
真实游戏算法：麻将胡牌判定、德州扑克牌型评估、斗地主牌型识别、中国象棋走子验证、五子棋胜负判定
simple_rule! 宏自动 生成 Rule trait + explain + 测试
CLI 工具 wr：list/show/stats/validate，支持 --json 输出
