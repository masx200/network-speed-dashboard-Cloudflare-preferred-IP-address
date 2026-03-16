# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/14 14:46:31
- **数据来源**: connectivity_results-20251214-144630.json
- **总测试数**: 443
- **失败测试数**: 167
- **成功测试数**: 276
- **失败率**: 37.70%
- **平均延迟**: 107.38ms
- **最小延迟**: 40ms
- **最大延迟**: 638ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/14 14:46:31
- **IP地址**: 20.161.70.177
- **国家/地区**: 美国 (US)
- **ASN**: 8075
- **网络组织**: Microsoft Corporation
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
| 352  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 441  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |
| 443  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |

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

**问题主机分析**: 以下主机出现多次失败：trevor.ns.cloudflare.com (3次),
wilson.ns.cloudflare.com (3次), moura.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 402  | 172.67.181.209                        | 172.67.181.209  | IPv4   | h3   | ✅ 成功 | 40       | cloudflare |
| 314  | zread.ai                              | 104.21.76.240   | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 265  | pranab.ns.cloudflare.com              | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 354  | cfip.xxxxxxxx.tk                      | 104.16.241.229  | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 420  | 108.162.198.54                        | 108.162.198.54  | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 418  | saas.sin.fan                          | 162.159.36.20   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 365  | www.digitalocean.com                  | 104.19.174.68   | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 253  | tasteatlas.com                        | 104.17.37.105   | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 378  | asia.877774.xyz                       | 104.16.211.153  | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 408  | ifconfig.co                           | 104.21.54.91    | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 136  | www.ipget.net                         | 104.21.15.212   | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 131  | www.okcupid.com                       | 104.16.239.254  | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 318  | decker.ns.cloudflare.com              | 108.162.195.155 | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 25   | ct.877774.xyz                         | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 115  | cmcc.877774.xyz                       | 104.16.148.3    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 219  | dnschecker.org                        | 104.26.6.89     | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 377  | asia.877774.xyz                       | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 401  | 104.19.223.58                         | 104.19.223.58   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 213  | ashton.ns.cloudflare.com              | 108.162.195.173 | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 380  | www.wto.org                           | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 295  | www.whatismyip.com                    | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 207  | ip.sb                                 | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 20   | moura.ns.cloudflare.com               | 108.162.195.217 | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 122  | sullivan.ns.cloudflare.com            | 108.162.195.161 | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 140  | cu.877774.xyz                         | 104.26.4.118    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 346  | braden.ns.cloudflare.com              | 172.64.35.169   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 413  | stock.hostmonit.com                   | 172.67.187.251  | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 128  | www.okcupid.com                       | 104.16.144.63   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 40   | steamdb.info                          | 172.66.175.250  | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 144  | cu.877774.xyz                         | 104.26.4.114    | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 181  | palera.in                             | 104.21.58.72    | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 270  | yx-auto.pages.dev                     | 172.66.44.144   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 331  | bestcf.030101.xyz                     | 104.17.211.46   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 358  | cf.zhetengsha.eu.org                  | 104.18.42.98    | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 404  | cf.090227.xyz                         | 104.18.43.174   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 86   | www.4chan.org                         | 104.16.229.229  | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 110  | cmcc.877774.xyz                       | 104.16.149.11   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 142  | cu.877774.xyz                         | 104.26.4.116    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 212  | ashton.ns.cloudflare.com              | 172.64.35.173   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 373  | otto.ns.cloudflare.com                | 172.64.35.135   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 412  | 172.64.151.55                         | 172.64.151.55   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 428  | www.csgo.com                          | 195.85.59.161   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 106  | cmcc.877774.xyz                       | 104.16.149.7    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 150  | cf.877771.xyz                         | 188.114.96.3    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 353  | cfip.xxxxxxxx.tk                      | 188.114.96.125  | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 359  | cf.zhetengsha.eu.org                  | 172.64.145.158  | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 362  | 104.18.37.40                          | 104.18.37.40    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 58   | 104.26.6.112                          | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 100  | cmcc.877774.xyz                       | 104.16.149.1    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 102  | cmcc.877774.xyz                       | 104.16.149.3    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 229  | julio.ns.cloudflare.com               | 108.162.195.209 | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 350  | cfip.xxxxxxxx.tk                      | 190.93.244.201  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 381  | www.wto.org                           | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 87   | www.4chan.org                         | 104.16.228.229  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 99   | cmcc.877774.xyz                       | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 114  | cmcc.877774.xyz                       | 104.16.148.2    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 145  | cu.877774.xyz                         | 104.26.4.113    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 148  | cu.877774.xyz                         | 104.26.4.119    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 307  | 104.17.79.11                          | 104.17.79.11    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 355  | 104.17.142.12                         | 104.17.142.12   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 108  | cmcc.877774.xyz                       | 104.16.149.9    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 169  | www.ipchicken.com                     | 104.26.7.112    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 185  | 104.17.68.85                          | 104.17.68.85    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 188  | singapore.com                         | 104.26.12.140   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 339  | icook.tw                              | 104.20.28.74    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 356  | 172.67.79.211                         | 172.67.79.211   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 154  | benedict.ns.cloudflare.com            | 108.162.195.205 | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 290  | toy-people.com                        | 104.26.3.36     | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 370  | eur.877774.xyz                        | 104.21.29.164   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 387  | 104.18.78.214                         | 104.18.78.214   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 43   | ipinfo.in                             | 104.21.21.129   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 57   | 104.18.37.13                          | 104.18.37.13    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 95   | cmcc.877774.xyz                       | 104.16.148.9    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 149  | cf.877771.xyz                         | 188.114.97.3    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 257  | www.udemy.com                         | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 275  | cloudflare-ip.mofashi.ltd             | 172.67.155.172  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 116  | cmcc.877774.xyz                       | 104.16.148.4    | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 129  | www.okcupid.com                       | 104.16.223.254  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 199  | ip.gs                                 | 104.21.14.176   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 306  | 104.16.223.179                        | 104.16.223.179  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 386  | cfip.1323123.xyz                      | 104.16.133.220  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 403  | cf.090227.xyz                         | 172.64.144.82   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 72   | www.visa.cn                           | 162.159.152.2   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 101  | cmcc.877774.xyz                       | 104.16.149.2    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 180  | palera.in                             | 172.67.157.122  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 206  | ip.sb                                 | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 266  | pranab.ns.cloudflare.com              | 172.64.35.199   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 285  | na.877774.xyz                         | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 286  | na.877774.xyz                         | 104.18.38.235   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 117  | cmcc.877774.xyz                       | 104.16.148.5    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 235  | uriah.ns.cloudflare.com               | 108.162.195.194 | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 315  | zread.ai                              | 172.67.202.78   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 369  | eur.877774.xyz                        | 104.21.26.150   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 407  | ifconfig.co                           | 172.67.168.106  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 26   | ct.877774.xyz                         | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 111  | cmcc.877774.xyz                       | 104.16.149.12   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 130  | www.okcupid.com                       | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 187  | singapore.com                         | 172.67.75.194   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 262  | www.visa.com.sg                       | 104.18.12.229   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 335  | xn--b6gac.eu.org                      | 172.67.153.253  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 50   | www.gov.ua                            | 172.67.209.127  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 76   | icook.hk                              | 104.21.90.210   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 391  | damien.ns.cloudflare.com              | 108.162.195.168 | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 28   | ct.877774.xyz                         | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 160  | gamer.com.tw                          | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 289  | toy-people.com                        | 104.26.2.36     | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 396  | japan.com                             | 104.26.4.60     | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 81   | huxley.ns.cloudflare.com              | 172.64.35.188   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 107  | cmcc.877774.xyz                       | 104.16.149.8    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 178  | www.glassdoor.com                     | 104.16.25.46    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 240  | 162.159.133.85                        | 162.159.133.85  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 309  | kyree.ns.cloudflare.com               | 172.64.35.207   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 357  | 198.62.62.4                           | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 397  | japan.com                             | 172.67.70.92    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 54   | ipv4.ip.sb                            | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 226  | 104.18.14.76                          | 104.18.14.76    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 243  | whatismyipaddress.com                 | 104.19.222.79   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 247  | www.visa.com.hk                       | 104.18.20.69    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 119  | cmcc.877774.xyz                       | 104.16.148.7    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 308  | kyree.ns.cloudflare.com               | 108.162.195.207 | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 31   | ct.877774.xyz                         | 172.64.229.185  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 105  | cmcc.877774.xyz                       | 104.16.149.6    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 242  | whatismyipaddress.com                 | 104.19.223.79   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 29   | ct.877774.xyz                         | 172.64.229.217  | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 55   | ipv4.ip.sb                            | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 61   | iplocation.io                         | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 179  | 104.16.61.163                         | 104.16.61.163   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 252  | tasteatlas.com                        | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 326  | lewis.ns.cloudflare.com               | 108.162.195.159 | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 417  | 104.19.175.123                        | 104.19.175.123  | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 8    | www.pcmag.com                         | 104.16.21.118   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 59   | iplocation.io                         | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 73   | www.visa.cn                           | 162.159.153.2   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 97   | cmcc.877774.xyz                       | 104.16.148.11   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 147  | cu.877774.xyz                         | 104.26.4.111    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 227  | julio.ns.cloudflare.com               | 172.64.35.209   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 421  | abdullah.ns.cloudflare.com            | 172.64.35.203   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 98   | cmcc.877774.xyz                       | 104.16.148.12   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 143  | cu.877774.xyz                         | 104.26.4.115    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 153  | benedict.ns.cloudflare.com            | 172.64.35.205   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 27   | ct.877774.xyz                         | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 120  | cmcc.877774.xyz                       | 104.16.148.8    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 385  | 104.26.13.31                          | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 419  | saas.sin.fan                          | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 80   | huxley.ns.cloudflare.com              | 108.162.195.188 | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 121  | sullivan.ns.cloudflare.com            | 172.64.35.161   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 132  | comicabc.com                          | 172.67.174.21   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 170  | www.ipchicken.com                     | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 177  | www.glassdoor.com                     | 104.17.64.70    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 296  | www.whatismyip.com                    | 104.26.13.23    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 371  | otto.ns.cloudflare.com                | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 75   | shopify.com                           | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 162  | time.is                               | 104.26.12.54    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 168  | www.ipchicken.com                     | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 261  | www.visa.com.sg                       | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 330  | bestcf.030101.xyz                     | 104.17.212.134  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 364  | www.digitalocean.com                  | 104.19.173.68   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 56   | ipv4.ip.sb                            | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 96   | cmcc.877774.xyz                       | 104.16.148.10   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 279  | cris.ns.cloudflare.com                | 172.64.35.202   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 302  | dylan.ns.cloudflare.com               | 172.64.35.187   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 340  | icook.tw                              | 172.66.158.115  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 390  | damien.ns.cloudflare.com              | 172.64.35.168   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 2    | trevor.ns.cloudflare.com              | 172.64.35.154   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 44   | ipinfo.in                             | 172.67.198.203  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 146  | cu.877774.xyz                         | 104.26.4.112    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 193  | 456.cloudflare.182682.xyz             | 104.26.9.160    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 280  | cris.ns.cloudflare.com                | 108.162.195.202 | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 325  | lewis.ns.cloudflare.com               | 172.64.35.159   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 434  | bowen.ns.cloudflare.com               | 172.64.35.83    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 3    | trevor.ns.cloudflare.com              | 108.162.195.154 | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 90   | www.hugedomains.com                   | 104.26.6.37     | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 103  | cmcc.877774.xyz                       | 104.16.149.4    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 414  | stock.hostmonit.com                   | 104.21.7.193    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 30   | ct.877774.xyz                         | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 113  | cmcc.877774.xyz                       | 104.16.148.1    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 163  | time.is                               | 104.26.13.54    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 225  | 172.67.106.26                         | 172.67.106.26   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 241  | 172.67.120.0                          | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 284  | na.877774.xyz                         | 104.18.187.25   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 294  | www.whatismyip.com                    | 172.67.69.129   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 379  | asia.877774.xyz                       | 104.17.139.62   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 68   | cf.0sm.com                            | 104.21.7.133    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 233  | uriah.ns.cloudflare.com               | 172.64.35.194   | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 422  | abdullah.ns.cloudflare.com            | 108.162.195.203 | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 429  | fbi.gov                               | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 133  | comicabc.com                          | 104.21.64.10    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 159  | gamer.com.tw                          | 104.18.2.197    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 203  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 205  | ip.sb                                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 274  | cloudflare-ip.mofashi.ltd             | 104.21.72.233   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 336  | xn--b6gac.eu.org                      | 104.21.90.78    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 395  | japan.com                             | 104.26.5.60     | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 24   | www.7749tv.com                        | 104.19.93.88    | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 49   | 172.67.75.172                         | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 218  | dnschecker.org                        | 172.67.73.216   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 363  | 172.64.35.24                          | 172.64.35.24    | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 368  | eur.877774.xyz                        | 104.21.47.209   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 435  | bowen.ns.cloudflare.com               | 108.162.195.83  | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 439  | freeyx.cloudflare88.eu.org            | 141.101.120.45  | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 1 条记录
- **快 (50-100ms)**: 156 条记录
- **正常 (100-200ms)**: 43 条记录
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
