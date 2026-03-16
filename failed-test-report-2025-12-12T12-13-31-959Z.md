# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/12 12:13:31
- **数据来源**: connectivity_results-20251212-121331.json
- **总测试数**: 445
- **失败测试数**: 168
- **成功测试数**: 277
- **失败率**: 37.75%
- **平均延迟**: 102.75ms
- **最小延迟**: 67ms
- **最大延迟**: 559ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/12 12:13:31
- **IP地址**: 128.24.163.36
- **国家/地区**: United States (US)
- **ASN**: AS8075
- **网络组织**: Microsoft Corporation
- **网络域名**: microsoft.com
- **大洲**: North America (NA)
- **数据源**: ipinfo.io

---

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 165 次 (98.2%)
- **连接超时: I/O超时**: 3 次 (1.8%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (165 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
| ---- | --------- | ------ | ------ | ---- | ------ | -------- | ------ | -------- |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 312  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 443  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |
| 445  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 165 次 (98.2%)
- **连接超时**: 3 次 (1.8%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 168 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 165 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：wilson.ns.cloudflare.com (3次),
trevor.ns.cloudflare.com (3次), huxley.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 132  | www.okcupid.com                       | 104.18.160.63   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 248  | saas.sin.fan                          | 104.26.1.111    | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 420  | 108.162.198.54                        | 108.162.198.54  | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 216  | toy-people.com                        | 172.67.72.18    | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 241  | na.877774.xyz                         | 104.18.38.235   | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 388  | 104.18.78.214                         | 104.18.78.214   | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 55   | cmcc.877774.xyz                       | 104.16.148.9    | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 375  | 104.18.37.40                          | 104.18.37.40    | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 252  | bowen.ns.cloudflare.com               | 172.64.35.83    | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 384  | www.digitalocean.com                  | 104.19.174.68   | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 108  | www.4chan.org                         | 104.16.228.229  | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 122  | cu.877774.xyz                         | 104.26.4.111    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 221  | xn--b6gac.eu.org                      | 104.21.90.78    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 185  | zread.ai                              | 188.114.97.3    | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 214  | toy-people.com                        | 104.26.3.36     | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 225  | cf.zhetengsha.eu.org                  | 104.18.35.15    | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 286  | ip.sb                                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 364  | www.udemy.com                         | 104.16.143.237  | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 151  | www.visa.com.sg                       | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 152  | www.visa.com.sg                       | 104.18.12.229   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 231  | braden.ns.cloudflare.com              | 108.162.195.169 | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 345  | uriah.ns.cloudflare.com               | 108.162.195.194 | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 350  | julio.ns.cloudflare.com               | 172.64.35.209   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 398  | damien.ns.cloudflare.com              | 108.162.195.168 | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 441  | www.csgo.com                          | 195.85.59.95    | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 6    | comicabc.com                          | 104.21.64.10    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 377  | icook.tw                              | 104.20.28.74    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 410  | eur.877774.xyz                        | 104.21.29.164   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 30   | ct.877774.xyz                         | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 43   | cmcc.877774.xyz                       | 104.16.149.10   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 274  | www.ipchicken.com                     | 104.26.7.112    | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 295  | 104.16.61.163                         | 104.16.61.163   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 195  | www.whatismyip.com                    | 172.67.69.129   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 291  | palera.in                             | 172.67.157.122  | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 327  | 172.67.106.26                         | 172.67.106.26   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 337  | dnschecker.org                        | 104.26.6.89     | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 53   | cmcc.877774.xyz                       | 104.16.148.7    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 134  | www.okcupid.com                       | 104.16.223.254  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 296  | 162.159.36.104                        | 162.159.36.104  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 44   | cmcc.877774.xyz                       | 104.16.149.11   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 262  | time.is                               | 104.26.13.54    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 391  | otto.ns.cloudflare.com                | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 336  | dnschecker.org                        | 104.26.7.89     | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 161  | cris.ns.cloudflare.com                | 172.64.35.202   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 236  | fbi.gov                               | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 247  | saas.sin.fan                          | 104.26.0.111    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 330  | ashton.ns.cloudflare.com              | 172.64.35.173   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 368  | tasteatlas.com                        | 104.17.37.105   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 124  | cu.877774.xyz                         | 104.26.4.113    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 139  | craig.ns.cloudflare.com               | 172.64.35.192   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 292  | palera.in                             | 104.21.58.72    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 19   | trevor.ns.cloudflare.com              | 172.64.35.154   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 45   | cmcc.877774.xyz                       | 104.16.149.12   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 155  | pranab.ns.cloudflare.com              | 172.64.35.199   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 264  | time.is                               | 172.67.68.157   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 281  | benedict.ns.cloudflare.com            | 108.162.195.205 | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 442  | www.csgo.com                          | 195.85.59.161   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 29   | ct.877774.xyz                         | 172.64.229.173  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 74   | 172.67.110.232                        | 172.67.110.232  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 99   | ipinfo.in                             | 172.67.198.203  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 369  | tasteatlas.com                        | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 435  | abdullah.ns.cloudflare.com            | 108.162.195.203 | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 2    | www.pcmag.com                         | 104.16.21.118   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 38   | steamdb.info                          | 172.66.175.250  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 133  | www.okcupid.com                       | 104.16.144.63   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 141  | craig.ns.cloudflare.com               | 108.162.195.192 | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 306  | 456.cloudflare.182682.xyz             | 104.26.9.160    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 35   | 104.18.39.196                         | 104.18.39.196   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 244  | cf.090227.xyz                         | 172.64.152.241  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 277  | www.glassdoor.com                     | 104.17.64.70    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 17   | trevor.ns.cloudflare.com              | 108.162.195.154 | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 23   | cf.0sm.com                            | 104.21.7.133    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 32   | ct.877774.xyz                         | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 49   | cmcc.877774.xyz                       | 104.16.148.3    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 12   | wilson.ns.cloudflare.com              | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 65   | cmcc.877774.xyz                       | 104.16.149.6    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 98   | ipv4.ip.sb                            | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 105  | www.gov.ua                            | 104.21.23.72    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 148  | yx-auto.pages.dev                     | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 304  | 456.cloudflare.182682.xyz             | 172.67.75.208   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 315  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 409  | 172.67.181.209                        | 172.67.181.209  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 428  | www.wto.org                           | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 232  | braden.ns.cloudflare.com              | 172.64.35.169   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 68   | 104.18.37.13                          | 104.18.37.13    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 123  | cu.877774.xyz                         | 104.26.4.112    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 257  | moura.ns.cloudflare.com               | 108.162.195.217 | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 373  | gamer.com.tw                          | 104.18.2.197    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 419  | 104.19.175.123                        | 104.19.175.123  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 89   | iplocation.io                         | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 96   | ipv4.ip.sb                            | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 110  | www.hugedomains.com                   | 172.67.70.191   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 147  | yx-auto.pages.dev                     | 172.66.44.144   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 169  | decker.ns.cloudflare.com              | 172.64.35.155   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 196  | www.whatismyip.com                    | 104.26.13.23    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 215  | toy-people.com                        | 104.26.2.36     | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 276  | www.ipchicken.com                     | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 311  | cfip.xxxxxxxx.tk                      | 104.16.232.223  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 423  | japan.com                             | 104.26.4.60     | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 60   | cmcc.877774.xyz                       | 104.16.149.1    | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 100 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 3 次
- **IPv6 失败**: 165 次

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
