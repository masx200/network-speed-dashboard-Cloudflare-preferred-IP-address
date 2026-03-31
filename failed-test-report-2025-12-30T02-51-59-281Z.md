# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/30 02:51:59
- **数据来源**: connectivity_results-20251230-025158.json
- **总测试数**: 633
- **失败测试数**: 203
- **成功测试数**: 430
- **失败率**: 32.07%
- **平均延迟**: 107.76ms
- **最小延迟**: 52ms
- **最大延迟**: 683ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/30 02:51:59
- **IP地址**: 20.49.61.54
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

- **网络不可达: 网络不可达**: 200 次 (98.5%)
- **连接超时: I/O超时**: 3 次 (1.5%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (200 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
| ---- | --------- | ------ | ------ | ---- | ------ | -------- | ------ | -------- |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 411  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 486  | cf.877774.xyz    | cf.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp cf.877774.xyz:443: i/o timeout  |
| 633  | ct.877774.xyz    | ct.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 200 次 (98.5%)
- **连接超时**: 3 次 (1.5%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 203 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 200 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次),
trevor.ns.cloudflare.com (3次), wilson.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 430 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 114  | cu.877774.xyz                         | 172.64.145.202  | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 383  | 104.26.13.31                          | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 417  | eur.877774.xyz                        | 104.21.29.164   | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 571  | 104.17.101.208                        | 104.17.101.208  | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 573  | 162.159.34.55                         | 162.159.34.55   | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 598  | 104.17.21.106                         | 104.17.21.106   | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 517  | 104.20.24.107                         | 104.20.24.107   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 316  | whatismyipaddress.com                 | 104.19.223.79   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 461  | 172.64.229.7                          | 172.64.229.7    | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 376  | 172.64.35.24                          | 172.64.35.24    | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 379  | icook.tw                              | 172.66.158.115  | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 415  | cfip.xxxxxxxx.tk                      | 104.16.232.223  | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 579  | 162.159.19.37                         | 162.159.19.37   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 103  | huxley.ns.cloudflare.com              | 108.162.195.188 | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 283  | ip.sb                                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 372  | 104.17.142.12                         | 104.17.142.12   | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 384  | www.digitalocean.com                  | 104.19.173.68   | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 623  | 172.64.151.235                        | 172.64.151.235  | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 271  | time.is                               | 104.26.12.54    | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 373  | 172.67.79.211                         | 172.67.79.211   | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 370  | gamer.com.tw                          | 104.18.2.197    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 436  | 198.41.194.162                        | 198.41.194.162  | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 577  | 104.18.39.228                         | 104.18.39.228   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 61   | 172.67.110.232                        | 172.67.110.232  | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 593  | 104.16.147.114                        | 104.16.147.114  | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 298  | singapore.com                         | 104.26.12.140   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 446  | cfip.1323123.xyz                      | 104.16.133.220  | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 600  | 104.16.144.235                        | 104.16.144.235  | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 367  | www.udemy.com                         | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 574  | 162.159.42.140                        | 162.159.42.140  | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 285  | ip.sb                                 | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 469  | 162.159.140.116                       | 162.159.140.116 | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 356  | tasteatlas.com                        | 104.17.37.105   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 485  | 104.17.167.134                        | 104.17.167.134  | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 526  | 104.17.156.81                         | 104.17.156.81   | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 199  | 104.16.223.179                        | 104.16.223.179  | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 513  | 172.67.68.211                         | 172.67.68.211   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 559  | 104.26.12.227                         | 104.26.12.227   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 473  | 198.41.208.224                        | 198.41.208.224  | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 6    | www.ipget.net                         | 104.21.15.212   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 549  | 172.64.52.110                         | 172.64.52.110   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 624  | 172.64.52.90                          | 172.64.52.90    | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 207  | freeyx.cloudflare88.eu.org            | 141.101.120.39  | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 516  | 172.67.64.214                         | 172.67.64.214   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 133  | www.okcupid.com                       | 104.18.160.63   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 273  | time.is                               | 104.26.13.54    | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 329  | ashton.ns.cloudflare.com              | 172.64.35.173   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 477  | 104.17.139.37                         | 104.17.139.37   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 177  | toy-people.com                        | 104.26.2.36     | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 507  | 104.18.47.253                         | 104.18.47.253   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 583  | 162.159.62.6                          | 162.159.62.6    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 24   | ipinfo.in                             | 172.67.198.203  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 95   | wilson.ns.cloudflare.com              | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 152  | www.visa.cn                           | 162.159.152.2   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 214  | zread.ai                              | 172.67.202.78   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 89   | cmcc.877774.xyz                       | 104.16.149.1    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 254  | saas.sin.fan                          | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 270  | www.ipchicken.com                     | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 570  | 104.17.100.254                        | 104.17.100.254  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 581  | 104.18.44.148                         | 104.18.44.148   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 115  | cu.877774.xyz                         | 104.18.42.54    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 183  | kyree.ns.cloudflare.com               | 172.64.35.207   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 223  | asia.877774.xyz                       | 104.16.211.153  | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 284  | ip.sb                                 | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 459  | 173.245.49.194                        | 173.245.49.194  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 474  | 104.18.166.129                        | 104.18.166.129  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 498  | 104.17.142.212                        | 104.17.142.212  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 530  | 104.17.170.110                        | 104.17.170.110  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 589  | 172.67.70.253                         | 172.67.70.253   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 594  | 104.17.24.232                         | 104.17.24.232   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 604  | 162.159.19.219                        | 162.159.19.219  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 5    | www.ipget.net                         | 172.67.207.26   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 19   | trevor.ns.cloudflare.com              | 172.64.35.154   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 195  | decker.ns.cloudflare.com              | 172.64.35.155   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 206  | 104.17.79.11                          | 104.17.79.11    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 269  | www.ipchicken.com                     | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 575  | 104.16.157.50                         | 104.16.157.50   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 30   | ct.877774.xyz                         | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 181  | kyree.ns.cloudflare.com               | 108.162.195.207 | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 201  | www.whatismyip.com                    | 172.67.69.129   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 340  | 162.159.133.85                        | 162.159.133.85  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 511  | 104.18.45.95                          | 104.18.45.95    | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 293  | 104.17.68.85                          | 104.17.68.85    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 390  | damien.ns.cloudflare.com              | 172.64.35.168   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 466  | 162.159.128.253                       | 162.159.128.253 | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 476  | 104.17.162.3                          | 104.17.162.3    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 536  | 172.64.50.51                          | 172.64.50.51    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 567  | 104.19.50.35                          | 104.19.50.35    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 33   | ct.877774.xyz                         | 172.64.229.173  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 73   | cmcc.877774.xyz                       | 104.16.149.11   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 87   | cmcc.877774.xyz                       | 104.16.148.12   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 132  | www.okcupid.com                       | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 441  | 104.26.2.166                          | 104.26.2.166    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 509  | www.csgo.com                          | 195.85.59.161   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 584  | 104.26.12.113                         | 104.26.12.113   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 17   | trevor.ns.cloudflare.com              | 108.162.195.154 | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 175  | toy-people.com                        | 104.26.3.36     | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 202  | www.whatismyip.com                    | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 228  | 172.64.151.55                         | 172.64.151.55   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 351  | cf.877774.xyz                         | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 472  | 104.18.89.52                          | 104.18.89.52    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 519  | 104.20.22.185                         | 104.20.22.185   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 555  | 104.26.8.148                          | 104.26.8.148    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 47   | cloudflare.182682.xyz                 | 104.16.250.22   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 244  | fbi.gov                               | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 311  | cf.090227.xyz                         | 104.18.35.15    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 317  | whatismyipaddress.com                 | 104.19.222.79   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 349  | www.visa.com.hk                       | 104.18.21.69    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 409  | cfip.xxxxxxxx.tk                      | 104.27.21.118   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 449  | stock.hostmonit.com                   | 172.67.187.251  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 470  | 104.26.8.117                          | 104.26.8.117    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 40   | steamdb.info                          | 104.20.34.212   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 146  | 172.67.243.218                        | 172.67.243.218  | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 169  | cris.ns.cloudflare.com                | 172.64.35.202   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 471  | 104.18.81.19                          | 104.18.81.19    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 512  | 172.64.150.30                         | 172.64.150.30   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 551  | 104.20.26.58                          | 104.20.26.58    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 45   | cloudflare.182682.xyz                 | 104.21.224.5    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 92   | cmcc.877774.xyz                       | 104.16.149.4    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 131  | www.okcupid.com                       | 104.16.239.254  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 161  | na.877774.xyz                         | 104.18.187.25   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 258  | moura.ns.cloudflare.com               | 172.64.35.217   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 268  | www.ipchicken.com                     | 104.26.7.112    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 304  | 456.cloudflare.182682.xyz             | 104.26.8.160    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 333  | 104.18.14.76                          | 104.18.14.76    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 462  | 104.26.4.90                           | 104.26.4.90     | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 464  | 162.159.58.65                         | 162.159.58.65   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 27   | ipv4.ip.sb                            | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 70   | cmcc.877774.xyz                       | 104.16.149.8    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 129  | www.okcupid.com                       | 104.16.144.63   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 170  | cris.ns.cloudflare.com                | 108.162.195.202 | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 189  | dylan.ns.cloudflare.com               | 172.64.35.187   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 256  | moura.ns.cloudflare.com               | 108.162.195.217 | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 299  | singapore.com                         | 172.67.75.194   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 350  | cf.877774.xyz                         | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 500  | 104.18.189.153                        | 104.18.189.153  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 564  | 104.17.60.233                         | 104.17.60.233   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 578  | 162.159.36.26                         | 162.159.36.26   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 603  | 172.64.147.235                        | 172.64.147.235  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 433  | 104.19.175.123                        | 104.19.175.123  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 479  | 104.19.220.22                         | 104.19.220.22   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 528  | 104.16.255.1                          | 104.16.255.1    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 529  | 104.17.16.248                         | 104.17.16.248   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 565  | 104.19.44.238                         | 104.19.44.238   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 28   | ipv4.ip.sb                            | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 36   | ct.877774.xyz                         | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 46   | cloudflare.182682.xyz                 | 104.21.227.134  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 63   | cf.0sm.com                            | 104.21.7.133    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 64   | cf.0sm.com                            | 172.67.187.145  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 78   | cmcc.877774.xyz                       | 104.16.148.3    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 85   | cmcc.877774.xyz                       | 104.16.148.10   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 107  | shopify.com                           | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 127  | www.4chan.org                         | 104.16.229.229  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 255  | saas.sin.fan                          | 162.159.36.20   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 263  | rustam.ns.cloudflare.com              | 108.162.195.148 | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 289  | 104.16.61.163                         | 104.16.61.163   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 403  | ifconfig.co                           | 172.67.168.106  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 502  | 162.159.21.116                        | 162.159.21.116  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 535  | 172.64.53.0                           | 172.64.53.0     | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 97   | wilson.ns.cloudflare.com              | 172.64.35.110   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 108  | iplocation.io                         | 104.26.10.222   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 160  | na.877774.xyz                         | 104.18.38.235   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 221  | asia.877774.xyz                       | 104.17.139.62   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 225  | xn--b6gac.eu.org                      | 172.67.153.253  | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 355  | tasteatlas.com                        | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 418  | eur.877774.xyz                        | 104.21.47.209   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 426  | 172.67.181.209                        | 172.67.181.209  | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 531  | 104.17.168.159                        | 104.17.168.159  | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 563  | 104.20.19.201                         | 104.20.19.201   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 628  | www.7749tv.com                        | 104.19.133.4    | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 86   | cmcc.877774.xyz                       | 104.16.148.11   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 336  | dnschecker.org                        | 104.26.6.89     | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 419  | eur.877774.xyz                        | 104.21.26.150   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 427  | japan.com                             | 104.26.4.60     | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 437  | www.wto.org                           | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 450  | stock.hostmonit.com                   | 104.21.7.193    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 522  | 104.26.3.176                          | 104.26.3.176    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 562  | 104.20.18.47                          | 104.20.18.47    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 54   | www.gov.ua                            | 104.21.23.72    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 72   | cmcc.877774.xyz                       | 104.16.149.10   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 241  | cloudflare-ip.mofashi.ltd             | 172.67.155.172  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 300  | singapore.com                         | 104.26.13.140   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 335  | dnschecker.org                        | 172.67.73.216   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 515  | 104.26.15.85                          | 104.26.15.85    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 597  | 104.17.30.164                         | 104.17.30.164   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 620  | 172.64.53.144                         | 172.64.53.144   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 23   | ipinfo.in                             | 104.21.21.129   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 391  | damien.ns.cloudflare.com              | 108.162.195.168 | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 407  | cfip.xxxxxxxx.tk                      | 104.25.105.1    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 463  | 162.159.136.89                        | 162.159.136.89  | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 482  | 104.17.69.244                         | 104.17.69.244   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 121  | www.hugedomains.com                   | 104.26.6.37     | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 291  | www.glassdoor.com                     | 104.17.64.70    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 326  | 172.67.106.26                         | 172.67.106.26   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 334  | dnschecker.org                        | 104.26.7.89     | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 385  | www.digitalocean.com                  | 104.19.174.68   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 487  | 104.18.223.253                        | 104.18.223.253  | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 499  | 104.16.105.166                        | 104.16.105.166  | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 13   | www.pcmag.com                         | 104.16.20.118   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 110  | iplocation.io                         | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 142  | craig.ns.cloudflare.com               | 172.64.35.192   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 176  | toy-people.com                        | 172.67.72.18    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 378  | icook.tw                              | 104.20.28.74    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 460  | 162.159.24.131                        | 162.159.24.131  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 568  | 104.18.40.39                          | 104.18.40.39    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 88   | cmcc.877774.xyz                       | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 90   | cmcc.877774.xyz                       | 104.16.149.2    | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 595  | 104.16.153.12                         | 104.16.153.12   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 48   | cloudflare.182682.xyz                 | 104.17.25.173   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 130  | www.okcupid.com                       | 104.16.223.254  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 135  | sullivan.ns.cloudflare.com            | 108.162.195.161 | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 558  | 104.20.21.161                         | 104.20.21.161   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 588  | 104.20.17.233                         | 104.20.17.233   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 14   | www.pcmag.com                         | 104.16.21.118   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 69   | cmcc.877774.xyz                       | 104.16.149.7    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 80   | cmcc.877774.xyz                       | 104.16.148.5    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 235  | braden.ns.cloudflare.com              | 172.64.35.169   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 467  | 162.159.137.204                       | 162.159.137.204 | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 468  | 104.26.3.162                          | 104.26.3.162    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 518  | 172.67.74.57                          | 172.67.74.57    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 32   | ct.877774.xyz                         | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 123  | www.hugedomains.com                   | 172.67.70.191   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 154  | www.visa.com.sg                       | 104.18.12.229   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 421  | lewis.ns.cloudflare.com               | 172.64.35.159   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 422  | lewis.ns.cloudflare.com               | 108.162.195.159 | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 525  | 104.17.101.37                         | 104.17.101.37   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 548  | 108.162.194.125                       | 108.162.194.125 | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 77   | cmcc.877774.xyz                       | 104.16.148.2    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 82   | cmcc.877774.xyz                       | 104.16.148.7    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 234  | braden.ns.cloudflare.com              | 108.162.195.169 | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 290  | www.glassdoor.com                     | 104.16.25.46    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 371  | gamer.com.tw                          | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 478  | 104.18.151.172                        | 104.18.151.172  | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 608  | 172.64.52.15                          | 172.64.52.15    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 94   | 104.18.254.88                         | 104.18.254.88   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 140  | craig.ns.cloudflare.com               | 108.162.195.192 | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 200  | www.whatismyip.com                    | 104.26.13.23    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 217  | bestcf.030101.xyz                     | 104.17.60.95    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 222  | asia.877774.xyz                       | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 240  | cloudflare-ip.mofashi.ltd             | 104.21.72.233   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 306  | 456.cloudflare.182682.xyz             | 172.67.75.208   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 396  | otto.ns.cloudflare.com                | 172.64.35.135   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 402  | 104.19.223.58                         | 104.19.223.58   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 410  | cfip.xxxxxxxx.tk                      | 198.41.214.141  | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 454  | abdullah.ns.cloudflare.com            | 172.64.35.203   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 576  | 104.18.35.166                         | 104.18.35.166   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 1    | 172.67.120.0                          | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 447  | 172.64.91.69                          | 172.64.91.69    | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 480  | 104.19.154.200                        | 104.19.154.200  | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 521  | 104.20.30.198                         | 104.20.30.198   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 596  | 104.17.50.237                         | 104.17.50.237   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 93   | cmcc.877774.xyz                       | 104.16.149.5    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 148  | cf.877771.xyz                         | 188.114.96.3    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 314  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 374  | 198.62.62.4                           | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 524  | 104.17.169.180                        | 104.17.169.180  | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 532  | 104.17.105.198                        | 104.17.105.198  | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 560  | 104.26.4.190                          | 104.26.4.190    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 580  | 108.162.195.1                         | 108.162.195.1   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 599  | 104.17.154.254                        | 104.17.154.254  | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 76   | cmcc.877774.xyz                       | 104.16.148.1    | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 91   | cmcc.877774.xyz                       | 104.16.149.3    | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 156  | yx-auto.pages.dev                     | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 159  | na.877774.xyz                         | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 388  | 104.18.78.214                         | 104.18.78.214   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 465  | 104.26.5.134                          | 104.26.5.134    | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 484  | 104.31.16.158                         | 104.31.16.158   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 569  | 104.17.119.199                        | 104.17.119.199  | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 601  | 104.17.53.25                          | 104.17.53.25    | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 2    | 172.64.154.18                         | 172.64.154.18   | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 9    | comicabc.com                          | 104.21.64.10    | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 37   | ct.877774.xyz                         | 172.64.229.217  | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 43   | 104.16.45.84                          | 104.16.45.84    | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 187  | dylan.ns.cloudflare.com               | 108.162.195.187 | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 295  | palera.in                             | 172.67.157.122  | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 348  | www.visa.com.hk                       | 104.18.20.69    | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 360  | uriah.ns.cloudflare.com               | 172.64.35.194   | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 435  | 172.64.82.114                         | 172.64.82.114   | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 503  | 162.159.6.115                         | 162.159.6.115   | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 591  | 104.26.8.192                          | 104.26.8.192    | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 68   | cmcc.877774.xyz                       | 104.16.149.6    | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 109  | iplocation.io                         | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 122  | www.hugedomains.com                   | 104.26.7.37     | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 230  | cf.zhetengsha.eu.org                  | 104.18.43.174   | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 3    | 162.159.140.85                        | 162.159.140.85  | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 151  | www.visa.cn                           | 162.159.153.2   | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 164  | pranab.ns.cloudflare.com              | 172.64.35.199   | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 321  | ip.gs                                 | 188.114.96.3    | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 481  | 104.18.255.167                        | 104.18.255.167  | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 34   | ct.877774.xyz                         | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 257  | moura.ns.cloudflare.com               | 162.159.44.217  | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 320  | ip.gs                                 | 188.114.97.3    | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 413  | cfip.xxxxxxxx.tk                      | 190.93.247.169  | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 561  | 104.20.20.42                          | 104.20.20.42    | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 38   | 104.18.39.196                         | 104.18.39.196   | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 71   | cmcc.877774.xyz                       | 104.16.149.9    | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 101  | huxley.ns.cloudflare.com              | 172.64.35.188   | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 262  | rustam.ns.cloudflare.com              | 162.159.44.148  | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 392  | damien.ns.cloudflare.com              | 162.159.44.168  | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 397  | otto.ns.cloudflare.com                | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 622  | 162.159.45.145                        | 162.159.45.145  | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 60   | 104.26.6.112                          | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 245  | fbi.gov                               | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 572  | 104.16.155.76                         | 104.16.155.76   | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 81   | cmcc.877774.xyz                       | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 84   | cmcc.877774.xyz                       | 104.16.148.9    | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 155  | yx-auto.pages.dev                     | 172.66.44.144   | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 305  | 456.cloudflare.182682.xyz             | 104.26.9.160    | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 429  | japan.com                             | 172.67.70.92    | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 443  | 198.41.208.15                         | 198.41.208.15   | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 504  | 108.162.192.66                        | 108.162.192.66  | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 585  | 172.67.68.252                         | 172.67.68.252   | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 342  | julio.ns.cloudflare.com               | 108.162.195.209 | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 408  | cfip.xxxxxxxx.tk                      | 104.16.241.229  | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 557  | 172.67.67.5                           | 172.67.67.5     | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 602  | 104.16.251.143                        | 104.16.251.143  | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 618  | 162.159.44.128                        | 162.159.44.128  | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 248  | bowen.ns.cloudflare.com               | 108.162.195.83  | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 250  | bowen.ns.cloudflare.com               | 172.64.35.83    | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 505  | 162.159.13.51                         | 162.159.13.51   | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 514  | 173.245.58.237                        | 173.245.58.237  | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 629  | 108.162.198.170                       | 108.162.198.170 | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 74   | cmcc.877774.xyz                       | 104.16.149.12   | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 278  | benedict.ns.cloudflare.com            | 108.162.195.205 | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 406  | cfip.xxxxxxxx.tk                      | 104.17.127.110  | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 510  | 162.159.46.238                        | 162.159.46.238  | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 523  | 172.67.72.254                         | 172.67.72.254   | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 527  | 172.64.229.191                        | 172.64.229.191  | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 75   | cmcc.877774.xyz                       | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 141  | craig.ns.cloudflare.com               | 162.159.44.192  | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 162  | pranab.ns.cloudflare.com              | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 592  | 104.26.4.44                           | 104.26.4.44     | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 39   | steamdb.info                          | 172.66.175.250  | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 55   | www.gov.ua                            | 172.67.209.127  | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 188  | dylan.ns.cloudflare.com               | 162.159.44.187  | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 218  | bestcf.030101.xyz                     | 104.17.55.198   | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 324  | 104.18.42.26                          | 104.18.42.26    | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 455  | abdullah.ns.cloudflare.com            | 108.162.195.203 | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 556  | 172.67.77.196                         | 172.67.77.196   | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 566  | 104.16.148.187                        | 104.16.148.187  | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 606  | 108.162.198.69                        | 108.162.198.69  | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 117  | icook.hk                              | 104.21.90.210   | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 279  | benedict.ns.cloudflare.com            | 172.64.35.205   | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 501  | 104.16.65.1                           | 104.16.65.1     | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 533  | 104.18.39.15                          | 104.18.39.15    | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 627  | 172.64.53.181                         | 172.64.53.181   | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 520  | 172.67.65.159                         | 172.67.65.159   | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 625  | 162.159.39.177                        | 162.159.39.177  | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 118  | icook.hk                              | 172.67.161.104  | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 294  | palera.in                             | 104.21.58.72    | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 428  | japan.com                             | 104.26.5.60     | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 224  | xn--b6gac.eu.org                      | 104.21.90.78    | IPv4   | h3   | ✅ 成功 | 124      | cloudflare |
| 359  | uriah.ns.cloudflare.com               | 108.162.195.194 | IPv4   | h3   | ✅ 成功 | 124      | cloudflare |
| 537  | 162.159.38.192                        | 162.159.38.192  | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 605  | 162.159.45.176                        | 162.159.45.176  | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 231  | cf.zhetengsha.eu.org                  | 172.64.144.82   | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 264  | rustam.ns.cloudflare.com              | 172.64.35.148   | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 416  | cfip.xxxxxxxx.tk                      | 190.93.246.67   | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 587  | 104.20.16.244                         | 104.20.16.244   | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 590  | 104.26.6.159                          | 104.26.6.159    | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 134  | sullivan.ns.cloudflare.com            | 162.159.44.161  | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 292  | 162.159.36.104                        | 162.159.36.104  | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 343  | julio.ns.cloudflare.com               | 162.159.44.209  | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 534  | 108.162.198.48                        | 108.162.198.48  | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 621  | 162.159.38.35                         | 162.159.38.35   | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 31   | ct.877774.xyz                         | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 128      | cloudflare |
| 182  | kyree.ns.cloudflare.com               | 162.159.44.207  | IPv4   | h3   | ✅ 成功 | 128      | cloudflare |
| 194  | decker.ns.cloudflare.com              | 162.159.44.155  | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 213  | zread.ai                              | 104.21.76.240   | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 236  | braden.ns.cloudflare.com              | 162.159.44.169  | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 310  | cf.090227.xyz                         | 172.64.152.241  | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 617  | 162.159.39.99                         | 162.159.39.99   | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 10   | comicabc.com                          | 172.67.174.21   | IPv4   | h3   | ✅ 成功 | 131      | cloudflare |
| 277  | benedict.ns.cloudflare.com            | 162.159.44.205  | IPv4   | h3   | ✅ 成功 | 131      | cloudflare |
| 483  | 104.19.212.207                        | 104.19.212.207  | IPv4   | h3   | ✅ 成功 | 131      | cloudflare |
| 4    | 172.64.145.242                        | 172.64.145.242  | IPv4   | h3   | ✅ 成功 | 132      | cloudflare |
| 552  | 162.159.39.62                         | 162.159.39.62   | IPv4   | h3   | ✅ 成功 | 132      | cloudflare |
| 153  | www.visa.com.sg                       | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 133      | cloudflare |
| 438  | www.wto.org                           | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 133      | cloudflare |
| 58   | 104.18.37.13                          | 104.18.37.13    | IPv4   | h3   | ✅ 成功 | 134      | cloudflare |
| 67   | 103.160.204.59                        | 103.160.204.59  | IPv4   | h3   | ✅ 成功 | 134      | cloudflare |
| 147  | cf.877771.xyz                         | 188.114.97.3    | IPv4   | h3   | ✅ 成功 | 134      | cloudflare |
| 272  | time.is                               | 172.67.68.157   | IPv4   | h3   | ✅ 成功 | 134      | cloudflare |
| 136  | sullivan.ns.cloudflare.com            | 172.64.35.161   | IPv4   | h3   | ✅ 成功 | 135      | cloudflare |
| 554  | 172.67.76.195                         | 172.67.76.195   | IPv4   | h3   | ✅ 成功 | 135      | cloudflare |
| 550  | 162.159.0.115                         | 162.159.0.115   | IPv4   | h3   | ✅ 成功 | 136      | cloudflare |
| 398  | otto.ns.cloudflare.com                | 162.159.44.135  | IPv4   | h3   | ✅ 成功 | 137      | cloudflare |
| 508  | www.csgo.com                          | 195.85.59.95    | IPv4   | h3   | ✅ 成功 | 137      | cloudflare |
| 366  | www.udemy.com                         | 104.16.143.237  | IPv4   | h3   | ✅ 成功 | 138      | cloudflare |
| 444  | 162.159.61.183                        | 162.159.61.183  | IPv4   | h3   | ✅ 成功 | 138      | cloudflare |
| 475  | 104.19.148.121                        | 104.19.148.121  | IPv4   | h3   | ✅ 成功 | 138      | cloudflare |
| 553  | 162.159.45.93                         | 162.159.45.93   | IPv4   | h3   | ✅ 成功 | 138      | cloudflare |
| 412  | cfip.xxxxxxxx.tk                      | 190.93.244.201  | IPv4   | h3   | ✅ 成功 | 141      | cloudflare |
| 582  | 104.18.37.110                         | 104.18.37.110   | IPv4   | h3   | ✅ 成功 | 141      | cloudflare |
| 619  | 162.159.7.12                          | 162.159.7.12    | IPv4   | h3   | ✅ 成功 | 141      | cloudflare |
| 44   | cloudflare.182682.xyz                 | 104.18.185.26   | IPv4   | h3   | ✅ 成功 | 142      | cloudflare |
| 79   | cmcc.877774.xyz                       | 104.16.148.4    | IPv4   | h3   | ✅ 成功 | 143      | cloudflare |
| 193  | decker.ns.cloudflare.com              | 108.162.195.155 | IPv4   | h3   | ✅ 成功 | 144      | cloudflare |
| 328  | ashton.ns.cloudflare.com              | 162.159.44.173  | IPv4   | h3   | ✅ 成功 | 144      | cloudflare |
| 420  | lewis.ns.cloudflare.com               | 162.159.44.159  | IPv4   | h3   | ✅ 成功 | 144      | cloudflare |
| 327  | ashton.ns.cloudflare.com              | 108.162.195.173 | IPv4   | h3   | ✅ 成功 | 145      | cloudflare |
| 448  | 172.64.48.226                         | 172.64.48.226   | IPv4   | h3   | ✅ 成功 | 146      | cloudflare |
| 414  | cfip.xxxxxxxx.tk                      | 188.114.97.144  | IPv4   | h3   | ✅ 成功 | 148      | cloudflare |
| 163  | pranab.ns.cloudflare.com              | 162.159.44.199  | IPv4   | h3   | ✅ 成功 | 150      | cloudflare |
| 453  | abdullah.ns.cloudflare.com            | 162.159.44.203  | IPv4   | h3   | ✅ 成功 | 150      | cloudflare |
| 35   | ct.877774.xyz                         | 172.64.229.185  | IPv4   | h3   | ✅ 成功 | 151      | cloudflare |
| 375  | 104.18.37.40                          | 104.18.37.40    | IPv4   | h3   | ✅ 成功 | 151      | cloudflare |
| 445  | 172.64.52.127                         | 172.64.52.127   | IPv4   | h3   | ✅ 成功 | 153      | cloudflare |
| 506  | 162.159.17.243                        | 162.159.17.243  | IPv4   | h3   | ✅ 成功 | 153      | cloudflare |
| 83   | cmcc.877774.xyz                       | 104.16.148.8    | IPv4   | h3   | ✅ 成功 | 154      | cloudflare |
| 626  | 162.159.38.171                        | 162.159.38.171  | IPv4   | h3   | ✅ 成功 | 154      | cloudflare |
| 344  | julio.ns.cloudflare.com               | 172.64.35.209   | IPv4   | h3   | ✅ 成功 | 155      | cloudflare |
| 96   | wilson.ns.cloudflare.com              | 162.159.44.110  | IPv4   | h3   | ✅ 成功 | 157      | cloudflare |
| 539  | 162.159.44.176                        | 162.159.44.176  | IPv4   | h3   | ✅ 成功 | 157      | cloudflare |
| 168  | cris.ns.cloudflare.com                | 162.159.44.202  | IPv4   | h3   | ✅ 成功 | 161      | cloudflare |
| 102  | huxley.ns.cloudflare.com              | 162.159.44.188  | IPv4   | h3   | ✅ 成功 | 162      | cloudflare |
| 18   | trevor.ns.cloudflare.com              | 162.159.44.154  | IPv4   | h3   | ✅ 成功 | 163      | cloudflare |
| 434  | 108.162.198.54                        | 108.162.198.54  | IPv4   | h3   | ✅ 成功 | 167      | cloudflare |
| 128  | www.4chan.org                         | 104.16.228.229  | IPv4   | h3   | ✅ 成功 | 173      | cloudflare |
| 630  | 162.159.41.141                        | 162.159.41.141  | IPv4   | h3   | ✅ 成功 | 178      | cloudflare |
| 29   | ipv4.ip.sb                            | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 189      | cloudflare |
| 405  | cfip.xxxxxxxx.tk                      | 188.114.96.125  | IPv4   | h3   | ✅ 成功 | 191      | cloudflare |
| 249  | bowen.ns.cloudflare.com               | 162.159.44.83   | IPv4   | h3   | ✅ 成功 | 217      | cloudflare |
| 631  | 172.64.34.153                         | 172.64.34.153   | IPv4   | h3   | ✅ 成功 | 226      | cloudflare |
| 361  | uriah.ns.cloudflare.com               | 162.159.44.194  | IPv4   | h3   | ✅ 成功 | 232      | cloudflare |
| 59   | 172.67.75.172                         | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 256      | cloudflare |
| 377  | 141.147.185.63                        | 141.147.185.63  | IPv4   | h2   | ✅ 成功 | 398      | cloudflare |
| 174  | 34.143.159.175                        | 34.143.159.175  | IPv4   | h2   | ✅ 成功 | 529      | cloudflare |
| 632  | 3.0.50.69                             | 3.0.50.69       | IPv4   | h2   | ✅ 成功 | 533      | cloudflare |
| 211  | 168.138.165.174                       | 168.138.165.174 | IPv4   | h2   | ✅ 成功 | 683      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 205 条记录
- **正常 (100-200ms)**: 217 条记录
- **慢 (200-500ms)**: 5 条记录
- **很慢 (>500ms)**: 3 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 3 次
- **IPv6 失败**: 200 次

### 按协议统计

- **none**: 203 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
