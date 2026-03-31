# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/8 13:43:29
- **数据来源**: connectivity_results.json
- **总测试数**: 486
- **失败测试数**: 19
- **成功测试数**: 467
- **失败率**: 3.91%
- **平均延迟**: 659.41ms
- **最小延迟**: 117ms
- **最大延迟**: 6402ms

---

## 失败测试详情

### 错误类型统计

- **连接超时**: 13 次
- **连接错误**: 6 次

### 失败测试列表

| 序号 | 主机/域名                | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                                                             |
| ---- | ------------------------ | --------------- | ------ | ---- | ------ | -------- | ------ | -------------------------------------------------------------------------------------------------------------------- |
| 213  | cfip.xxxxxxxx.tk         | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout                                                                             |
| 215  | 175.215.175.175          | 175.215.175.175 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 175.215.175.175:443: connectex: No connection could be made because the target machine actively refused it. |
| 217  | 61.85.1.77               | 61.85.1.77      | IPv4   | none | N/A    | 0        | N/A    | dial tcp 61.85.1.77:443: connectex: No connection could be made because the target machine actively refused it.      |
| 237  | 211.229.77.184           | 211.229.77.184  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 211.229.77.184:443: connectex: No connection could be made because the target machine actively refused it.  |
| 242  | cf.877774.xyz            | cf.877774.xyz   | IPv4   | none | N/A    | 0        | N/A    | dial tcp cf.877774.xyz:443: i/o timeout                                                                              |
| 326  | 175.212.207.13           | 175.212.207.13  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 175.212.207.13:443: i/o timeout                                                                             |
| 327  | 222.105.131.225          | 222.105.131.225 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 222.105.131.225:443: i/o timeout                                                                            |
| 342  | 111.171.108.67           | 111.171.108.67  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 111.171.108.67:443: i/o timeout                                                                             |
| 348  | 3.0.50.69                | 3.0.50.69       | IPv4   | none | N/A    | 0        | N/A    | dial tcp 3.0.50.69:443: i/o timeout                                                                                  |
| 351  | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout                                                                            |
| 423  | 43.153.179.6             | 43.153.179.6    | IPv4   | none | N/A    | 0        | N/A    | dial tcp 43.153.179.6:443: connectex: No connection could be made because the target machine actively refused it.    |
| 438  | 59.31.68.195             | 59.31.68.195    | IPv4   | none | N/A    | 0        | N/A    | dial tcp 59.31.68.195:443: connectex: No connection could be made because the target machine actively refused it.    |
| 473  | 61.83.202.17             | 61.83.202.17    | IPv4   | none | N/A    | 0        | N/A    | dial tcp 61.83.202.17:443: connectex: No connection could be made because the target machine actively refused it.    |
| 481  | 115.22.115.218           | 115.22.115.218  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 115.22.115.218:443: i/o timeout                                                                             |
| 482  | 52.76.110.129            | 52.76.110.129   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 52.76.110.129:443: i/o timeout                                                                              |
| 483  | 121.188.182.190          | 121.188.182.190 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 121.188.182.190:443: i/o timeout                                                                            |
| 484  | 138.2.18.82              | 138.2.18.82     | IPv4   | none | N/A    | 0        | N/A    | dial tcp 138.2.18.82:443: i/o timeout                                                                                |
| 485  | 119.194.220.146          | 119.194.220.146 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 119.194.220.146:443: i/o timeout                                                                            |
| 486  | ct.877774.xyz            | ct.877774.xyz   | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout                                                                              |

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 53 |
ashton.ns.cloudflare.com | 162.159.44.173 | IPv4 | h3 | ✅ 成功 | 117 |
cloudflare | | 27 | sullivan.ns.cloudflare.com | 162.159.44.161 | IPv4 | h3 | ✅
成功 | 130 | cloudflare | | 39 | ct.877774.xyz | 172.64.229.217 | IPv4 | h3 | ✅
成功 | 152 | cloudflare | | 312 | dylan.ns.cloudflare.com | 162.159.44.187 |
IPv4 | h3 | ✅ 成功 | 154 | cloudflare | | 417 | julio.ns.cloudflare.com |
162.159.44.209 | IPv4 | h3 | ✅ 成功 | 157 | cloudflare | | 35 | ct.877774.xyz |
172.64.229.173 | IPv4 | h3 | ✅ 成功 | 158 | cloudflare | | 34 | ct.877774.xyz |
172.64.229.161 | IPv4 | h3 | ✅ 成功 | 162 | cloudflare | | 189 | 162.159.38.89
| 162.159.38.89 | IPv4 | h3 | ✅ 成功 | 162 | cloudflare | | 365 |
cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h3 | ✅ 成功 | 162 | cloudflare
| | 131 | 162.159.44.208 | 162.159.44.208 | IPv4 | h3 | ✅ 成功 | 164 |
cloudflare | | 15 | kyree.ns.cloudflare.com | 162.159.44.207 | IPv4 | h3 | ✅
成功 | 167 | cloudflare | | 46 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4
| h3 | ✅ 成功 | 168 | cloudflare | | 275 | rustam.ns.cloudflare.com |
162.159.44.148 | IPv4 | h3 | ✅ 成功 | 169 | cloudflare | | 9 |
craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h3 | ✅ 成功 | 172 |
cloudflare | | 162 | benedict.ns.cloudflare.com | 162.159.44.205 | IPv4 | h3 |
✅ 成功 | 172 | cloudflare | | 182 | braden.ns.cloudflare.com | 162.159.44.169 |
IPv4 | h3 | ✅ 成功 | 172 | cloudflare | | 38 | ct.877774.xyz | 172.64.229.195 |
IPv4 | h3 | ✅ 成功 | 180 | cloudflare | | 140 | 162.159.39.118 | 162.159.39.118
| IPv4 | h3 | ✅ 成功 | 184 | cloudflare | | 10 | craig.ns.cloudflare.com |
172.64.35.192 | IPv4 | h3 | ✅ 成功 | 186 | cloudflare | | 176 | 172.64.229.249
| 172.64.229.249 | IPv4 | h3 | ✅ 成功 | 186 | cloudflare | | 323 |
otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | h3 | ✅ 成功 | 189 |
cloudflare | | 36 | ct.877774.xyz | 172.64.229.174 | IPv4 | h3 | ✅ 成功 | 190 |
cloudflare | | 1 | 108.162.198.54 | 108.162.198.54 | IPv4 | h3 | ✅ 成功 | 192 |
cloudflare | | 152 | huxley.ns.cloudflare.com | 162.159.44.188 | IPv4 | h3 | ✅
成功 | 193 | cloudflare | | 122 | pranab.ns.cloudflare.com |
2a06:98c1:50::ac40:23c7 | IPv6 | h3 | ✅ 成功 | 194 | cloudflare | | 322 |
otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | h3 | ✅ 成功 | 194 |
cloudflare | | 145 | abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h3 |
✅ 成功 | 198 | cloudflare | | 452 | saas.sin.fan | 162.159.36.5 | IPv4 | h3 |
✅ 成功 | 198 | cloudflare | | 320 | otto.ns.cloudflare.com | 162.159.44.135 |
IPv4 | h3 | ✅ 成功 | 199 | cloudflare | | 456 | moura.ns.cloudflare.com |
2606:4700:58::a29f:2cd9 | IPv6 | h3 | ✅ 成功 | 199 | cloudflare | | 436 |
172.64.49.165 | 172.64.49.165 | IPv4 | h3 | ✅ 成功 | 201 | cloudflare | | 451 |
saas.sin.fan | 162.159.36.20 | IPv4 | h3 | ✅ 成功 | 202 | cloudflare | | 454 |
moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h3 | ✅ 成功 | 204 |
cloudflare | | 45 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h3 | ✅
成功 | 205 | cloudflare | | 32 | ct.877774.xyz | 172.64.229.236 | IPv4 | h3 | ✅
成功 | 207 | cloudflare | | 134 | decker.ns.cloudflare.com | 162.159.44.155 |
IPv4 | h3 | ✅ 成功 | 207 | cloudflare | | 401 | wilson.ns.cloudflare.com |
162.159.44.110 | IPv4 | h3 | ✅ 成功 | 207 | cloudflare | | 4 | www.wto.org |
2606:4700:4406::ac40:9242 | IPv6 | h3 | ✅ 成功 | 209 | cloudflare | | 402 |
wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h3 | ✅ 成功 | 209 |
cloudflare | | 33 | ct.877774.xyz | 172.64.229.44 | IPv4 | h3 | ✅ 成功 | 212 |
cloudflare | | 56 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 |
h3 | ✅ 成功 | 213 | cloudflare | | 183 | braden.ns.cloudflare.com |
172.64.35.169 | IPv4 | h3 | ✅ 成功 | 213 | cloudflare | | 297 | cf.877774.xyz |
104.18.41.190 | IPv4 | h3 | ✅ 成功 | 215 | cloudflare | | 17 |
kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | h3 | ✅ 成功 | 216 |
cloudflare | | 147 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6
| h3 | ✅ 成功 | 216 | cloudflare | | 144 | abdullah.ns.cloudflare.com |
108.162.195.203 | IPv4 | h3 | ✅ 成功 | 217 | cloudflare | | 437 |
[2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | h3 | ✅ 成功 |
217 | cloudflare | | 274 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 |
h3 | ✅ 成功 | 218 | cloudflare | | 139 | 104.18.42.26 | 104.18.42.26 | IPv4 |
h3 | ✅ 成功 | 219 | cloudflare | | 141 | 172.64.33.67 | 172.64.33.67 | IPv4 |
h3 | ✅ 成功 | 219 | cloudflare | | 30 | sullivan.ns.cloudflare.com |
2803:f800:50::6ca2:c3a1 | IPv6 | h3 | ✅ 成功 | 220 | cloudflare | | 422 |
104.18.39.196 | 104.18.39.196 | IPv4 | h3 | ✅ 成功 | 221 | cloudflare | | 166 |
benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | h3 | ✅ 成功 | 222
| cloudflare | | 219 | 104.18.37.13 | 104.18.37.13 | IPv4 | h3 | ✅ 成功 | 222 |
cloudflare | | 277 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 |
h3 | ✅ 成功 | 222 | cloudflare | | 398 | 172.64.157.120 | 172.64.157.120 | IPv4
| h3 | ✅ 成功 | 223 | cloudflare | | 28 | sullivan.ns.cloudflare.com |
172.64.35.161 | IPv4 | h3 | ✅ 成功 | 225 | cloudflare | | 357 |
[2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | h3 | ✅ 成功 |
225 | cloudflare | | 416 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h3
| ✅ 成功 | 225 | cloudflare | | 311 | dylan.ns.cloudflare.com | 108.162.195.187
| IPv4 | h3 | ✅ 成功 | 226 | cloudflare | | 136 | decker.ns.cloudflare.com |
2606:4700:58::a29f:2c9b | IPv6 | h3 | ✅ 成功 | 227 | cloudflare | | 137 |
decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | h3 | ✅ 成功 | 227 |
cloudflare | | 321 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h3 | ✅
成功 | 228 | cloudflare | | 419 | julio.ns.cloudflare.com |
2606:4700:58::a29f:2cd1 | IPv6 | h3 | ✅ 成功 | 228 | cloudflare | | 16 |
kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h3 | ✅ 成功 | 230 | cloudflare
| | 161 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h3 | ✅ 成功 |
230 | cloudflare | | 296 | cf.877774.xyz | 172.64.146.66 | IPv4 | h3 | ✅ 成功 |
230 | cloudflare | | 379 | 172.64.153.172 | 172.64.153.172 | IPv4 | h3 | ✅ 成功
| 230 | cloudflare | | 421 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 |
IPv6 | h3 | ✅ 成功 | 230 | cloudflare | | 49 | bowen.ns.cloudflare.com |
2803:f800:50::6ca2:c353 | IPv6 | h3 | ✅ 成功 | 231 | cloudflare | | 151 |
huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h3 | ✅ 成功 | 231 |
cloudflare | | 153 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h3 | ✅
成功 | 231 | cloudflare | | 154 | huxley.ns.cloudflare.com |
2606:4700:58::a29f:2cbc | IPv6 | h3 | ✅ 成功 | 231 | cloudflare | | 207 |
172.64.146.16 | 172.64.146.16 | IPv4 | h3 | ✅ 成功 | 231 | cloudflare | | 276 |
rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h3 | ✅ 成功 | 231 |
cloudflare | | 367 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 |
h3 | ✅ 成功 | 231 | cloudflare | | 385 | [2606:4700:440f::53aa:4126] |
2606:4700:440f::53aa:4126 | IPv6 | h3 | ✅ 成功 | 231 | cloudflare | | 8 |
craig.ns.cloudflare.com | 108.162.195.192 | IPv4 | h3 | ✅ 成功 | 232 |
cloudflare | | 11 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 |
h3 | ✅ 成功 | 232 | cloudflare | | 3 | www.wto.org | 104.18.41.190 | IPv4 | h3
| ✅ 成功 | 233 | cloudflare | | 150 | [2606:4700:4409::5b5b:7758] |
2606:4700:4409::5b5b:7758 | IPv6 | h3 | ✅ 成功 | 233 | cloudflare | | 37 |
ct.877774.xyz | 172.64.229.185 | IPv4 | h3 | ✅ 成功 | 234 | cloudflare | | 369
| cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | h3 | ✅ 成功 | 234 |
cloudflare | | 404 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 |
h3 | ✅ 成功 | 234 | cloudflare | | 55 | ashton.ns.cloudflare.com |
2606:4700:58::a29f:2cad | IPv6 | h3 | ✅ 成功 | 235 | cloudflare | | 366 |
cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h3 | ✅ 成功 | 235 | cloudflare
| | 26 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h3 | ✅ 成功 |
236 | cloudflare | | 47 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h3 |
✅ 成功 | 236 | cloudflare | | 155 | huxley.ns.cloudflare.com |
2803:f800:50::6ca2:c3bc | IPv6 | h3 | ✅ 成功 | 236 | cloudflare | | 316 |
dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | h3 | ✅ 成功 | 236 |
cloudflare | | 405 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 |
h3 | ✅ 成功 | 237 | cloudflare | | 23 | bestcf.030101.xyz | 104.18.33.227 |
IPv4 | h3 | ✅ 成功 | 238 | cloudflare | | 52 | ashton.ns.cloudflare.com |
108.162.195.173 | IPv4 | h3 | ✅ 成功 | 238 | cloudflare | | 148 |
abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | h3 | ✅ 成功 | 238
| cloudflare | | 403 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h3 |
✅ 成功 | 238 | cloudflare | | 13 | craig.ns.cloudflare.com |
2a06:98c1:50::ac40:23c0 | IPv6 | h3 | ✅ 成功 | 239 | cloudflare | | 319 |
otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h3 | ✅ 成功 | 239 |
cloudflare | | 2 | www.wto.org | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 240 |
cloudflare | | 12 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 |
h3 | ✅ 成功 | 240 | cloudflare | | 305 | 172.64.40.9 | 172.64.40.9 | IPv4 | h3
| ✅ 成功 | 240 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 0 条记录
- **正常 (100-200ms)**: 30 条记录
- **慢 (200-500ms)**: 70 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 19 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 19 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
