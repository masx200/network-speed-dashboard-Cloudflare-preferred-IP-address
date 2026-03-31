# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/1 16:55:23
- **数据来源**: connectivity_results.json
- **总测试数**: 412
- **失败测试数**: 6
- **成功测试数**: 406
- **失败率**: 1.46%
- **平均延迟**: 478.50ms
- **最小延迟**: 124ms
- **最大延迟**: 2885ms

---

## 失败测试详情

### 错误类型统计

- **未知错误**: 1 次
- **连接超时**: 5 次

### 失败测试列表

| 序号 | 主机/域名            | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器     | 错误信息                                  |
| ---- | -------------------- | --------------- | ------ | ---- | ------ | -------- | ---------- | ----------------------------------------- |
| 93   | yx-auto.pages.dev    | 192.133.11.1    | IPv4   | h3   | 403    | 754      | cloudflare | No error message                          |
| 357  | ct.877774.xyz        | ct.877774.xyz   | IPv4   | none | N/A    | 0        | N/A        | dial tcp ct.877774.xyz:443: i/o timeout   |
| 392  | cf.877774.xyz        | cf.877774.xyz   | IPv4   | none | N/A    | 0        | N/A        | dial tcp cf.877774.xyz:443: i/o timeout   |
| 393  | trevor.ns.cloudfl... | 108.162.195.154 | IPv4   | none | N/A    | 0        | N/A        | dial tcp 108.162.195.154:443: i/o timeout |
| 402  | cfip.xxxxxxxx.tk     | 104.20.255.53   | IPv4   | none | N/A    | 0        | N/A        | dial tcp 104.20.255.53:443: i/o timeout   |
| 406  | cfip.xxxxxxxx.tk     | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A        | dial tcp 198.41.212.130:443: i/o timeout  |

---

