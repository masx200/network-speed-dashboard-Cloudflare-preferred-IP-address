# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/12 07:03:52
- **数据来源**: connectivity_results-20251212-070351.json
- **总测试数**: 458
- **失败测试数**: 176
- **成功测试数**: 282
- **失败率**: 38.43%
- **平均延迟**: 121.83ms
- **最小延迟**: 55ms
- **最大延迟**: 556ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/12 07:03:52
- **IP地址**: 132.196.82.1
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
| 456  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 457  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |
| 458  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 173 次 (98.3%)
- **连接超时**: 3 次 (1.7%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 176 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 173 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：trevor.ns.cloudflare.com (3次),
wilson.ns.cloudflare.com (3次), iplocation.io
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 80   | www.4chan.org                         | 104.16.228.229  | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 242  | braden.ns.cloudflare.com              | 172.64.35.169   | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 400  | damien.ns.cloudflare.com              | 108.162.195.168 | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 378  | www.udemy.com                         | 104.16.143.237  | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 259  | saas.sin.fan                          | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 38   | ct.877774.xyz                         | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 299  | 162.159.36.104                        | 162.159.36.104  | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 431  | eur.877774.xyz                        | 104.21.29.164   | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 439  | 108.162.198.54                        | 108.162.198.54  | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 142  | cmcc.877774.xyz                       | 104.16.148.10   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 154  | cmcc.877774.xyz                       | 104.16.149.9    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 446  | www.csgo.com                          | 195.85.59.161   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 171  | toy-people.com                        | 104.26.2.36     | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 341  | uriah.ns.cloudflare.com               | 108.162.195.194 | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 95   | craig.ns.cloudflare.com               | 108.162.195.192 | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 427  | lewis.ns.cloudflare.com               | 172.64.35.159   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 107  | www.visa.cn                           | 162.159.152.2   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 228  | time.is                               | 104.26.12.54    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 335  | dnschecker.org                        | 104.26.7.89     | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 130  | pranab.ns.cloudflare.com              | 172.64.35.199   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 157  | cmcc.877774.xyz                       | 104.16.149.12   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 178  | decker.ns.cloudflare.com              | 172.64.35.155   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 289  | singapore.com                         | 104.26.12.140   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 31   | 104.18.39.196                         | 104.18.39.196   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 208  | zread.ai                              | 104.21.76.240   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 330  | julio.ns.cloudflare.com               | 172.64.35.209   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 190  | na.877774.xyz                         | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 393  | otto.ns.cloudflare.com                | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 416  | ifconfig.co                           | 104.21.54.91    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 33   | steamdb.info                          | 104.20.34.212   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 353  | 104.18.42.26                          | 104.18.42.26    | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 67   | 172.67.110.232                        | 172.67.110.232  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 91   | sullivan.ns.cloudflare.com            | 172.64.35.161   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 161  | cmcc.877774.xyz                       | 104.16.148.3    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 306  | 456.cloudflare.182682.xyz             | 104.26.8.160    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 383  | gamer.com.tw                          | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 158  | cmcc.877774.xyz                       | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 197  | www.whatismyip.com                    | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 258  | saas.sin.fan                          | 162.159.36.20   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 310  | ip.gs                                 | 172.67.160.28   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 48   | ipv4.ip.sb                            | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 128  | pranab.ns.cloudflare.com              | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 188  | na.877774.xyz                         | 104.18.38.235   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 361  | yx-auto.pages.dev                     | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 141  | cmcc.877774.xyz                       | 104.16.148.9    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 209  | zread.ai                              | 172.67.202.78   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 355  | 104.18.14.76                          | 104.18.14.76    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 382  | 104.17.142.12                         | 104.17.142.12   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 97   | craig.ns.cloudflare.com               | 172.64.35.192   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 266  | cf.090227.xyz                         | 172.64.144.82   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 276  | www.ipchicken.com                     | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 392  | 172.64.35.24                          | 172.64.35.24    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 447  | abdullah.ns.cloudflare.com            | 108.162.195.203 | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 254  | moura.ns.cloudflare.com               | 172.64.35.217   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 315  | ae8a9c24-83de.masx200.ddns-ip.net     | 104.21.14.41    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 160  | cmcc.877774.xyz                       | 104.16.148.2    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 206  | 104.17.79.11                          | 104.17.79.11    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 323  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 101  | www.okcupid.com                       | 104.16.144.63   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 114  | cu.877774.xyz                         | 104.26.4.114    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 156  | cmcc.877774.xyz                       | 104.16.149.11   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 225  | cloudflare-ip.mofashi.ltd             | 104.21.72.233   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 365  | tasteatlas.com                        | 104.17.37.105   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 419  | japan.com                             | 104.26.4.60     | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 76   | huxley.ns.cloudflare.com              | 172.64.35.188   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 138  | cmcc.877774.xyz                       | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 356  | www.visa.com.hk                       | 104.18.20.69    | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 357  | www.visa.com.hk                       | 104.18.21.69    | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 395  | otto.ns.cloudflare.com                | 172.64.35.135   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 74   | huxley.ns.cloudflare.com              | 108.162.195.188 | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 85   | www.hugedomains.com                   | 104.26.6.37     | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 146  | cmcc.877774.xyz                       | 104.16.149.1    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 324  | local-aria2-webui.masx200.ddns-ip.net | 104.21.14.41    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 328  | julio.ns.cloudflare.com               | 108.162.195.209 | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 360  | yx-auto.pages.dev                     | 172.66.44.144   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 17   | trevor.ns.cloudflare.com              | 172.64.35.154   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 40   | ct.877774.xyz                         | 172.64.229.217  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 41   | ct.877774.xyz                         | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 272  | rustam.ns.cloudflare.com              | 172.64.35.148   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 319  | whatismyipaddress.com                 | 104.19.223.79   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 364  | tasteatlas.com                        | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 433  | eur.877774.xyz                        | 104.21.26.150   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 68   | 104.18.254.88                         | 104.18.254.88   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 106  | www.visa.cn                           | 162.159.153.2   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 112  | cu.877774.xyz                         | 104.26.4.112    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 140  | cmcc.877774.xyz                       | 104.16.148.8    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 220  | cf.zhetengsha.eu.org                  | 172.64.152.241  | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 22   | comicabc.com                          | 172.67.174.21   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 39   | ct.877774.xyz                         | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 104  | www.okcupid.com                       | 104.16.239.254  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 373  | yx-auto.pages.dev                     | 172.67.161.98   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 217  | asia.877774.xyz                       | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 445  | www.csgo.com                          | 195.85.59.95    | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 109  | cu.877774.xyz                         | 104.26.4.118    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 191  | dylan.ns.cloudflare.com               | 108.162.195.187 | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 172  | toy-people.com                        | 104.26.3.36     | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 103  | www.okcupid.com                       | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 115  | cu.877774.xyz                         | 104.26.4.115    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 219  | asia.877774.xyz                       | 104.16.211.153  | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 239  | 172.64.151.55                         | 172.64.151.55   | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 91 条记录
- **正常 (100-200ms)**: 9 条记录
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
