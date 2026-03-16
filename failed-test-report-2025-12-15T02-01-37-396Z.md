# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/15 02:01:37
- **数据来源**: connectivity_results-20251215-020136.json
- **总测试数**: 431
- **失败测试数**: 170
- **成功测试数**: 261
- **失败率**: 39.44%
- **平均延迟**: 88.91ms
- **最小延迟**: 37ms
- **最大延迟**: 1418ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/15 02:01:37
- **IP地址**: 52.159.227.195
- **国家/地区**: 美国 (US)
- **ASN**: 8075
- **网络组织**: Microsoft Corporation
- **网络域名**: microsoft.com
- **大洲**: North America (NA)
- **地理坐标**: 37.3388, -121.8916
- **时区**: America/Los_Angeles
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 166 次 (97.6%)
- **连接超时: I/O超时**: 3 次 (1.8%)
- **DNS解析错误: 其他DNS错误**: 1 次 (0.6%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (166 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
| ---- | --------- | ------ | ------ | ---- | ------ | -------- | ------ | -------- |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 230  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 342  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |
| 431  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |

#### DNS解析错误: 其他DNS错误 (1 次测试)

| 序号 | 主机/域名     | 目标IP  | IP版本  | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息      |
| ---- | ------------- | ------- | ------- | ---- | ------ | -------- | ------ | ------------- |
| 212  | www.4chan.org | Unknown | Unknown | none | N/A    | 0        | N/A    | DNS解析无结果 |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 166 次 (97.6%)
- **连接超时**: 3 次 (1.8%)
- **DNS解析错误**: 1 次 (0.6%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 170 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 166 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次),
trevor.ns.cloudflare.com (3次), iplocation.io
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 291  | 456.cloudflare.182682.xyz             | 104.26.8.160    | IPv4   | h3   | ✅ 成功 | 37       | cloudflare |
| 25   | iplocation.io                         | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 38       | cloudflare |
| 36   | cloudflare.182682.xyz                 | 104.16.250.22   | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 396  | 104.19.175.123                        | 104.19.175.123  | IPv4   | h3   | ✅ 成功 | 40       | cloudflare |
| 349  | www.udemy.com                         | 104.16.143.237  | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 378  | 104.17.142.12                         | 104.17.142.12   | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 127  | craig.ns.cloudflare.com               | 108.162.195.192 | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 352  | gamer.com.tw                          | 104.18.2.197    | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 368  | japan.com                             | 104.26.5.60     | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 399  | lewis.ns.cloudflare.com               | 108.162.195.159 | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 69   | cmcc.877774.xyz                       | 104.16.148.1    | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 149  | na.877774.xyz                         | 104.18.187.25   | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 168  | toy-people.com                        | 104.26.3.36     | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 393  | eur.877774.xyz                        | 104.21.26.150   | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 394  | 198.62.62.4                           | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 112  | www.okcupid.com                       | 104.16.144.63   | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 7    | comicabc.com                          | 104.21.64.10    | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 317  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 362  | ifconfig.co                           | 104.21.54.91    | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 160  | ct.877774.xyz                         | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 219  | xn--b6gac.eu.org                      | 104.21.90.78    | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 236  | cloudflare-ip.mofashi.ltd             | 172.67.155.172  | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 91   | cu.877774.xyz                         | 104.26.4.118    | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 204  | zread.ai                              | 104.21.76.240   | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 341  | 162.159.133.85                        | 162.159.133.85  | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 213  | braden.ns.cloudflare.com              | 172.64.35.169   | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 5    | www.7749tv.com                        | 104.16.0.23     | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 305  | dnschecker.org                        | 104.26.6.89     | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 403  | 172.67.181.209                        | 172.67.181.209  | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 209  | asia.877774.xyz                       | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 407  | 108.162.198.54                        | 108.162.198.54  | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 66   | cmcc.877774.xyz                       | 104.16.149.11   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 258  | time.is                               | 172.67.68.157   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 395  | 104.18.37.40                          | 104.18.37.40    | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 92   | cu.877774.xyz                         | 104.26.4.119    | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 301  | ashton.ns.cloudflare.com              | 108.162.195.173 | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 77   | cmcc.877774.xyz                       | 104.16.148.9    | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 283  | 162.159.36.104                        | 162.159.36.104  | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 300  | ashton.ns.cloudflare.com              | 172.64.35.173   | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 392  | eur.877774.xyz                        | 104.21.47.209   | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 74   | cmcc.877774.xyz                       | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 183  | kyree.ns.cloudflare.com               | 108.162.195.207 | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 195  | 104.17.79.11                          | 104.17.79.11    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 324  | whatismyipaddress.com                 | 104.19.222.79   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 135  | 172.67.243.218                        | 172.67.243.218  | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 70   | cmcc.877774.xyz                       | 104.16.148.2    | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 220  | xn--b6gac.eu.org                      | 172.67.153.253  | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 277  | singapore.com                         | 104.26.12.140   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 331  | cf.877774.xyz                         | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 353  | gamer.com.tw                          | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 3    | ipv4.ip.sb                            | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 51   | cf.0sm.com                            | 104.21.7.133    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 62   | cmcc.877774.xyz                       | 104.16.149.7    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 140  | www.visa.com.sg                       | 104.18.12.229   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 260  | time.is                               | 104.26.13.54    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 148  | na.877774.xyz                         | 104.18.38.235   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 413  | www.ipchicken.com                     | 104.26.7.112    | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 13   | trevor.ns.cloudflare.com              | 108.162.195.154 | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 55   | 104.18.37.13                          | 104.18.37.13    | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 133  | www.visa.cn                           | 162.159.152.2   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 150  | na.877774.xyz                         | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 161  | ct.877774.xyz                         | 172.64.229.217  | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 4    | ipv4.ip.sb                            | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 18   | shopify.com                           | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 67   | cmcc.877774.xyz                       | 104.16.149.12   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 251  | 172.64.154.18                         | 172.64.154.18   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 259  | time.is                               | 104.26.12.54    | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 321  | 104.18.42.26                          | 104.18.42.26    | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 76   | cmcc.877774.xyz                       | 104.16.148.8    | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 137  | yx-auto.pages.dev                     | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 174  | huxley.ns.cloudflare.com              | 108.162.195.188 | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 214  | braden.ns.cloudflare.com              | 108.162.195.169 | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 266  | rustam.ns.cloudflare.com              | 172.64.35.148   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 307  | dnschecker.org                        | 104.26.7.89     | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 408  | stock.hostmonit.com                   | 172.67.187.251  | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 34   | cloudflare.182682.xyz                 | 104.18.185.26   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 163  | ct.877774.xyz                         | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 278  | singapore.com                         | 104.26.13.140   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 330  | cf.877774.xyz                         | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 64   | cmcc.877774.xyz                       | 104.16.149.9    | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 79   | cmcc.877774.xyz                       | 104.16.148.11   | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 136  | yx-auto.pages.dev                     | 172.66.44.144   | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 243  | saas.sin.fan                          | 162.159.36.20   | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 285  | cf.zhetengsha.eu.org                  | 172.64.144.82   | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 409  | stock.hostmonit.com                   | 104.21.7.193    | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 50   | www.glassdoor.com                     | 104.17.64.70    | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 94   | cu.877774.xyz                         | 104.26.4.112    | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 115  | sullivan.ns.cloudflare.com            | 108.162.195.161 | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 232  | fbi.gov                               | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 240  | cf.090227.xyz                         | 104.18.42.98    | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 319  | www.visa.com.hk                       | 104.18.21.69    | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 388  | www.wto.org                           | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 1    | 172.67.75.172                         | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 10   | 104.18.39.196                         | 104.18.39.196   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 56   | 104.26.6.112                          | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 72   | cmcc.877774.xyz                       | 104.16.148.4    | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 265  | rustam.ns.cloudflare.com              | 108.162.195.148 | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 382  | damien.ns.cloudflare.com              | 172.64.35.168   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 164  | ct.877774.xyz                         | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 167  | toy-people.com                        | 104.26.2.36     | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 288  | 104.17.68.85                          | 104.17.68.85    | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 289  | 456.cloudflare.182682.xyz             | 104.26.9.160    | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 306  | dnschecker.org                        | 172.67.73.216   | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 329  | 104.18.14.76                          | 104.18.14.76    | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 383  | damien.ns.cloudflare.com              | 108.162.195.168 | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 414  | www.ipchicken.com                     | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 19   | www.ipget.net                         | 104.21.15.212   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 20   | www.ipget.net                         | 172.67.207.26   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 99   | www.hugedomains.com                   | 172.67.70.191   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 111  | www.okcupid.com                       | 104.16.223.254  | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 176  | decker.ns.cloudflare.com              | 108.162.195.155 | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 202  | freeyx.cloudflare88.eu.org            | 141.101.121.228 | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 231  | fbi.gov                               | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 244  | saas.sin.fan                          | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 245  | bowen.ns.cloudflare.com               | 172.64.35.83    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 358  | icook.tw                              | 172.66.158.115  | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 60   | cmcc.877774.xyz                       | 104.16.149.5    | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 75   | cmcc.877774.xyz                       | 104.16.148.7    | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 81   | cmcc.877774.xyz                       | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 96   | cu.877774.xyz                         | 104.26.4.114    | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 114  | www.okcupid.com                       | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 141  | www.visa.com.sg                       | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 194  | 104.16.223.179                        | 104.16.223.179  | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 205  | zread.ai                              | 172.67.202.78   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 423  | abdullah.ns.cloudflare.com            | 108.162.195.203 | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 95   | cu.877774.xyz                         | 104.26.4.113    | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 379  | 172.67.79.211                         | 172.67.79.211   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 59   | cmcc.877774.xyz                       | 104.16.149.4    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 87   | 104.18.254.88                         | 104.18.254.88   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 284  | cf.zhetengsha.eu.org                  | 104.18.43.174   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 122  | cf.877771.xyz                         | 172.67.152.183  | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 254  | moura.ns.cloudflare.com               | 172.64.35.217   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 367  | japan.com                             | 172.67.70.92    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 405  | cfip.1323123.xyz                      | 104.16.133.220  | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 63   | cmcc.877774.xyz                       | 104.16.149.8    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 71   | cmcc.877774.xyz                       | 104.16.148.3    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 129  | craig.ns.cloudflare.com               | 172.64.35.192   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 276  | singapore.com                         | 172.67.75.194   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 68   | cmcc.877774.xyz                       | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 134  | www.visa.cn                           | 162.159.153.2   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 158  | ct.877774.xyz                         | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 175  | huxley.ns.cloudflare.com              | 162.159.44.188  | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 246  | bowen.ns.cloudflare.com               | 108.162.195.83  | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 290  | 456.cloudflare.182682.xyz             | 172.67.75.208   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 61   | cmcc.877774.xyz                       | 104.16.149.6    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 65   | cmcc.877774.xyz                       | 104.16.149.10   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 169  | toy-people.com                        | 172.67.72.18    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 348  | www.udemy.com                         | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 11   | 104.16.45.84                          | 104.16.45.84    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 44   | wilson.ns.cloudflare.com              | 172.64.35.110   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 49   | www.glassdoor.com                     | 104.16.25.46    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 78   | cmcc.877774.xyz                       | 104.16.148.10   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 90   | cu.877774.xyz                         | 104.26.4.117    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 82   | cmcc.877774.xyz                       | 104.16.149.1    | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 110  | www.okcupid.com                       | 104.18.160.63   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 253  | moura.ns.cloudflare.com               | 108.162.195.217 | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 313  | julio.ns.cloudflare.com               | 108.162.195.209 | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 380  | 172.64.35.24                          | 172.64.35.24    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 89   | cu.877774.xyz                         | 104.26.4.116    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 144  | pranab.ns.cloudflare.com              | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 153  | cris.ns.cloudflare.com                | 172.64.35.202   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 282  | 104.16.61.163                         | 104.16.61.163   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 361  | 104.19.223.58                         | 104.19.223.58   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 57   | 172.67.110.232                        | 172.67.110.232  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 125  | 172.67.120.0                          | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 229  | cfip.xxxxxxxx.tk                      | 190.93.244.201  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 406  | 104.26.13.31                          | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 23   | iplocation.io                         | 104.26.10.222   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 43   | wilson.ns.cloudflare.com              | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 178  | decker.ns.cloudflare.com              | 172.64.35.155   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 190  | www.whatismyip.com                    | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 211  | asia.877774.xyz                       | 104.16.211.153  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 226  | bestcf.030101.xyz                     | 104.17.214.216  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 312  | julio.ns.cloudflare.com               | 172.64.35.209   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 391  | eur.877774.xyz                        | 104.21.29.164   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 412  | www.ipchicken.com                     | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 117  | sullivan.ns.cloudflare.com            | 172.64.35.161   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 210  | asia.877774.xyz                       | 104.17.139.62   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 272  | benedict.ns.cloudflare.com            | 108.162.195.205 | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 320  | www.visa.com.hk                       | 104.18.20.69    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 323  | whatismyipaddress.com                 | 104.19.223.79   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 335  | uriah.ns.cloudflare.com               | 108.162.195.194 | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 357  | icook.tw                              | 104.20.28.74    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 363  | ifconfig.co                           | 172.67.168.106  | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 429  | www.csgo.com                          | 195.85.59.95    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 52   | cf.0sm.com                            | 172.67.187.145  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 98   | www.hugedomains.com                   | 104.26.6.37     | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 196  | dylan.ns.cloudflare.com               | 108.162.195.187 | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 173  | huxley.ns.cloudflare.com              | 172.64.35.188   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 182  | kyree.ns.cloudflare.com               | 172.64.35.207   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 296  | ip.gs                                 | 172.67.160.28   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 327  | 172.67.106.26                         | 172.67.106.26   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 372  | otto.ns.cloudflare.com                | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 422  | abdullah.ns.cloudflare.com            | 172.64.35.203   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 35   | cloudflare.182682.xyz                 | 104.17.25.173   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 83   | cmcc.877774.xyz                       | 104.16.149.2    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 113  | www.okcupid.com                       | 104.16.239.254  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 188  | www.whatismyip.com                    | 104.26.13.23    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 223  | 172.64.151.55                         | 172.64.151.55   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 225  | bestcf.030101.xyz                     | 104.17.54.87    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 23 条记录
- **快 (50-100ms)**: 177 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 3 次
- **IPv6 失败**: 166 次

### 按协议统计

- **none**: 170 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
