# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/8 09:49:21
- **数据来源**: connectivity_results.json
- **总测试数**: 488
- **失败测试数**: 23
- **成功测试数**: 465
- **失败率**: 4.71%
- **平均延迟**: 453.96ms
- **最小延迟**: 120ms
- **最大延迟**: 8769ms

---

## 失败测试详情

### 错误类型统计

- **连接错误**: 6 次
- **连接超时**: 14 次
- **DNS解析错误**: 1 次
- **未知错误**: 2 次

### 失败测试列表

| 序号 | 主机/域名                             | 目标IP                  | IP版本  | 协议 | 状态码 | 延迟(ms) | 服务器     | 错误信息                                           |
| ---- | ------------------------------------- | ----------------------- | ------- | ---- | ------ | -------- | ---------- | -------------------------------------------------- |
| 183  | 61.83.202.17                          | 61.83.202.17            | IPv4    | none | N/A    | 0        | N/A        | dial tcp 61.83.202.17:443: connectex: No connec... |
| 184  | 211.229.77.184                        | 211.229.77.184          | IPv4    | none | N/A    | 0        | N/A        | dial tcp 211.229.77.184:443: connectex: No conn... |
| 224  | cfip.xxxxxxxx.tk                      | 198.41.212.130          | IPv4    | none | N/A    | 0        | N/A        | dial tcp 198.41.212.130:443: i/o timeout           |
| 229  | 175.212.207.13                        | 175.212.207.13          | IPv4    | none | N/A    | 0        | N/A        | dial tcp 175.212.207.13:443: i/o timeout           |
| 235  | 52.76.110.129                         | 52.76.110.129           | IPv4    | none | N/A    | 0        | N/A        | dial tcp 52.76.110.129:443: i/o timeout            |
| 242  | 119.194.220.146                       | 119.194.220.146         | IPv4    | none | N/A    | 0        | N/A        | dial tcp 119.194.220.146:443: i/o timeout          |
| 265  | local-aria2-webui.masx200.ddns-ip.net | Unknown                 | Unknown | none | N/A    | 0        | N/A        | DNS解析无结果                                      |
| 277  | ifconfig.co                           | 104.16.123.96           | IPv4    | h3   | 403    | 324      | cloudflare | No error message                                   |
| 278  | ifconfig.co                           | 104.16.124.96           | IPv4    | h3   | 403    | 325      | cloudflare | No error message                                   |
| 292  | 175.215.175.175                       | 175.215.175.175         | IPv4    | none | N/A    | 0        | N/A        | dial tcp 175.215.175.175:443: connectex: No con... |
| 317  | 121.188.182.190                       | 121.188.182.190         | IPv4    | none | N/A    | 0        | N/A        | dial tcp 121.188.182.190:443: i/o timeout          |
| 344  | 43.153.179.6                          | 43.153.179.6            | IPv4    | none | N/A    | 0        | N/A        | dial tcp 43.153.179.6:443: connectex: No connec... |
| 383  | cf.877774.xyz                         | cf.877774.xyz           | IPv4    | none | N/A    | 0        | N/A        | dial tcp cf.877774.xyz:443: i/o timeout            |
| 407  | ct.877774.xyz                         | ct.877774.xyz           | IPv4    | none | N/A    | 0        | N/A        | dial tcp ct.877774.xyz:443: i/o timeout            |
| 408  | 222.105.131.225                       | 222.105.131.225         | IPv4    | none | N/A    | 0        | N/A        | dial tcp 222.105.131.225:443: i/o timeout          |
| 417  | 111.171.108.67                        | 111.171.108.67          | IPv4    | none | N/A    | 0        | N/A        | dial tcp 111.171.108.67:443: i/o timeout           |
| 418  | 138.2.18.82                           | 138.2.18.82             | IPv4    | none | N/A    | 0        | N/A        | dial tcp 138.2.18.82:443: i/o timeout              |
| 439  | 115.22.115.218                        | 115.22.115.218          | IPv4    | none | N/A    | 0        | N/A        | dial tcp 115.22.115.218:443: i/o timeout           |
| 443  | 59.31.68.195                          | 59.31.68.195            | IPv4    | none | N/A    | 0        | N/A        | dial tcp 59.31.68.195:443: connectex: No connec... |
| 444  | trevor.ns.cloudfl...                  | 4444.cloudflare.182682.xyz         | IPv4    | none | N/A    | 0        | N/A        | dial tcp 4444.cloudflare.182682.xyz:443: i/o timeout          |
| 451  | 61.85.1.77                            | 61.85.1.77              | IPv4    | none | N/A    | 0        | N/A        | dial tcp 61.85.1.77:443: connectex: No connecti... |
| 478  | 3.0.50.69                             | 3.0.50.69               | IPv4    | none | N/A    | 0        | N/A        | dial tcp 3.0.50.69:443: i/o timeout                |
| 485  | cloudflare.182682...                  | 6666.cloudflare.182682.xyz | IPv6    | none | N/A    | 0        | N/A        | dial tcp 6666.cloudflare.182682.xyz:443: i/o tim... |

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 | | ----
| --------- | ------ | ------ | ---- | ---- | -------- | ------ || 57 |
abdullah.ns.cloud... | 162.159.44.203 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 21 | julio.ns.cloudfla... | 162.159.44.209 | IPv4 | h3 | ✅ 成功 | 125 |
cloudflare | | 15 | rustam.ns.cloudfl... | 162.159.44.148 | IPv4 | h3 | ✅ 成功
| 134 | cloudflare | | 139 | braden.ns.cloudfl... | 162.159.44.169 | IPv4 | h3 |
✅ 成功 | 134 | cloudflare | | 44 | 162.159.39.118 | 162.159.39.118 | IPv4 | h3
| ✅ 成功 | 140 | cloudflare | | 188 | 172.64.229.249 | 172.64.229.249 | IPv4 |
h3 | ✅ 成功 | 140 | cloudflare | | 86 | craig.ns.cloudfla... | 162.159.44.192 |
IPv4 | h3 | ✅ 成功 | 145 | cloudflare | | 386 | lewis.ns.cloudfla... |
162.159.44.159 | IPv4 | h3 | ✅ 成功 | 149 | cloudflare | | 252 |
kyree.ns.cloudfla... | 162.159.44.207 | IPv4 | h3 | ✅ 成功 | 150 | cloudflare |
| 48 | 162.159.44.208 | 162.159.44.208 | IPv4 | h3 | ✅ 成功 | 157 | cloudflare
| | 128 | dylan.ns.cloudfla... | 162.159.44.187 | IPv4 | h3 | ✅ 成功 | 158 |
cloudflare | | 263 | 108.162.198.54 | 108.162.198.54 | IPv4 | h3 | ✅ 成功 | 158
| cloudflare | | 107 | cris.ns.cloudflar... | 162.159.44.202 | IPv4 | h3 | ✅
成功 | 160 | cloudflare | | 314 | 162.159.38.89 | 162.159.38.89 | IPv4 | h3 | ✅
成功 | 167 | cloudflare | | 64 | damien.ns.cloudfl... | 172.64.35.168 | IPv4 |
h3 | ✅ 成功 | 169 | cloudflare | | 327 | ashton.ns.cloudfl... | 162.159.44.173
| IPv4 | h3 | ✅ 成功 | 170 | cloudflare | | 63 | damien.ns.cloudfl... |
162.159.44.168 | IPv4 | h3 | ✅ 成功 | 176 | cloudflare | | 358 |
benedict.ns.cloud... | 2a06:98c1:50::ac40:23cd | IPv6 | h3 | ✅ 成功 | 180 |
cloudflare | | 60 | abdullah.ns.cloud... | 2803:f800:50::6ca2:c3cb | IPv6 | h3 |
✅ 成功 | 183 | cloudflare | | 395 | sullivan.ns.cloud... | 162.159.44.161 |
IPv4 | h3 | ✅ 成功 | 184 | cloudflare | | 89 | craig.ns.cloudfla... |
2803:f800:50::6ca2:c3c0 | IPv6 | h3 | ✅ 成功 | 185 | cloudflare | | 58 |
abdullah.ns.cloud... | 172.64.35.203 | IPv4 | h3 | ✅ 成功 | 187 | cloudflare |
| 436 | uriah.ns.cloudfla... | 2a06:98c1:50::ac40:23c2 | IPv6 | h3 | ✅ 成功 |
187 | cloudflare | | 74 | wilson.ns.cloudfl... | 172.64.35.110 | IPv4 | h3 | ✅
成功 | 190 | cloudflare | | 258 | bowen.ns.cloudfla... | 162.159.44.83 | IPv4 |
h3 | ✅ 成功 | 191 | cloudflare | | 354 | benedict.ns.cloud... | 162.159.44.205
| IPv4 | h3 | ✅ 成功 | 191 | cloudflare | | 357 | benedict.ns.cloud... |
2803:f800:50::6ca2:c3cd | IPv6 | h3 | ✅ 成功 | 191 | cloudflare | | 213 |
decker.ns.cloudfl... | 162.159.44.155 | IPv4 | h3 | ✅ 成功 | 194 | cloudflare |
| 398 | sullivan.ns.cloud... | 2803:f800:50::6ca2:c3a1 | IPv6 | h3 | ✅ 成功 |
195 | cloudflare | | 108 | cris.ns.cloudflar... | 172.64.35.202 | IPv4 | h3 | ✅
成功 | 196 | cloudflare | | 351 | otto.ns.cloudflar... | 2803:f800:50::6ca2:c387
| IPv6 | h3 | ✅ 成功 | 196 | cloudflare | | 335 | pranab.ns.cloudfl... |
162.159.44.199 | IPv4 | h3 | ✅ 成功 | 197 | cloudflare | | 325 |
[2606:4700:4408::... | 2606:4700:4408::18c5:3304 | IPv6 | h3 | ✅ 成功 | 198 |
cloudflare | | 365 | 162.159.45.170 | 162.159.45.170 | IPv4 | h3 | ✅ 成功 | 198
| cloudflare | | 385 | lewis.ns.cloudfla... | 108.162.195.159 | IPv4 | h3 | ✅
成功 | 198 | cloudflare | | 261 | bowen.ns.cloudfla... | 2803:f800:50::6ca2:c353
| IPv6 | h3 | ✅ 成功 | 199 | cloudflare | | 328 | ashton.ns.cloudfl... |
172.64.35.173 | IPv4 | h3 | ✅ 成功 | 199 | cloudflare | | 110 |
cris.ns.cloudflar... | 2803:f800:50::6ca2:c3ca | IPv6 | h3 | ✅ 成功 | 201 |
cloudflare | | 47 | 172.64.41.88 | 172.64.41.88 | IPv4 | h3 | ✅ 成功 | 202 |
cloudflare | | 59 | abdullah.ns.cloud... | 2606:4700:58::a29f:2ccb | IPv6 | h3 |
✅ 成功 | 202 | cloudflare | | 302 | huxley.ns.cloudfl... |
2606:4700:58::a29f:2cbc | IPv6 | h3 | ✅ 成功 | 202 | cloudflare | | 356 |
benedict.ns.cloud... | 2606:4700:58::a29f:2ccd | IPv6 | h3 | ✅ 成功 | 203 |
cloudflare | | 394 | sullivan.ns.cloud... | 108.162.195.161 | IPv4 | h3 | ✅
成功 | 203 | cloudflare | | 10 | www.wto.org | 172.64.146.66 | IPv4 | h3 | ✅
成功 | 204 | cloudflare | | 141 | braden.ns.cloudfl... | 2606:4700:58::a29f:2ca9
| IPv6 | h3 | ✅ 成功 | 204 | cloudflare | | 387 | lewis.ns.cloudfla... |
172.64.35.159 | IPv4 | h3 | ✅ 成功 | 204 | cloudflare | | 14 |
rustam.ns.cloudfl... | 108.162.195.148 | IPv4 | h3 | ✅ 成功 | 205 | cloudflare
| | 85 | craig.ns.cloudfla... | 108.162.195.192 | IPv4 | h3 | ✅ 成功 | 205 |
cloudflare | | 111 | cris.ns.cloudflar... | 2a06:98c1:50::ac40:23ca | IPv6 | h3
| ✅ 成功 | 206 | cloudflare | | 435 | uriah.ns.cloudfla... |
2803:f800:50::6ca2:c3c2 | IPv6 | h3 | ✅ 成功 | 206 | cloudflare | | 25 |
julio.ns.cloudfla... | 2a06:98c1:50::ac40:23d1 | IPv6 | h3 | ✅ 成功 | 207 |
cloudflare | | 56 | abdullah.ns.cloud... | 108.162.195.203 | IPv4 | h3 | ✅ 成功
| 208 | cloudflare | | 127 | dylan.ns.cloudfla... | 108.162.195.187 | IPv4 | h3
| ✅ 成功 | 208 | cloudflare | | 353 | benedict.ns.cloud... | 108.162.195.205 |
IPv4 | h3 | ✅ 成功 | 208 | cloudflare | | 447 | trevor.ns.cloudfl... |
2606:4700:58::a29f:2c9a | IPv6 | h3 | ✅ 成功 | 208 | cloudflare | | 24 |
julio.ns.cloudfla... | 2803:f800:50::6ca2:c3d1 | IPv6 | h3 | ✅ 成功 | 210 |
cloudflare | | 67 | damien.ns.cloudfl... | 2a06:98c1:50::ac40:23a8 | IPv6 | h3 |
✅ 成功 | 210 | cloudflare | | 1 | 172.64.157.120 | 172.64.157.120 | IPv4 | h3 |
✅ 成功 | 211 | cloudflare | | 61 | abdullah.ns.cloud... |
2a06:98c1:50::ac40:23cb | IPv6 | h3 | ✅ 成功 | 211 | cloudflare | | 256 |
kyree.ns.cloudfla... | 2a06:98c1:50::ac40:23cf | IPv6 | h3 | ✅ 成功 | 211 |
cloudflare | | 264 | 172.64.154.18 | 172.64.154.18 | IPv4 | h3 | ✅ 成功 | 211 |
cloudflare | | 352 | otto.ns.cloudflar... | 2a06:98c1:50::ac40:2387 | IPv6 | h3
| ✅ 成功 | 211 | cloudflare | | 12 | www.wto.org | 2606:4700:4406::ac40:9242 |
IPv6 | h3 | ✅ 成功 | 212 | cloudflare | | 87 | craig.ns.cloudfla... |
172.64.35.192 | IPv4 | h3 | ✅ 成功 | 213 | cloudflare | | 138 |
braden.ns.cloudfl... | 108.162.195.169 | IPv4 | h3 | ✅ 成功 | 213 | cloudflare
| | 446 | trevor.ns.cloudfl... | 172.64.35.154 | IPv4 | h3 | ✅ 成功 | 213 |
cloudflare | | 72 | wilson.ns.cloudfl... | 108.162.195.110 | IPv4 | h3 | ✅ 成功
| 214 | cloudflare | | 338 | pranab.ns.cloudfl... | 2803:f800:50::6ca2:c3c7 |
IPv6 | h3 | ✅ 成功 | 214 | cloudflare | | 13 | www.wto.org |
2a06:98c1:3102::6812:29be | IPv6 | h3 | ✅ 成功 | 215 | cloudflare | | 29 |
cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | h3 | ✅ 成功 | 215 |
cloudflare | | 132 | dylan.ns.cloudfla... | 2a06:98c1:50::ac40:23bb | IPv6 | h3
| ✅ 成功 | 215 | cloudflare | | 255 | kyree.ns.cloudfla... |
2803:f800:50::6ca2:c3cf | IPv6 | h3 | ✅ 成功 | 215 | cloudflare | | 299 |
huxley.ns.cloudfl... | 108.162.195.188 | IPv4 | h3 | ✅ 成功 | 215 | cloudflare
| | 396 | sullivan.ns.cloud... | 172.64.35.161 | IPv4 | h3 | ✅ 成功 | 215 |
cloudflare | | 106 | cris.ns.cloudflar... | 108.162.195.202 | IPv4 | h3 | ✅
成功 | 216 | cloudflare | | 316 | saas.sin.fan | 162.159.36.5 | IPv4 | h3 | ✅
成功 | 216 | cloudflare | | 360 | moura.ns.cloudfla... | 162.159.44.217 | IPv4 |
h3 | ✅ 成功 | 216 | cloudflare | | 361 | moura.ns.cloudfla... | 172.64.35.217 |
IPv4 | h3 | ✅ 成功 | 216 | cloudflare | | 2 | 162.159.36.104 | 162.159.36.104 |
IPv4 | h3 | ✅ 成功 | 217 | cloudflare | | 19 | rustam.ns.cloudfl... |
2a06:98c1:50::ac40:2394 | IPv6 | h3 | ✅ 成功 | 217 | cloudflare | | 315 |
saas.sin.fan | 162.159.36.20 | IPv4 | h3 | ✅ 成功 | 217 | cloudflare | | 397 |
sullivan.ns.cloud... | 2606:4700:58::a29f:2ca1 | IPv6 | h3 | ✅ 成功 | 217 |
cloudflare | | 429 | ct.877774.xyz | 172.64.229.173 | IPv4 | h3 | ✅ 成功 | 217
| cloudflare | | 66 | damien.ns.cloudfl... | 2803:f800:50::6ca2:c3a8 | IPv6 | h3
| ✅ 成功 | 218 | cloudflare | | 73 | wilson.ns.cloudfl... | 162.159.44.110 |
IPv4 | h3 | ✅ 成功 | 218 | cloudflare | | 140 | braden.ns.cloudfl... |
172.64.35.169 | IPv4 | h3 | ✅ 成功 | 218 | cloudflare | | 214 |
decker.ns.cloudfl... | 172.64.35.155 | IPv4 | h3 | ✅ 成功 | 218 | cloudflare |
| 449 | trevor.ns.cloudfl... | 2a06:98c1:50::ac40:239a | IPv6 | h3 | ✅ 成功 |
218 | cloudflare | | 330 | ashton.ns.cloudfl... | 2803:f800:50::6ca2:c3ad | IPv6
| h3 | ✅ 成功 | 219 | cloudflare | | 131 | dylan.ns.cloudfla... |
2803:f800:50::6ca2:c3bb | IPv6 | h3 | ✅ 成功 | 220 | cloudflare | | 212 |
decker.ns.cloudfl... | 108.162.195.155 | IPv4 | h3 | ✅ 成功 | 221 | cloudflare
| | 251 | kyree.ns.cloudfla... | 108.162.195.207 | IPv4 | h3 | ✅ 成功 | 221 |
cloudflare | | 350 | otto.ns.cloudflar... | 2606:4700:58::a29f:2c87 | IPv6 | h3
| ✅ 成功 | 221 | cloudflare | | 17 | rustam.ns.cloudfl... |
2606:4700:58::a29f:2c94 | IPv6 | h3 | ✅ 成功 | 222 | cloudflare | | 30 |
cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | h3 | ✅ 成功 | 222 |
cloudflare | | 65 | damien.ns.cloudfl... | 2606:4700:58::a29f:2ca8 | IPv6 | h3 |
✅ 成功 | 222 | cloudflare | | 77 | wilson.ns.cloudfl... |
2a06:98c1:50::ac40:236e | IPv6 | h3 | ✅ 成功 | 223 | cloudflare | | 118 |
[2606:4700:440f::... | 2606:4700:440f::53aa:4126 | IPv6 | h3 | ✅ 成功 | 223 |
cloudflare | | 259 | bowen.ns.cloudfla... | 172.64.35.83 | IPv4 | h3 | ✅ 成功 |
223 | cloudflare | | 142 | braden.ns.cloudfl... | 2803:f800:50::6ca2:c3a9 | IPv6
| h3 | ✅ 成功 | 224 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 0 条记录
- **正常 (100-200ms)**: 37 条记录
- **慢 (200-500ms)**: 63 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 21 次
- **IPv6 失败**: 1 次

### 按协议统计

- **none**: 21 次失败
- **h3**: 2 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
