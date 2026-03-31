# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/12 12:11:31
- **数据来源**: connectivity_results-20251212-121131.json
- **总测试数**: 443
- **失败测试数**: 167
- **成功测试数**: 276
- **失败率**: 37.70%
- **平均延迟**: 114.93ms
- **最小延迟**: 66ms
- **最大延迟**: 563ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/12 12:11:31
- **IP地址**: 52.165.251.160
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
| 442  | cf.877774.xyz    | cf.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp cf.877774.xyz:443: i/o timeout  |
| 443  | ct.877774.xyz    | ct.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout  |

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

**问题主机分析**: 以下主机出现多次失败：time.is (3次), trevor.ns.cloudflare.com
(3次), moura.ns.cloudflare.com (3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 210  | kyree.ns.cloudflare.com               | 172.64.35.207   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 390  | stock.hostmonit.com                   | 172.67.187.251  | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 398  | eur.877774.xyz                        | 104.21.29.164   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 37   | wilson.ns.cloudflare.com              | 172.64.35.110   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 229  | bestcf.030101.xyz                     | 104.17.27.231   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 354  | 172.67.106.26                         | 172.67.106.26   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 391  | stock.hostmonit.com                   | 104.21.7.193    | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 139  | yx-auto.pages.dev                     | 172.66.44.144   | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 157  | cmcc.877774.xyz                       | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 171  | cmcc.877774.xyz                       | 104.16.149.1    | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 292  | 162.159.36.104                        | 162.159.36.104  | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 384  | 162.159.133.85                        | 162.159.133.85  | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 82   | ct.877774.xyz                         | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 411  | abdullah.ns.cloudflare.com            | 108.162.195.203 | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 81   | ct.877774.xyz                         | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 298  | singapore.com                         | 172.67.75.194   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 335  | cf.877774.xyz                         | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 77   | 172.67.110.232                        | 172.67.110.232  | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 155  | cmcc.877774.xyz                       | 104.16.149.11   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 346  | uriah.ns.cloudflare.com               | 108.162.195.194 | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 50   | ipinfo.in                             | 104.21.21.129   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 252  | braden.ns.cloudflare.com              | 172.64.35.169   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 14   | time.is                               | 104.26.12.54    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 58   | www.gov.ua                            | 172.67.209.127  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 71   | iplocation.io                         | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 167  | cmcc.877774.xyz                       | 104.16.148.10   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 226  | asia.877774.xyz                       | 104.17.139.62   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 369  | 104.18.37.40                          | 104.18.37.40    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 410  | 172.67.181.209                        | 172.67.181.209  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 129  | cu.877774.xyz                         | 104.26.4.119    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 140  | yx-auto.pages.dev                     | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 381  | tasteatlas.com                        | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 78   | shopify.com                           | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 234  | cf.zhetengsha.eu.org                  | 172.64.152.241  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 146  | freeyx.cloudflare88.eu.org            | 141.101.120.244 | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 227  | asia.877774.xyz                       | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 242  | fbi.gov                               | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 329  | julio.ns.cloudflare.com               | 172.64.35.209   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 274  | www.glassdoor.com                     | 104.16.25.46    | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 282  | ip.sb                                 | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 419  | japan.com                             | 104.26.4.60     | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 38   | wilson.ns.cloudflare.com              | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 123  | www.okcupid.com                       | 104.16.239.254  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 175  | cmcc.877774.xyz                       | 104.16.149.5    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 355  | www.digitalocean.com                  | 104.19.174.68   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 9    | www.ipget.net                         | 104.21.15.212   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 121  | www.okcupid.com                       | 104.16.144.63   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 162  | cmcc.877774.xyz                       | 104.16.148.5    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 277  | benedict.ns.cloudflare.com            | 172.64.35.205   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 334  | cf.877774.xyz                         | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 79   | 103.160.204.59                        | 103.160.204.59  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 154  | cmcc.877774.xyz                       | 104.16.149.10   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 364  | otto.ns.cloudflare.com                | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 417  | 104.19.175.123                        | 104.19.175.123  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 62   | 104.16.45.84                          | 104.16.45.84    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 119  | www.okcupid.com                       | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 132  | cu.877774.xyz                         | 104.26.4.113    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 158  | cmcc.877774.xyz                       | 104.16.148.1    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 216  | www.whatismyip.com                    | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 371  | damien.ns.cloudflare.com              | 108.162.195.168 | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 64   | ipv4.ip.sb                            | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 80   | 104.18.254.88                         | 104.18.254.88   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 94   | huxley.ns.cloudflare.com              | 108.162.195.188 | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 293  | ip.gs                                 | 104.21.14.176   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 315  | ashton.ns.cloudflare.com              | 172.64.35.173   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 348  | uriah.ns.cloudflare.com               | 172.64.35.194   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 66   | ipv4.ip.sb                            | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 67   | 172.67.75.172                         | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 70   | iplocation.io                         | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 114  | cf.877771.xyz                         | 104.21.80.180   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 118  | 172.67.120.0                          | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 238  | cf.090227.xyz                         | 172.64.152.241  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 250  | saas.sin.fan                          | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 270  | www.ipchicken.com                     | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 287  | palera.in                             | 172.67.157.122  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 19   | trevor.ns.cloudflare.com              | 108.162.195.154 | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 273  | www.glassdoor.com                     | 104.17.64.70    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 283  | ip.sb                                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 203  | dylan.ns.cloudflare.com               | 172.64.35.187   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 241  | fbi.gov                               | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 320  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 438  | cfip.xxxxxxxx.tk                      | 104.16.232.223  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 166  | cmcc.877774.xyz                       | 104.16.148.9    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 198  | decker.ns.cloudflare.com              | 108.162.195.155 | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 230  | bestcf.030101.xyz                     | 104.17.99.183   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 314  | ashton.ns.cloudflare.com              | 108.162.195.173 | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 51   | ipinfo.in                             | 172.67.198.203  | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 135  | cu.877774.xyz                         | 104.26.4.116    | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 168  | cmcc.877774.xyz                       | 104.16.148.11   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 297  | singapore.com                         | 104.26.13.140   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 328  | julio.ns.cloudflare.com               | 108.162.195.209 | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 115  | cf.877771.xyz                         | 172.67.152.183  | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 130  | cu.877774.xyz                         | 104.26.4.111    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 174  | cmcc.877774.xyz                       | 104.16.149.4    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 217  | www.whatismyip.com                    | 104.26.13.23    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 266  | rustam.ns.cloudflare.com              | 108.162.195.148 | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 143  | www.visa.com.sg                       | 104.18.12.229   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 190  | toy-people.com                        | 104.26.2.36     | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 83   | ct.877774.xyz                         | 172.64.229.173  | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 103  | www.hugedomains.com                   | 104.26.6.37     | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |

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
