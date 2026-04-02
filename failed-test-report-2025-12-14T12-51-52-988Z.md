# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/14 20:51:52
- **数据来源**: connectivity_results-20251214-125142.json
- **总测试数**: 450
- **失败测试数**: 168
- **成功测试数**: 282
- **失败率**: 37.33%
- **平均延迟**: 975.92ms
- **最小延迟**: 168ms
- **最大延迟**: 7400ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/14 20:51:53
- **IP地址**: 101.83.152.37
- **国家/地区**: China (CN)
- **ASN**: AS4812
- **网络组织**: China Telecom Group
- **网络域名**: chinatelecom.com.cn
- **大洲**: Asia (AS)
- **地理坐标**: 31.2222, 121.4581
- **时区**: Asia/Shanghai
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 161 次 (95.8%)
- **连接超时: I/O超时**: 7 次 (4.2%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (161 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
| ---- | --------- | ------ | ------ | ---- | ------ | -------- | ------ | -------- |

#### 连接超时: I/O超时 (7 次测试)

| 序号 | 主机/域名                | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                  |
| ---- | ------------------------ | --------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------- |
| 105  | trevor.ns.cloudflare.com | 4444.cloudflare.182682.xyz | IPv4   | none | N/A    | 0        | N/A    | dial tcp 4444.cloudflare.182682.xyz:443: i/o timeout |
| 273  | 141.147.185.63           | 141.147.185.63  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 141.147.185.63:443: i/o timeout  |
| 293  | cfip.xxxxxxxx.tk         | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout  |
| 299  | cfip.xxxxxxxx.tk         | 104.20.255.53   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.20.255.53:443: i/o timeout   |
| 320  | cf.877774.xyz            | cf.877774.xyz   | IPv4   | none | N/A    | 0        | N/A    | dial tcp cf.877774.xyz:443: i/o timeout   |
| 381  | ct.877774.xyz            | ct.877774.xyz   | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout   |
| 450  | 3.0.50.69                | 3.0.50.69       | IPv4   | none | N/A    | 0        | N/A    | dial tcp 3.0.50.69:443: i/o timeout       |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 161 次 (95.8%)
- **连接超时**: 7 次 (4.2%)

#### 错误模式分析

**超时集中度分析**: 共有 7 次超时，主要集中在IP段 108.162（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 168 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 7 次，IPv6失败 161 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：trevor.ns.cloudflare.com (4次),
uriah.ns.cloudflare.com (3次), bowen.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                  | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | -------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 362  | craig.ns.cloudflare.com    | 162.159.44.192  | IPv4   | h3   | ✅ 成功 | 168      | cloudflare |
| 74   | ct.877774.xyz              | 172.64.229.173  | IPv4   | h3   | ✅ 成功 | 177      | cloudflare |
| 58   | huxley.ns.cloudflare.com   | 162.159.44.188  | IPv4   | h3   | ✅ 成功 | 178      | cloudflare |
| 71   | ct.877774.xyz              | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 188      | cloudflare |
| 314  | otto.ns.cloudflare.com     | 162.159.44.135  | IPv4   | h3   | ✅ 成功 | 206      | cloudflare |
| 28   | julio.ns.cloudflare.com    | 162.159.44.209  | IPv4   | h3   | ✅ 成功 | 212      | cloudflare |
| 4    | uriah.ns.cloudflare.com    | 162.159.44.194  | IPv4   | h3   | ✅ 成功 | 214      | cloudflare |
| 68   | ct.877774.xyz              | 172.64.229.185  | IPv4   | h3   | ✅ 成功 | 215      | cloudflare |
| 269  | kyree.ns.cloudflare.com    | 162.159.44.207  | IPv4   | h3   | ✅ 成功 | 218      | cloudflare |
| 73   | ct.877774.xyz              | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 232      | cloudflare |
| 186  | benedict.ns.cloudflare.com | 162.159.44.205  | IPv4   | h3   | ✅ 成功 | 237      | cloudflare |
| 137  | decker.ns.cloudflare.com   | 172.64.35.155   | IPv4   | h3   | ✅ 成功 | 239      | cloudflare |
| 227  | sullivan.ns.cloudflare.com | 162.159.44.161  | IPv4   | h3   | ✅ 成功 | 244      | cloudflare |
| 390  | 162.159.36.104             | 162.159.36.104  | IPv4   | h3   | ✅ 成功 | 248      | cloudflare |
| 415  | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4   | h3   | ✅ 成功 | 249      | cloudflare |
| 363  | craig.ns.cloudflare.com    | 172.64.35.192   | IPv4   | h3   | ✅ 成功 | 250      | cloudflare |
| 171  | na.877774.xyz              | 104.18.38.235   | IPv4   | h3   | ✅ 成功 | 253      | cloudflare |
| 357  | lewis.ns.cloudflare.com    | 162.159.44.159  | IPv4   | h3   | ✅ 成功 | 256      | cloudflare |
| 60   | huxley.ns.cloudflare.com   | 108.162.195.188 | IPv4   | h3   | ✅ 成功 | 257      | cloudflare |
| 165  | cf.090227.xyz              | 172.64.145.158  | IPv4   | h3   | ✅ 成功 | 258      | cloudflare |
| 328  | damien.ns.cloudflare.com   | 162.159.44.168  | IPv4   | h3   | ✅ 成功 | 258      | cloudflare |
| 414  | abdullah.ns.cloudflare.com | 172.64.35.203   | IPv4   | h3   | ✅ 成功 | 259      | cloudflare |
| 48   | wilson.ns.cloudflare.com   | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 263      | cloudflare |
| 24   | 172.64.154.18              | 172.64.154.18   | IPv4   | h3   | ✅ 成功 | 264      | cloudflare |
| 124  | braden.ns.cloudflare.com   | 108.162.195.169 | IPv4   | h3   | ✅ 成功 | 264      | cloudflare |
| 3    | uriah.ns.cloudflare.com    | 108.162.195.194 | IPv4   | h3   | ✅ 成功 | 265      | cloudflare |
| 215  | rustam.ns.cloudflare.com   | 172.64.35.148   | IPv4   | h3   | ✅ 成功 | 265      | cloudflare |
| 225  | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4   | h3   | ✅ 成功 | 266      | cloudflare |
| 315  | otto.ns.cloudflare.com     | 172.64.35.135   | IPv4   | h3   | ✅ 成功 | 267      | cloudflare |
| 275  | dylan.ns.cloudflare.com    | 162.159.44.187  | IPv4   | h3   | ✅ 成功 | 269      | cloudflare |
| 276  | dylan.ns.cloudflare.com    | 172.64.35.187   | IPv4   | h3   | ✅ 成功 | 270      | cloudflare |
| 442  | pranab.ns.cloudflare.com   | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 270      | cloudflare |
| 345  | www.wto.org                | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 272      | cloudflare |
| 330  | damien.ns.cloudflare.com   | 172.64.35.168   | IPv4   | h3   | ✅ 成功 | 279      | cloudflare |
| 27   | julio.ns.cloudflare.com    | 172.64.35.209   | IPv4   | h3   | ✅ 成功 | 280      | cloudflare |
| 103  | trevor.ns.cloudflare.com   | 162.159.44.154  | IPv4   | h3   | ✅ 成功 | 281      | cloudflare |
| 152  | cris.ns.cloudflare.com     | 172.64.35.202   | IPv4   | h3   | ✅ 成功 | 282      | cloudflare |
| 416  | abdullah.ns.cloudflare.com | 162.159.44.203  | IPv4   | h3   | ✅ 成功 | 282      | cloudflare |
| 9    | bowen.ns.cloudflare.com    | 162.159.44.83   | IPv4   | h3   | ✅ 成功 | 284      | cloudflare |
| 49   | wilson.ns.cloudflare.com   | 162.159.44.110  | IPv4   | h3   | ✅ 成功 | 284      | cloudflare |
| 426  | ashton.ns.cloudflare.com   | 162.159.44.173  | IPv4   | h3   | ✅ 成功 | 284      | cloudflare |
| 424  | ashton.ns.cloudflare.com   | 108.162.195.173 | IPv4   | h3   | ✅ 成功 | 293      | cloudflare |
| 101  | 104.18.37.13               | 104.18.37.13    | IPv4   | h3   | ✅ 成功 | 295      | cloudflare |
| 104  | trevor.ns.cloudflare.com   | 172.64.35.154   | IPv4   | h3   | ✅ 成功 | 295      | cloudflare |
| 361  | craig.ns.cloudflare.com    | 108.162.195.192 | IPv4   | h3   | ✅ 成功 | 298      | cloudflare |
| 5    | uriah.ns.cloudflare.com    | 172.64.35.194   | IPv4   | h3   | ✅ 成功 | 299      | cloudflare |
| 172  | moura.ns.cloudflare.com    | 108.162.195.217 | IPv4   | h3   | ✅ 成功 | 301      | cloudflare |
| 59   | huxley.ns.cloudflare.com   | 172.64.35.188   | IPv4   | h3   | ✅ 成功 | 309      | cloudflare |
| 153  | cris.ns.cloudflare.com     | 162.159.44.202  | IPv4   | h3   | ✅ 成功 | 316      | cloudflare |
| 216  | rustam.ns.cloudflare.com   | 162.159.44.148  | IPv4   | h3   | ✅ 成功 | 327      | cloudflare |
| 70   | ct.877774.xyz              | 172.64.229.217  | IPv4   | h3   | ✅ 成功 | 328      | cloudflare |
| 226  | sullivan.ns.cloudflare.com | 172.64.35.161   | IPv4   | h3   | ✅ 成功 | 335      | cloudflare |
| 448  | saas.sin.fan               | 162.159.36.20   | IPv4   | h3   | ✅ 成功 | 337      | cloudflare |
| 125  | braden.ns.cloudflare.com   | 172.64.35.169   | IPv4   | h3   | ✅ 成功 | 339      | cloudflare |
| 72   | ct.877774.xyz              | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 340      | cloudflare |
| 99   | 34.143.159.175             | 34.143.159.175  | IPv4   | h2   | ✅ 成功 | 348      | cloudflare |
| 174  | moura.ns.cloudflare.com    | 162.159.44.217  | IPv4   | h3   | ✅ 成功 | 352      | cloudflare |
| 334  | 104.18.78.214              | 104.18.78.214   | IPv4   | h3   | ✅ 成功 | 352      | cloudflare |
| 10   | bowen.ns.cloudflare.com    | 108.162.195.83  | IPv4   | h3   | ✅ 成功 | 354      | cloudflare |
| 69   | ct.877774.xyz              | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 355      | cloudflare |
| 173  | moura.ns.cloudflare.com    | 172.64.35.217   | IPv4   | h3   | ✅ 成功 | 355      | cloudflare |
| 1    | 104.18.42.26               | 104.18.42.26    | IPv4   | h3   | ✅ 成功 | 356      | cloudflare |
| 88   | 104.18.39.196              | 104.18.39.196   | IPv4   | h3   | ✅ 成功 | 363      | cloudflare |
| 234  | www.okcupid.com            | 104.16.144.63   | IPv4   | h3   | ✅ 成功 | 363      | cloudflare |
| 261  | cmcc.877774.xyz            | 104.16.148.12   | IPv4   | h3   | ✅ 成功 | 370      | cloudflare |
| 188  | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4   | h3   | ✅ 成功 | 375      | cloudflare |
| 22   | www.visa.com.hk            | 104.18.20.69    | IPv4   | h3   | ✅ 成功 | 379      | cloudflare |
| 441  | www.visa.com.sg            | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 379      | cloudflare |
| 300  | cfip.xxxxxxxx.tk           | 190.93.247.169  | IPv4   | h3   | ✅ 成功 | 383      | cloudflare |
| 46   | asia.877774.xyz            | 104.16.211.153  | IPv4   | h3   | ✅ 成功 | 384      | cloudflare |
| 75   | ct.877774.xyz              | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 384      | cloudflare |
| 437  | yx-auto.pages.dev          | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 384      | cloudflare |
| 443  | pranab.ns.cloudflare.com   | 162.159.44.199  | IPv4   | h3   | ✅ 成功 | 385      | cloudflare |
| 184  | www.glassdoor.com          | 104.16.25.46    | IPv4   | h3   | ✅ 成功 | 387      | cloudflare |
| 111  | 103.160.204.59             | 103.160.204.59  | IPv4   | h3   | ✅ 成功 | 388      | cloudflare |
| 274  | dylan.ns.cloudflare.com    | 108.162.195.187 | IPv4   | h3   | ✅ 成功 | 389      | cloudflare |
| 42   | bestcf.030101.xyz          | 104.19.61.113   | IPv4   | h3   | ✅ 成功 | 390      | cloudflare |
| 65   | www.pcmag.com              | 104.16.20.118   | IPv4   | h3   | ✅ 成功 | 400      | cloudflare |
| 329  | damien.ns.cloudflare.com   | 108.162.195.168 | IPv4   | h3   | ✅ 成功 | 402      | cloudflare |
| 118  | 104.18.254.88              | 104.18.254.88   | IPv4   | h3   | ✅ 成功 | 405      | cloudflare |
| 295  | cfip.xxxxxxxx.tk           | 104.16.232.223  | IPv4   | h3   | ✅ 成功 | 406      | cloudflare |
| 33   | cf.zhetengsha.eu.org       | 104.18.42.98    | IPv4   | h3   | ✅ 成功 | 411      | cloudflare |
| 409  | ip.gs                      | 104.21.14.176   | IPv4   | h3   | ✅ 成功 | 412      | cloudflare |
| 164  | cf.090227.xyz              | 104.18.42.98    | IPv4   | h3   | ✅ 成功 | 413      | cloudflare |
| 50   | wilson.ns.cloudflare.com   | 172.64.35.110   | IPv4   | h3   | ✅ 成功 | 415      | cloudflare |
| 23   | www.7749tv.com             | 104.19.133.4    | IPv4   | h3   | ✅ 成功 | 417      | cloudflare |
| 29   | julio.ns.cloudflare.com    | 108.162.195.209 | IPv4   | h3   | ✅ 成功 | 417      | cloudflare |
| 313  | otto.ns.cloudflare.com     | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 418      | cloudflare |
| 221  | www.ipchicken.com          | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 423      | cloudflare |
| 268  | kyree.ns.cloudflare.com    | 108.162.195.207 | IPv4   | h3   | ✅ 成功 | 423      | cloudflare |
| 425  | ashton.ns.cloudflare.com   | 172.64.35.173   | IPv4   | h3   | ✅ 成功 | 423      | cloudflare |
| 346  | www.wto.org                | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 425      | cloudflare |
| 233  | www.okcupid.com            | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 432      | cloudflare |
| 303  | www.udemy.com              | 104.16.143.237  | IPv4   | h3   | ✅ 成功 | 432      | cloudflare |
| 214  | rustam.ns.cloudflare.com   | 108.162.195.148 | IPv4   | h3   | ✅ 成功 | 435      | cloudflare |
| 289  | cfip.xxxxxxxx.tk           | 104.21.91.19    | IPv4   | h3   | ✅ 成功 | 436      | cloudflare |
| 371  | eur.877774.xyz             | 104.21.47.209   | IPv4   | h3   | ✅ 成功 | 436      | cloudflare |
| 154  | cris.ns.cloudflare.com     | 108.162.195.202 | IPv4   | h3   | ✅ 成功 | 437      | cloudflare |
| 138  | decker.ns.cloudflare.com   | 162.159.44.155  | IPv4   | h3   | ✅ 成功 | 440      | cloudflare |
| 393  | 108.162.198.54             | 108.162.198.54  | IPv4   | h3   | ✅ 成功 | 442      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 0 条记录
- **正常 (100-200ms)**: 4 条记录
- **慢 (200-500ms)**: 96 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 7 次
- **IPv6 失败**: 161 次

### 按协议统计

- **none**: 168 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