## 🚀 延迟最低的 50 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 | | ----
| --------- | ------ | ------ | ---- | ---- | -------- | ------ || 12 |
sullivan.ns.cloud... | 162.159.44.161 | IPv4 | h3 | ✅ 成功 | 124 | cloudflare |
| 95 | moura.ns.cloudfla... | 162.159.44.217 | IPv4 | h3 | ✅ 成功 | 124 |
cloudflare | | 22 | cris.ns.cloudflar... | 162.159.44.202 | IPv4 | h3 | ✅ 成功
| 128 | cloudflare | | 295 | ct.877774.xyz | 172.64.229.195 | IPv4 | h3 | ✅
成功 | 128 | cloudflare | | 177 | huxley.ns.cloudfl... | 162.159.44.188 | IPv4 |
h3 | ✅ 成功 | 133 | cloudflare | | 206 | julio.ns.cloudfla... | 162.159.44.209
| IPv4 | h3 | ✅ 成功 | 138 | cloudflare | | 298 | ct.877774.xyz | 172.64.229.44
| IPv4 | h3 | ✅ 成功 | 138 | cloudflare | | 116 | 162.159.38.89 | 162.159.38.89
| IPv4 | h3 | ✅ 成功 | 143 | cloudflare | | 296 | ct.877774.xyz |
172.64.229.217 | IPv4 | h3 | ✅ 成功 | 145 | cloudflare | | 100 | 172.64.229.249
| 172.64.229.249 | IPv4 | h3 | ✅ 成功 | 149 | cloudflare | | 70 |
dylan.ns.cloudfla... | 162.159.44.187 | IPv4 | h3 | ✅ 成功 | 150 | cloudflare |
| 322 | craig.ns.cloudfla... | 162.159.44.192 | IPv4 | h3 | ✅ 成功 | 151 |
cloudflare | | 293 | ct.877774.xyz | 172.64.229.174 | IPv4 | h3 | ✅ 成功 | 157
| cloudflare | | 255 | damien.ns.cloudfl... | 162.159.44.168 | IPv4 | h3 | ✅
成功 | 161 | cloudflare | | 196 | ashton.ns.cloudfl... | 162.159.44.173 | IPv4 |
h3 | ✅ 成功 | 162 | cloudflare | | 297 | ct.877774.xyz | 172.64.229.236 | IPv4
| h3 | ✅ 成功 | 162 | cloudflare | | 28 | braden.ns.cloudfl... | 162.159.44.169
| IPv4 | h3 | ✅ 成功 | 163 | cloudflare | | 294 | ct.877774.xyz |
172.64.229.185 | IPv4 | h3 | ✅ 成功 | 163 | cloudflare | | 299 | ct.877774.xyz
| 172.64.229.161 | IPv4 | h3 | ✅ 成功 | 163 | cloudflare | | 394 |
trevor.ns.cloudfl... | 162.159.44.154 | IPv4 | h3 | ✅ 成功 | 167 | cloudflare |
| 261 | wilson.ns.cloudfl... | 162.159.44.110 | IPv4 | h3 | ✅ 成功 | 173 |
cloudflare | | 8 | 162.159.44.208 | 162.159.44.208 | IPv4 | h3 | ✅ 成功 | 174 |
cloudflare | | 76 | 162.159.45.170 | 162.159.45.170 | IPv4 | h3 | ✅ 成功 | 179
| cloudflare | | 88 | lewis.ns.cloudfla... | 162.159.44.159 | IPv4 | h3 | ✅
成功 | 179 | cloudflare | | 133 | decker.ns.cloudfl... | 162.159.44.155 | IPv4 |
h3 | ✅ 成功 | 179 | cloudflare | | 168 | 162.159.39.118 | 162.159.39.118 | IPv4
| h3 | ✅ 成功 | 180 | cloudflare | | 118 | pranab.ns.cloudfl... |
162.159.44.199 | IPv4 | h3 | ✅ 成功 | 182 | cloudflare | | 230 |
kyree.ns.cloudfla... | 162.159.44.207 | IPv4 | h3 | ✅ 成功 | 185 | cloudflare |
| 45 | 108.162.198.54 | 108.162.198.54 | IPv4 | h3 | ✅ 成功 | 191 | cloudflare
| | 64 | abdullah.ns.cloud... | 162.159.44.203 | IPv4 | h3 | ✅ 成功 | 191 |
cloudflare | | 326 | craig.ns.cloudfla... | 2a06:98c1:50::ac40:23c0 | IPv6 | h3
| ✅ 成功 | 193 | cloudflare | | 242 | otto.ns.cloudflar... | 162.159.44.135 |
IPv4 | h3 | ✅ 成功 | 199 | cloudflare | | 32 | braden.ns.cloudfl... |
2a06:98c1:50::ac40:23a9 | IPv6 | h3 | ✅ 成功 | 200 | cloudflare | | 209 |
julio.ns.cloudfla... | 2803:f800:50::6ca2:c3d1 | IPv6 | h3 | ✅ 成功 | 201 |
cloudflare | | 41 | bowen.ns.cloudfla... | 172.64.35.83 | IPv4 | h3 | ✅ 成功 |
202 | cloudflare | | 136 | decker.ns.cloudfl... | 2803:f800:50::6ca2:c39b | IPv6
| h3 | ✅ 成功 | 203 | cloudflare | | 173 | uriah.ns.cloudfla... |
2606:4700:58::a29f:2cc2 | IPv6 | h3 | ✅ 成功 | 203 | cloudflare | | 200 |
ashton.ns.cloudfl... | 2a06:98c1:50::ac40:23ad | IPv6 | h3 | ✅ 成功 | 203 |
cloudflare | | 300 | ct.877774.xyz | 172.64.229.173 | IPv4 | h3 | ✅ 成功 | 203
| cloudflare | | 397 | trevor.ns.cloudfl... | 2803:f800:50::6ca2:c39a | IPv6 |
h3 | ✅ 成功 | 203 | cloudflare | | 34 | rustam.ns.cloudfl... | 162.159.44.148 |
IPv4 | h3 | ✅ 成功 | 207 | cloudflare | | 23 | cris.ns.cloudflar... |
172.64.35.202 | IPv4 | h3 | ✅ 成功 | 209 | cloudflare | | 119 |
pranab.ns.cloudfl... | 172.64.35.199 | IPv4 | h3 | ✅ 成功 | 210 | cloudflare |
| 15 | sullivan.ns.cloud... | 2803:f800:50::6ca2:c3a1 | IPv6 | h3 | ✅ 成功 |
212 | cloudflare | | 263 | wilson.ns.cloudfl... | 2606:4700:58::a29f:2c6e | IPv6
| h3 | ✅ 成功 | 214 | cloudflare | | 38 | rustam.ns.cloudfl... |
2a06:98c1:50::ac40:2394 | IPv6 | h3 | ✅ 成功 | 215 | cloudflare | | 254 |
damien.ns.cloudfl... | 108.162.195.168 | IPv4 | h3 | ✅ 成功 | 215 | cloudflare
| | 24 | cris.ns.cloudflar... | 2606:4700:58::a29f:2cca | IPv6 | h3 | ✅ 成功 |
216 | cloudflare | | 344 | benedict.ns.cloud... | 2803:f800:50::6ca2:c3cd | IPv6
| h3 | ✅ 成功 | 217 | cloudflare | | 174 | uriah.ns.cloudfla... |
2803:f800:50::6ca2:c3c2 | IPv6 | h3 | ✅ 成功 | 219 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 0 条记录
- **正常 (100-200ms)**: 32 条记录
- **慢 (200-500ms)**: 18 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 6 次
- **IPv6 失败**: 0 次

### 按协议统计

- **h3**: 1 次失败
- **none**: 5 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
