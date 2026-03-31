# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2026/1/19 02:30:57
- **数据来源**: connectivity_results-20260119-023056.json
- **总测试数**: 729
- **失败测试数**: 222
- **成功测试数**: 507
- **失败率**: 30.45%
- **平均延迟**: 114.10ms
- **最小延迟**: 50ms
- **最大延迟**: 1751ms

## 🌐 当前测试环境信息

- **获取时间**: 2026/1/19 02:30:57
- **IP地址**: 74.249.78.35
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

- **网络不可达: 网络不可达**: 219 次 (98.6%)
- **连接超时: I/O超时**: 3 次 (1.4%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (219 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
| ---- | --------- | ------ | ------ | ---- | ------ | -------- | ------ | -------- |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 449  | cf.877774.xyz    | cf.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp cf.877774.xyz:443: i/o timeout  |
| 727  | ct.877774.xyz    | ct.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout  |
| 729  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 219 次 (98.6%)
- **连接超时**: 3 次 (1.4%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 172.67（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 222 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 219 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次),
sullivan.ns.cloudflare.com (3次), trevor.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 507 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 238  | zread.ai                              | 172.67.202.78   | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 295  | 172.67.76.195                         | 172.67.76.195   | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 441  | 172.64.146.67                         | 172.64.146.67   | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 568  | 162.159.140.116                       | 162.159.140.116 | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 610  | 172.64.49.146                         | 172.64.49.146   | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 657  | www.pcmag.com                         | 104.16.20.118   | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 612  | 162.159.1.145                         | 162.159.1.145   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 710  | 104.26.6.112                          | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 264  | 104.20.16.244                         | 104.20.16.244   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 304  | 104.17.50.237                         | 104.17.50.237   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 486  | 104.18.78.214                         | 104.18.78.214   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 622  | 104.26.4.4                            | 104.26.4.4      | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 623  | 162.159.36.52                         | 162.159.36.52   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 593  | stock.hostmonit.com                   | 172.67.187.251  | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 605  | 172.64.32.77                          | 172.64.32.77    | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 589  | 104.17.162.3                          | 104.17.162.3    | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 596  | 172.64.154.113                        | 172.64.154.113  | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 694  | iplocation.io                         | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 451  | 104.26.5.101                          | 104.26.5.101    | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 206  | decker.ns.cloudflare.com              | 108.162.195.155 | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 315  | rustam.ns.cloudflare.com              | 108.162.195.148 | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 473  | 198.62.62.4                           | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 617  | 172.67.78.67                          | 172.67.78.67    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 103  | 104.26.12.227                         | 104.26.12.227   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 437  | 162.159.1.111                         | 162.159.1.111   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 670  | cu.877774.xyz                         | 104.26.4.111    | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 683  | 104.16.45.84                          | 104.16.45.84    | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 718  | www.hugedomains.com                   | 104.26.6.37     | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 520  | 172.64.52.67                          | 172.64.52.67    | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 492  | otto.ns.cloudflare.com                | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 348  | 104.17.21.106                         | 104.17.21.106   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 406  | www.visa.com.hk                       | 104.18.21.69    | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 597  | 104.19.220.22                         | 104.19.220.22   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 363  | 456.cloudflare.182682.xyz             | 172.67.75.208   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 586  | 104.19.148.121                        | 104.19.148.121  | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 130  | www.visa.com.sg                       | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 194  | www.ipget.net                         | 172.67.207.26   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 609  | 104.26.11.160                         | 104.26.11.160   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 478  | 104.19.34.231                         | 104.19.34.231   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 532  | 198.41.208.15                         | 198.41.208.15   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 619  | 172.67.79.150                         | 172.67.79.150   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 690  | 172.67.110.232                        | 172.67.110.232  | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 419  | 172.64.34.153                         | 172.64.34.153   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 296  | 104.26.4.44                           | 104.26.4.44     | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 291  | cloudflare-ip.mofashi.ltd             | 172.67.155.172  | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 9    | 104.16.105.166                        | 104.16.105.166  | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 465  | www.udemy.com                         | 104.16.143.237  | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 717  | www.hugedomains.com                   | 104.26.7.37     | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 6    | 104.31.16.158                         | 104.31.16.158   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 11   | 104.18.189.153                        | 104.18.189.153  | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 189  | cris.ns.cloudflare.com                | 172.64.35.202   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 243  | bestcf.030101.xyz                     | 104.17.99.183   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 494  | otto.ns.cloudflare.com                | 172.64.35.135   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 525  | www.wto.org                           | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 161  | cmcc.877774.xyz                       | 104.16.149.7    | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 282  | cf.090227.xyz                         | 104.18.43.174   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 599  | 172.64.41.216                         | 172.64.41.216   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 678  | cf.0sm.com                            | 172.67.187.145  | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 703  | ct.877774.xyz                         | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 263  | 104.26.2.166                          | 104.26.2.166    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 461  | tasteatlas.com                        | 104.17.37.105   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 471  | 172.67.79.211                         | 172.67.79.211   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 542  | 173.245.49.194                        | 173.245.49.194  | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 269  | cf.zhetengsha.eu.org                  | 172.64.152.241  | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 309  | time.is                               | 172.67.68.157   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 469  | 104.17.142.12                         | 104.17.142.12   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 10   | 104.17.142.212                        | 104.17.142.212  | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 54   | 104.17.101.37                         | 104.17.101.37   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 539  | 172.64.48.226                         | 172.64.48.226   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 164  | cmcc.877774.xyz                       | 104.16.149.10   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 166  | cmcc.877774.xyz                       | 104.16.149.12   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 178  | 104.16.155.76                         | 104.16.155.76   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 182  | 104.16.157.50                         | 104.16.157.50   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 277  | asia.877774.xyz                       | 104.17.139.62   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 448  | 104.18.40.216                         | 104.18.40.216   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 508  | lewis.ns.cloudflare.com               | 108.162.195.159 | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 604  | 104.18.41.101                         | 104.18.41.101   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 700  | ct.877774.xyz                         | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 728  | cfip.xxxxxxxx.tk                      | 190.93.244.201  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 5    | 104.17.69.244                         | 104.17.69.244   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 332  | ip.sb                                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 340  | singapore.com                         | 104.26.13.140   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 425  | 172.64.145.242                        | 172.64.145.242  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 499  | 104.17.215.66                         | 104.17.215.66   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 626  | 104.18.40.202                         | 104.18.40.202   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 714  | www.4chan.org                         | 104.16.228.229  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 171  | www.okcupid.com                       | 104.16.223.254  | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 502  | ifconfig.co                           | 172.67.168.106  | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 611  | 104.18.32.174                         | 104.18.32.174   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 643  | 104.18.44.25                          | 104.18.44.25    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 138  | pranab.ns.cloudflare.com              | 172.64.35.199   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 396  | 172.67.106.26                         | 172.67.106.26   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 443  | cf.877774.xyz                         | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 477  | 104.18.37.40                          | 104.18.37.40    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 555  | damien.ns.cloudflare.com              | 172.64.35.168   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 39   | 173.245.58.237                        | 173.245.58.237  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 55   | 104.17.156.81                         | 104.17.156.81   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 186  | na.877774.xyz                         | 104.18.38.235   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 299  | moura.ns.cloudflare.com               | 172.64.35.217   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 319  | benedict.ns.cloudflare.com            | 108.162.195.205 | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 333  | ip.sb                                 | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 354  | 162.159.19.219                        | 162.159.19.219  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 431  | uriah.ns.cloudflare.com               | 108.162.195.194 | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 601  | 172.64.152.215                        | 172.64.152.215  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 67   | 104.17.105.198                        | 104.17.105.198  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 109  | cloudflare.182682.xyz                 | 104.17.25.173   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 156  | cmcc.877774.xyz                       | 104.16.149.2    | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 327  | palera.in                             | 172.67.157.122  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 361  | 104.17.68.85                          | 104.17.68.85    | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 362  | 456.cloudflare.182682.xyz             | 104.26.8.160    | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 428  | 172.64.145.119                        | 172.64.145.119  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 450  | 104.26.2.2                            | 104.26.2.2      | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 535  | eur.877774.xyz                        | 104.21.47.209   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 3    | 104.18.255.167                        | 104.18.255.167  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 32   | sullivan.ns.cloudflare.com            | 172.64.35.161   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 95   | 172.67.77.196                         | 172.67.77.196   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 180  | www.visa.cn                           | 162.159.152.2   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 328  | palera.in                             | 104.21.58.72    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 524  | 104.19.175.123                        | 104.19.175.123  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 530  | 172.64.82.114                         | 172.64.82.114   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 565  | www.csgo.com                          | 195.85.59.161   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 665  | icook.hk                              | 104.21.90.210   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 702  | ct.877774.xyz                         | 172.64.229.185  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 106  | 104.20.19.201                         | 104.20.19.201   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 135  | 104.17.119.199                        | 104.17.119.199  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 167  | cmcc.877774.xyz                       | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 248  | xn--b6gac.eu.org                      | 104.21.90.78    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 257  | 162.159.62.6                          | 162.159.62.6    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 326  | www.glassdoor.com                     | 104.17.64.70    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 353  | 162.159.36.104                        | 162.159.36.104  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 498  | 104.19.223.58                         | 104.19.223.58   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 575  | 162.159.61.106                        | 162.159.61.106  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 615  | 172.64.40.196                         | 172.64.40.196   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 646  | 104.16.251.254                        | 104.16.251.254  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 673  | cu.877774.xyz                         | 104.26.4.114    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 716  | www.hugedomains.com                   | 172.67.70.191   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 197  | kyree.ns.cloudflare.com               | 172.64.35.207   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 229  | 104.18.37.110                         | 104.18.37.110   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 398  | 104.18.14.76                          | 104.18.14.76    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 637  | 104.17.153.151                        | 104.17.153.151  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 658  | www.pcmag.com                         | 104.16.21.118   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 357  | www.ipchicken.com                     | 104.26.7.112    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 364  | 456.cloudflare.182682.xyz             | 104.26.9.160    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 442  | 172.64.153.141                        | 172.64.153.141  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 505  | 104.17.25.241                         | 104.17.25.241   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 569  | 104.26.8.117                          | 104.26.8.117    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 675  | cu.877774.xyz                         | 104.26.4.116    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 50   | 104.26.3.176                          | 104.26.3.176    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 112  | cloudflare.182682.xyz                 | 104.21.227.134  | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 133  | 104.19.44.238                         | 104.19.44.238   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 136  | pranab.ns.cloudflare.com              | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 217  | toy-people.com                        | 104.26.2.36     | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 334  | ip.sb                                 | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 472  | 104.17.170.137                        | 104.17.170.137  | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 512  | 104.17.189.30                         | 104.17.189.30   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 513  | japan.com                             | 172.67.70.92    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 536  | eur.877774.xyz                        | 104.21.26.150   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 537  | 172.64.52.127                         | 172.64.52.127   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 577  | 104.18.89.52                          | 104.18.89.52    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 121  | yx-auto.pages.dev                     | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 126  | craig.ns.cloudflare.com               | 172.64.35.192   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 144  | cmcc.877774.xyz                       | 104.16.148.3    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 501  | 104.17.209.79                         | 104.17.209.79   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 662  | ipinfo.in                             | 104.21.21.129   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 45   | 172.67.64.214                         | 172.67.64.214   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 142  | cmcc.877774.xyz                       | 104.16.148.1    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 154  | cmcc.877774.xyz                       | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 481  | 104.26.13.31                          | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 548  | abdullah.ns.cloudflare.com            | 172.64.35.203   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 616  | 104.18.36.1                           | 104.18.36.1     | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 704  | ct.877774.xyz                         | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 28   | comicabc.com                          | 172.67.174.21   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 41   | 104.18.45.95                          | 104.18.45.95    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 53   | 104.17.169.180                        | 104.17.169.180  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 149  | cmcc.877774.xyz                       | 104.16.148.8    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 169  | www.okcupid.com                       | 104.16.239.254  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 253  | braden.ns.cloudflare.com              | 108.162.195.169 | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 331  | 104.16.61.163                         | 104.16.61.163   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 564  | www.csgo.com                          | 195.85.59.95    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 711  | 103.160.204.59                        | 103.160.204.59  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 47   | 172.67.65.159                         | 172.67.65.159   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 179  | 104.17.100.254                        | 104.17.100.254  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 359  | 172.64.52.15                          | 172.64.52.15    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 500  | 104.19.153.47                         | 104.19.153.47   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 624  | 104.20.17.51                          | 104.20.17.51    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 33   | sullivan.ns.cloudflare.com            | 108.162.195.161 | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 48   | 104.20.22.185                         | 104.20.22.185   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 193  | www.ipget.net                         | 104.21.15.212   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 213  | dylan.ns.cloudflare.com               | 172.64.35.187   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 281  | cf.090227.xyz                         | 172.64.144.82   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 358  | www.ipchicken.com                     | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 400  | dnschecker.org                        | 104.26.7.89     | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 401  | dnschecker.org                        | 172.67.73.216   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 570  | 162.159.128.253                       | 162.159.128.253 | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 606  | 104.18.37.177                         | 104.18.37.177   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 148  | cmcc.877774.xyz                       | 104.16.148.7    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 199  | kyree.ns.cloudflare.com               | 108.162.195.207 | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 203  | 162.159.34.55                         | 162.159.34.55   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 223  | www.whatismyip.com                    | 172.67.69.129   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 237  | zread.ai                              | 104.21.76.240   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 538  | 162.159.61.183                        | 162.159.61.183  | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 7    | 104.17.167.134                        | 104.17.167.134  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 545  | 162.159.58.65                         | 162.159.58.65   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 621  | 104.20.25.161                         | 104.20.25.161   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 639  | 104.19.144.159                        | 104.19.144.159  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 715  | www.4chan.org                         | 104.16.229.229  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 107  | 104.17.60.233                         | 104.17.60.233   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 110  | cloudflare.182682.xyz                 | 104.18.185.26   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 131  | www.visa.com.sg                       | 104.18.12.229   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 159  | cmcc.877774.xyz                       | 104.16.149.5    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 273  | 172.67.68.252                         | 172.67.68.252   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 405  | www.visa.com.hk                       | 104.18.20.69    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 598  | 104.17.139.37                         | 104.17.139.37   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 669  | cu.877774.xyz                         | 104.26.4.119    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 68   | 104.18.39.15                          | 104.18.39.15    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 70   | 172.64.53.0                           | 172.64.53.0     | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 100  | 172.67.67.5                           | 172.67.67.5     | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 108  | cloudflare.182682.xyz                 | 104.16.250.22   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 285  | bowen.ns.cloudflare.com               | 172.64.35.83    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 325  | www.glassdoor.com                     | 104.16.25.46    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 415  | julio.ns.cloudflare.com               | 108.162.195.209 | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 452  | 172.67.73.120                         | 172.67.73.120   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 454  | 104.26.1.181                          | 104.26.1.181    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 523  | 172.67.181.209                        | 172.67.181.209  | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 676  | cu.877774.xyz                         | 104.26.4.117    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 69   | 108.162.198.48                        | 108.162.198.48  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 204  | 104.18.35.166                         | 104.18.35.166   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 219  | toy-people.com                        | 104.26.3.36     | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 280  | 104.26.8.192                          | 104.26.8.192    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 349  | 104.16.144.235                        | 104.16.144.235  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 409  | ashton.ns.cloudflare.com              | 172.64.35.173   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 447  | 104.18.42.61                          | 104.18.42.61    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 514  | japan.com                             | 104.26.4.60     | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 534  | eur.877774.xyz                        | 104.21.29.164   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 588  | 104.18.166.129                        | 104.18.166.129  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 666  | icook.hk                              | 172.67.161.104  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 671  | cu.877774.xyz                         | 104.26.4.112    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 26   | 108.162.192.66                        | 108.162.192.66  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 172  | www.okcupid.com                       | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 286  | bowen.ns.cloudflare.com               | 108.162.195.83  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 347  | 104.16.251.143                        | 104.16.251.143  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 423  | 162.159.41.141                        | 162.159.41.141  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 475  | gamer.com.tw                          | 104.18.2.197    | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 693  | iplocation.io                         | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 59   | 104.17.168.159                        | 104.17.168.159  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 168  | www.okcupid.com                       | 104.18.160.63   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 414  | julio.ns.cloudflare.com               | 172.64.35.209   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 488  | www.digitalocean.com                  | 104.19.173.68   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 642  | 172.64.154.226                        | 172.64.154.226  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 650  | 104.18.37.13                          | 104.18.37.13    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 44   | 104.26.15.85                          | 104.26.15.85    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 81   | 172.64.52.110                         | 172.64.52.110   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 346  | 104.17.53.25                          | 104.17.53.25    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 378  | ip.gs                                 | 172.67.160.28   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 458  | 104.20.25.82                          | 104.20.25.82    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 547  | abdullah.ns.cloudflare.com            | 108.162.195.203 | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 705  | ct.877774.xyz                         | 172.64.229.217  | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 40   | 104.18.47.253                         | 104.18.47.253   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 63   | trevor.ns.cloudflare.com              | 172.64.35.154   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 72   | 108.162.194.125                       | 108.162.194.125 | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 94   | 172.64.151.55                         | 172.64.151.55   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 101  | 104.20.21.161                         | 104.20.21.161   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 124  | craig.ns.cloudflare.com               | 108.162.195.192 | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 274  | 104.26.6.159                          | 104.26.6.159    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 339  | singapore.com                         | 104.26.12.140   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 350  | 104.18.42.26                          | 104.18.42.26    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 521  | 162.159.42.146                        | 162.159.42.146  | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 691  | 104.18.254.88                         | 104.18.254.88   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 321  | benedict.ns.cloudflare.com            | 172.64.35.205   | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 351  | 172.64.147.235                        | 172.64.147.235  | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 653  | huxley.ns.cloudflare.com              | 108.162.195.188 | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 688  | 172.67.75.172                         | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 706  | www.gov.ua                            | 104.21.23.72    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 42   | 172.64.150.30                         | 172.64.150.30   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 49   | 104.20.24.107                         | 104.20.24.107   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 60   | 104.17.170.110                        | 104.17.170.110  | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 270  | cf.zhetengsha.eu.org                  | 104.18.35.15    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 421  | 172.64.52.90                          | 172.64.52.90    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 620  | 104.18.47.46                          | 104.18.47.46    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 644  | 104.17.25.87                          | 104.17.25.87    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 685  | steamdb.info                          | 172.66.175.250  | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 61   | trevor.ns.cloudflare.com              | 108.162.195.154 | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 187  | cris.ns.cloudflare.com                | 108.162.195.202 | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 224  | www.whatismyip.com                    | 104.26.13.23    | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 356  | www.ipchicken.com                     | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 392  | whatismyipaddress.com                 | 104.19.222.79   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 526  | www.wto.org                           | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 592  | stock.hostmonit.com                   | 104.21.7.193    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 27   | comicabc.com                          | 104.21.64.10    | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 57   | 104.16.255.1                          | 104.16.255.1    | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 146  | cmcc.877774.xyz                       | 104.16.148.5    | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 184  | na.877774.xyz                         | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 234  | 104.16.223.179                        | 104.16.223.179  | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 276  | 172.67.70.253                         | 172.67.70.253   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 305  | 104.16.147.114                        | 104.16.147.114  | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 352  | 108.162.198.69                        | 108.162.198.69  | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 397  | 162.159.45.145                        | 162.159.45.145  | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 460  | tasteatlas.com                        | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 591  | 162.159.6.44                          | 162.159.6.44    | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 698  | ct.877774.xyz                         | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 119  | 104.16.148.187                        | 104.16.148.187  | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 143  | cmcc.877774.xyz                       | 104.16.148.2    | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 260  | freeyx.cloudflare88.eu.org            | 141.101.121.14  | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 278  | asia.877774.xyz                       | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 369  | 162.159.7.12                          | 162.159.7.12    | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 433  | uriah.ns.cloudflare.com               | 172.64.35.194   | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 507  | lewis.ns.cloudflare.com               | 172.64.35.159   | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 533  | 198.41.194.162                        | 198.41.194.162  | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 712  | shopify.com                           | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 218  | toy-people.com                        | 172.67.72.18    | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 258  | 104.26.12.113                         | 104.26.12.113   | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 266  | fbi.gov                               | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 345  | 104.17.154.254                        | 104.17.154.254  | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 464  | www.udemy.com                         | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 563  | 104.26.3.162                          | 104.26.3.162    | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 590  | 198.41.208.224                        | 198.41.208.224  | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 692  | iplocation.io                         | 104.26.10.222   | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 724  | ipv4.ip.sb                            | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 99   | 104.26.8.148                          | 104.26.8.148    | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 120  | yx-auto.pages.dev                     | 172.66.44.144   | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 181  | www.visa.cn                           | 162.159.153.2   | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 183  | 104.17.101.208                        | 104.17.101.208  | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 150  | cmcc.877774.xyz                       | 104.16.148.9    | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 158  | cmcc.877774.xyz                       | 104.16.149.4    | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 468  | 104.26.10.239                         | 104.26.10.239   | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 541  | 162.159.24.131                        | 162.159.24.131  | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 544  | 162.159.136.89                        | 162.159.136.89  | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 638  | 104.18.42.106                         | 104.18.42.106   | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 672  | cu.877774.xyz                         | 104.26.4.113    | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 707  | www.gov.ua                            | 172.67.209.127  | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 147  | cmcc.877774.xyz                       | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 247  | xn--b6gac.eu.org                      | 172.67.153.253  | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 422  | 162.159.133.85                        | 162.159.133.85  | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 470  | 104.17.211.218                        | 104.17.211.218  | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 476  | gamer.com.tw                          | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 479  | 172.64.35.24                          | 172.64.35.24    | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 242  | 162.159.19.37                         | 162.159.19.37   | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 297  | 104.17.24.232                         | 104.17.24.232   | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 300  | moura.ns.cloudflare.com               | 108.162.195.217 | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 307  | time.is                               | 104.26.13.54    | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 427  | 172.64.151.235                        | 172.64.151.235  | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 627  | 104.26.7.7                            | 104.26.7.7      | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 652  | huxley.ns.cloudflare.com              | 162.159.44.188  | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 682  | 104.18.39.196                         | 104.18.39.196   | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 56   | 172.64.229.191                        | 172.64.229.191  | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 603  | 162.159.38.35                         | 162.159.38.35   | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 37   | 162.159.13.51                         | 162.159.13.51   | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 185  | na.877774.xyz                         | 104.18.187.25   | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 225  | www.whatismyip.com                    | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 230  | 108.162.195.1                         | 108.162.195.1   | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 314  | rustam.ns.cloudflare.com              | 172.64.35.148   | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 474  | 104.17.115.224                        | 104.17.115.224  | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 549  | abdullah.ns.cloudflare.com            | 162.159.44.203  | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 602  | 104.19.154.200                        | 104.19.154.200  | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 8    | 104.18.223.253                        | 104.18.223.253  | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 77   | wilson.ns.cloudflare.com              | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 162  | cmcc.877774.xyz                       | 104.16.149.8    | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 211  | dylan.ns.cloudflare.com               | 108.162.195.187 | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 244  | bestcf.030101.xyz                     | 104.17.27.231   | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 483  | icook.tw                              | 104.20.28.74    | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 600  | 104.18.151.172                        | 104.18.151.172  | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 614  | 104.26.1.194                          | 104.26.1.194    | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 701  | ct.877774.xyz                         | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 151  | cmcc.877774.xyz                       | 104.16.148.10   | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 155  | cmcc.877774.xyz                       | 104.16.149.1    | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 212  | dylan.ns.cloudflare.com               | 162.159.44.187  | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 382  | saas.sin.fan                          | 162.159.36.20   | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 640  | 104.17.53.129                         | 104.17.53.129   | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 679  | cf.0sm.com                            | 104.21.7.133    | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 722  | ipv4.ip.sb                            | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 24   | 162.159.21.116                        | 162.159.21.116  | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 132  | 172.67.243.218                        | 172.67.243.218  | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 134  | 104.18.40.39                          | 104.18.40.39    | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 275  | 104.20.17.233                         | 104.20.17.233   | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 540  | 172.64.91.69                          | 172.64.91.69    | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 645  | 104.16.155.230                        | 104.16.155.230  | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 677  | cu.877774.xyz                         | 104.26.4.118    | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 58   | 104.17.16.248                         | 104.17.16.248   | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 145  | cmcc.877774.xyz                       | 104.16.148.4    | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 292  | cloudflare-ip.mofashi.ltd             | 104.21.72.233   | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 393  | whatismyipaddress.com                 | 104.19.223.79   | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 459  | 172.67.72.36                          | 172.67.72.36    | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 618  | 104.26.6.171                          | 104.26.6.171    | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 649  | www.7749tv.com                        | 104.19.133.4    | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 684  | steamdb.info                          | 104.20.34.212   | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 74   | 172.64.50.51                          | 172.64.50.51    | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 233  | 104.18.39.228                         | 104.18.39.228   | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 699  | ct.877774.xyz                         | 172.64.229.173  | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 308  | time.is                               | 104.26.12.54    | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 651  | huxley.ns.cloudflare.com              | 172.64.35.188   | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 25   | 162.159.6.115                         | 162.159.6.115   | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 97   | 104.20.26.58                          | 104.20.26.58    | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 375  | 172.64.53.144                         | 172.64.53.144   | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 390  | 162.159.38.171                        | 162.159.38.171  | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 429  | 162.159.3.89                          | 162.159.3.89    | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 648  | 104.17.187.186                        | 104.17.187.186  | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 641  | 104.17.110.226                        | 104.17.110.226  | IPv4   | h3   | ✅ 成功 | 124      | cloudflare |
| 661  | ipinfo.in                             | 172.67.198.203  | IPv4   | h3   | ✅ 成功 | 124      | cloudflare |
| 152  | cmcc.877774.xyz                       | 104.16.148.11   | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 207  | decker.ns.cloudflare.com              | 162.159.44.155  | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 444  | cf.877774.xyz                         | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 647  | 104.17.214.136                        | 104.17.214.136  | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 674  | cu.877774.xyz                         | 104.26.4.115    | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 252  | braden.ns.cloudflare.com              | 172.64.35.169   | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 265  | fbi.gov                               | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 370  | 162.159.44.128                        | 162.159.44.128  | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 556  | damien.ns.cloudflare.com              | 108.162.195.168 | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 625  | 172.67.67.152                         | 172.67.67.152   | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 43   | 172.67.68.211                         | 172.67.68.211   | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 574  | 108.162.198.148                       | 108.162.198.148 | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 613  | 162.159.16.136                        | 162.159.16.136  | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 205  | decker.ns.cloudflare.com              | 172.64.35.155   | IPv4   | h3   | ✅ 成功 | 128      | cloudflare |
| 232  | 162.159.36.26                         | 162.159.36.26   | IPv4   | h3   | ✅ 成功 | 128      | cloudflare |
| 306  | 104.16.153.12                         | 104.16.153.12   | IPv4   | h3   | ✅ 成功 | 128      | cloudflare |
| 368  | 162.159.39.99                         | 162.159.39.99   | IPv4   | h3   | ✅ 成功 | 128      | cloudflare |
| 456  | 172.67.75.11                          | 172.67.75.11    | IPv4   | h3   | ✅ 成功 | 128      | cloudflare |
| 170  | www.okcupid.com                       | 104.16.144.63   | IPv4   | h3   | ✅ 成功 | 129      | cloudflare |
| 231  | 162.159.42.140                        | 162.159.42.140  | IPv4   | h3   | ✅ 成功 | 129      | cloudflare |
| 529  | 108.162.198.54                        | 108.162.198.54  | IPv4   | h3   | ✅ 成功 | 129      | cloudflare |
| 573  | 104.18.81.19                          | 104.18.81.19    | IPv4   | h3   | ✅ 成功 | 129      | cloudflare |
| 23   | 162.159.17.243                        | 162.159.17.243  | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 174  | cf.877771.xyz                         | 172.67.152.183  | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 531  | cfip.1323123.xyz                      | 104.16.133.220  | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 608  | 172.64.145.108                        | 172.64.145.108  | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 1    | 172.64.154.18                         | 172.64.154.18   | IPv4   | h3   | ✅ 成功 | 132      | cloudflare |
| 424  | 108.162.198.170                       | 108.162.198.170 | IPv4   | h3   | ✅ 成功 | 132      | cloudflare |
| 546  | 172.64.229.7                          | 172.64.229.7    | IPv4   | h3   | ✅ 成功 | 132      | cloudflare |
| 515  | japan.com                             | 104.26.5.60     | IPv4   | h3   | ✅ 成功 | 133      | cloudflare |
| 383  | saas.sin.fan                          | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 134      | cloudflare |
| 489  | www.digitalocean.com                  | 104.19.174.68   | IPv4   | h3   | ✅ 成功 | 134      | cloudflare |
| 543  | 104.26.4.90                           | 104.26.4.90     | IPv4   | h3   | ✅ 成功 | 134      | cloudflare |
| 553  | 104.26.5.134                          | 104.26.5.134    | IPv4   | h3   | ✅ 成功 | 134      | cloudflare |
| 607  | 162.159.36.223                        | 162.159.36.223  | IPv4   | h3   | ✅ 成功 | 134      | cloudflare |
| 46   | 172.67.74.57                          | 172.67.74.57    | IPv4   | h3   | ✅ 成功 | 135      | cloudflare |
| 173  | cf.877771.xyz                         | 104.21.80.180   | IPv4   | h3   | ✅ 成功 | 135      | cloudflare |
| 38   | 162.159.46.238                        | 162.159.46.238  | IPv4   | h3   | ✅ 成功 | 136      | cloudflare |
| 438  | 162.159.22.29                         | 162.159.22.29   | IPv4   | h3   | ✅ 成功 | 136      | cloudflare |
| 102  | 104.20.20.42                          | 104.20.20.42    | IPv4   | h3   | ✅ 成功 | 137      | cloudflare |
| 235  | 104.17.79.11                          | 104.17.79.11    | IPv4   | h3   | ✅ 成功 | 137      | cloudflare |
| 236  | 104.18.44.148                         | 104.18.44.148   | IPv4   | h3   | ✅ 成功 | 137      | cloudflare |
| 522  | 162.159.140.85                        | 162.159.140.85  | IPv4   | h3   | ✅ 成功 | 137      | cloudflare |
| 377  | ip.gs                                 | 104.21.14.176   | IPv4   | h3   | ✅ 成功 | 139      | cloudflare |
| 157  | cmcc.877774.xyz                       | 104.16.149.3    | IPv4   | h3   | ✅ 成功 | 140      | cloudflare |
| 338  | singapore.com                         | 172.67.75.194   | IPv4   | h3   | ✅ 成功 | 141      | cloudflare |
| 399  | dnschecker.org                        | 104.26.6.89     | IPv4   | h3   | ✅ 成功 | 141      | cloudflare |
| 430  | 162.159.18.240                        | 162.159.18.240  | IPv4   | h3   | ✅ 成功 | 141      | cloudflare |
| 453  | 104.20.21.147                         | 104.20.21.147   | IPv4   | h3   | ✅ 成功 | 142      | cloudflare |
| 493  | otto.ns.cloudflare.com                | 162.159.44.135  | IPv4   | h3   | ✅ 成功 | 144      | cloudflare |
| 105  | 104.20.18.47                          | 104.20.18.47    | IPv4   | h3   | ✅ 成功 | 145      | cloudflare |
| 344  | 104.17.30.164                         | 104.17.30.164   | IPv4   | h3   | ✅ 成功 | 146      | cloudflare |
| 504  | 104.19.35.242                         | 104.19.35.242   | IPv4   | h3   | ✅ 成功 | 146      | cloudflare |
| 92   | 162.159.0.115                         | 162.159.0.115   | IPv4   | h3   | ✅ 成功 | 147      | cloudflare |
| 153  | cmcc.877774.xyz                       | 104.16.148.12   | IPv4   | h3   | ✅ 成功 | 147      | cloudflare |
| 163  | cmcc.877774.xyz                       | 104.16.149.9    | IPv4   | h3   | ✅ 成功 | 148      | cloudflare |
| 408  | ashton.ns.cloudflare.com              | 108.162.195.173 | IPv4   | h3   | ✅ 成功 | 148      | cloudflare |
| 12   | 104.16.65.1                           | 104.16.65.1     | IPv4   | h3   | ✅ 成功 | 150      | cloudflare |
| 298  | moura.ns.cloudflare.com               | 162.159.44.217  | IPv4   | h3   | ✅ 成功 | 152      | cloudflare |
| 279  | asia.877774.xyz                       | 104.16.211.153  | IPv4   | h3   | ✅ 成功 | 153      | cloudflare |
| 51   | 172.67.72.254                         | 172.67.72.254   | IPv4   | h3   | ✅ 成功 | 154      | cloudflare |
| 104  | 104.26.4.190                          | 104.26.4.190    | IPv4   | h3   | ✅ 成功 | 155      | cloudflare |
| 455  | 104.26.3.120                          | 104.26.3.120    | IPv4   | h3   | ✅ 成功 | 156      | cloudflare |
| 118  | 172.67.120.0                          | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 158      | cloudflare |
| 407  | ashton.ns.cloudflare.com              | 162.159.44.173  | IPv4   | h3   | ✅ 成功 | 158      | cloudflare |
| 562  | 162.159.44.133                        | 162.159.44.133  | IPv4   | h3   | ✅ 成功 | 158      | cloudflare |
| 432  | uriah.ns.cloudflare.com               | 162.159.44.194  | IPv4   | h3   | ✅ 成功 | 160      | cloudflare |
| 482  | icook.tw                              | 172.66.158.115  | IPv4   | h3   | ✅ 成功 | 162      | cloudflare |
| 566  | 162.159.137.204                       | 162.159.137.204 | IPv4   | h3   | ✅ 成功 | 163      | cloudflare |
| 506  | lewis.ns.cloudflare.com               | 162.159.44.159  | IPv4   | h3   | ✅ 成功 | 164      | cloudflare |
| 160  | cmcc.877774.xyz                       | 104.16.149.6    | IPv4   | h3   | ✅ 成功 | 165      | cloudflare |
| 165  | cmcc.877774.xyz                       | 104.16.149.11   | IPv4   | h3   | ✅ 成功 | 165      | cloudflare |
| 320  | benedict.ns.cloudflare.com            | 162.159.44.205  | IPv4   | h3   | ✅ 成功 | 165      | cloudflare |
| 93   | 162.159.45.93                         | 162.159.45.93   | IPv4   | h3   | ✅ 成功 | 166      | cloudflare |
| 125  | craig.ns.cloudflare.com               | 162.159.44.192  | IPv4   | h3   | ✅ 成功 | 167      | cloudflare |
| 111  | cloudflare.182682.xyz                 | 104.21.224.5    | IPv4   | h3   | ✅ 成功 | 170      | cloudflare |
| 387  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 170      | cloudflare |
| 560  | 162.159.21.222                        | 162.159.21.222  | IPv4   | h3   | ✅ 成功 | 171      | cloudflare |
| 75   | wilson.ns.cloudflare.com              | 172.64.35.110   | IPv4   | h3   | ✅ 成功 | 172      | cloudflare |
| 723  | ipv4.ip.sb                            | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 172      | cloudflare |
| 137  | pranab.ns.cloudflare.com              | 162.159.44.199  | IPv4   | h3   | ✅ 成功 | 174      | cloudflare |
| 440  | 172.64.42.158                         | 172.64.42.158   | IPv4   | h3   | ✅ 成功 | 178      | cloudflare |
| 561  | 162.159.39.136                        | 162.159.39.136  | IPv4   | h3   | ✅ 成功 | 179      | cloudflare |
| 76   | wilson.ns.cloudflare.com              | 162.159.44.110  | IPv4   | h3   | ✅ 成功 | 181      | cloudflare |
| 585  | 162.159.38.67                         | 162.159.38.67   | IPv4   | h3   | ✅ 成功 | 183      | cloudflare |
| 251  | braden.ns.cloudflare.com              | 162.159.44.169  | IPv4   | h3   | ✅ 成功 | 184      | cloudflare |
| 177  | 104.19.50.35                          | 104.19.50.35    | IPv4   | h3   | ✅ 成功 | 185      | cloudflare |
| 62   | trevor.ns.cloudflare.com              | 162.159.44.154  | IPv4   | h3   | ✅ 成功 | 186      | cloudflare |
| 355  | 162.159.45.176                        | 162.159.45.176  | IPv4   | h3   | ✅ 成功 | 186      | cloudflare |
| 426  | 172.64.53.181                         | 172.64.53.181   | IPv4   | h3   | ✅ 成功 | 194      | cloudflare |
| 587  | 162.159.45.165                        | 162.159.45.165  | IPv4   | h3   | ✅ 成功 | 195      | cloudflare |
| 287  | bowen.ns.cloudflare.com               | 162.159.44.83   | IPv4   | h3   | ✅ 成功 | 198      | cloudflare |
| 313  | rustam.ns.cloudflare.com              | 162.159.44.148  | IPv4   | h3   | ✅ 成功 | 198      | cloudflare |
| 4    | 104.19.212.207                        | 104.19.212.207  | IPv4   | h3   | ✅ 成功 | 199      | cloudflare |
| 96   | 162.159.39.62                         | 162.159.39.62   | IPv4   | h3   | ✅ 成功 | 203      | cloudflare |
| 73   | 162.159.38.192                        | 162.159.38.192  | IPv4   | h3   | ✅ 成功 | 204      | cloudflare |
| 391  | 162.159.39.177                        | 162.159.39.177  | IPv4   | h3   | ✅ 成功 | 207      | cloudflare |
| 567  | 172.64.53.40                          | 172.64.53.40    | IPv4   | h3   | ✅ 成功 | 207      | cloudflare |
| 413  | julio.ns.cloudflare.com               | 162.159.44.209  | IPv4   | h3   | ✅ 成功 | 208      | cloudflare |
| 188  | cris.ns.cloudflare.com                | 162.159.44.202  | IPv4   | h3   | ✅ 成功 | 214      | cloudflare |
| 71   | 162.159.44.176                        | 162.159.44.176  | IPv4   | h3   | ✅ 成功 | 224      | cloudflare |
| 52   | 104.20.30.198                         | 104.20.30.198   | IPv4   | h3   | ✅ 成功 | 241      | cloudflare |
| 31   | sullivan.ns.cloudflare.com            | 162.159.44.161  | IPv4   | h3   | ✅ 成功 | 253      | cloudflare |
| 198  | kyree.ns.cloudflare.com               | 162.159.44.207  | IPv4   | h3   | ✅ 成功 | 366      | cloudflare |
| 554  | damien.ns.cloudflare.com              | 162.159.44.168  | IPv4   | h3   | ✅ 成功 | 385      | cloudflare |
| 725  | 34.143.159.175                        | 34.143.159.175  | IPv4   | h2   | ✅ 成功 | 592      | cloudflare |
| 726  | 168.138.165.174                       | 168.138.165.174 | IPv4   | h2   | ✅ 成功 | 608      | cloudflare |
| 713  | 3.0.50.69                             | 3.0.50.69       | IPv4   | h2   | ✅ 成功 | 846      | cloudflare |
| 519  | 141.147.185.63                        | 141.147.185.63  | IPv4   | h2   | ✅ 成功 | 1751     | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 237 条记录
- **正常 (100-200ms)**: 255 条记录
- **慢 (200-500ms)**: 11 条记录
- **很慢 (>500ms)**: 4 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 3 次
- **IPv6 失败**: 219 次

### 按协议统计

- **none**: 222 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
