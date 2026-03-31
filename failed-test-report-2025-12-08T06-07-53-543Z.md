# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/8 14:07:53
- **数据来源**: connectivity_results.json
- **总测试数**: 481
- **失败测试数**: 16
- **成功测试数**: 465
- **失败率**: 3.33%
- **平均延迟**: 504.05ms
- **最小延迟**: 116ms
- **最大延迟**: 6039ms

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: I/O超时**: 14 次 (87.5%)
- **DNS解析错误: 其他DNS错误**: 2 次 (12.5%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (14 次测试)

| 序号 | 主机/域名                | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                  |
| ---- | ------------------------ | --------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------- |
| 192  | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout |
| 238  | 121.188.182.190          | 121.188.182.190 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 121.188.182.190:443: i/o timeout |
| 245  | ct.877774.xyz            | ct.877774.xyz   | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout   |
| 276  | 52.76.110.129            | 52.76.110.129   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 52.76.110.129:443: i/o timeout   |
| 308  | 3.0.50.69                | 3.0.50.69       | IPv4   | none | N/A    | 0        | N/A    | dial tcp 3.0.50.69:443: i/o timeout       |
| 360  | cfip.xxxxxxxx.tk         | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout  |
| 405  | 175.212.207.13           | 175.212.207.13  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 175.212.207.13:443: i/o timeout  |
| 464  | 222.105.131.225          | 222.105.131.225 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 222.105.131.225:443: i/o timeout |
| 471  | 119.194.220.146          | 119.194.220.146 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 119.194.220.146:443: i/o timeout |
| 477  | cf.877774.xyz            | cf.877774.xyz   | IPv4   | none | N/A    | 0        | N/A    | dial tcp cf.877774.xyz:443: i/o timeout   |
| 478  | 115.22.115.218           | 115.22.115.218  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 115.22.115.218:443: i/o timeout  |
| 479  | 141.147.185.63           | 141.147.185.63  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 141.147.185.63:443: i/o timeout  |
| 480  | 138.2.18.82              | 138.2.18.82     | IPv4   | none | N/A    | 0        | N/A    | dial tcp 138.2.18.82:443: i/o timeout     |
| 481  | 111.171.108.67           | 111.171.108.67  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 111.171.108.67:443: i/o timeout  |

#### DNS解析错误: 其他DNS错误 (2 次测试)

| 序号 | 主机/域名 | 目标IP            | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                                                                                                                                                                                   |
| ---- | --------- | ----------------- | ------ | ---- | ------ | -------- | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 433  | ping0.cc  | 104.218.235.243   | IPv4   | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": tls: failed to verify certificate: x509: certificate is valid for intl.ping0.cc, ipv4.ping0.cc, ipv6.ping0.cc, ping0.cc, www.ping0.cc, not local-aria2-webui.masx200.ddns-ip.net |
| 434  | ping0.cc  | 2604:9cc0:1c0e::a | IPv6   | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": tls: failed to verify certificate: x509: certificate is valid for intl.ping0.cc, ipv4.ping0.cc, ipv6.ping0.cc, ping0.cc, www.ping0.cc, not local-aria2-webui.masx200.ddns-ip.net |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 14 次 (87.5%)
- **DNS解析错误**: 2 次 (12.5%)

#### 错误模式分析

**超时集中度分析**: 共有 14 次超时，主要集中在IP段 108.162（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 14 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 15 次，IPv6失败 1 次，两种协议都存在问题

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 111 |
162.159.45.170 | 162.159.45.170 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare | | 8 |
ashton.ns.cloudflare.com | 162.159.44.173 | IPv4 | h3 | ✅ 成功 | 123 |
cloudflare | | 110 | 162.159.38.89 | 162.159.38.89 | IPv4 | h3 | ✅ 成功 | 123 |
cloudflare | | 152 | damien.ns.cloudflare.com | 162.159.44.168 | IPv4 | h3 | ✅
成功 | 129 | cloudflare | | 247 | craig.ns.cloudflare.com | 162.159.44.192 |
IPv4 | h3 | ✅ 成功 | 130 | cloudflare | | 363 | otto.ns.cloudflare.com |
162.159.44.135 | IPv4 | h3 | ✅ 成功 | 136 | cloudflare | | 15 |
kyree.ns.cloudflare.com | 162.159.44.207 | IPv4 | h3 | ✅ 成功 | 138 |
cloudflare | | 240 | benedict.ns.cloudflare.com | 162.159.44.205 | IPv4 | h3 |
✅ 成功 | 143 | cloudflare | | 199 | huxley.ns.cloudflare.com | 162.159.44.188 |
IPv4 | h3 | ✅ 成功 | 145 | cloudflare | | 113 | dylan.ns.cloudflare.com |
162.159.44.187 | IPv4 | h3 | ✅ 成功 | 150 | cloudflare | | 184 |
abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h3 | ✅ 成功 | 151 |
cloudflare | | 421 | ct.877774.xyz | 172.64.229.236 | IPv4 | h3 | ✅ 成功 | 153
| cloudflare | | 229 | cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h3 | ✅
成功 | 159 | cloudflare | | 424 | ct.877774.xyz | 172.64.229.173 | IPv4 | h3 |
✅ 成功 | 162 | cloudflare | | 193 | trevor.ns.cloudflare.com | 162.159.44.154 |
IPv4 | h3 | ✅ 成功 | 165 | cloudflare | | 286 | braden.ns.cloudflare.com |
162.159.44.169 | IPv4 | h3 | ✅ 成功 | 167 | cloudflare | | 413 |
uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h3 | ✅ 成功 | 167 |
cloudflare | | 54 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h3 | ✅
成功 | 168 | cloudflare | | 268 | wilson.ns.cloudflare.com | 162.159.44.110 |
IPv4 | h3 | ✅ 成功 | 171 | cloudflare | | 371 | moura.ns.cloudflare.com |
162.159.44.217 | IPv4 | h3 | ✅ 成功 | 172 | cloudflare | | 73 |
julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h3 | ✅ 成功 | 174 |
cloudflare | | 105 | sullivan.ns.cloudflare.com | 162.159.44.161 | IPv4 | h3 |
✅ 成功 | 175 | cloudflare | | 419 | ct.877774.xyz | 172.64.229.195 | IPv4 | h3
| ✅ 成功 | 175 | cloudflare | | 436 | pranab.ns.cloudflare.com | 162.159.44.199
| IPv4 | h3 | ✅ 成功 | 178 | cloudflare | | 48 | decker.ns.cloudflare.com |
162.159.44.155 | IPv4 | h3 | ✅ 成功 | 183 | cloudflare | | 425 | ct.877774.xyz
| 172.64.229.174 | IPv4 | h3 | ✅ 成功 | 184 | cloudflare | | 275 |
[2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | h3 | ✅ 成功 |
189 | cloudflare | | 118 | 162.159.44.208 | 162.159.44.208 | IPv4 | h3 | ✅ 成功
| 190 | cloudflare | | 46 | 162.159.39.118 | 162.159.39.118 | IPv4 | h3 | ✅
成功 | 194 | cloudflare | | 420 | ct.877774.xyz | 172.64.229.217 | IPv4 | h3 |
✅ 成功 | 194 | cloudflare | | 370 | moura.ns.cloudflare.com | 108.162.195.217 |
IPv4 | h3 | ✅ 成功 | 197 | cloudflare | | 61 | rustam.ns.cloudflare.com |
162.159.44.148 | IPv4 | h3 | ✅ 成功 | 204 | cloudflare | | 389 |
lewis.ns.cloudflare.com | 162.159.44.159 | IPv4 | h3 | ✅ 成功 | 204 |
cloudflare | | 200 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h3 | ✅
成功 | 209 | cloudflare | | 415 | uriah.ns.cloudflare.com |
2606:4700:58::a29f:2cc2 | IPv6 | h3 | ✅ 成功 | 212 | cloudflare | | 456 |
[2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | h3 | ✅ 成功 |
212 | cloudflare | | 11 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad |
IPv6 | h3 | ✅ 成功 | 213 | cloudflare | | 202 | huxley.ns.cloudflare.com |
2803:f800:50::6ca2:c3bc | IPv6 | h3 | ✅ 成功 | 213 | cloudflare | | 232 |
cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | h3 | ✅ 成功 | 215 |
cloudflare | | 112 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h3 | ✅
成功 | 216 | cloudflare | | 368 | saas.sin.fan | 162.159.36.20 | IPv4 | h3 | ✅
成功 | 217 | cloudflare | | 12 | ashton.ns.cloudflare.com |
2a06:98c1:50::ac40:23ad | IPv6 | h3 | ✅ 成功 | 218 | cloudflare | | 204 |
172.64.148.15 | 172.64.148.15 | IPv4 | h3 | ✅ 成功 | 218 | cloudflare | | 255 |
[2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | h3 | ✅ 成功 |
218 | cloudflare | | 454 | na.877774.xyz | 104.18.38.235 | IPv4 | h3 | ✅ 成功 |
218 | cloudflare | | 252 | 172.64.154.18 | 172.64.154.18 | IPv4 | h3 | ✅ 成功 |
219 | cloudflare | | 107 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1
| IPv6 | h3 | ✅ 成功 | 220 | cloudflare | | 9 | ashton.ns.cloudflare.com |
172.64.35.173 | IPv4 | h3 | ✅ 成功 | 222 | cloudflare | | 194 |
trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h3 | ✅ 成功 | 222 |
cloudflare | | 195 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 |
h3 | ✅ 成功 | 222 | cloudflare | | 440 | pranab.ns.cloudflare.com |
2a06:98c1:50::ac40:23c7 | IPv6 | h3 | ✅ 成功 | 222 | cloudflare | | 198 |
huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h3 | ✅ 成功 | 223 |
cloudflare | | 51 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 |
h3 | ✅ 成功 | 224 | cloudflare | | 246 | craig.ns.cloudflare.com |
108.162.195.192 | IPv4 | h3 | ✅ 成功 | 224 | cloudflare | | 197 |
trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | h3 | ✅ 成功 | 225 |
cloudflare | | 72 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h3 | ✅
成功 | 226 | cloudflare | | 114 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4
| h3 | ✅ 成功 | 226 | cloudflare | | 319 | www.wto.org |
2a06:98c1:3102::6812:29be | IPv6 | h3 | ✅ 成功 | 226 | cloudflare | | 155 |
damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | h3 | ✅ 成功 | 228 |
cloudflare | | 164 | 108.162.198.54 | 108.162.198.54 | IPv4 | h3 | ✅ 成功 | 228
| cloudflare | | 57 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 |
h3 | ✅ 成功 | 229 | cloudflare | | 62 | rustam.ns.cloudflare.com |
172.64.35.148 | IPv4 | h3 | ✅ 成功 | 229 | cloudflare | | 241 |
benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h3 | ✅ 成功 | 229 |
cloudflare | | 392 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 |
h3 | ✅ 成功 | 229 | cloudflare | | 20 | 172.64.156.195 | 172.64.156.195 | IPv4
| h3 | ✅ 成功 | 230 | cloudflare | | 59 | 172.64.159.6 | 172.64.159.6 | IPv4 |
h3 | ✅ 成功 | 230 | cloudflare | | 63 | rustam.ns.cloudflare.com |
2606:4700:58::a29f:2c94 | IPv6 | h3 | ✅ 成功 | 230 | cloudflare | | 196 |
trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | h3 | ✅ 成功 | 230 |
cloudflare | | 285 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h3 | ✅
成功 | 230 | cloudflare | | 448 | 172.64.153.172 | 172.64.153.172 | IPv4 | h3 |
✅ 成功 | 230 | cloudflare | | 13 | 172.64.151.55 | 172.64.151.55 | IPv4 | h3 |
✅ 成功 | 231 | cloudflare | | 270 | wilson.ns.cloudflare.com |
2606:4700:58::a29f:2c6e | IPv6 | h3 | ✅ 成功 | 231 | cloudflare | | 318 |
www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | h3 | ✅ 成功 | 231 | cloudflare
| | 364 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h3 | ✅ 成功 | 231 |
cloudflare | | 249 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 |
h3 | ✅ 成功 | 232 | cloudflare | | 49 | decker.ns.cloudflare.com |
172.64.35.155 | IPv4 | h3 | ✅ 成功 | 233 | cloudflare | | 115 |
dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | h3 | ✅ 成功 | 233 |
cloudflare | | 53 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h3 | ✅
成功 | 234 | cloudflare | | 398 | cf.877774.xyz | 2a06:98c1:3102::6812:29be |
IPv6 | h3 | ✅ 成功 | 234 | cloudflare | | 444 | [2606:4700:4408::18c5:3304] |
2606:4700:4408::18c5:3304 | IPv6 | h3 | ✅ 成功 | 234 | cloudflare | | 203 |
huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | h3 | ✅ 成功 | 235 |
cloudflare | | 287 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h3 | ✅
成功 | 235 | cloudflare | | 295 | 104.18.37.40 | 104.18.37.40 | IPv4 | h3 | ✅
成功 | 235 | cloudflare | | 396 | cf.877774.xyz | 172.64.146.66 | IPv4 | h3 | ✅
成功 | 235 | cloudflare | | 7 | ashton.ns.cloudflare.com | 108.162.195.173 |
IPv4 | h3 | ✅ 成功 | 236 | cloudflare | | 156 | damien.ns.cloudflare.com |
2a06:98c1:50::ac40:23a8 | IPv6 | h3 | ✅ 成功 | 236 | cloudflare | | 186 |
abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | h3 | ✅ 成功 | 236
| cloudflare | | 233 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 |
h3 | ✅ 成功 | 236 | cloudflare | | 89 | 172.64.41.88 | 172.64.41.88 | IPv4 | h3
| ✅ 成功 | 238 | cloudflare | | 234 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 |
✅ 成功 | 238 | cloudflare | | 414 | uriah.ns.cloudflare.com | 172.64.35.194 |
IPv4 | h3 | ✅ 成功 | 238 | cloudflare | | 117 | dylan.ns.cloudflare.com |
2a06:98c1:50::ac40:23bb | IPv6 | h3 | ✅ 成功 | 239 | cloudflare | | 185 |
abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h3 | ✅ 成功 | 239 |
cloudflare | | 239 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h3 |
✅ 成功 | 240 | cloudflare | | 283 | 104.18.39.196 | 104.18.39.196 | IPv4 | h3 |
✅ 成功 | 240 | cloudflare | | 17 | kyree.ns.cloudflare.com |
2606:4700:58::a29f:2ccf | IPv6 | h3 | ✅ 成功 | 241 | cloudflare | | 55 |
bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h3 | ✅ 成功 | 241 | cloudflare
| | 88 | 172.64.33.67 | 172.64.33.67 | IPv4 | h3 | ✅ 成功 | 241 | cloudflare |
| 244 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | h3 | ✅
成功 | 241 | cloudflare | | 391 | lewis.ns.cloudflare.com |
2606:4700:58::a29f:2c9f | IPv6 | h3 | ✅ 成功 | 241 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 0 条记录
- **正常 (100-200ms)**: 31 条记录
- **慢 (200-500ms)**: 69 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 15 次
- **IPv6 失败**: 1 次

### 按协议统计

- **none**: 14 次失败
- **h2**: 2 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
