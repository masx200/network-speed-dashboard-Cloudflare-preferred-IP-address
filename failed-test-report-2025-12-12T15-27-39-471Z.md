# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/12 15:27:39
- **数据来源**: connectivity_results-20251212-152739.json
- **总测试数**: 451
- **失败测试数**: 167
- **成功测试数**: 284
- **失败率**: 37.03%
- **平均延迟**: 121.82ms
- **最小延迟**: 41ms
- **最大延迟**: 598ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/12 15:27:39
- **IP地址**: 40.76.238.182
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

- **网络不可达: 网络不可达**: 163 次 (97.6%)
- **连接超时: I/O超时**: 4 次 (2.4%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (163 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
| ---- | --------- | ------ | ------ | ---- | ------ | -------- | ------ | -------- |

#### 连接超时: I/O超时 (4 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 436  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |
| 437  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |
| 443  | cfip.xxxxxxxx.tk | 104.20.255.53  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.20.255.53:443: i/o timeout  |
| 451  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 163 次 (97.6%)
- **连接超时**: 4 次 (2.4%)

#### 错误模式分析

**超时集中度分析**: 共有 4 次超时，主要集中在IP段 172.67（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 167 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 4 次，IPv6失败 163 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：wilson.ns.cloudflare.com (3次),
trevor.ns.cloudflare.com (3次), iplocation.io
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                  | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | -------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 400  | lewis.ns.cloudflare.com    | 108.162.195.159 | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 362  | www.udemy.com              | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 295  | 456.cloudflare.182682.xyz  | 172.67.75.208   | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 338  | uriah.ns.cloudflare.com    | 172.64.35.194   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 97   | www.okcupid.com            | 104.16.223.254  | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 361  | gamer.com.tw               | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 239  | cloudflare-ip.mofashi.ltd  | 188.114.97.3    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 333  | www.ipchicken.com          | 104.26.7.112    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 360  | gamer.com.tw               | 104.18.2.197    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 405  | ifconfig.co                | 172.67.168.106  | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 167  | cmcc.877774.xyz            | 104.16.148.11   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 256  | saas.sin.fan               | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 319  | ashton.ns.cloudflare.com   | 108.162.195.173 | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 39   | steamdb.info               | 172.66.175.250  | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 56   | 104.18.37.13               | 104.18.37.13    | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 335  | 104.18.14.76               | 104.18.14.76    | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 96   | www.okcupid.com            | 104.16.144.63   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 95   | www.okcupid.com            | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 58   | 172.67.110.232             | 172.67.110.232  | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 60   | ct.877774.xyz              | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 274  | palera.in                  | 104.21.58.72    | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 294  | 456.cloudflare.182682.xyz  | 104.26.8.160    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 428  | abdullah.ns.cloudflare.com | 172.64.35.203   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 144  | na.877774.xyz              | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 180  | cmcc.877774.xyz            | 104.16.149.11   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 277  | fbi.gov                    | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 49   | iplocation.io              | 104.26.10.222   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 411  | japan.com                  | 104.26.4.60     | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 129  | yx-auto.pages.dev          | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 18   | trevor.ns.cloudflare.com   | 108.162.195.154 | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 145  | na.877774.xyz              | 104.18.38.235   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 326  | dnschecker.org             | 104.26.6.89     | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 366  | tasteatlas.com             | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 373  | www.digitalocean.com       | 104.19.173.68   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 91   | sullivan.ns.cloudflare.com | 172.64.35.161   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 13   | www.pcmag.com              | 104.16.21.118   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 23   | www.ipget.net              | 172.67.207.26   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 331  | www.ipchicken.com          | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 396  | eur.877774.xyz             | 104.21.26.150   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 162  | cmcc.877774.xyz            | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 48   | shopify.com                | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 370  | 104.17.142.12              | 104.17.142.12   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 426  | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 265  | moura.ns.cloudflare.com    | 108.162.195.217 | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 445  | cfip.xxxxxxxx.tk           | 104.27.21.118   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 109  | cu.877774.xyz              | 104.26.4.117    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 284  | singapore.com              | 172.67.75.194   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 377  | 104.18.37.40               | 104.18.37.40    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 63   | ct.877774.xyz              | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 107  | cu.877774.xyz              | 104.26.4.115    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 415  | www.wto.org                | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 302  | ip.sb                      | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 35   | www.gov.ua                 | 172.67.209.127  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 125  | time.is                    | 172.67.68.157   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 137  | pranab.ns.cloudflare.com   | 172.64.35.199   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 158  | cmcc.877774.xyz            | 104.16.148.2    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 135  | pranab.ns.cloudflare.com   | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 386  | 104.26.13.31               | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 80   | www.4chan.org              | 104.16.228.229  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 233  | braden.ns.cloudflare.com   | 108.162.195.169 | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 112  | cu.877774.xyz              | 104.26.4.111    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 153  | toy-people.com             | 172.67.72.18    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 8    | wilson.ns.cloudflare.com   | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 17   | trevor.ns.cloudflare.com   | 172.64.35.154   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 105  | cu.877774.xyz              | 104.26.4.113    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 183  | cmcc.877774.xyz            | 104.16.148.1    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 191  | decker.ns.cloudflare.com   | 172.64.35.155   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 301  | ip.sb                      | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 399  | lewis.ns.cloudflare.com    | 172.64.35.159   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 217  | zread.ai                   | 172.67.202.78   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 404  | 104.19.223.58              | 104.19.223.58   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 46   | ipv4.ip.sb                 | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 81   | www.4chan.org              | 104.16.229.229  | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 98   | www.okcupid.com            | 104.18.160.63   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 100  | cf.877771.xyz              | 104.21.80.180   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 133  | www.visa.com.sg            | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 175  | cmcc.877774.xyz            | 104.16.149.6    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 307  | benedict.ns.cloudflare.com | 162.159.44.205  | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 27   | cf.0sm.com                 | 104.21.7.133    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 232  | braden.ns.cloudflare.com   | 172.64.35.169   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 262  | icook.tw                   | 172.66.158.115  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 289  | ip.gs                      | 104.21.14.176   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 358  | 104.18.78.214              | 104.18.78.214   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 186  | kyree.ns.cloudflare.com    | 172.64.35.207   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 240  | cloudflare-ip.mofashi.ltd  | 188.114.96.3    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 261  | icook.tw                   | 104.20.28.74    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 281  | 104.16.61.163              | 104.16.61.163   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 376  | 198.62.62.4                | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 416  | www.wto.org                | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 66   | ct.877774.xyz              | 172.64.229.173  | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 169  | cmcc.877774.xyz            | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 244  | xn--b6gac.eu.org           | 172.67.153.253  | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 423  | 104.19.175.123             | 104.19.175.123  | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 3    | comicabc.com               | 104.21.64.10    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 36   | www.gov.ua                 | 104.21.23.72    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 111  | cu.877774.xyz              | 104.26.4.119    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 178  | cmcc.877774.xyz            | 104.16.149.9    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 258  | cf.090227.xyz              | 172.64.145.158  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 173  | cmcc.877774.xyz            | 104.16.149.4    | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 227  | bowen.ns.cloudflare.com    | 172.64.35.83    | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 2 条记录
- **快 (50-100ms)**: 96 条记录
- **正常 (100-200ms)**: 2 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 4 次
- **IPv6 失败**: 163 次

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
