# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/12 12:10:01
- **数据来源**: connectivity_results-20251212-121001.json
- **总测试数**: 445
- **失败测试数**: 168
- **成功测试数**: 277
- **失败率**: 37.75%
- **平均延迟**: 99.84ms
- **最小延迟**: 43ms
- **最大延迟**: 595ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/12 12:10:01
- **IP地址**: 20.55.15.227
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
| 398  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 423  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |
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
trevor.ns.cloudflare.com (3次), iplocation.io
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                  | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | -------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 441  | stock.hostmonit.com        | 172.67.187.251  | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 265  | benedict.ns.cloudflare.com | 172.64.35.205   | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 97   | sullivan.ns.cloudflare.com | 172.64.35.161   | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 119  | cmcc.877774.xyz            | 104.16.148.11   | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 195  | www.whatismyip.com         | 172.67.69.129   | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 401  | cfip.xxxxxxxx.tk           | 190.93.244.201  | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 63   | icook.hk                   | 172.67.161.104  | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 108  | cmcc.877774.xyz            | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 136  | www.okcupid.com            | 104.16.223.254  | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 313  | 456.cloudflare.182682.xyz  | 104.26.8.160    | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 144  | na.877774.xyz              | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 122  | cmcc.877774.xyz            | 104.16.149.1    | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 20   | wilson.ns.cloudflare.com   | 172.64.35.110   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 279  | cloudflare-ip.mofashi.ltd  | 172.67.155.172  | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 83   | cu.877774.xyz              | 104.26.4.119    | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 218  | cf.zhetengsha.eu.org       | 172.64.152.241  | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 51   | 103.160.204.59             | 103.160.204.59  | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 82   | cu.877774.xyz              | 104.26.4.118    | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 306  | dnschecker.org             | 104.26.7.89     | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 333  | 104.18.14.76               | 104.18.14.76    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 354  | gamer.com.tw               | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 98   | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 287  | 104.17.68.85               | 104.17.68.85    | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 362  | 198.62.62.4                | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 84   | cu.877774.xyz              | 104.26.4.111    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 146  | yx-auto.pages.dev          | 172.66.44.144   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 19   | wilson.ns.cloudflare.com   | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 260  | cf.090227.xyz              | 172.64.145.158  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 368  | www.digitalocean.com       | 104.19.173.68   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 432  | www.ipget.net              | 104.21.15.212   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 431  | www.csgo.com               | 195.85.59.161   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 58   | ct.877774.xyz              | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 172  | dylan.ns.cloudflare.com    | 108.162.195.187 | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 280  | cloudflare-ip.mofashi.ltd  | 104.21.72.233   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 290  | singapore.com              | 172.67.75.194   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 52   | 104.18.254.88              | 104.18.254.88   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 179  | decker.ns.cloudflare.com   | 172.64.35.155   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 210  | asia.877774.xyz            | 104.17.139.62   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 107  | cmcc.877774.xyz            | 104.16.149.12   | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 357  | 172.67.79.211              | 172.67.79.211   | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 2    | www.7749tv.com             | 104.17.90.189   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 4    | www.pcmag.com              | 104.16.21.118   | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 118  | cmcc.877774.xyz            | 104.16.148.10   | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 440  | stock.hostmonit.com        | 104.21.7.193    | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 113  | cmcc.877774.xyz            | 104.16.148.5    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 303  | whatismyipaddress.com      | 104.19.223.79   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 367  | www.digitalocean.com       | 104.19.174.68   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 403  | japan.com                  | 172.67.70.92    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 116  | cmcc.877774.xyz            | 104.16.148.8    | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 124  | cmcc.877774.xyz            | 104.16.149.3    | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 202  | bestcf.030101.xyz          | 104.17.27.231   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 54   | ct.877774.xyz              | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 123  | cmcc.877774.xyz            | 104.16.149.2    | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 302  | whatismyipaddress.com      | 104.19.222.79   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 346  | tasteatlas.com             | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 404  | japan.com                  | 104.26.4.60     | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 55   | ct.877774.xyz              | 172.64.229.173  | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 89   | cu.877774.xyz              | 104.26.4.116    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 180  | decker.ns.cloudflare.com   | 108.162.195.155 | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 60   | ct.877774.xyz              | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 103  | cf.877771.xyz              | 172.67.152.183  | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 109  | cmcc.877774.xyz            | 104.16.148.1    | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 135  | www.visa.cn                | 162.159.153.2   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 307  | dnschecker.org             | 104.26.6.89     | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 86   | cu.877774.xyz              | 104.26.4.113    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 115  | cmcc.877774.xyz            | 104.16.148.7    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 138  | www.okcupid.com            | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 147  | yx-auto.pages.dev          | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 277  | 104.16.61.163              | 104.16.61.163   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 308  | dnschecker.org             | 172.67.73.216   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 397  | eur.877774.xyz             | 104.21.26.150   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 8    | steamdb.info               | 172.66.175.250  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 85   | cu.877774.xyz              | 104.26.4.112    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 174  | dylan.ns.cloudflare.com    | 172.64.35.187   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 211  | asia.877774.xyz            | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 328  | julio.ns.cloudflare.com    | 108.162.195.209 | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 376  | 104.18.78.214              | 104.18.78.214   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 3    | www.pcmag.com              | 104.16.20.118   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 120  | cmcc.877774.xyz            | 104.16.148.12   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 141  | www.visa.com.sg            | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 239  | time.is                    | 172.67.68.157   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 259  | cf.090227.xyz              | 104.18.42.98    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 263  | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 25   | 104.18.39.196              | 104.18.39.196   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 68   | huxley.ns.cloudflare.com   | 172.64.35.188   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 81   | cu.877774.xyz              | 104.26.4.117    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 201  | bestcf.030101.xyz          | 104.17.99.183   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 289  | singapore.com              | 104.26.12.140   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 345  | tasteatlas.com             | 104.17.37.105   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 66   | www.4chan.org              | 104.16.228.229  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 76   | www.hugedomains.com        | 104.26.6.37     | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 104  | cf.877771.xyz              | 104.21.80.180   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 272  | ip.sb                      | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 33   | www.gov.ua                 | 104.21.23.72    | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 53   | ct.877774.xyz              | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 145  | na.877774.xyz              | 104.18.38.235   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 226  | braden.ns.cloudflare.com   | 108.162.195.169 | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 363  | icook.tw                   | 104.20.28.74    | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 372  | 172.64.35.24               | 172.64.35.24    | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 67   | www.4chan.org              | 104.16.229.229  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 3 条记录
- **快 (50-100ms)**: 97 条记录
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
