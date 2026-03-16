# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/14 11:39:42
- **数据来源**: connectivity_results-20251214-113941.json
- **总测试数**: 452
- **失败测试数**: 168
- **成功测试数**: 284
- **失败率**: 37.17%
- **平均延迟**: 83.29ms
- **最小延迟**: 35ms
- **最大延迟**: 757ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/14 11:39:42
- **IP地址**: 64.236.135.130
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

- **网络不可达: 网络不可达**: 164 次 (97.6%)
- **连接超时: I/O超时**: 4 次 (2.4%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (164 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
| ---- | --------- | ------ | ------ | ---- | ------ | -------- | ------ | -------- |

#### 连接超时: I/O超时 (4 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 387  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |
| 438  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |
| 441  | cfip.xxxxxxxx.tk | 104.20.255.53  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.20.255.53:443: i/o timeout  |
| 445  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 164 次 (97.6%)
- **连接超时**: 4 次 (2.4%)

#### 错误模式分析

**超时集中度分析**: 共有 4 次超时，主要集中在IP段 172.67（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 168 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 4 次，IPv6失败 164 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：wilson.ns.cloudflare.com (3次),
www.hugedomains.com (3次), trevor.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                  | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | -------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 168  | zread.ai                   | 172.67.202.78   | IPv4   | h3   | ✅ 成功 | 35       | cloudflare |
| 163  | www.whatismyip.com         | 104.26.13.23    | IPv4   | h3   | ✅ 成功 | 38       | cloudflare |
| 328  | www.digitalocean.com       | 104.19.173.68   | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 293  | ashton.ns.cloudflare.com   | 172.64.35.173   | IPv4   | h3   | ✅ 成功 | 40       | cloudflare |
| 341  | 172.64.35.24               | 172.64.35.24    | IPv4   | h3   | ✅ 成功 | 40       | cloudflare |
| 408  | www.pcmag.com              | 104.16.21.118   | IPv4   | h3   | ✅ 成功 | 40       | cloudflare |
| 199  | saas.sin.fan               | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 210  | asia.877774.xyz            | 104.17.139.62   | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 327  | www.digitalocean.com       | 104.19.174.68   | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 429  | ct.877774.xyz              | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 390  | stock.hostmonit.com        | 104.21.7.193    | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 416  | cf.0sm.com                 | 172.67.187.145  | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 420  | iplocation.io              | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 121  | cris.ns.cloudflare.com     | 108.162.195.202 | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 442  | cfip.xxxxxxxx.tk           | 190.93.246.67   | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 444  | cfip.xxxxxxxx.tk           | 104.25.105.1    | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 343  | gamer.com.tw               | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 380  | www.wto.org                | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 292  | ashton.ns.cloudflare.com   | 108.162.195.173 | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 337  | tasteatlas.com             | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 388  | www.csgo.com               | 195.85.59.161   | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 421  | iplocation.io              | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 195  | bowen.ns.cloudflare.com    | 108.162.195.83  | IPv4   | h3   | ✅ 成功 | 47       | cloudflare |
| 452  | cfip.xxxxxxxx.tk           | 104.17.127.110  | IPv4   | h3   | ✅ 成功 | 47       | cloudflare |
| 307  | julio.ns.cloudflare.com    | 108.162.195.209 | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 430  | ct.877774.xyz              | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 227  | cf.zhetengsha.eu.org       | 172.64.152.241  | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 347  | otto.ns.cloudflare.com     | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 116  | yx-auto.pages.dev          | 172.66.44.144   | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 331  | cf.877774.xyz              | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 300  | uriah.ns.cloudflare.com    | 108.162.195.194 | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 312  | 104.26.13.31               | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 405  | ipinfo.in                  | 104.21.21.129   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 65   | cu.877774.xyz              | 104.26.4.111    | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 268  | ip.sb                      | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 84   | www.okcupid.com            | 104.18.160.63   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 64   | cu.877774.xyz              | 104.26.4.119    | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 136  | toy-people.com             | 172.67.72.18    | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 298  | 162.159.133.85             | 162.159.133.85  | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 51   | www.hugedomains.com        | 104.26.7.37     | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 68   | cu.877774.xyz              | 104.26.4.114    | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 149  | kyree.ns.cloudflare.com    | 172.64.35.207   | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 335  | 104.18.37.40               | 104.18.37.40    | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 39   | cmcc.877774.xyz            | 104.16.149.9    | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 41   | cmcc.877774.xyz            | 104.16.149.11   | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 169  | zread.ai                   | 104.21.76.240   | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 245  | palera.in                  | 172.67.157.122  | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 250  | cf.090227.xyz              | 172.64.152.241  | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 282  | 104.18.42.26               | 104.18.42.26    | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 36   | cmcc.877774.xyz            | 104.16.149.6    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 127  | pranab.ns.cloudflare.com   | 172.64.35.199   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 134  | na.877774.xyz              | 104.18.38.235   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 137  | toy-people.com             | 104.26.2.36     | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 204  | moura.ns.cloudflare.com    | 172.64.35.217   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 213  | time.is                    | 104.26.13.54    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 272  | whatismyipaddress.com      | 104.19.222.79   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 439  | cfip.xxxxxxxx.tk           | 190.93.247.169  | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 67   | cu.877774.xyz              | 104.26.4.113    | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 203  | moura.ns.cloudflare.com    | 108.162.195.217 | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 225  | www.ipchicken.com          | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 241  | www.glassdoor.com          | 104.17.64.70    | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 413  | ipv4.ip.sb                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 57   | www.ipget.net              | 172.67.207.26   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 76   | trevor.ns.cloudflare.com   | 108.162.195.154 | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 138  | toy-people.com             | 104.26.3.36     | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 243  | 104.16.61.163              | 104.16.61.163   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 322  | www.udemy.com              | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 360  | damien.ns.cloudflare.com   | 172.64.35.168   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 391  | stock.hostmonit.com        | 172.67.187.251  | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 450  | cfip.xxxxxxxx.tk           | 104.18.228.35   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 9    | 104.26.6.112               | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 27   | cmcc.877774.xyz            | 104.16.148.10   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 45   | 104.18.254.88              | 104.18.254.88   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 69   | cu.877774.xyz              | 104.26.4.115    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 70   | cu.877774.xyz              | 104.26.4.116    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 88   | 172.67.120.0               | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 209  | asia.877774.xyz            | 104.16.211.153  | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 228  | cf.zhetengsha.eu.org       | 104.18.35.15    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 244  | 162.159.36.104             | 162.159.36.104  | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 314  | www.visa.com.hk            | 104.18.20.69    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 325  | 104.17.142.12              | 104.17.142.12   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 364  | japan.com                  | 104.26.5.60     | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 409  | www.pcmag.com              | 104.16.20.118   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 449  | cfip.xxxxxxxx.tk           | 198.41.214.141  | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 13   | wilson.ns.cloudflare.com   | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 24   | cmcc.877774.xyz            | 104.16.148.7    | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 82   | www.7749tv.com             | 104.19.93.88    | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 92   | sullivan.ns.cloudflare.com | 172.64.35.161   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 103  | huxley.ns.cloudflare.com   | 108.162.195.188 | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 267  | ip.sb                      | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 376  | cfip.1323123.xyz           | 104.16.133.220  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 432  | ct.877774.xyz              | 172.64.229.173  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 447  | cfip.xxxxxxxx.tk           | 188.114.97.144  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 5    | 172.67.75.172              | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 19   | cmcc.877774.xyz            | 104.16.148.2    | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 66   | cu.877774.xyz              | 104.26.4.112    | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 87   | www.okcupid.com            | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 115  | yx-auto.pages.dev          | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 135  | na.877774.xyz              | 104.18.187.25   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 290  | 172.67.106.26              | 172.67.106.26   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 26 条记录
- **快 (50-100ms)**: 74 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 4 次
- **IPv6 失败**: 164 次

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
