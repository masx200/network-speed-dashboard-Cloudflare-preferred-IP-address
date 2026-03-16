# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/12 12:11:16
- **数据来源**: connectivity_results-20251212-121115.json
- **总测试数**: 443
- **失败测试数**: 167
- **成功测试数**: 276
- **失败率**: 37.70%
- **平均延迟**: 79.38ms
- **最小延迟**: 34ms
- **最大延迟**: 2500ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/12 12:11:16
- **IP地址**: 172.184.173.244
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

- **网络不可达: 网络不可达**: 164 次 (98.2%)
- **连接超时: I/O超时**: 3 次 (1.8%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (164 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
| ---- | --------- | ------ | ------ | ---- | ------ | -------- | ------ | -------- |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 440  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 442  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |
| 443  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |

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

**问题主机分析**: 以下主机出现多次失败：decker.ns.cloudflare.com (3次),
kyree.ns.cloudflare.com (3次), dylan.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 365  | www.hugedomains.com                   | 104.26.7.37     | IPv4   | h3   | ✅ 成功 | 34       | cloudflare |
| 137  | ip.sb                                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 35       | cloudflare |
| 230  | iplocation.io                         | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 36       | cloudflare |
| 275  | 104.18.42.26                          | 104.18.42.26    | IPv4   | h3   | ✅ 成功 | 36       | cloudflare |
| 322  | www.digitalocean.com                  | 104.19.173.68   | IPv4   | h3   | ✅ 成功 | 36       | cloudflare |
| 418  | na.877774.xyz                         | 104.18.38.235   | IPv4   | h3   | ✅ 成功 | 36       | cloudflare |
| 28   | dylan.ns.cloudflare.com               | 108.162.195.187 | IPv4   | h3   | ✅ 成功 | 37       | cloudflare |
| 81   | www.ipchicken.com                     | 104.26.7.112    | IPv4   | h3   | ✅ 成功 | 37       | cloudflare |
| 392  | 172.67.120.0                          | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 37       | cloudflare |
| 283  | www.wto.org                           | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 38       | cloudflare |
| 420  | na.877774.xyz                         | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 38       | cloudflare |
| 107  | saas.sin.fan                          | 162.159.36.20   | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 245  | cmcc.877774.xyz                       | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 318  | tasteatlas.com                        | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 400  | cf.877771.xyz                         | 104.21.80.180   | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 423  | cris.ns.cloudflare.com                | 172.64.35.202   | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 45   | 172.64.151.55                         | 172.64.151.55   | IPv4   | h3   | ✅ 成功 | 40       | cloudflare |
| 132  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 40       | cloudflare |
| 209  | ct.877774.xyz                         | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 40       | cloudflare |
| 250  | cmcc.877774.xyz                       | 104.16.148.11   | IPv4   | h3   | ✅ 成功 | 40       | cloudflare |
| 300  | eur.877774.xyz                        | 104.21.29.164   | IPv4   | h3   | ✅ 成功 | 40       | cloudflare |
| 378  | cu.877774.xyz                         | 104.26.4.117    | IPv4   | h3   | ✅ 成功 | 40       | cloudflare |
| 390  | www.okcupid.com                       | 104.16.223.254  | IPv4   | h3   | ✅ 成功 | 40       | cloudflare |
| 113  | palera.in                             | 104.21.58.72    | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 204  | ct.877774.xyz                         | 172.64.229.173  | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 287  | 162.159.133.85                        | 162.159.133.85  | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 351  | stock.hostmonit.com                   | 172.67.187.251  | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 435  | freeyx.cloudflare88.eu.org            | 141.101.120.244 | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 438  | cfip.xxxxxxxx.tk                      | 104.16.232.223  | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 15   | kyree.ns.cloudflare.com               | 172.64.35.207   | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 44   | www.7749tv.com                        | 104.19.133.4    | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 117  | singapore.com                         | 172.67.75.194   | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 178  | cloudflare-ip.mofashi.ltd             | 104.21.72.233   | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 355  | abdullah.ns.cloudflare.com            | 108.162.195.203 | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 389  | www.okcupid.com                       | 104.16.144.63   | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 411  | cfip.1323123.xyz                      | 104.16.133.220  | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 179  | cloudflare-ip.mofashi.ltd             | 172.67.155.172  | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 219  | cf.0sm.com                            | 104.21.7.133    | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 401  | cf.877771.xyz                         | 172.67.152.183  | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 408  | 172.67.243.218                        | 172.67.243.218  | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 409  | www.visa.cn                           | 162.159.153.2   | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 206  | ct.877774.xyz                         | 172.64.229.185  | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 229  | iplocation.io                         | 104.26.10.222   | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 282  | 104.18.14.76                          | 104.18.14.76    | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 356  | abdullah.ns.cloudflare.com            | 172.64.35.203   | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 373  | cu.877774.xyz                         | 104.26.4.112    | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 377  | cu.877774.xyz                         | 104.26.4.116    | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 419  | na.877774.xyz                         | 104.18.187.25   | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 293  | japan.com                             | 172.67.70.92    | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 290  | ifconfig.co                           | 172.67.168.106  | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 253  | cmcc.877774.xyz                       | 104.16.149.1    | IPv4   | h3   | ✅ 成功 | 47       | cloudflare |
| 295  | japan.com                             | 104.26.5.60     | IPv4   | h3   | ✅ 成功 | 47       | cloudflare |
| 414  | pranab.ns.cloudflare.com              | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 47       | cloudflare |
| 405  | yx-auto.pages.dev                     | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 36   | www.whatismyip.com                    | 104.26.13.23    | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 116  | singapore.com                         | 104.26.13.140   | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 167  | ashton.ns.cloudflare.com              | 172.64.35.173   | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 246  | cmcc.877774.xyz                       | 104.16.148.7    | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 380  | www.visa.com.sg                       | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 14   | kyree.ns.cloudflare.com               | 162.159.44.207  | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 228  | shopify.com                           | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 352  | stock.hostmonit.com                   | 104.21.7.193    | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 3    | www.visa.com.hk                       | 104.18.20.69    | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 49   | asia.877774.xyz                       | 104.16.211.153  | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 51   | braden.ns.cloudflare.com              | 108.162.195.169 | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 56   | fbi.gov                               | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 99   | benedict.ns.cloudflare.com            | 108.162.195.205 | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 251  | cmcc.877774.xyz                       | 104.16.148.12   | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 127  | 456.cloudflare.182682.xyz             | 104.26.9.160    | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 223  | www.gov.ua                            | 172.67.209.127  | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 271  | huxley.ns.cloudflare.com              | 172.64.35.188   | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 338  | www.csgo.com                          | 195.85.59.95    | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 74   | time.is                               | 172.67.68.157   | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 82   | www.ipchicken.com                     | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 141  | 172.64.154.18                         | 172.64.154.18   | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 428  | toy-people.com                        | 104.26.3.36     | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 23   | cf.877774.xyz                         | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 24   | cf.877774.xyz                         | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 66   | cf.zhetengsha.eu.org                  | 104.18.35.15    | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 152  | comicabc.com                          | 104.21.64.10    | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 183  | wilson.ns.cloudflare.com              | 172.64.35.110   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 47   | asia.877774.xyz                       | 104.17.139.62   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 135  | ip.sb                                 | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 202  | ct.877774.xyz                         | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 210  | 104.18.39.196                         | 104.18.39.196   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 218  | 104.18.37.13                          | 104.18.37.13    | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 312  | www.udemy.com                         | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 335  | 172.67.181.209                        | 172.67.181.209  | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 336  | 104.19.175.123                        | 104.19.175.123  | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 19   | zread.ai                              | 172.67.202.78   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 29   | dylan.ns.cloudflare.com               | 172.64.35.187   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 80   | www.ipchicken.com                     | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 195  | ipinfo.in                             | 172.67.198.203  | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 236  | cmcc.877774.xyz                       | 104.16.149.10   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 262  | ipv4.ip.sb                            | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 376  | cu.877774.xyz                         | 104.26.4.115    | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 84   | moura.ns.cloudflare.com               | 172.64.35.217   | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 190  | trevor.ns.cloudflare.com              | 172.64.35.154   | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 203  | ct.877774.xyz                         | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 205  | ct.877774.xyz                         | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 59 条记录
- **快 (50-100ms)**: 41 条记录
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
