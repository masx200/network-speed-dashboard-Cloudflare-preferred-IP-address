# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/14 14:41:56
- **数据来源**: connectivity_results-20251214-144156.json
- **总测试数**: 441
- **失败测试数**: 166
- **成功测试数**: 275
- **失败率**: 37.64%
- **平均延迟**: 77.18ms
- **最小延迟**: 35ms
- **最大延迟**: 679ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/14 14:41:56
- **IP地址**: 52.225.73.163
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

- **网络不可达: 网络不可达**: 163 次 (98.2%)
- **连接超时: I/O超时**: 3 次 (1.8%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (163 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
| ---- | --------- | ------ | ------ | ---- | ------ | -------- | ------ | -------- |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 435  | ct.877774.xyz    | ct.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout  |
| 440  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 441  | cf.877774.xyz    | cf.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp cf.877774.xyz:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 163 次 (98.2%)
- **连接超时**: 3 次 (1.8%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 172.64（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 166 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 163 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：decker.ns.cloudflare.com (3次),
cris.ns.cloudflare.com (3次), dylan.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 45   | zread.ai                              | 104.21.76.240   | IPv4   | h3   | ✅ 成功 | 35       | cloudflare |
| 82   | na.877774.xyz                         | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 35       | cloudflare |
| 112  | www.glassdoor.com                     | 104.16.25.46    | IPv4   | h3   | ✅ 成功 | 35       | cloudflare |
| 147  | ashton.ns.cloudflare.com              | 172.64.35.173   | IPv4   | h3   | ✅ 成功 | 36       | cloudflare |
| 329  | 104.18.37.13                          | 104.18.37.13    | IPv4   | h3   | ✅ 成功 | 36       | cloudflare |
| 233  | www.visa.com.hk                       | 104.18.21.69    | IPv4   | h3   | ✅ 成功 | 37       | cloudflare |
| 289  | trevor.ns.cloudflare.com              | 172.64.35.154   | IPv4   | h3   | ✅ 成功 | 37       | cloudflare |
| 324  | 172.67.75.172                         | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 37       | cloudflare |
| 362  | cmcc.877774.xyz                       | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 37       | cloudflare |
| 427  | pranab.ns.cloudflare.com              | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 37       | cloudflare |
| 35   | kyree.ns.cloudflare.com               | 172.64.35.207   | IPv4   | h3   | ✅ 成功 | 38       | cloudflare |
| 243  | otto.ns.cloudflare.com                | 172.64.35.135   | IPv4   | h3   | ✅ 成功 | 38       | cloudflare |
| 425  | www.visa.com.sg                       | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 38       | cloudflare |
| 437  | www.4444.cloudflare.182682.xyz                           | 162.159.152.2   | IPv4   | h3   | ✅ 成功 | 38       | cloudflare |
| 215  | icook.tw                              | 104.20.28.74    | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 393  | cu.877774.xyz                         | 104.26.4.112    | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 403  | yx-auto.pages.dev                     | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 399  | www.okcupid.com                       | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 40       | cloudflare |
| 114  | ip.sb                                 | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 391  | cu.877774.xyz                         | 104.26.4.113    | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 51   | comicabc.com                          | 172.67.174.21   | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 282  | www.csgo.com                          | 195.85.59.161   | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 316  | steamdb.info                          | 172.66.175.250  | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 368  | cmcc.877774.xyz                       | 104.16.148.3    | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 228  | www.digitalocean.com                  | 104.19.174.68   | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 308  | www.pcmag.com                         | 104.16.21.118   | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 300  | ct.877774.xyz                         | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 396  | cu.877774.xyz                         | 104.26.4.119    | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 320  | 104.16.45.84                          | 104.16.45.84    | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 429  | pranab.ns.cloudflare.com              | 172.64.35.199   | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 303  | ct.877774.xyz                         | 172.64.229.185  | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 404  | yx-auto.pages.dev                     | 172.66.44.144   | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 281  | www.csgo.com                          | 195.85.59.95    | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 54   | bestcf.030101.xyz                     | 104.19.61.113   | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 354  | cmcc.877774.xyz                       | 104.16.148.4    | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 4    | decker.ns.cloudflare.com              | 172.64.35.155   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 94   | www.ipchicken.com                     | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 129  | palera.in                             | 188.114.96.3    | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 111  | www.glassdoor.com                     | 104.17.64.70    | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 284  | stock.hostmonit.com                   | 104.21.7.193    | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 31   | freeyx.cloudflare88.eu.org            | 141.101.121.70  | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 95   | benedict.ns.cloudflare.com            | 108.162.195.205 | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 146  | ashton.ns.cloudflare.com              | 108.162.195.173 | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 378  | cmcc.877774.xyz                       | 104.16.149.12   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 108  | cf.zhetengsha.eu.org                  | 172.64.152.241  | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 185  | cloudflare-ip.mofashi.ltd             | 172.67.155.172  | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 242  | otto.ns.cloudflare.com                | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 24   | toy-people.com                        | 104.26.2.36     | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 97   | benedict.ns.cloudflare.com            | 172.64.35.205   | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 219  | 104.26.13.31                          | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 220  | time.is                               | 104.26.13.54    | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 383  | www.hugedomains.com                   | 172.67.70.191   | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 58   | www.whatismyip.com                    | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 162  | julio.ns.cloudflare.com               | 108.162.195.209 | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 213  | gamer.com.tw                          | 104.18.2.197    | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 302  | ct.877774.xyz                         | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 394  | cu.877774.xyz                         | 104.26.4.117    | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 139  | ip.gs                                 | 104.21.14.176   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 160  | 104.18.42.26                          | 104.18.42.26    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 195  | www.udemy.com                         | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 222  | time.is                               | 104.26.12.54    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 343  | huxley.ns.cloudflare.com              | 108.162.195.188 | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 365  | cmcc.877774.xyz                       | 104.16.148.12   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 376  | cmcc.877774.xyz                       | 104.16.148.5    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 397  | cu.877774.xyz                         | 104.26.4.118    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 212  | gamer.com.tw                          | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 221  | time.is                               | 172.67.68.157   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 261  | japan.com                             | 172.67.70.92    | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 270  | eur.877774.xyz                        | 104.21.47.209   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 271  | www.wto.org                           | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 276  | abdullah.ns.cloudflare.com            | 162.159.44.203  | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 357  | cmcc.877774.xyz                       | 104.16.149.11   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 371  | cmcc.877774.xyz                       | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 46   | zread.ai                              | 172.67.202.78   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 55   | bestcf.030101.xyz                     | 104.19.42.208   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 65   | xn--b6gac.eu.org                      | 104.21.90.78    | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 70   | fbi.gov                               | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 143  | whatismyipaddress.com                 | 104.19.223.79   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 174  | 172.67.106.26                         | 172.67.106.26   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 186  | cloudflare-ip.mofashi.ltd             | 104.21.72.233   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 293  | wilson.ns.cloudflare.com              | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 304  | ct.877774.xyz                         | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 321  | ipv4.ip.sb                            | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 330  | iplocation.io                         | 104.26.10.222   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 336  | 104.26.6.112                          | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 364  | cmcc.877774.xyz                       | 104.16.149.8    | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 366  | cmcc.877774.xyz                       | 104.16.149.5    | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 408  | sullivan.ns.cloudflare.com            | 108.162.195.161 | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 414  | 172.67.243.218                        | 172.67.243.218  | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 83   | bowen.ns.cloudflare.com               | 108.162.195.83  | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 120  | 162.159.36.104                        | 162.159.36.104  | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 190  | cf.877774.xyz                         | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 214  | icook.tw                              | 172.66.158.115  | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 265  | 104.19.175.123                        | 104.19.175.123  | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 355  | cmcc.877774.xyz                       | 104.16.149.7    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 374  | cmcc.877774.xyz                       | 104.16.148.11   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 377  | cmcc.877774.xyz                       | 104.16.149.10   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 400  | www.okcupid.com                       | 104.18.160.63   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 407  | 172.67.120.0                          | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 416  | 4444.cloudflare.182682.xyz                         | 172.67.152.183  | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 18   | dylan.ns.cloudflare.com               | 172.64.35.187   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 19   | dylan.ns.cloudflare.com               | 108.162.195.187 | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 25   | toy-people.com                        | 104.26.3.36     | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 36   | kyree.ns.cloudflare.com               | 108.162.195.207 | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 64   | xn--b6gac.eu.org                      | 172.67.153.253  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 90   | saas.sin.fan                          | 162.159.36.20   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 91   | saas.sin.fan                          | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 93   | www.ipchicken.com                     | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 113  | ip.sb                                 | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 209  | 198.62.62.4                           | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 226  | 104.18.78.214                         | 104.18.78.214   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 259  | japan.com                             | 104.26.4.60     | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 379  | cmcc.877774.xyz                       | 104.16.148.2    | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 384  | www.hugedomains.com                   | 104.26.7.37     | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 438  | cfip.xxxxxxxx.tk                      | 190.93.244.201  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 439  | cfip.xxxxxxxx.tk                      | 104.16.232.223  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 34   | 104.16.223.179                        | 104.16.223.179  | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 152  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 253  | 172.67.181.209                        | 172.67.181.209  | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 319  | 104.18.39.196                         | 104.18.39.196   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 351  | icook.hk                              | 104.21.90.210   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 361  | cmcc.877774.xyz                       | 104.16.149.1    | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 44   | asia.877774.xyz                       | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 68   | 172.64.151.55                         | 172.64.151.55   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 74   | braden.ns.cloudflare.com              | 108.162.195.169 | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 80   | na.877774.xyz                         | 104.18.38.235   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 115  | ip.sb                                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 154  | dnschecker.org                        | 104.26.6.89     | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 155  | dnschecker.org                        | 172.67.73.216   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 176  | 162.159.133.85                        | 162.159.133.85  | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 232  | 104.19.223.58                         | 104.19.223.58   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 255  | ifconfig.co                           | 172.67.168.106  | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 260  | japan.com                             | 104.26.5.60     | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 267  | 172.67.110.232                        | 172.67.110.232  | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 311  | ipinfo.in                             | 172.67.198.203  | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 359  | cmcc.877774.xyz                       | 104.16.148.10   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 367  | cmcc.877774.xyz                       | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 41   | 104.17.79.11                          | 104.17.79.11    | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 71   | fbi.gov                               | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 170  | moura.ns.cloudflare.com               | 172.64.35.217   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 203  | 104.17.142.12                         | 104.17.142.12   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 205  | cf.090227.xyz                         | 172.64.144.82   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 305  | ct.877774.xyz                         | 172.64.229.217  | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 415  | 4444.cloudflare.182682.xyz                         | 104.21.80.180   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 81   | na.877774.xyz                         | 104.18.187.25   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 210  | 104.18.37.40                          | 104.18.37.40    | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 288  | trevor.ns.cloudflare.com              | 4444.cloudflare.182682.xyz | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 306  | ct.877774.xyz                         | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 369  | cmcc.877774.xyz                       | 104.16.149.2    | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 389  | cu.877774.xyz                         | 104.26.4.115    | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 42   | asia.877774.xyz                       | 104.16.211.153  | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 30   | freeyx.cloudflare88.eu.org            | 141.101.121.80  | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 92   | www.ipchicken.com                     | 104.26.7.112    | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 229  | www.digitalocean.com                  | 104.19.173.68   | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 247  | lewis.ns.cloudflare.com               | 108.162.195.159 | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 322  | ipv4.ip.sb                            | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 323  | ipv4.ip.sb                            | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 358  | cmcc.877774.xyz                       | 104.16.149.4    | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 15   | www.ipget.net                         | 172.67.207.26   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 191  | cf.877774.xyz                         | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 235  | damien.ns.cloudflare.com              | 108.162.195.168 | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 339  | shopify.com                           | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 345  | huxley.ns.cloudflare.com              | 172.64.35.188   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 401  | www.okcupid.com                       | 104.16.223.254  | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 107  | cf.zhetengsha.eu.org                  | 104.18.35.15    | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 121  | singapore.com                         | 104.26.13.140   | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 133  | 456.cloudflare.182682.xyz             | 172.67.75.208   | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 194  | www.udemy.com                         | 104.16.143.237  | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 204  | cf.090227.xyz                         | 104.18.43.174   | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 312  | ipinfo.in                             | 104.21.21.129   | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 370  | cmcc.877774.xyz                       | 104.16.148.9    | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 381  | www.4chan.org                         | 104.16.228.229  | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 390  | cu.877774.xyz                         | 104.26.4.116    | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 395  | cu.877774.xyz                         | 104.26.4.111    | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 156  | dnschecker.org                        | 104.26.7.89     | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 211  | 172.64.35.24                          | 172.64.35.24    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 249  | lewis.ns.cloudflare.com               | 172.64.35.159   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 409  | sullivan.ns.cloudflare.com            | 172.64.35.161   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 89   | www.7749tv.com                        | 104.19.133.4    | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 134  | 456.cloudflare.182682.xyz             | 104.26.8.160    | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 269  | eur.877774.xyz                        | 104.21.29.164   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 344  | huxley.ns.cloudflare.com              | 162.159.44.188  | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 85   | bowen.ns.cloudflare.com               | 172.64.35.83    | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 102  | rustam.ns.cloudflare.com              | 108.162.195.148 | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 234  | www.visa.com.hk                       | 104.18.20.69    | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 299  | ct.877774.xyz                         | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 301  | ct.877774.xyz                         | 172.64.229.173  | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 419  | craig.ns.cloudflare.com               | 172.64.35.192   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 76   | braden.ns.cloudflare.com              | 172.64.35.169   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 208  | 172.67.79.211                         | 172.67.79.211   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 332  | iplocation.io                         | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 349  | 104.18.254.88                         | 104.18.254.88   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 380  | www.4chan.org                         | 104.16.229.229  | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 392  | cu.877774.xyz                         | 104.26.4.114    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 26   | toy-people.com                        | 172.67.72.18    | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 237  | damien.ns.cloudflare.com              | 172.64.35.168   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 340  | cf.0sm.com                            | 172.67.187.145  | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 341  | cf.0sm.com                            | 104.21.7.133    | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 372  | cmcc.877774.xyz                       | 104.16.149.9    | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 385  | www.hugedomains.com                   | 104.26.6.37     | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 31 条记录
- **快 (50-100ms)**: 169 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 3 次
- **IPv6 失败**: 163 次

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
