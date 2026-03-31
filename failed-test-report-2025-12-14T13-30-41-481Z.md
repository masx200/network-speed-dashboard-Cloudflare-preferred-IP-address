# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/14 13:30:41
- **数据来源**: connectivity_results-20251214-133040.json
- **总测试数**: 444
- **失败测试数**: 167
- **成功测试数**: 277
- **失败率**: 37.61%
- **平均延迟**: 107.60ms
- **最小延迟**: 45ms
- **最大延迟**: 818ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/14 13:30:41
- **IP地址**: 20.42.42.212
- **国家/地区**: 美国 (US)
- **ASN**: 8075
- **网络组织**: Microsoft Corporation
- **网络域名**: microsoft.com
- **大洲**: North America (NA)
- **地理坐标**: 38.7095, -78.1539
- **时区**: America/New_York
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 164 次 (98.2%)
- **连接超时: I/O超时**: 3 次 (1.8%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (164 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
| ---- | --------- | ------ | ------ | ---- | ------ | -------- | ------ | -------- |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 349  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 427  | cf.877774.xyz    | cf.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp cf.877774.xyz:443: i/o timeout  |
| 444  | ct.877774.xyz    | ct.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 164 次 (98.2%)
- **连接超时**: 3 次 (1.8%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 167 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 164 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：wilson.ns.cloudflare.com (3次),
huxley.ns.cloudflare.com (3次), iplocation.io
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 321  | julio.ns.cloudflare.com               | 172.64.35.209   | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 120  | cu.877774.xyz                         | 104.26.4.115    | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 351  | cfip.xxxxxxxx.tk                      | 104.16.232.223  | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 54   | iplocation.io                         | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 268  | www.ipchicken.com                     | 104.26.7.112    | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 375  | gamer.com.tw                          | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 177  | decker.ns.cloudflare.com              | 172.64.35.155   | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 386  | icook.tw                              | 172.66.158.115  | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 103  | www.4chan.org                         | 104.16.229.229  | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 359  | tasteatlas.com                        | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 81   | cmcc.877774.xyz                       | 104.16.149.5    | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 8    | www.ipget.net                         | 172.67.207.26   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 284  | www.glassdoor.com                     | 104.17.64.70    | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 86   | cmcc.877774.xyz                       | 104.16.149.10   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 318  | 104.18.42.26                          | 104.18.42.26    | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 346  | cfip.xxxxxxxx.tk                      | 188.114.96.125  | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 3    | www.7749tv.com                        | 104.19.133.4    | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 221  | xn--b6gac.eu.org                      | 104.21.90.78    | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 246  | asia.877774.xyz                       | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 35   | ct.877774.xyz                         | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 56   | iplocation.io                         | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 95   | singapore.com                         | 172.67.75.194   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 424  | www.wto.org                           | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 73   | cmcc.877774.xyz                       | 104.16.148.10   | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 215  | braden.ns.cloudflare.com              | 108.162.195.169 | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 354  | whatismyipaddress.com                 | 104.19.223.79   | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 416  | damien.ns.cloudflare.com              | 108.162.195.168 | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 433  | eur.877774.xyz                        | 104.21.47.209   | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 34   | ct.877774.xyz                         | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 40   | ipinfo.in                             | 172.67.198.203  | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 44   | comicabc.com                          | 104.21.64.10    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 48   | 104.26.6.112                          | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 310  | ashton.ns.cloudflare.com              | 108.162.195.173 | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 415  | damien.ns.cloudflare.com              | 172.64.35.168   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 16   | wilson.ns.cloudflare.com              | 172.64.35.110   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 29   | www.pcmag.com                         | 104.16.21.118   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 61   | 103.160.204.59                        | 103.160.204.59  | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 66   | cmcc.877774.xyz                       | 104.16.148.3    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 293  | ip.sb                                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 194  | 104.16.223.179                        | 104.16.223.179  | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 216  | braden.ns.cloudflare.com              | 172.64.35.169   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 278  | cf.090227.xyz                         | 172.64.144.82   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 79   | cmcc.877774.xyz                       | 104.16.149.3    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 83   | cmcc.877774.xyz                       | 104.16.149.7    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 118  | cu.877774.xyz                         | 104.26.4.113    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 185  | www.whatismyip.com                    | 104.26.13.23    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 213  | freeyx.cloudflare88.eu.org            | 141.101.120.18  | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 261  | rustam.ns.cloudflare.com              | 108.162.195.148 | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 291  | 104.17.68.85                          | 104.17.68.85    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 428  | stock.hostmonit.com                   | 104.21.7.193    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 82   | cmcc.877774.xyz                       | 104.16.149.6    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 222  | xn--b6gac.eu.org                      | 172.67.153.253  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 78   | cmcc.877774.xyz                       | 104.16.149.2    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 260  | rustam.ns.cloudflare.com              | 172.64.35.148   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 282  | shopify.com                           | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 301  | 456.cloudflare.182682.xyz             | 172.67.75.208   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 131  | cf.877771.xyz                         | 104.21.80.180   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 167  | na.877774.xyz                         | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 171  | toy-people.com                        | 104.26.2.36     | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 241  | bestcf.030101.xyz                     | 104.19.42.208   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 270  | benedict.ns.cloudflare.com            | 108.162.195.205 | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 50   | steamdb.info                          | 104.20.34.212   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 116  | cu.877774.xyz                         | 104.26.4.111    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 162  | pranab.ns.cloudflare.com              | 172.64.35.199   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 232  | bowen.ns.cloudflare.com               | 172.64.35.83    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 289  | 162.159.36.104                        | 162.159.36.104  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 316  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 109  | www.hugedomains.com                   | 104.26.6.37     | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 382  | www.digitalocean.com                  | 104.19.174.68   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 23   | huxley.ns.cloudflare.com              | 172.64.35.188   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 135  | www.okcupid.com                       | 104.16.239.254  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 176  | decker.ns.cloudflare.com              | 108.162.195.155 | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 209  | kyree.ns.cloudflare.com               | 172.64.35.207   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 364  | www.udemy.com                         | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 15   | wilson.ns.cloudflare.com              | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 36   | ct.877774.xyz                         | 172.64.229.217  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 85   | cmcc.877774.xyz                       | 104.16.149.9    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 145  | craig.ns.cloudflare.com               | 108.162.195.192 | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 157  | cris.ns.cloudflare.com                | 108.162.195.202 | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 191  | cloudflare-ip.mofashi.ltd             | 104.21.72.233   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 163  | pranab.ns.cloudflare.com              | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 45   | comicabc.com                          | 172.67.174.21   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 276  | 172.67.75.172                         | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 298  | saas.sin.fan                          | 162.159.36.20   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 369  | cf.877774.xyz                         | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 372  | 172.67.79.211                         | 172.67.79.211   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 119  | cu.877774.xyz                         | 104.26.4.114    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 201  | 104.17.79.11                          | 104.17.79.11    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 334  | 162.159.133.85                        | 162.159.133.85  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 338  | uriah.ns.cloudflare.com               | 108.162.195.194 | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 403  | lewis.ns.cloudflare.com               | 172.64.35.159   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 94   | singapore.com                         | 104.26.12.140   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 104  | www.4chan.org                         | 104.16.228.229  | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 136  | www.okcupid.com                       | 104.18.160.63   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 150  | www.visa.com.sg                       | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 249  | time.is                               | 104.26.12.54    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 69   | cmcc.877774.xyz                       | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 87   | cmcc.877774.xyz                       | 104.16.149.11   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 152  | yx-auto.pages.dev                     | 172.66.44.144   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 285  | palera.in                             | 104.21.58.72    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 6 条记录
- **快 (50-100ms)**: 94 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 3 次
- **IPv6 失败**: 164 次

### 按协议统计

- **none**: 167 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
