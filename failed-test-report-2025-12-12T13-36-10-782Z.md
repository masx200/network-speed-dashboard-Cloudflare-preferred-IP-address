# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/12 13:36:10
- **数据来源**: connectivity_results-20251212-133610.json
- **总测试数**: 441
- **失败测试数**: 166
- **成功测试数**: 275
- **失败率**: 37.64%
- **平均延迟**: 122.67ms
- **最小延迟**: 40ms
- **最大延迟**: 910ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/12 13:36:10
- **IP地址**: 172.178.117.214
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

- **网络不可达: 网络不可达**: 163 次 (98.2%)
- **连接超时: I/O超时**: 3 次 (1.8%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (163 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
| ---- | --------- | ------ | ------ | ---- | ------ | -------- | ------ | -------- |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 263  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 373  | cf.877774.xyz    | cf.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp cf.877774.xyz:443: i/o timeout  |
| 441  | ct.877774.xyz    | ct.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 163 次 (98.2%)
- **连接超时**: 3 次 (1.8%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 166 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 163 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：time.is (3次), iplocation.io (3次),
trevor.ns.cloudflare.com (3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 259  | www.ipchicken.com                     | 104.26.7.112    | IPv4   | h3   | ✅ 成功 | 40       | cloudflare |
| 184  | decker.ns.cloudflare.com              | 172.64.35.155   | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 84   | www.hugedomains.com                   | 104.26.7.37     | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 291  | 456.cloudflare.182682.xyz             | 104.26.8.160    | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 14   | ct.877774.xyz                         | 172.64.229.173  | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 260  | www.ipchicken.com                     | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 345  | tasteatlas.com                        | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 185  | decker.ns.cloudflare.com              | 108.162.195.155 | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 118  | cmcc.877774.xyz                       | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 308  | ip.gs                                 | 104.21.14.176   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 205  | bestcf.030101.xyz                     | 104.19.153.222  | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 2    | time.is                               | 104.26.12.54    | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 125  | cmcc.877774.xyz                       | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 59   | 103.160.204.59                        | 103.160.204.59  | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 380  | damien.ns.cloudflare.com              | 108.162.195.168 | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 124  | cmcc.877774.xyz                       | 104.16.148.12   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 235  | cf.zhetengsha.eu.org                  | 172.64.144.82   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 312  | whatismyipaddress.com                 | 104.19.222.79   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 104  | 172.67.120.0                          | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 116  | cmcc.877774.xyz                       | 104.16.148.4    | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 36   | iplocation.io                         | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 46   | trevor.ns.cloudflare.com              | 4444.cloudflare.182682.xyz | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 293  | 456.cloudflare.182682.xyz             | 104.26.9.160    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 321  | ashton.ns.cloudflare.com              | 172.64.35.173   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 123  | cmcc.877774.xyz                       | 104.16.148.11   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 120  | cmcc.877774.xyz                       | 104.16.148.8    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 200  | dylan.ns.cloudflare.com               | 172.64.35.187   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 45   | trevor.ns.cloudflare.com              | 172.64.35.154   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 142  | yx-auto.pages.dev                     | 172.66.44.144   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 251  | saas.sin.fan                          | 162.159.36.20   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 418  | www.wto.org                           | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 60   | shopify.com                           | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 238  | bowen.ns.cloudflare.com               | 108.162.195.83  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 99   | cu.877774.xyz                         | 104.26.4.114    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 106  | cmcc.877774.xyz                       | 104.16.149.7    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 217  | asia.877774.xyz                       | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 301  | singapore.com                         | 104.26.13.140   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 163  | na.877774.xyz                         | 104.18.187.25   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 358  | www.udemy.com                         | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 128  | cmcc.877774.xyz                       | 104.16.149.3    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 156  | pranab.ns.cloudflare.com              | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 179  | kyree.ns.cloudflare.com               | 108.162.195.207 | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 228  | braden.ns.cloudflare.com              | 108.162.195.169 | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 287  | palera.in                             | 104.21.58.72    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 234  | cf.zhetengsha.eu.org                  | 104.18.43.174   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 252  | rustam.ns.cloudflare.com              | 108.162.195.148 | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 323  | ashton.ns.cloudflare.com              | 108.162.195.173 | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 375  | icook.tw                              | 104.20.28.74    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 10   | ipinfo.in                             | 172.67.198.203  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 151  | www.okcupid.com                       | 104.16.144.63   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 315  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 90   | sullivan.ns.cloudflare.com            | 172.64.35.161   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 122  | cmcc.877774.xyz                       | 104.16.148.10   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 219  | freeyx.cloudflare88.eu.org            | 141.101.120.86  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 311  | whatismyipaddress.com                 | 104.19.223.79   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 83   | www.hugedomains.com                   | 172.67.70.191   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 167  | cris.ns.cloudflare.com                | 108.162.195.202 | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 42   | 172.67.75.172                         | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 95   | cu.877774.xyz                         | 104.26.4.119    | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 192  | www.whatismyip.com                    | 104.26.13.23    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 365  | julio.ns.cloudflare.com               | 108.162.195.209 | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 371  | 172.64.35.24                          | 172.64.35.24    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 37   | iplocation.io                         | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 38   | iplocation.io                         | 104.26.10.222   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 105  | cmcc.877774.xyz                       | 104.16.149.6    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 152  | www.okcupid.com                       | 104.16.239.254  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 220  | freeyx.cloudflare88.eu.org            | 141.101.120.187 | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 225  | fbi.gov                               | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 292  | 456.cloudflare.182682.xyz             | 172.67.75.208   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 362  | 172.67.79.211                         | 172.67.79.211   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 80   | www.4chan.org                         | 104.16.228.229  | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 438  | eur.877774.xyz                        | 104.21.26.150   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 17   | ct.877774.xyz                         | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 130  | cmcc.877774.xyz                       | 104.16.149.5    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 212  | xn--b6gac.eu.org                      | 104.21.90.78    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 302  | singapore.com                         | 104.26.12.140   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 401  | japan.com                             | 104.26.4.60     | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 15   | ct.877774.xyz                         | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 129  | cmcc.877774.xyz                       | 104.16.149.4    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 154  | www.okcupid.com                       | 104.18.160.63   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 197  | 104.17.79.11                          | 104.17.79.11    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 307  | ip.gs                                 | 172.67.160.28   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 62   | www.ipget.net                         | 104.21.15.212   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 112  | cmcc.877774.xyz                       | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 209  | zread.ai                              | 172.67.202.78   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 270  | ip.sb                                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 282  | www.glassdoor.com                     | 104.16.25.46    | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 19   | ct.877774.xyz                         | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 53   | wilson.ns.cloudflare.com              | 172.64.35.110   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 190  | www.whatismyip.com                    | 172.67.69.129   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 222  | 172.64.151.55                         | 172.64.151.55   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 27   | www.gov.ua                            | 104.21.23.72    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 115  | cmcc.877774.xyz                       | 104.16.148.3    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 338  | uriah.ns.cloudflare.com               | 172.64.35.194   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 339  | uriah.ns.cloudflare.com               | 108.162.195.194 | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 349  | gamer.com.tw                          | 104.18.2.197    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 398  | 104.19.175.123                        | 104.19.175.123  | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 399  | japan.com                             | 172.67.70.92    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 9    | ipinfo.in                             | 104.21.21.129   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 286  | 104.17.68.85                          | 104.17.68.85    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 3 条记录
- **快 (50-100ms)**: 79 条记录
- **正常 (100-200ms)**: 18 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 3 次
- **IPv6 失败**: 163 次

### 按协议统计

- **none**: 166 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
