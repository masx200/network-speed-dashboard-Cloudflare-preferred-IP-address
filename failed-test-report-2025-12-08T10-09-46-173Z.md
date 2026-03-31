# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/8 18:09:46
- **数据来源**: connectivity_results.json
- **总测试数**: 493
- **失败测试数**: 13
- **成功测试数**: 480
- **失败率**: 2.64%
- **平均延迟**: 841.89ms
- **最小延迟**: 118ms
- **最大延迟**: 9305ms

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: I/O超时**: 12 次 (92.3%)
- **连接超时: 上下文超时**: 1 次 (7.7%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (12 次测试)

| 序号 | 主机/域名                | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                  |
| ---- | ------------------------ | --------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------- |
| 176  | 119.194.220.146          | 119.194.220.146 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 119.194.220.146:443: i/o timeout |
| 184  | 121.188.182.190          | 121.188.182.190 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 121.188.182.190:443: i/o timeout |
| 187  | cfip.xxxxxxxx.tk         | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout  |
| 235  | 115.22.115.218           | 115.22.115.218  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 115.22.115.218:443: i/o timeout  |
| 241  | ct.877774.xyz            | ct.877774.xyz   | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout   |
| 250  | 111.171.108.67           | 111.171.108.67  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 111.171.108.67:443: i/o timeout  |
| 257  | cf.877774.xyz            | cf.877774.xyz   | IPv4   | none | N/A    | 0        | N/A    | dial tcp cf.877774.xyz:443: i/o timeout   |
| 317  | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout |
| 343  | 175.212.207.13           | 175.212.207.13  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 175.212.207.13:443: i/o timeout  |
| 487  | 104.26.6.112             | 104.26.6.112    | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.26.6.112:443: i/o timeout    |
| 492  | 52.76.110.129            | 52.76.110.129   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 52.76.110.129:443: i/o timeout   |
| 493  | 3.0.50.69                | 3.0.50.69       | IPv4   | none | N/A    | 0        | N/A    | dial tcp 3.0.50.69:443: i/o timeout       |

#### 连接超时: 上下文超时 (1 次测试)

| 序号 | 主机/域名   | 目标IP      | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                            |
| ---- | ----------- | ----------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------------------------------------------------- |
| 464  | 198.62.62.4 | 198.62.62.4 | IPv4   | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 13 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 12 次超时，主要集中在IP段 119.194（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 12 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**:
所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 387 |
ashton.ns.cloudflare.com | 162.159.44.173 | IPv4 | h3 | ✅ 成功 | 118 |
cloudflare | | 413 | kyree.ns.cloudflare.com | 162.159.44.207 | IPv4 | h3 | ✅
成功 | 128 | cloudflare | | 201 | ct.877774.xyz | 172.64.229.44 | IPv4 | h3 | ✅
成功 | 141 | cloudflare | | 202 | ct.877774.xyz | 172.64.229.161 | IPv4 | h3 |
✅ 成功 | 147 | cloudflare | | 137 | sullivan.ns.cloudflare.com | 162.159.44.161
| IPv4 | h3 | ✅ 成功 | 148 | cloudflare | | 324 | decker.ns.cloudflare.com |
162.159.44.155 | IPv4 | h3 | ✅ 成功 | 150 | cloudflare | | 303 |
otto.ns.cloudflare.com | 162.159.44.135 | IPv4 | h3 | ✅ 成功 | 152 | cloudflare
| | 380 | abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h3 | ✅ 成功 |
155 | cloudflare | | 318 | trevor.ns.cloudflare.com | 162.159.44.154 | IPv4 | h3
| ✅ 成功 | 156 | cloudflare | | 205 | huxley.ns.cloudflare.com | 162.159.44.188
| IPv4 | h3 | ✅ 成功 | 159 | cloudflare | | 199 | ct.877774.xyz |
172.64.229.217 | IPv4 | h3 | ✅ 成功 | 164 | cloudflare | | 252 |
julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h3 | ✅ 成功 | 164 |
cloudflare | | 197 | ct.877774.xyz | 172.64.229.185 | IPv4 | h3 | ✅ 成功 | 172
| cloudflare | | 230 | dylan.ns.cloudflare.com | 162.159.44.187 | IPv4 | h3 | ✅
成功 | 173 | cloudflare | | 167 | craig.ns.cloudflare.com | 162.159.44.192 |
IPv4 | h3 | ✅ 成功 | 175 | cloudflare | | 105 | braden.ns.cloudflare.com |
162.159.44.169 | IPv4 | h3 | ✅ 成功 | 177 | cloudflare | | 196 | ct.877774.xyz
| 172.64.229.174 | IPv4 | h3 | ✅ 成功 | 178 | cloudflare | | 85 |
108.162.198.54 | 108.162.198.54 | IPv4 | h3 | ✅ 成功 | 179 | cloudflare | | 407
| bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h3 | ✅ 成功 | 181 |
cloudflare | | 50 | rustam.ns.cloudflare.com | 162.159.44.148 | IPv4 | h3 | ✅
成功 | 182 | cloudflare | | 371 | wilson.ns.cloudflare.com | 162.159.44.110 |
IPv4 | h3 | ✅ 成功 | 184 | cloudflare | | 115 | benedict.ns.cloudflare.com |
162.159.44.205 | IPv4 | h3 | ✅ 成功 | 185 | cloudflare | | 198 | ct.877774.xyz
| 172.64.229.195 | IPv4 | h3 | ✅ 成功 | 188 | cloudflare | | 297 |
moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h3 | ✅ 成功 | 188 |
cloudflare | | 414 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h3 | ✅
成功 | 191 | cloudflare | | 203 | ct.877774.xyz | 172.64.229.173 | IPv4 | h3 |
✅ 成功 | 194 | cloudflare | | 420 | 162.159.38.89 | 162.159.38.89 | IPv4 | h3 |
✅ 成功 | 199 | cloudflare | | 249 | 172.64.154.18 | 172.64.154.18 | IPv4 | h3 |
✅ 成功 | 201 | cloudflare | | 49 | rustam.ns.cloudflare.com | 108.162.195.148 |
IPv4 | h3 | ✅ 成功 | 203 | cloudflare | | 411 | bowen.ns.cloudflare.com |
2a06:98c1:50::ac40:2353 | IPv6 | h3 | ✅ 成功 | 203 | cloudflare | | 232 |
dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | h3 | ✅ 成功 | 205 |
cloudflare | | 211 | lewis.ns.cloudflare.com | 162.159.44.159 | IPv4 | h3 | ✅
成功 | 206 | cloudflare | | 164 | [2606:4700:4403::7357:544f] |
2606:4700:4403::7357:544f | IPv6 | h3 | ✅ 成功 | 207 | cloudflare | | 135 |
172.64.147.73 | 172.64.147.73 | IPv4 | h3 | ✅ 成功 | 211 | cloudflare | | 236 |
cf.877774.xyz | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 214 | cloudflare | | 301 |
moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | h3 | ✅ 成功 | 214 |
cloudflare | | 165 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 |
IPv6 | h3 | ✅ 成功 | 215 | cloudflare | | 253 | julio.ns.cloudflare.com |
172.64.35.209 | IPv4 | h3 | ✅ 成功 | 215 | cloudflare | | 298 |
moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h3 | ✅ 成功 | 215 | cloudflare
| | 415 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | h3 | ✅
成功 | 216 | cloudflare | | 408 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4
| h3 | ✅ 成功 | 219 | cloudflare | | 233 | dylan.ns.cloudflare.com |
2803:f800:50::6ca2:c3bb | IPv6 | h3 | ✅ 成功 | 221 | cloudflare | | 381 |
abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h3 | ✅ 成功 | 221 |
cloudflare | | 392 | 172.64.153.172 | 172.64.153.172 | IPv4 | h3 | ✅ 成功 | 221
| cloudflare | | 351 | 172.64.35.24 | 172.64.35.24 | IPv4 | h3 | ✅ 成功 | 222 |
cloudflare | | 53 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 |
h3 | ✅ 成功 | 223 | cloudflare | | 237 | cf.877774.xyz | 104.18.41.190 | IPv4 |
h3 | ✅ 成功 | 224 | cloudflare | | 340 | 104.18.37.40 | 104.18.37.40 | IPv4 |
h3 | ✅ 成功 | 224 | cloudflare | | 258 | 172.64.49.165 | 172.64.49.165 | IPv4 |
h3 | ✅ 成功 | 227 | cloudflare | | 204 | huxley.ns.cloudflare.com |
108.162.195.188 | IPv4 | h3 | ✅ 成功 | 228 | cloudflare | | 231 |
dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h3 | ✅ 成功 | 228 | cloudflare
| | 370 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h3 | ✅ 成功 | 228 |
cloudflare | | 42 | www.wto.org | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 230 |
cloudflare | | 254 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 |
h3 | ✅ 成功 | 230 | cloudflare | | 304 | otto.ns.cloudflare.com | 172.64.35.135
| IPv4 | h3 | ✅ 成功 | 231 | cloudflare | | 383 | abdullah.ns.cloudflare.com |
2606:4700:58::a29f:2ccb | IPv6 | h3 | ✅ 成功 | 231 | cloudflare | | 110 |
braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | h3 | ✅ 成功 | 232 |
cloudflare | | 140 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6
| h3 | ✅ 成功 | 232 | cloudflare | | 210 | lewis.ns.cloudflare.com |
108.162.195.159 | IPv4 | h3 | ✅ 成功 | 232 | cloudflare | | 224 |
damien.ns.cloudflare.com | 162.159.44.168 | IPv4 | h3 | ✅ 成功 | 232 |
cloudflare | | 234 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 |
h3 | ✅ 成功 | 234 | cloudflare | | 364 | pranab.ns.cloudflare.com |
162.159.44.199 | IPv4 | h3 | ✅ 成功 | 234 | cloudflare | | 168 |
craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h3 | ✅ 成功 | 235 | cloudflare
| | 322 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | h3 | ✅
成功 | 235 | cloudflare | | 213 | lewis.ns.cloudflare.com |
2606:4700:58::a29f:2c9f | IPv6 | h3 | ✅ 成功 | 236 | cloudflare | | 238 |
cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | h3 | ✅ 成功 | 236 |
cloudflare | | 240 | 172.64.159.6 | 172.64.159.6 | IPv4 | h3 | ✅ 成功 | 236 |
cloudflare | | 39 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 237 |
cloudflare | | 215 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 |
h3 | ✅ 成功 | 237 | cloudflare | | 116 | benedict.ns.cloudflare.com |
172.64.35.205 | IPv4 | h3 | ✅ 成功 | 238 | cloudflare | | 138 |
sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h3 | ✅ 成功 | 238 |
cloudflare | | 419 | 172.64.40.9 | 172.64.40.9 | IPv4 | h3 | ✅ 成功 | 238 |
cloudflare | | 109 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 |
h3 | ✅ 成功 | 239 | cloudflare | | 118 | benedict.ns.cloudflare.com |
2803:f800:50::6ca2:c3cd | IPv6 | h3 | ✅ 成功 | 239 | cloudflare | | 251 |
julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h3 | ✅ 成功 | 239 |
cloudflare | | 412 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h3 | ✅
成功 | 239 | cloudflare | | 136 | sullivan.ns.cloudflare.com | 108.162.195.161 |
IPv4 | h3 | ✅ 成功 | 241 | cloudflare | | 170 | craig.ns.cloudflare.com |
2803:f800:50::6ca2:c3c0 | IPv6 | h3 | ✅ 成功 | 241 | cloudflare | | 367 |
pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | h3 | ✅ 成功 | 242 |
cloudflare | | 368 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 |
h3 | ✅ 成功 | 242 | cloudflare | | 114 | benedict.ns.cloudflare.com |
108.162.195.205 | IPv4 | h3 | ✅ 成功 | 244 | cloudflare | | 172 | 104.18.39.196
| 104.18.39.196 | IPv4 | h3 | ✅ 成功 | 244 | cloudflare | | 248 | 172.64.146.16
| 172.64.146.16 | IPv4 | h3 | ✅ 成功 | 244 | cloudflare | | 333 |
[2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | h3 | ✅ 成功 |
244 | cloudflare | | 366 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 |
IPv6 | h3 | ✅ 成功 | 244 | cloudflare | | 386 | ashton.ns.cloudflare.com |
172.64.35.173 | IPv4 | h3 | ✅ 成功 | 244 | cloudflare | | 208 |
huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | h3 | ✅ 成功 | 245 |
cloudflare | | 382 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h3 |
✅ 成功 | 245 | cloudflare | | 416 | kyree.ns.cloudflare.com |
2a06:98c1:50::ac40:23cf | IPv6 | h3 | ✅ 成功 | 245 | cloudflare | | 52 |
rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | h3 | ✅ 成功 | 246 |
cloudflare | | 206 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h3 | ✅
成功 | 246 | cloudflare | | 227 | damien.ns.cloudflare.com |
2803:f800:50::6ca2:c3a8 | IPv6 | h3 | ✅ 成功 | 246 | cloudflare | | 228 |
damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | h3 | ✅ 成功 | 246 |
cloudflare | | 51 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h3 | ✅
成功 | 247 | cloudflare | | 175 | [2606:4700:440f::53aa:4126] |
2606:4700:440f::53aa:4126 | IPv6 | h3 | ✅ 成功 | 247 | cloudflare | | 209 |
huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | h3 | ✅ 成功 | 247 |
cloudflare | | 212 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h3 | ✅
成功 | 247 | cloudflare | | 325 | decker.ns.cloudflare.com | 172.64.35.155 |
IPv4 | h3 | ✅ 成功 | 247 | cloudflare | | 372 | wilson.ns.cloudflare.com |
2606:4700:58::a29f:2c6e | IPv6 | h3 | ✅ 成功 | 247 | cloudflare | | 417 |
kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | h3 | ✅ 成功 | 247 |
cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 0 条记录
- **正常 (100-200ms)**: 27 条记录
- **慢 (200-500ms)**: 73 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 13 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 12 次失败
- **h2**: 1 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
