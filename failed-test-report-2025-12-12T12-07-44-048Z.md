# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/12 12:07:44
- **数据来源**: connectivity_results-20251212-120743.json
- **总测试数**: 443
- **失败测试数**: 167
- **成功测试数**: 276
- **失败率**: 37.70%
- **平均延迟**: 105.00ms
- **最小延迟**: 46ms
- **最大延迟**: 742ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/12 12:07:44
- **IP地址**: 135.119.236.54
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
| 287  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 376  | cf.877774.xyz    | cf.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp cf.877774.xyz:443: i/o timeout  |
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

**问题主机分析**: 以下主机出现多次失败：huxley.ns.cloudflare.com (3次),
wilson.ns.cloudflare.com (3次), trevor.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                  | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | -------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 269  | www.glassdoor.com          | 104.17.64.70    | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 349  | cf.877774.xyz              | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 131  | www.visa.com.sg            | 104.18.12.229   | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 301  | ip.gs                      | 104.21.14.176   | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 434  | www.csgo.com               | 195.85.59.161   | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 251  | rustam.ns.cloudflare.com   | 172.64.35.148   | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 431  | stock.hostmonit.com        | 104.21.7.193    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 61   | cmcc.877774.xyz            | 104.16.149.8    | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 35   | ct.877774.xyz              | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 205  | braden.ns.cloudflare.com   | 172.64.35.169   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 95   | cu.877774.xyz              | 104.26.4.119    | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 27   | www.pcmag.com              | 104.16.20.118   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 96   | cu.877774.xyz              | 104.26.4.111    | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 356  | dnschecker.org             | 104.26.6.89     | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 110  | www.okcupid.com            | 104.16.223.254  | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 213  | cf.zhetengsha.eu.org       | 104.18.35.15    | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 262  | ip.sb                      | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 277  | singapore.com              | 104.26.12.140   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 63   | cmcc.877774.xyz            | 104.16.149.10   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 65   | cf.0sm.com                 | 104.21.7.133    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 167  | toy-people.com             | 104.26.2.36     | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 239  | moura.ns.cloudflare.com    | 172.64.35.217   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 368  | gamer.com.tw               | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 405  | 172.67.181.209             | 172.67.181.209  | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 18   | wilson.ns.cloudflare.com   | 172.64.35.110   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 62   | cmcc.877774.xyz            | 104.16.149.9    | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 127  | ipinfo.in                  | 104.21.21.129   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 268  | www.glassdoor.com          | 104.16.25.46    | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 305  | iplocation.io              | 104.26.10.222   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 367  | 172.67.79.211              | 172.67.79.211   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 32   | ct.877774.xyz              | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 93   | www.4chan.org              | 104.16.228.229  | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 111  | www.okcupid.com            | 104.16.144.63   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 210  | 172.64.151.55              | 172.64.151.55   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 271  | palera.in                  | 104.21.58.72    | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 381  | 104.18.78.214              | 104.18.78.214   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 5    | 104.16.45.84               | 104.16.45.84    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 146  | yx-auto.pages.dev          | 172.66.44.144   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 294  | 104.17.68.85               | 104.17.68.85    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 387  | 104.19.175.123             | 104.19.175.123  | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 36   | ct.877774.xyz              | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 103  | sullivan.ns.cloudflare.com | 172.64.35.161   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 157  | na.877774.xyz              | 104.18.187.25   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 201  | xn--b6gac.eu.org           | 104.21.90.78    | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 33   | ct.877774.xyz              | 172.64.229.173  | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 45   | cmcc.877774.xyz            | 104.16.148.5    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 59   | cmcc.877774.xyz            | 104.16.149.6    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 71   | 104.18.254.88              | 104.18.254.88   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 189  | zread.ai                   | 172.67.202.78   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 206  | braden.ns.cloudflare.com   | 108.162.195.169 | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 222  | www.whatismyip.com         | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 302  | ip.gs                      | 172.67.160.28   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 306  | iplocation.io              | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 344  | uriah.ns.cloudflare.com    | 108.162.195.194 | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 411  | lewis.ns.cloudflare.com    | 108.162.195.159 | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 50   | cmcc.877774.xyz            | 104.16.148.10   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 60   | cmcc.877774.xyz            | 104.16.149.7    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 79   | comicabc.com               | 172.67.174.21   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 160  | kyree.ns.cloudflare.com    | 108.162.195.207 | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 178  | 104.16.223.179             | 104.16.223.179  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 188  | zread.ai                   | 104.21.76.240   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 256  | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 362  | tasteatlas.com             | 104.17.37.105   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 7    | 104.18.37.13               | 104.18.37.13    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 70   | www.7749tv.com             | 104.19.133.4    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 228  | cloudflare-ip.mofashi.ltd  | 172.67.155.172  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 46   | cmcc.877774.xyz            | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 197  | asia.877774.xyz            | 104.16.211.153  | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 418  | japan.com                  | 104.26.4.60     | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 424  | eur.877774.xyz             | 104.21.26.150   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 51   | cmcc.877774.xyz            | 104.16.148.11   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 69   | 103.160.204.59             | 103.160.204.59  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 276  | www.ipchicken.com          | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 286  | cfip.xxxxxxxx.tk           | 190.93.244.201  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 288  | 104.17.142.12              | 104.17.142.12   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 41   | cmcc.877774.xyz            | 104.16.148.1    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 336  | ashton.ns.cloudflare.com   | 172.64.35.173   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 40   | cmcc.877774.xyz            | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 9    | 104.26.6.112               | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 64   | 104.16.61.163              | 104.16.61.163   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 104  | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 109  | www.okcupid.com            | 104.18.160.63   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 162  | kyree.ns.cloudflare.com    | 172.64.35.207   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 181  | www.4444.cloudflare.182682.xyz                | 162.159.152.2   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 199  | asia.877774.xyz            | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 229  | cloudflare-ip.mofashi.ltd  | 104.21.72.233   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 245  | time.is                    | 172.67.68.157   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 366  | 104.19.223.58              | 104.19.223.58   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 375  | 104.18.37.40               | 104.18.37.40    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 438  | abdullah.ns.cloudflare.com | 172.64.35.203   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 39   | cmcc.877774.xyz            | 104.16.149.12   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 100  | cu.877774.xyz              | 104.26.4.115    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 150  | decker.ns.cloudflare.com   | 108.162.195.155 | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 30   | ct.877774.xyz              | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 52   | cmcc.877774.xyz            | 104.16.148.12   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 66   | cf.0sm.com                 | 172.67.187.145  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 112  | www.okcupid.com            | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 119  | www.hugedomains.com        | 172.67.70.191   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 198  | asia.877774.xyz            | 104.17.139.62   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 221  | saas.sin.fan               | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 2 条记录
- **快 (50-100ms)**: 98 条记录
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
