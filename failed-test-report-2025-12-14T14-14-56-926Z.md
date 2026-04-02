# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/14 14:14:56
- **数据来源**: connectivity_results-20251214-141456.json
- **总测试数**: 441
- **失败测试数**: 167
- **成功测试数**: 274
- **失败率**: 37.87%
- **平均延迟**: 103.88ms
- **最小延迟**: 48ms
- **最大延迟**: 739ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/14 14:14:56
- **IP地址**: 172.172.119.100
- **国家/地区**: 美国 (US)
- **ASN**: 8075
- **网络组织**: cloud
- **网络域名**: microsoft.com
- **大洲**: North America (NA)
- **地理坐标**: 36.6694, -78.3877
- **时区**: America/New_York
- **数据源**: combined

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
| 319  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 439  | cf.877774.xyz    | cf.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp cf.877774.xyz:443: i/o timeout  |
| 441  | ct.877774.xyz    | ct.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout  |

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

**问题主机分析**: 以下主机出现多次失败：wilson.ns.cloudflare.com (3次),
trevor.ns.cloudflare.com (3次), iplocation.io
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 290  | saas.sin.fan                          | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 256  | ip.sb                                 | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 202  | zread.ai                              | 104.21.76.240   | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 437  | www.csgo.com                          | 195.85.59.95    | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 361  | tasteatlas.com                        | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 366  | www.udemy.com                         | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 241  | www.glassdoor.com                     | 104.16.25.46    | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 270  | palera.in                             | 104.21.58.72    | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 427  | stock.hostmonit.com                   | 172.67.187.251  | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 164  | cmcc.877774.xyz                       | 104.16.149.7    | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 196  | dylan.ns.cloudflare.com               | 172.64.35.187   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 365  | www.udemy.com                         | 104.16.143.237  | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 381  | 104.18.78.214                         | 104.18.78.214   | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 267  | 104.16.61.163                         | 104.16.61.163   | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 32   | 104.16.45.84                          | 104.16.45.84    | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 35   | ct.877774.xyz                         | 172.64.229.173  | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 360  | tasteatlas.com                        | 104.17.37.105   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 197  | dylan.ns.cloudflare.com               | 108.162.195.187 | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 223  | asia.877774.xyz                       | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 310  | ashton.ns.cloudflare.com              | 108.162.195.173 | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 194  | 104.16.223.179                        | 104.16.223.179  | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 240  | www.glassdoor.com                     | 104.17.64.70    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 273  | 104.17.68.85                          | 104.17.68.85    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 357  | cf.877774.xyz                         | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 13   | ipinfo.in                             | 172.67.198.203  | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 47   | iplocation.io                         | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 206  | freeyx.cloudflare88.eu.org            | 141.101.120.184 | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 356  | cf.877774.xyz                         | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 38   | ct.877774.xyz                         | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 394  | damien.ns.cloudflare.com              | 172.64.35.168   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 9    | www.pcmag.com                         | 104.16.20.118   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 84   | cu.877774.xyz                         | 104.26.4.113    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 113  | 172.67.120.0                          | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 215  | www.whatismyip.com                    | 104.26.13.23    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 49   | iplocation.io                         | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 70   | ipv4.ip.sb                            | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 165  | cmcc.877774.xyz                       | 104.16.149.8    | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 181  | na.877774.xyz                         | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 341  | moura.ns.cloudflare.com               | 108.162.195.217 | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 372  | 172.64.35.24                          | 172.64.35.24    | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 10   | www.pcmag.com                         | 104.16.21.118   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 195  | 104.17.79.11                          | 104.17.79.11    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 324  | www.visa.com.hk                       | 104.18.21.69    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 369  | 172.67.79.211                         | 172.67.79.211   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 385  | otto.ns.cloudflare.com                | 172.64.35.135   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 2    | www.7749tv.com                        | 104.19.133.4    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 18   | trevor.ns.cloudflare.com              | 172.64.35.154   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 225  | asia.877774.xyz                       | 104.17.139.62   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 4    | wilson.ns.cloudflare.com              | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 90   | rustam.ns.cloudflare.com              | 108.162.195.148 | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 250  | benedict.ns.cloudflare.com            | 172.64.35.205   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 321  | 172.67.106.26                         | 172.67.106.26   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 23   | cf.0sm.com                            | 172.67.187.145  | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 45   | 172.67.75.172                         | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 87   | cu.877774.xyz                         | 104.26.4.116    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 91   | rustam.ns.cloudflare.com              | 172.64.35.148   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 110  | www.okcupid.com                       | 104.16.223.254  | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 152  | cmcc.877774.xyz                       | 104.16.148.8    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 156  | cmcc.877774.xyz                       | 104.16.148.12   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 157  | cmcc.877774.xyz                       | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 170  | cmcc.877774.xyz                       | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 247  | www.ipchicken.com                     | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 276  | singapore.com                         | 104.26.12.140   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 280  | ip.gs                                 | 188.114.96.3    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 287  | cf.090227.xyz                         | 104.18.42.98    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 401  | ifconfig.co                           | 172.67.168.106  | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 108  | www.okcupid.com                       | 104.18.160.63   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 214  | www.whatismyip.com                    | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 255  | ip.sb                                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 411  | cfip.1323123.xyz                      | 104.16.133.220  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 138  | www.visa.com.sg                       | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 159  | cmcc.877774.xyz                       | 104.16.149.2    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 189  | kyree.ns.cloudflare.com               | 172.64.35.207   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 213  | www.whatismyip.com                    | 172.67.69.129   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 243  | fbi.gov                               | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 304  | dnschecker.org                        | 104.26.6.89     | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 306  | dnschecker.org                        | 172.67.73.216   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 376  | 104.26.13.31                          | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 36   | ct.877774.xyz                         | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 59   | icook.hk                              | 172.67.161.104  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 123  | 172.67.243.218                        | 172.67.243.218  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 129  | yx-auto.pages.dev                     | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 132  | pranab.ns.cloudflare.com              | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 150  | cmcc.877774.xyz                       | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 158  | cmcc.877774.xyz                       | 104.16.149.1    | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 173  | toy-people.com                        | 104.26.3.36     | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 389  | icook.tw                              | 172.66.158.115  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 48   | iplocation.io                         | 104.26.10.222   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 57   | 103.160.204.59                        | 103.160.204.59  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 58   | icook.hk                              | 104.21.90.210   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 128  | yx-auto.pages.dev                     | 172.66.44.144   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 190  | kyree.ns.cloudflare.com               | 108.162.195.207 | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 203  | zread.ai                              | 172.67.202.78   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 232  | xn--b6gac.eu.org                      | 104.21.90.78    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 325  | julio.ns.cloudflare.com               | 108.162.195.209 | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 374  | gamer.com.tw                          | 104.18.2.197    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 39   | ct.877774.xyz                         | 172.64.229.217  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 151  | cmcc.877774.xyz                       | 104.16.148.7    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 268  | 162.159.36.104                        | 162.159.36.104  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 269  | palera.in                             | 172.67.157.122  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 378  | www.digitalocean.com                  | 104.19.174.68   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 400  | ifconfig.co                           | 104.21.54.91    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 422  | japan.com                             | 172.67.70.92    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 428  | stock.hostmonit.com                   | 104.21.7.193    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 433  | abdullah.ns.cloudflare.com            | 172.64.35.203   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 117  | craig.ns.cloudflare.com               | 172.64.35.192   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 182  | decker.ns.cloudflare.com              | 108.162.195.155 | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 219  | cloudflare-ip.mofashi.ltd             | 188.114.96.3    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 383  | otto.ns.cloudflare.com                | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 64   | huxley.ns.cloudflare.com              | 108.162.195.188 | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 75   | www.hugedomains.com                   | 104.26.7.37     | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 111  | www.okcupid.com                       | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 149  | cmcc.877774.xyz                       | 104.16.148.5    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 153  | cmcc.877774.xyz                       | 104.16.148.9    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 226  | braden.ns.cloudflare.com              | 108.162.195.169 | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 237  | cf.zhetengsha.eu.org                  | 172.64.144.82   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 275  | singapore.com                         | 172.67.75.194   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 371  | 104.18.37.40                          | 104.18.37.40    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 236  | cf.zhetengsha.eu.org                  | 104.18.43.174   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 364  | 104.17.142.12                         | 104.17.142.12   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 421  | japan.com                             | 104.26.5.60     | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 34   | ct.877774.xyz                         | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 71   | ipv4.ip.sb                            | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 73   | www.4chan.org                         | 104.16.228.229  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 134  | pranab.ns.cloudflare.com              | 172.64.35.199   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 161  | cmcc.877774.xyz                       | 104.16.149.4    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 281  | ip.gs                                 | 188.114.97.3    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 323  | www.visa.com.hk                       | 104.18.20.69    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 412  | eur.877774.xyz                        | 104.21.47.209   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 37   | ct.877774.xyz                         | 172.64.229.185  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 76   | www.hugedomains.com                   | 172.67.70.191   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 82   | cu.877774.xyz                         | 104.26.4.112    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 142  | cris.ns.cloudflare.com                | 172.64.35.202   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 284  | 172.64.151.55                         | 172.64.151.55   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 296  | 456.cloudflare.182682.xyz             | 104.26.8.160    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 302  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 405  | lewis.ns.cloudflare.com               | 108.162.195.159 | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 28   | steamdb.info                          | 172.66.175.250  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 41   | www.gov.ua                            | 104.21.23.72    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 72   | www.4chan.org                         | 104.16.229.229  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 124  | 4444.cloudflare.182682.xyz                         | 104.21.80.180   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 242  | fbi.gov                               | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 318  | cfip.xxxxxxxx.tk                      | 190.93.244.201  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 414  | eur.877774.xyz                        | 104.21.29.164   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 438  | www.csgo.com                          | 195.85.59.161   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 88   | cu.877774.xyz                         | 104.26.4.114    | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 162  | cmcc.877774.xyz                       | 104.16.149.5    | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 168  | cmcc.877774.xyz                       | 104.16.149.11   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 172  | cmcc.877774.xyz                       | 104.16.148.2    | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 179  | na.877774.xyz                         | 104.18.38.235   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 274  | singapore.com                         | 104.26.13.140   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 31   | 104.18.39.196                         | 104.18.39.196   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 42   | www.gov.ua                            | 172.67.209.127  | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 104  | sullivan.ns.cloudflare.com            | 172.64.35.161   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 174  | toy-people.com                        | 104.26.2.36     | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 331  | 162.159.133.85                        | 162.159.133.85  | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 347  | www.ipget.net                         | 104.21.15.212   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 370  | 198.62.62.4                           | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 413  | eur.877774.xyz                        | 104.21.26.150   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 140  | cris.ns.cloudflare.com                | 108.162.195.202 | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 147  | cmcc.877774.xyz                       | 104.16.148.3    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 169  | cmcc.877774.xyz                       | 104.16.149.12   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 233  | xn--b6gac.eu.org                      | 172.67.153.253  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 293  | whatismyipaddress.com                 | 104.19.223.79   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 351  | comicabc.com                          | 172.67.174.21   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 415  | 104.19.175.123                        | 104.19.175.123  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 74   | www.hugedomains.com                   | 104.26.6.37     | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 118  | craig.ns.cloudflare.com               | 108.162.195.192 | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 166  | cmcc.877774.xyz                       | 104.16.149.9    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 257  | ip.sb                                 | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 96   | time.is                               | 172.67.68.157   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 209  | bestcf.030101.xyz                     | 104.17.212.134  | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 33   | ct.877774.xyz                         | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 63   | huxley.ns.cloudflare.com              | 172.64.35.188   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 115  | www.4444.cloudflare.182682.xyz                           | 162.159.153.2   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 160  | cmcc.877774.xyz                       | 104.16.149.3    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 180  | na.877774.xyz                         | 104.18.187.25   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 248  | www.ipchicken.com                     | 104.26.7.112    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 286  | cf.090227.xyz                         | 172.64.145.158  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 395  | damien.ns.cloudflare.com              | 108.162.195.168 | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 404  | lewis.ns.cloudflare.com               | 172.64.35.159   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 114  | www.4444.cloudflare.182682.xyz                           | 162.159.152.2   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 262  | bowen.ns.cloudflare.com               | 108.162.195.83  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 335  | uriah.ns.cloudflare.com               | 172.64.35.194   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 416  | www.wto.org                           | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 27   | steamdb.info                          | 104.20.34.212   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 56   | shopify.com                           | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 86   | cu.877774.xyz                         | 104.26.4.118    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 167  | cmcc.877774.xyz                       | 104.16.149.10   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 261  | bowen.ns.cloudflare.com               | 172.64.35.83    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 46   | 104.18.37.13                          | 104.18.37.13    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 62   | 104.18.254.88                         | 104.18.254.88   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 80   | cu.877774.xyz                         | 104.26.4.119    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 125  | 4444.cloudflare.182682.xyz                         | 172.67.152.183  | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 431  | abdullah.ns.cloudflare.com            | 108.162.195.203 | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 19   | trevor.ns.cloudflare.com              | 4444.cloudflare.182682.xyz | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 83   | cu.877774.xyz                         | 104.26.4.115    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 373  | gamer.com.tw                          | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 40   | ct.877774.xyz                         | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 97   | time.is                               | 104.26.13.54    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 2 条记录
- **快 (50-100ms)**: 179 条记录
- **正常 (100-200ms)**: 19 条记录
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
