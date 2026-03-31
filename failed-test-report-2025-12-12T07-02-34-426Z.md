# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/12 07:02:34
- **数据来源**: connectivity_results-20251212-070234.json
- **总测试数**: 457
- **失败测试数**: 176
- **成功测试数**: 281
- **失败率**: 38.51%
- **平均延迟**: 81.40ms
- **最小延迟**: 37ms
- **最大延迟**: 742ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/12 07:02:34
- **IP地址**: 135.232.225.18
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

- **网络不可达: 网络不可达**: 173 次 (98.3%)
- **连接超时: I/O超时**: 3 次 (1.7%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (173 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
| ---- | --------- | ------ | ------ | ---- | ------ | -------- | ------ | -------- |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 455  | ct.877774.xyz    | ct.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout  |
| 456  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 457  | cf.877774.xyz    | cf.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp cf.877774.xyz:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 173 次 (98.3%)
- **连接超时**: 3 次 (1.7%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 172.64（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 176 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 173 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：time.is (3次), wilson.ns.cloudflare.com
(3次), rustam.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 277  | icook.hk                              | 172.67.161.104  | IPv4   | h3   | ✅ 成功 | 37       | cloudflare |
| 184  | japan.com                             | 104.26.5.60     | IPv4   | h3   | ✅ 成功 | 38       | cloudflare |
| 204  | eur.877774.xyz                        | 104.21.26.150   | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 207  | abdullah.ns.cloudflare.com            | 108.162.195.203 | IPv4   | h3   | ✅ 成功 | 40       | cloudflare |
| 350  | www.visa.cn                           | 162.159.153.2   | IPv4   | h3   | ✅ 成功 | 40       | cloudflare |
| 165  | otto.ns.cloudflare.com                | 172.64.35.135   | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 250  | 104.16.45.84                          | 104.16.45.84    | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 356  | cu.877774.xyz                         | 104.26.4.113    | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 423  | bowen.ns.cloudflare.com               | 108.162.195.83  | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 138  | gamer.com.tw                          | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 257  | shopify.com                           | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 361  | cu.877774.xyz                         | 104.26.4.118    | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 220  | www.pcmag.com                         | 104.16.21.118   | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 392  | www.whatismyip.com                    | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 430  | fbi.gov                               | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 343  | pranab.ns.cloudflare.com              | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 304  | cmcc.877774.xyz                       | 104.16.149.7    | IPv4   | h3   | ✅ 成功 | 47       | cloudflare |
| 416  | 104.16.223.179                        | 104.16.223.179  | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 444  | saas.sin.fan                          | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 327  | www.okcupid.com                       | 104.16.144.63   | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 158  | 104.18.37.40                          | 104.18.37.40    | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 246  | www.gov.ua                            | 172.67.209.127  | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 251  | iplocation.io                         | 104.26.10.222   | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 360  | cu.877774.xyz                         | 104.26.4.117    | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 94   | dnschecker.org                        | 104.26.7.89     | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 278  | icook.hk                              | 104.21.90.210   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 302  | cmcc.877774.xyz                       | 104.16.149.5    | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 88   | ae8a9c24-83de.masx200.ddns-ip.net     | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 241  | ct.877774.xyz                         | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 316  | cmcc.877774.xyz                       | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 373  | toy-people.com                        | 104.26.3.36     | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 183  | japan.com                             | 172.67.70.92    | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 245  | www.gov.ua                            | 104.21.23.72    | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 252  | iplocation.io                         | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 3    | www.ipget.net                         | 172.67.207.26   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 55   | ip.sb                                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 56   | ip.sb                                 | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 85   | whatismyipaddress.com                 | 104.19.223.79   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 148  | 104.17.142.12                         | 104.17.142.12   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 221  | www.pcmag.com                         | 104.16.20.118   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 293  | cmcc.877774.xyz                       | 104.16.148.9    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 353  | yx-auto.pages.dev                     | 104.21.6.60     | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 101  | yx-auto.pages.dev                     | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 235  | ipv4.ip.sb                            | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 240  | ct.877774.xyz                         | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 285  | www.hugedomains.com                   | 104.26.6.37     | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 306  | cmcc.877774.xyz                       | 104.16.149.9    | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 324  | www.okcupid.com                       | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 341  | www.visa.com.sg                       | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 342  | www.visa.com.sg                       | 104.18.12.229   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 351  | www.visa.cn                           | 162.159.152.2   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 421  | asia.877774.xyz                       | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 425  | bowen.ns.cloudflare.com               | 172.64.35.83    | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 141  | yx-auto.pages.dev                     | 172.67.161.98   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 237  | ct.877774.xyz                         | 172.64.229.185  | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 239  | ct.877774.xyz                         | 172.64.229.217  | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 249  | 104.18.39.196                         | 104.18.39.196   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 305  | cmcc.877774.xyz                       | 104.16.149.8    | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 386  | kyree.ns.cloudflare.com               | 108.162.195.207 | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 388  | kyree.ns.cloudflare.com               | 172.64.35.207   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 15   | time.is                               | 104.26.13.54    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 112  | 172.67.106.26                         | 172.67.106.26   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 134  | www.udemy.com                         | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 209  | abdullah.ns.cloudflare.com            | 172.64.35.203   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 296  | cmcc.877774.xyz                       | 104.16.148.12   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 334  | 172.67.120.0                          | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 47   | 104.16.61.163                         | 104.16.61.163   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 149  | 172.67.79.211                         | 172.67.79.211   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 174  | damien.ns.cloudflare.com              | 172.64.35.168   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 215  | cf.877774.xyz                         | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 225  | ipinfo.in                             | 104.21.21.129   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 266  | cf.0sm.com                            | 104.21.7.133    | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 308  | cmcc.877774.xyz                       | 104.16.149.11   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 310  | cmcc.877774.xyz                       | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 314  | cmcc.877774.xyz                       | 104.16.148.4    | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 326  | www.okcupid.com                       | 104.16.239.254  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 380  | decker.ns.cloudflare.com              | 108.162.195.155 | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 440  | cloudflare-ip.mofashi.ltd             | 104.21.72.233   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 111  | 104.18.42.26                          | 104.18.42.26    | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 231  | steamdb.info                          | 104.20.34.212   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 258  | 172.67.75.172                         | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 298  | cmcc.877774.xyz                       | 104.16.149.1    | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 23   | wilson.ns.cloudflare.com              | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 114  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 159  | 198.62.62.4                           | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 345  | pranab.ns.cloudflare.com              | 172.64.35.199   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 394  | www.whatismyip.com                    | 172.67.69.129   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 414  | freeyx.cloudflare88.eu.org            | 141.101.120.15  | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 57   | ip.sb                                 | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 63   | singapore.com                         | 172.67.75.194   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 139  | gamer.com.tw                          | 104.18.2.197    | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 162  | 104.26.13.31                          | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 163  | otto.ns.cloudflare.com                | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 267  | cf.0sm.com                            | 172.67.187.145  | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 35   | moura.ns.cloudflare.com               | 108.162.195.217 | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 80   | 456.cloudflare.182682.xyz             | 104.26.8.160    | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 102  | yx-auto.pages.dev                     | 172.66.44.144   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 145  | tasteatlas.com                        | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 150  | www.digitalocean.com                  | 104.19.174.68   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 182  | japan.com                             | 104.26.4.60     | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 20 条记录
- **快 (50-100ms)**: 80 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 3 次
- **IPv6 失败**: 173 次

### 按协议统计

- **none**: 176 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
