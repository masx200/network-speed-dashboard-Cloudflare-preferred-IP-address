# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2026/1/19 02:35:13
- **数据来源**: connectivity_results-20260119-023512.json
- **总测试数**: 714
- **失败测试数**: 221
- **成功测试数**: 493
- **失败率**: 30.95%
- **平均延迟**: 106.98ms
- **最小延迟**: 42ms
- **最大延迟**: 1508ms

## 🌐 当前测试环境信息

- **获取时间**: 2026/1/19 02:35:13
- **IP地址**: 172.183.157.176
- **国家/地区**: 美国 (US)
- **ASN**: 8075
- **网络组织**: cloud
- **网络域名**: microsoft.com
- **大洲**: North America (NA)
- **地理坐标**: 41.8835, -87.6305
- **时区**: America/Chicago
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 218 次 (98.6%)
- **连接超时: I/O超时**: 3 次 (1.4%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (218 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
| ---- | --------- | ------ | ------ | ---- | ------ | -------- | ------ | -------- |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 309  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |
| 553  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 713  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 218 次 (98.6%)
- **连接超时**: 3 次 (1.4%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 172.67（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 221 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 218 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次),
iplocation.io (3次), www.hugedomains.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 493 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 552  | cfip.xxxxxxxx.tk                      | 104.16.232.223  | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 105  | 172.67.243.218                        | 172.67.243.218  | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 182  | fbi.gov                               | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 345  | otto.ns.cloudflare.com                | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 462  | 172.67.74.57                          | 172.67.74.57    | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 261  | cf.090227.xyz                         | 104.18.35.15    | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 592  | 172.64.146.67                         | 172.64.146.67   | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 393  | 162.159.140.85                        | 162.159.140.85  | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 400  | 172.64.91.69                          | 172.64.91.69    | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 478  | 104.17.170.110                        | 104.17.170.110  | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 642  | 104.18.42.35                          | 104.18.42.35    | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 260  | www.glassdoor.com                     | 104.16.25.46    | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 415  | 104.18.166.129                        | 104.18.166.129  | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 578  | 172.64.34.153                         | 172.64.34.153   | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 615  | 104.17.189.30                         | 104.17.189.30   | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 619  | 162.159.42.146                        | 162.159.42.146  | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 645  | 162.159.26.213                        | 162.159.26.213  | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 646  | huxley.ns.cloudflare.com              | 172.64.35.188   | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 363  | japan.com                             | 104.26.4.60     | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 583  | 172.64.151.235                        | 172.64.151.235  | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 589  | 172.64.42.158                         | 172.64.42.158   | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 655  | 104.20.23.184                         | 104.20.23.184   | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 76   | www.okcupid.com                       | 104.16.239.254  | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 106  | www.visa.cn                           | 162.159.153.2   | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 361  | ifconfig.co                           | 172.67.168.106  | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 661  | 104.20.28.71                          | 104.20.28.71    | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 102  | yx-auto.pages.dev                     | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 534  | 104.26.2.166                          | 104.26.2.166    | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 200  | bowen.ns.cloudflare.com               | 172.64.35.83    | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 407  | 172.64.229.7                          | 172.64.229.7    | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 514  | 104.19.44.238                         | 104.19.44.238   | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 342  | 104.18.78.214                         | 104.18.78.214   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 394  | abdullah.ns.cloudflare.com            | 108.162.195.203 | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 432  | 104.19.212.207                        | 104.19.212.207  | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 550  | 104.17.21.106                         | 104.17.21.106   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 430  | www.csgo.com                          | 195.85.59.95    | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 486  | 108.162.194.125                       | 108.162.194.125 | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 667  | 104.17.153.151                        | 104.17.153.151  | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 67   | www.hugedomains.com                   | 172.67.70.191   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 179  | xn--b6gac.eu.org                      | 172.67.153.253  | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 225  | rustam.ns.cloudflare.com              | 172.64.35.148   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 537  | 104.20.17.233                         | 104.20.17.233   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 643  | 162.159.49.39                         | 162.159.49.39   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 669  | 104.19.144.159                        | 104.19.144.159  | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 680  | 172.64.157.187                        | 172.64.157.187  | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 437  | 104.17.167.134                        | 104.17.167.134  | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 544  | 104.17.50.237                         | 104.17.50.237   | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 640  | 172.64.48.205                         | 172.64.48.205   | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 665  | 104.26.6.112                          | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 320  | tasteatlas.com                        | 104.17.37.105   | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 325  | 172.67.79.211                         | 172.67.79.211   | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 673  | 104.18.44.25                          | 104.18.44.25    | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 224  | rustam.ns.cloudflare.com              | 108.162.195.148 | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 301  | dnschecker.org                        | 172.67.73.216   | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 466  | 172.67.64.214                         | 172.67.64.214   | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 526  | 104.18.39.228                         | 104.18.39.228   | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 186  | braden.ns.cloudflare.com              | 108.162.195.169 | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 369  | lewis.ns.cloudflare.com               | 108.162.195.159 | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 581  | 172.64.53.181                         | 172.64.53.181   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 588  | 172.64.145.119                        | 172.64.145.119  | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 509  | 104.26.12.227                         | 104.26.12.227   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 557  | 172.64.147.235                        | 172.64.147.235  | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 488  | 162.159.0.115                         | 162.159.0.115   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 599  | 104.26.5.101                          | 104.26.5.101    | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 217  | www.ipchicken.com                     | 104.26.7.112    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 110  | pranab.ns.cloudflare.com              | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 379  | stock.hostmonit.com                   | 104.21.7.193    | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 659  | 172.67.69.156                         | 172.67.69.156   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 252  | julio.ns.cloudflare.com               | 108.162.195.209 | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 390  | eur.877774.xyz                        | 104.21.29.164   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 483  | 172.64.53.0                           | 172.64.53.0     | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 80   | www.okcupid.com                       | 104.16.144.63   | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 463  | 172.67.68.211                         | 172.67.68.211   | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 614  | 104.17.209.79                         | 104.17.209.79   | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 334  | 172.64.35.24                          | 172.64.35.24    | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 467  | 104.20.24.107                         | 104.20.24.107   | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 39   | cmcc.877774.xyz                       | 104.16.149.6    | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 195  | saas.sin.fan                          | 162.159.36.20   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 271  | singapore.com                         | 104.26.12.140   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 332  | 198.62.62.4                           | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 50   | cmcc.877774.xyz                       | 104.16.148.4    | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 380  | stock.hostmonit.com                   | 172.67.187.251  | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 453  | 162.159.6.115                         | 162.159.6.115   | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 16   | ct.877774.xyz                         | 172.64.229.217  | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 45   | cmcc.877774.xyz                       | 104.16.149.12   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 59   | cmcc.877774.xyz                       | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 91   | 172.67.120.0                          | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 220  | moura.ns.cloudflare.com               | 172.64.35.217   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 392  | 198.41.208.15                         | 198.41.208.15   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 672  | 104.17.25.87                          | 104.17.25.87    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 215  | www.ipchicken.com                     | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 238  | ip.sb                                 | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 575  | 172.64.53.144                         | 172.64.53.144   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 28   | cf.0sm.com                            | 172.67.187.145  | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 40   | cmcc.877774.xyz                       | 104.16.149.7    | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 194  | saas.sin.fan                          | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 253  | julio.ns.cloudflare.com               | 172.64.35.209   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 654  | 172.64.154.12                         | 172.64.154.12   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 94   | craig.ns.cloudflare.com               | 108.162.195.192 | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 127  | na.877774.xyz                         | 104.18.187.25   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 146  | dylan.ns.cloudflare.com               | 172.64.35.187   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 175  | bestcf.030101.xyz                     | 104.16.254.88   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 218  | moura.ns.cloudflare.com               | 108.162.195.217 | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 269  | singapore.com                         | 172.67.75.194   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 318  | www.visa.com.hk                       | 104.18.20.69    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 512  | 104.20.19.201                         | 104.20.19.201   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 587  | 172.64.145.242                        | 172.64.145.242  | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 621  | 172.64.53.40                          | 172.64.53.40    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 677  | 104.17.214.136                        | 104.17.214.136  | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 46   | cmcc.877774.xyz                       | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 160  | zread.ai                              | 104.21.76.240   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 165  | freeyx.cloudflare88.eu.org            | 141.101.121.231 | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 207  | asia.877774.xyz                       | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 279  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 350  | www.digitalocean.com                  | 104.19.173.68   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 356  | damien.ns.cloudflare.com              | 108.162.195.168 | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 403  | 162.159.24.131                        | 162.159.24.131  | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 459  | 104.18.45.95                          | 104.18.45.95    | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 535  | 172.67.68.252                         | 172.67.68.252   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 34   | iplocation.io                         | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 68   | www.hugedomains.com                   | 104.26.7.37     | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 132  | toy-people.com                        | 172.67.72.18    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 159  | 104.17.79.11                          | 104.17.79.11    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 258  | 104.17.68.85                          | 104.17.68.85    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 265  | ip.gs                                 | 172.67.160.28   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 351  | www.digitalocean.com                  | 104.19.174.68   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 383  | 172.64.82.114                         | 172.64.82.114   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 516  | 104.18.40.39                          | 104.18.40.39    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 530  | 104.18.44.148                         | 104.18.44.148   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 580  | 172.64.52.90                          | 172.64.52.90    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 638  | 162.159.61.106                        | 162.159.61.106  | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 19   | ct.877774.xyz                         | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 54   | cmcc.877774.xyz                       | 104.16.148.8    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 62   | cmcc.877774.xyz                       | 104.16.149.3    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 99   | www.visa.com.sg                       | 104.18.12.229   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 276  | whatismyipaddress.com                 | 104.19.223.79   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 472  | 104.17.101.37                         | 104.17.101.37   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 518  | 104.19.50.35                          | 104.19.50.35    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 549  | 104.17.53.25                          | 104.17.53.25    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 605  | 104.20.25.82                          | 104.20.25.82    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 73   | cu.877774.xyz                         | 104.18.42.54    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 250  | 104.16.61.163                         | 104.16.61.163   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 480  | 104.18.39.15                          | 104.18.39.15    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 532  | 162.159.62.6                          | 162.159.62.6    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 595  | 104.18.42.61                          | 104.18.42.61    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 668  | 104.26.1.44                           | 104.26.1.44     | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 25   | ipv4.ip.sb                            | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 51   | cmcc.877774.xyz                       | 104.16.148.5    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 77   | www.okcupid.com                       | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 128  | na.877774.xyz                         | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 196  | cf.zhetengsha.eu.org                  | 172.64.152.241  | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 406  | 104.26.4.90                           | 104.26.4.90     | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 418  | 198.41.208.224                        | 198.41.208.224  | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 429  | www.csgo.com                          | 195.85.59.161   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 510  | 104.20.18.47                          | 104.20.18.47    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 653  | 104.18.41.10                          | 104.18.41.10    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 681  | 172.64.156.31                         | 172.64.156.31   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 10   | comicabc.com                          | 172.67.174.21   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 44   | cmcc.877774.xyz                       | 104.16.149.11   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 243  | cloudflare-ip.mofashi.ltd             | 188.114.97.3    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 313  | uriah.ns.cloudflare.com               | 108.162.195.194 | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 326  | gamer.com.tw                          | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 338  | icook.tw                              | 104.20.28.74    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 371  | lewis.ns.cloudflare.com               | 172.64.35.159   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 617  | 104.17.25.241                         | 104.17.25.241   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 17   | ct.877774.xyz                         | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 42   | cmcc.877774.xyz                       | 104.16.149.9    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 209  | time.is                               | 104.26.12.54    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 329  | www.udemy.com                         | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 337  | 104.26.13.31                          | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 365  | japan.com                             | 172.67.70.92    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 475  | 172.64.229.191                        | 172.64.229.191  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 678  | 162.159.40.85                         | 162.159.40.85   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 709  | icook.hk                              | 172.67.161.104  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 14   | ct.877774.xyz                         | 172.64.229.185  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 29   | cf.0sm.com                            | 104.21.7.133    | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 211  | time.is                               | 172.67.68.157   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 216  | www.ipchicken.com                     | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 295  | 104.18.14.76                          | 104.18.14.76    | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 618  | 104.19.35.242                         | 104.19.35.242   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 6    | www.ipget.net                         | 104.21.15.212   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 237  | ip.sb                                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 306  | cf.877774.xyz                         | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 503  | 104.26.8.148                          | 104.26.8.148    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 515  | 104.17.60.233                         | 104.17.60.233   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 556  | 104.16.251.143                        | 104.16.251.143  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 558  | 104.16.144.235                        | 104.16.144.235  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 590  | 162.159.3.89                          | 162.159.3.89    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 656  | 104.20.29.127                         | 104.20.29.127   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 663  | 172.67.73.236                         | 172.67.73.236   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 230  | benedict.ns.cloudflare.com            | 172.64.35.205   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 321  | tasteatlas.com                        | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 328  | www.udemy.com                         | 104.16.143.237  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 378  | cfip.1323123.xyz                      | 104.16.133.220  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 396  | abdullah.ns.cloudflare.com            | 172.64.35.203   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 401  | 172.64.52.127                         | 172.64.52.127   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 438  | 104.18.189.153                        | 104.18.189.153  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 452  | 162.159.21.116                        | 162.159.21.116  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 542  | 104.26.4.44                           | 104.26.4.44     | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 606  | 172.67.72.36                          | 172.67.72.36    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 140  | decker.ns.cloudflare.com              | 172.64.35.155   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 147  | dylan.ns.cloudflare.com               | 108.162.195.187 | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 168  | www.whatismyip.com                    | 104.26.13.23    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 174  | bestcf.030101.xyz                     | 104.17.120.229  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 360  | 104.19.223.58                         | 104.19.223.58   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 502  | 104.20.26.58                          | 104.20.26.58    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 533  | 162.159.19.37                         | 162.159.19.37   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 622  | 162.159.21.222                        | 162.159.21.222  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 5    | www.ipget.net                         | 172.67.207.26   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 15   | ct.877774.xyz                         | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 22   | www.gov.ua                            | 172.67.209.127  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 52   | cmcc.877774.xyz                       | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 55   | cmcc.877774.xyz                       | 104.16.148.9    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 436  | 104.31.16.158                         | 104.31.16.158   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 464  | 104.26.15.85                          | 104.26.15.85    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 476  | 104.16.255.1                          | 104.16.255.1    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 521  | 104.17.101.208                        | 104.17.101.208  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 593  | 162.159.22.29                         | 162.159.22.29   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 602  | 104.26.3.120                          | 104.26.3.120    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 620  | 172.64.52.67                          | 172.64.52.67    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 636  | 162.159.45.165                        | 162.159.45.165  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 694  | 172.64.48.175                         | 172.64.48.175   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 27   | ipv4.ip.sb                            | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 33   | iplocation.io                         | 104.26.10.222   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 58   | cmcc.877774.xyz                       | 104.16.148.12   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 266  | ip.gs                                 | 104.21.14.176   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 600  | 104.20.21.147                         | 104.20.21.147   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 623  | shopify.com                           | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 9    | comicabc.com                          | 104.21.64.10    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 100  | www.visa.com.sg                       | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 183  | fbi.gov                               | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 402  | 162.159.61.183                        | 162.159.61.183  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 456  | 108.162.192.66                        | 108.162.192.66  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 540  | 104.26.8.192                          | 104.26.8.192    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 547  | 104.17.30.164                         | 104.17.30.164   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 551  | cfip.xxxxxxxx.tk                      | 190.93.244.201  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 603  | 104.26.1.181                          | 104.26.1.181    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 664  | 104.26.7.153                          | 104.26.7.153    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 158  | 104.16.223.179                        | 104.16.223.179  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 170  | www.whatismyip.com                    | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 305  | cf.877774.xyz                         | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 388  | eur.877774.xyz                        | 104.21.47.209   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 414  | 162.159.140.116                       | 162.159.140.116 | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 504  | 172.67.77.196                         | 172.67.77.196   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 641  | 162.159.42.32                         | 162.159.42.32   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 676  | 104.16.251.254                        | 104.16.251.254  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 18   | ct.877774.xyz                         | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 93   | craig.ns.cloudflare.com               | 172.64.35.192   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 178  | xn--b6gac.eu.org                      | 104.21.90.78    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 236  | ip.sb                                 | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 317  | www.visa.com.hk                       | 104.18.21.69    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 327  | gamer.com.tw                          | 104.18.2.197    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 385  | www.wto.org                           | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 417  | 104.18.89.52                          | 104.18.89.52    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 511  | 104.26.4.190                          | 104.26.4.190    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 520  | 104.17.100.254                        | 104.17.100.254  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 611  | 104.17.115.224                        | 104.17.115.224  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 435  | 104.17.69.244                         | 104.17.69.244   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 666  | 172.67.110.232                        | 172.67.110.232  | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 107  | www.visa.cn                           | 162.159.152.2   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 428  | 104.17.139.37                         | 104.17.139.37   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 524  | 162.159.34.55                         | 162.159.34.55   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 657  | 162.159.34.78                         | 162.159.34.78   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 60   | cmcc.877774.xyz                       | 104.16.149.1    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 61   | cmcc.877774.xyz                       | 104.16.149.2    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 88   | cf.877771.xyz                         | 172.67.152.183  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 208  | asia.877774.xyz                       | 104.16.211.153  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 355  | damien.ns.cloudflare.com              | 172.64.35.168   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 391  | 198.41.194.162                        | 198.41.194.162  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 541  | 172.67.70.253                         | 172.67.70.253   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 32   | 104.18.254.88                         | 104.18.254.88   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 35   | iplocation.io                         | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 101  | yx-auto.pages.dev                     | 172.66.44.144   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 262  | cf.090227.xyz                         | 172.64.152.241  | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 517  | 104.16.148.187                        | 104.16.148.187  | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 660  | 172.67.79.160                         | 172.67.79.160   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 43   | cmcc.877774.xyz                       | 104.16.149.10   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 87   | cf.877771.xyz                         | 104.21.80.180   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 210  | time.is                               | 104.26.13.54    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 247  | palera.in                             | 104.21.58.72    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 290  | 456.cloudflare.182682.xyz             | 104.26.9.160    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 339  | icook.tw                              | 172.66.158.115  | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 375  | 172.67.181.209                        | 172.67.181.209  | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 421  | 104.18.37.13                          | 104.18.37.13    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 658  | 172.67.70.163                         | 172.67.70.163   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 697  | wilson.ns.cloudflare.com              | 162.159.44.110  | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 78   | www.okcupid.com                       | 104.18.160.63   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 193  | 172.64.151.55                         | 172.64.151.55   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 471  | 172.67.72.254                         | 172.67.72.254   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 527  | 104.18.37.110                         | 104.18.37.110   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 679  | 104.17.187.186                        | 104.17.187.186  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 134  | toy-people.com                        | 104.26.3.36     | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 152  | kyree.ns.cloudflare.com               | 108.162.195.207 | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 164  | freeyx.cloudflare88.eu.org            | 141.101.120.232 | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 289  | 456.cloudflare.182682.xyz             | 104.26.8.160    | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 296  | 172.67.106.26                         | 172.67.106.26   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 468  | 104.20.22.185                         | 104.20.22.185   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 69   | www.hugedomains.com                   | 104.26.6.37     | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 109  | pranab.ns.cloudflare.com              | 172.64.35.199   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 133  | toy-people.com                        | 104.26.2.36     | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 297  | 162.159.133.85                        | 162.159.133.85  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 384  | www.wto.org                           | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 470  | 104.20.30.198                         | 104.20.30.198   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 525  | 104.18.35.166                         | 104.18.35.166   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 21   | www.gov.ua                            | 104.21.23.72    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 201  | bowen.ns.cloudflare.com               | 108.162.195.83  | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 226  | rustam.ns.cloudflare.com              | 162.159.44.148  | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 364  | japan.com                             | 104.26.5.60     | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 507  | 104.20.21.161                         | 104.20.21.161   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 647  | huxley.ns.cloudflare.com              | 162.159.44.188  | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 116  | cris.ns.cloudflare.com                | 172.64.35.202   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 246  | palera.in                             | 172.67.157.122  | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 409  | 162.159.136.89                        | 162.159.136.89  | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 519  | 104.16.155.76                         | 104.16.155.76   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 543  | 172.67.76.195                         | 172.67.76.195   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 607  | 172.67.75.11                          | 172.67.75.11    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 41   | cmcc.877774.xyz                       | 104.16.149.8    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 65   | www.4chan.org                         | 104.16.228.229  | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 197  | cf.zhetengsha.eu.org                  | 104.18.35.15    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 489  | 172.64.52.110                         | 172.64.52.110   | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 333  | 104.18.37.40                          | 104.18.37.40    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 431  | 104.19.220.22                         | 104.19.220.22   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 505  | www.7749tv.com                        | 104.19.133.4    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 49   | cmcc.877774.xyz                       | 104.16.148.3    | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 66   | www.4chan.org                         | 104.16.229.229  | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 523  | 104.16.157.50                         | 104.16.157.50   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 585  | 162.159.41.141                        | 162.159.41.141  | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 671  | 104.17.53.129                         | 104.17.53.129   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 675  | 104.16.155.230                        | 104.16.155.230  | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 53   | cmcc.877774.xyz                       | 104.16.148.7    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 129  | na.877774.xyz                         | 104.18.38.235   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 231  | benedict.ns.cloudflare.com            | 162.159.44.205  | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 699  | wilson.ns.cloudflare.com              | 172.64.35.110   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 710  | icook.hk                              | 104.21.90.210   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 20   | ct.877774.xyz                         | 172.64.229.173  | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 47   | cmcc.877774.xyz                       | 104.16.148.1    | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 57   | cmcc.877774.xyz                       | 104.16.148.11   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 284  | ashton.ns.cloudflare.com              | 172.64.35.173   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 291  | 456.cloudflare.182682.xyz             | 172.67.75.208   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 608  | 104.17.211.218                        | 104.17.211.218  | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 609  | 104.17.170.137                        | 104.17.170.137  | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 48   | cmcc.877774.xyz                       | 104.16.148.2    | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 79   | www.okcupid.com                       | 104.16.223.254  | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 108  | pranab.ns.cloudflare.com              | 162.159.44.199  | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 139  | decker.ns.cloudflare.com              | 162.159.44.155  | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 410  | 104.26.3.162                          | 104.26.3.162    | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 427  | 104.18.151.172                        | 104.18.151.172  | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 601  | 104.26.2.2                            | 104.26.2.2      | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 613  | 104.17.215.66                         | 104.17.215.66   | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 695  | 162.159.24.205                        | 162.159.24.205  | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 138  | decker.ns.cloudflare.com              | 108.162.195.155 | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 283  | ashton.ns.cloudflare.com              | 108.162.195.173 | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 420  | 162.159.19.219                        | 162.159.19.219  | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 546  | 104.16.147.114                        | 104.16.147.114  | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 395  | abdullah.ns.cloudflare.com            | 162.159.44.203  | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 404  | 172.64.48.226                         | 172.64.48.226   | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 457  | 162.159.13.51                         | 162.159.13.51   | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 596  | 172.64.153.141                        | 172.64.153.141  | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 674  | 104.17.110.226                        | 104.17.110.226  | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 161  | zread.ai                              | 172.67.202.78   | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 419  | 104.19.148.121                        | 104.19.148.121  | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 433  | 104.18.255.167                        | 104.18.255.167  | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 586  | 108.162.198.170                       | 108.162.198.170 | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 598  | 104.18.40.216                         | 104.18.40.216   | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 538  | 104.26.6.159                          | 104.26.6.159    | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 612  | 104.19.34.231                         | 104.19.34.231   | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 683  | 162.159.58.15                         | 162.159.58.15   | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 696  | 172.64.144.130                        | 172.64.144.130  | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 83   | sullivan.ns.cloudflare.com            | 172.64.35.161   | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 251  | 162.159.36.104                        | 162.159.36.104  | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 81   | sullivan.ns.cloudflare.com            | 108.162.195.161 | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 242  | cloudflare-ip.mofashi.ltd             | 188.114.96.3    | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 555  | cfip.xxxxxxxx.tk                      | 104.16.241.229  | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 573  | 162.159.7.12                          | 162.159.7.12    | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 648  | huxley.ns.cloudflare.com              | 108.162.195.188 | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 206  | asia.877774.xyz                       | 104.17.139.62   | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 416  | 104.18.81.19                          | 104.18.81.19    | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 439  | 104.18.223.253                        | 104.18.223.253  | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 465  | 172.64.150.30                         | 172.64.150.30   | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 487  | 172.64.50.51                          | 172.64.50.51    | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 693  | 162.159.13.208                        | 162.159.13.208  | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 454  | 162.159.17.243                        | 162.159.17.243  | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 529  | 162.159.36.26                         | 162.159.36.26   | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 637  | 172.67.64.153                         | 172.67.64.153   | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 644  | 162.159.3.132                         | 162.159.3.132   | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 539  | 104.20.16.244                         | 104.20.16.244   | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 616  | 104.19.153.47                         | 104.19.153.47   | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 501  | 162.159.39.62                         | 162.159.39.62   | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 591  | 162.159.18.240                        | 162.159.18.240  | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 670  | 172.64.42.141                         | 172.64.42.141   | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 299  | dnschecker.org                        | 104.26.6.89     | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 312  | uriah.ns.cloudflare.com               | 172.64.35.194   | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 281  | 104.18.42.26                          | 104.18.42.26    | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 115  | cris.ns.cloudflare.com                | 162.159.44.202  | IPv4   | h3   | ✅ 成功 | 124      | cloudflare |
| 426  | 104.17.162.3                          | 104.17.162.3    | IPv4   | h3   | ✅ 成功 | 124      | cloudflare |
| 232  | benedict.ns.cloudflare.com            | 108.162.195.205 | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 424  | 172.67.75.172                         | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 425  | 162.159.45.176                        | 162.159.45.176  | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 461  | 173.245.58.237                        | 173.245.58.237  | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 554  | cfip.xxxxxxxx.tk                      | 188.114.96.125  | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 632  | 162.159.44.133                        | 162.159.44.133  | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 450  | 104.16.105.166                        | 104.16.105.166  | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 536  | 104.26.12.113                         | 104.26.12.113   | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 574  | 162.159.39.99                         | 162.159.39.99   | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 579  | 162.159.38.171                        | 162.159.38.171  | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 479  | 104.17.168.159                        | 104.17.168.159  | IPv4   | h3   | ✅ 成功 | 128      | cloudflare |
| 561  | 172.64.52.15                          | 172.64.52.15    | IPv4   | h3   | ✅ 成功 | 128      | cloudflare |
| 26   | ipv4.ip.sb                            | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 129      | cloudflare |
| 682  | 162.159.18.2                          | 162.159.18.2    | IPv4   | h3   | ✅ 成功 | 129      | cloudflare |
| 285  | ashton.ns.cloudflare.com              | 162.159.44.173  | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 482  | 104.17.105.198                        | 104.17.105.198  | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 560  | 162.159.38.67                         | 162.159.38.67   | IPv4   | h3   | ✅ 成功 | 131      | cloudflare |
| 604  | 172.67.73.120                         | 172.67.73.120   | IPv4   | h3   | ✅ 成功 | 131      | cloudflare |
| 114  | cris.ns.cloudflare.com                | 108.162.195.202 | IPv4   | h3   | ✅ 成功 | 132      | cloudflare |
| 259  | www.glassdoor.com                     | 104.17.64.70    | IPv4   | h3   | ✅ 成功 | 133      | cloudflare |
| 652  | 162.159.40.159                        | 162.159.40.159  | IPv4   | h3   | ✅ 成功 | 133      | cloudflare |
| 92   | craig.ns.cloudflare.com               | 162.159.44.192  | IPv4   | h3   | ✅ 成功 | 134      | cloudflare |
| 473  | 104.26.3.176                          | 104.26.3.176    | IPv4   | h3   | ✅ 成功 | 134      | cloudflare |
| 562  | 108.162.198.69                        | 108.162.198.69  | IPv4   | h3   | ✅ 成功 | 134      | cloudflare |
| 508  | 172.67.67.5                           | 172.67.67.5     | IPv4   | h3   | ✅ 成功 | 135      | cloudflare |
| 469  | 172.67.65.159                         | 172.67.65.159   | IPv4   | h3   | ✅ 成功 | 136      | cloudflare |
| 577  | 162.159.38.35                         | 162.159.38.35   | IPv4   | h3   | ✅ 成功 | 136      | cloudflare |
| 597  | 162.159.1.111                         | 162.159.1.111   | IPv4   | h3   | ✅ 成功 | 136      | cloudflare |
| 576  | 162.159.44.128                        | 162.159.44.128  | IPv4   | h3   | ✅ 成功 | 138      | cloudflare |
| 639  | 108.162.198.148                       | 108.162.198.148 | IPv4   | h3   | ✅ 成功 | 138      | cloudflare |
| 460  | 104.18.47.253                         | 104.18.47.253   | IPv4   | h3   | ✅ 成功 | 139      | cloudflare |
| 528  | 108.162.195.1                         | 108.162.195.1   | IPv4   | h3   | ✅ 成功 | 140      | cloudflare |
| 188  | braden.ns.cloudflare.com              | 172.64.35.169   | IPv4   | h3   | ✅ 成功 | 141      | cloudflare |
| 64   | cmcc.877774.xyz                       | 104.16.149.5    | IPv4   | h3   | ✅ 成功 | 142      | cloudflare |
| 63   | cmcc.877774.xyz                       | 104.16.149.4    | IPv4   | h3   | ✅ 成功 | 144      | cloudflare |
| 474  | 104.17.169.180                        | 104.17.169.180  | IPv4   | h3   | ✅ 成功 | 144      | cloudflare |
| 389  | eur.877774.xyz                        | 104.21.26.150   | IPv4   | h3   | ✅ 成功 | 145      | cloudflare |
| 624  | 162.159.39.136                        | 162.159.39.136  | IPv4   | h3   | ✅ 成功 | 145      | cloudflare |
| 270  | singapore.com                         | 104.26.13.140   | IPv4   | h3   | ✅ 成功 | 146      | cloudflare |
| 484  | 162.159.38.192                        | 162.159.38.192  | IPv4   | h3   | ✅ 成功 | 146      | cloudflare |
| 300  | dnschecker.org                        | 104.26.7.89     | IPv4   | h3   | ✅ 成功 | 147      | cloudflare |
| 610  | 104.26.10.239                         | 104.26.10.239   | IPv4   | h3   | ✅ 成功 | 147      | cloudflare |
| 423  | 104.16.45.84                          | 104.16.45.84    | IPv4   | h3   | ✅ 成功 | 148      | cloudflare |
| 500  | 162.159.45.93                         | 162.159.45.93   | IPv4   | h3   | ✅ 成功 | 148      | cloudflare |
| 703  | trevor.ns.cloudflare.com              | 162.159.44.154  | IPv4   | h3   | ✅ 成功 | 148      | cloudflare |
| 704  | trevor.ns.cloudflare.com              | 172.64.35.154   | IPv4   | h3   | ✅ 成功 | 148      | cloudflare |
| 346  | otto.ns.cloudflare.com                | 162.159.44.135  | IPv4   | h3   | ✅ 成功 | 149      | cloudflare |
| 451  | 104.17.142.212                        | 104.17.142.212  | IPv4   | h3   | ✅ 成功 | 149      | cloudflare |
| 408  | 173.245.49.194                        | 173.245.49.194  | IPv4   | h3   | ✅ 成功 | 150      | cloudflare |
| 545  | 104.17.24.232                         | 104.17.24.232   | IPv4   | h3   | ✅ 成功 | 150      | cloudflare |
| 82   | sullivan.ns.cloudflare.com            | 162.159.44.161  | IPv4   | h3   | ✅ 成功 | 152      | cloudflare |
| 522  | 104.17.119.199                        | 104.17.119.199  | IPv4   | h3   | ✅ 成功 | 152      | cloudflare |
| 434  | 104.19.154.200                        | 104.19.154.200  | IPv4   | h3   | ✅ 成功 | 153      | cloudflare |
| 698  | wilson.ns.cloudflare.com              | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 153      | cloudflare |
| 548  | 104.16.153.12                         | 104.16.153.12   | IPv4   | h3   | ✅ 成功 | 154      | cloudflare |
| 154  | kyree.ns.cloudflare.com               | 162.159.44.207  | IPv4   | h3   | ✅ 成功 | 158      | cloudflare |
| 412  | 104.26.8.117                          | 104.26.8.117    | IPv4   | h3   | ✅ 成功 | 158      | cloudflare |
| 219  | moura.ns.cloudflare.com               | 162.159.44.217  | IPv4   | h3   | ✅ 成功 | 159      | cloudflare |
| 477  | 104.17.156.81                         | 104.17.156.81   | IPv4   | h3   | ✅ 成功 | 159      | cloudflare |
| 202  | bowen.ns.cloudflare.com               | 162.159.44.83   | IPv4   | h3   | ✅ 成功 | 160      | cloudflare |
| 411  | 162.159.128.253                       | 162.159.128.253 | IPv4   | h3   | ✅ 成功 | 160      | cloudflare |
| 275  | whatismyipaddress.com                 | 104.19.222.79   | IPv4   | h3   | ✅ 成功 | 162      | cloudflare |
| 153  | kyree.ns.cloudflare.com               | 172.64.35.207   | IPv4   | h3   | ✅ 成功 | 164      | cloudflare |
| 405  | 162.159.58.65                         | 162.159.58.65   | IPv4   | h3   | ✅ 成功 | 164      | cloudflare |
| 582  | 162.159.45.145                        | 162.159.45.145  | IPv4   | h3   | ✅ 成功 | 165      | cloudflare |
| 376  | 104.19.175.123                        | 104.19.175.123  | IPv4   | h3   | ✅ 成功 | 167      | cloudflare |
| 2    | 104.18.39.196                         | 104.18.39.196   | IPv4   | h3   | ✅ 成功 | 170      | cloudflare |
| 458  | 162.159.46.238                        | 162.159.46.238  | IPv4   | h3   | ✅ 成功 | 170      | cloudflare |
| 74   | cu.877774.xyz                         | 172.64.145.202  | IPv4   | h3   | ✅ 成功 | 171      | cloudflare |
| 344  | otto.ns.cloudflare.com                | 172.64.35.135   | IPv4   | h3   | ✅ 成功 | 173      | cloudflare |
| 455  | 104.16.65.1                           | 104.16.65.1     | IPv4   | h3   | ✅ 成功 | 173      | cloudflare |
| 13   | ct.877774.xyz                         | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 175      | cloudflare |
| 377  | 108.162.198.54                        | 108.162.198.54  | IPv4   | h3   | ✅ 成功 | 179      | cloudflare |
| 354  | damien.ns.cloudflare.com              | 162.159.44.168  | IPv4   | h3   | ✅ 成功 | 181      | cloudflare |
| 3    | 172.64.154.18                         | 172.64.154.18   | IPv4   | h3   | ✅ 成功 | 184      | cloudflare |
| 169  | www.whatismyip.com                    | 172.67.69.129   | IPv4   | h3   | ✅ 成功 | 184      | cloudflare |
| 584  | 162.159.39.177                        | 162.159.39.177  | IPv4   | h3   | ✅ 成功 | 185      | cloudflare |
| 485  | 108.162.198.48                        | 108.162.198.48  | IPv4   | h3   | ✅ 成功 | 187      | cloudflare |
| 254  | julio.ns.cloudflare.com               | 162.159.44.209  | IPv4   | h3   | ✅ 成功 | 189      | cloudflare |
| 311  | uriah.ns.cloudflare.com               | 162.159.44.194  | IPv4   | h3   | ✅ 成功 | 190      | cloudflare |
| 324  | 104.17.142.12                         | 104.17.142.12   | IPv4   | h3   | ✅ 成功 | 194      | cloudflare |
| 705  | trevor.ns.cloudflare.com              | 108.162.195.154 | IPv4   | h3   | ✅ 成功 | 200      | cloudflare |
| 481  | 104.17.16.248                         | 104.17.16.248   | IPv4   | h3   | ✅ 成功 | 208      | cloudflare |
| 4    | 103.160.204.59                        | 103.160.204.59  | IPv4   | h3   | ✅ 成功 | 211      | cloudflare |
| 559  | 104.17.154.254                        | 104.17.154.254  | IPv4   | h3   | ✅ 成功 | 219      | cloudflare |
| 413  | 104.26.5.134                          | 104.26.5.134    | IPv4   | h3   | ✅ 成功 | 228      | cloudflare |
| 187  | braden.ns.cloudflare.com              | 162.159.44.169  | IPv4   | h3   | ✅ 成功 | 234      | cloudflare |
| 513  | 104.20.20.42                          | 104.20.20.42    | IPv4   | h3   | ✅ 成功 | 234      | cloudflare |
| 531  | 162.159.42.140                        | 162.159.42.140  | IPv4   | h3   | ✅ 成功 | 241      | cloudflare |
| 148  | dylan.ns.cloudflare.com               | 162.159.44.187  | IPv4   | h3   | ✅ 成功 | 273      | cloudflare |
| 370  | lewis.ns.cloudflare.com               | 162.159.44.159  | IPv4   | h3   | ✅ 成功 | 316      | cloudflare |
| 335  | 141.147.185.63                        | 141.147.185.63  | IPv4   | h2   | ✅ 成功 | 351      | cloudflare |
| 422  | 162.159.137.204                       | 162.159.137.204 | IPv4   | h3   | ✅ 成功 | 413      | cloudflare |
| 56   | cmcc.877774.xyz                       | 104.16.148.10   | IPv4   | h3   | ✅ 成功 | 525      | cloudflare |
| 98   | 34.143.159.175                        | 34.143.159.175  | IPv4   | h2   | ✅ 成功 | 542      | cloudflare |
| 594  | 3.0.50.69                             | 3.0.50.69       | IPv4   | h2   | ✅ 成功 | 550      | cloudflare |
| 506  | 162.159.44.176                        | 162.159.44.176  | IPv4   | h3   | ✅ 成功 | 601      | cloudflare |
| 714  | 168.138.165.174                       | 168.138.165.174 | IPv4   | h2   | ✅ 成功 | 1508     | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 18 条记录
- **快 (50-100ms)**: 273 条记录
- **正常 (100-200ms)**: 185 条记录
- **慢 (200-500ms)**: 12 条记录
- **很慢 (>500ms)**: 5 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 3 次
- **IPv6 失败**: 218 次

### 按协议统计

- **none**: 221 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
