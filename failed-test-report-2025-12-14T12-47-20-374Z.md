# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/14 12:47:20
- **数据来源**: connectivity_results-20251214-124712.json
- **总测试数**: 452
- **失败测试数**: 166
- **成功测试数**: 286
- **失败率**: 36.73%
- **平均延迟**: 168.74ms
- **最小延迟**: 60ms
- **最大延迟**: 679ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/14 12:47:20
- **IP地址**: 20.169.77.233
- **国家/地区**: 美国 (US)
- **ASN**: 8075
- **网络组织**: Microsoft Corporation
- **网络域名**: microsoft.com
- **大洲**: North America (NA)
- **地理坐标**: 33.4532, -112.0748
- **时区**: America/Phoenix
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 162 次 (97.6%)
- **连接超时: I/O超时**: 4 次 (2.4%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (162 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
| ---- | --------- | ------ | ------ | ---- | ------ | -------- | ------ | -------- |

#### 连接超时: I/O超时 (4 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 382  | cf.877774.xyz    | cf.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp cf.877774.xyz:443: i/o timeout  |
| 437  | ct.877774.xyz    | ct.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout  |
| 442  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 447  | cfip.xxxxxxxx.tk | 104.20.255.53  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.20.255.53:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 162 次 (97.6%)
- **连接超时**: 4 次 (2.4%)

#### 错误模式分析

**超时集中度分析**: 共有 4 次超时，主要集中在IP段 172.67（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 166 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 4 次，IPv6失败 162 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：wilson.ns.cloudflare.com (3次), time.is
(3次), trevor.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                  | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | -------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 255  | xn--b6gac.eu.org           | 104.21.90.78    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 212  | rustam.ns.cloudflare.com   | 108.162.195.148 | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 236  | benedict.ns.cloudflare.com | 162.159.44.205  | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 322  | 104.18.37.40               | 104.18.37.40    | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 115  | cu.877774.xyz              | 104.26.4.113    | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 96   | ct.877774.xyz              | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 181  | decker.ns.cloudflare.com   | 108.162.195.155 | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 408  | www.csgo.com               | 195.85.59.95    | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 86   | huxley.ns.cloudflare.com   | 172.64.35.188   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 24   | steamdb.info               | 104.20.34.212   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 28   | ipinfo.in                  | 172.67.198.203  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 31   | cf.0sm.com                 | 104.21.7.133    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 175  | toy-people.com             | 104.26.3.36     | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 16   | www.7749tv.com             | 104.19.93.88    | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 98   | ct.877774.xyz              | 172.64.229.217  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 268  | 456.cloudflare.182682.xyz  | 172.67.75.208   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 402  | abdullah.ns.cloudflare.com | 162.159.44.203  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 205  | www.whatismyip.com         | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 27   | ipinfo.in                  | 104.21.21.129   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 158  | pranab.ns.cloudflare.com   | 172.64.35.199   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 109  | cu.877774.xyz              | 104.26.4.116    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 278  | braden.ns.cloudflare.com   | 162.159.44.169  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 417  | damien.ns.cloudflare.com   | 162.159.44.168  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 213  | rustam.ns.cloudflare.com   | 162.159.44.148  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 122  | sullivan.ns.cloudflare.com | 172.64.35.161   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 180  | decker.ns.cloudflare.com   | 172.64.35.155   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 348  | moura.ns.cloudflare.com    | 162.159.44.217  | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 81   | trevor.ns.cloudflare.com   | 162.159.44.154  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 399  | 172.67.79.211              | 172.67.79.211   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 347  | moura.ns.cloudflare.com    | 108.162.195.217 | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 19   | time.is                    | 172.67.68.157   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 135  | iplocation.io              | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 217  | bestcf.030101.xyz          | 104.19.61.113   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 128  | 172.67.120.0               | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 173  | na.877774.xyz              | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 234  | www.ipchicken.com          | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 381  | eur.877774.xyz             | 104.21.47.209   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 190  | kyree.ns.cloudflare.com    | 162.159.44.207  | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 141  | 172.67.243.218             | 172.67.243.218  | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 342  | cf.090227.xyz              | 172.64.145.158  | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 451  | cfip.xxxxxxxx.tk           | 104.21.91.19    | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 241  | ip.sb                      | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 113  | cu.877774.xyz              | 104.26.4.111    | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 267  | 456.cloudflare.182682.xyz  | 104.26.8.160    | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 272  | 104.17.68.85               | 104.17.68.85    | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 386  | japan.com                  | 172.67.70.92    | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 222  | zread.ai                   | 104.21.76.240   | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 361  | lewis.ns.cloudflare.com    | 162.159.44.159  | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 201  | freeyx.cloudflare88.eu.org | 141.101.121.70  | IPv4   | h3   | ✅ 成功 | 128      | cloudflare |
| 56   | cmcc.877774.xyz            | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 300  | bowen.ns.cloudflare.com    | 172.64.35.83    | IPv4   | h3   | ✅ 成功 | 132      | cloudflare |
| 273  | ip.gs                      | 104.21.14.176   | IPv4   | h3   | ✅ 成功 | 137      | cloudflare |
| 355  | 104.18.78.214              | 104.18.78.214   | IPv4   | h3   | ✅ 成功 | 137      | cloudflare |
| 356  | cf.877774.xyz              | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 138      | cloudflare |
| 3    | www.pcmag.com              | 104.16.21.118   | IPv4   | h3   | ✅ 成功 | 139      | cloudflare |
| 284  | cloudflare-ip.mofashi.ltd  | 172.67.155.172  | IPv4   | h3   | ✅ 成功 | 140      | cloudflare |
| 425  | otto.ns.cloudflare.com     | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 140      | cloudflare |
| 97   | ct.877774.xyz              | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 141      | cloudflare |
| 134  | iplocation.io              | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 141      | cloudflare |
| 176  | toy-people.com             | 172.67.72.18    | IPv4   | h3   | ✅ 成功 | 141      | cloudflare |
| 229  | cf.zhetengsha.eu.org       | 172.64.152.241  | IPv4   | h3   | ✅ 成功 | 141      | cloudflare |
| 274  | ip.gs                      | 172.67.160.28   | IPv4   | h3   | ✅ 成功 | 141      | cloudflare |
| 379  | eur.877774.xyz             | 104.21.29.164   | IPv4   | h3   | ✅ 成功 | 141      | cloudflare |
| 111  | cu.877774.xyz              | 104.26.4.118    | IPv4   | h3   | ✅ 成功 | 142      | cloudflare |
| 174  | toy-people.com             | 104.26.2.36     | IPv4   | h3   | ✅ 成功 | 142      | cloudflare |
| 288  | saas.sin.fan               | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 142      | cloudflare |
| 333  | www.visa.com.hk            | 104.18.20.69    | IPv4   | h3   | ✅ 成功 | 142      | cloudflare |
| 372  | ifconfig.co                | 172.67.168.106  | IPv4   | h3   | ✅ 成功 | 143      | cloudflare |
| 435  | cfip.1323123.xyz           | 104.16.133.220  | IPv4   | h3   | ✅ 成功 | 143      | cloudflare |
| 74   | cmcc.877774.xyz            | 104.16.148.1    | IPv4   | h3   | ✅ 成功 | 144      | cloudflare |
| 327  | julio.ns.cloudflare.com    | 162.159.44.209  | IPv4   | h3   | ✅ 成功 | 144      | cloudflare |
| 77   | shopify.com                | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 145      | cloudflare |
| 368  | tasteatlas.com             | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 145      | cloudflare |
| 380  | eur.877774.xyz             | 104.21.26.150   | IPv4   | h3   | ✅ 成功 | 145      | cloudflare |
| 392  | 104.19.175.123             | 104.19.175.123  | IPv4   | h3   | ✅ 成功 | 145      | cloudflare |
| 398  | 104.17.142.12              | 104.17.142.12   | IPv4   | h3   | ✅ 成功 | 145      | cloudflare |
| 452  | cfip.xxxxxxxx.tk           | 104.25.105.1    | IPv4   | h3   | ✅ 成功 | 145      | cloudflare |
| 44   | ipv4.ip.sb                 | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 146      | cloudflare |
| 160  | pranab.ns.cloudflare.com   | 162.159.44.199  | IPv4   | h3   | ✅ 成功 | 146      | cloudflare |
| 203  | www.whatismyip.com         | 104.26.13.23    | IPv4   | h3   | ✅ 成功 | 146      | cloudflare |
| 343  | cf.090227.xyz              | 104.18.42.98    | IPv4   | h3   | ✅ 成功 | 146      | cloudflare |
| 360  | lewis.ns.cloudflare.com    | 108.162.195.159 | IPv4   | h3   | ✅ 成功 | 146      | cloudflare |
| 411  | stock.hostmonit.com        | 104.21.7.193    | IPv4   | h3   | ✅ 成功 | 146      | cloudflare |
| 412  | stock.hostmonit.com        | 172.67.187.251  | IPv4   | h3   | ✅ 成功 | 146      | cloudflare |
| 112  | cu.877774.xyz              | 104.26.4.119    | IPv4   | h3   | ✅ 成功 | 147      | cloudflare |
| 172  | na.877774.xyz              | 104.18.187.25   | IPv4   | h3   | ✅ 成功 | 147      | cloudflare |
| 33   | www.gov.ua                 | 172.67.209.127  | IPv4   | h3   | ✅ 成功 | 148      | cloudflare |
| 66   | cmcc.877774.xyz            | 104.16.149.5    | IPv4   | h3   | ✅ 成功 | 148      | cloudflare |
| 277  | braden.ns.cloudflare.com   | 108.162.195.169 | IPv4   | h3   | ✅ 成功 | 148      | cloudflare |
| 308  | ashton.ns.cloudflare.com   | 172.64.35.173   | IPv4   | h3   | ✅ 成功 | 148      | cloudflare |
| 316  | dnschecker.org             | 172.67.73.216   | IPv4   | h3   | ✅ 成功 | 148      | cloudflare |
| 357  | cf.877774.xyz              | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 148      | cloudflare |
| 400  | 198.62.62.4                | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 148      | cloudflare |
| 439  | cfip.xxxxxxxx.tk           | 198.41.214.141  | IPv4   | h3   | ✅ 成功 | 148      | cloudflare |
| 37   | 104.18.39.196              | 104.18.39.196   | IPv4   | h3   | ✅ 成功 | 149      | cloudflare |
| 117  | www.hugedomains.com        | 104.26.7.37     | IPv4   | h3   | ✅ 成功 | 149      | cloudflare |
| 237  | benedict.ns.cloudflare.com | 172.64.35.205   | IPv4   | h3   | ✅ 成功 | 149      | cloudflare |
| 290  | fbi.gov                    | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 149      | cloudflare |
| 427  | otto.ns.cloudflare.com     | 162.159.44.135  | IPv4   | h3   | ✅ 成功 | 149      | cloudflare |
| 52   | cmcc.877774.xyz            | 104.16.149.8    | IPv4   | h3   | ✅ 成功 | 150      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 33 条记录
- **正常 (100-200ms)**: 67 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 4 次
- **IPv6 失败**: 162 次

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
