# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/30 02:01:29
- **数据来源**: connectivity_results-20251230-020128.json
- **总测试数**: 506
- **失败测试数**: 181
- **成功测试数**: 325
- **失败率**: 35.77%
- **平均延迟**: 123.16ms
- **最小延迟**: 64ms
- **最大延迟**: 1500ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/30 02:01:29
- **IP地址**: 52.173.163.131
- **国家/地区**: 美国 (US)
- **ASN**: 8075
- **网络组织**: Microsoft Corporation
- **网络域名**: microsoft.com
- **大洲**: North America (NA)
- **地理坐标**: 41.6015, -93.6127
- **时区**: America/Chicago
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 178 次 (98.3%)
- **连接超时: I/O超时**: 3 次 (1.7%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (178 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
| ---- | --------- | ------ | ------ | ---- | ------ | -------- | ------ | -------- |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 314  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 505  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |
| 506  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 178 次 (98.3%)
- **连接超时**: 3 次 (1.7%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 181 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 178 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次),
sullivan.ns.cloudflare.com (3次), craig.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 325 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 78   | www.visa.com.sg                       | 104.18.12.229   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 128  | 172.64.151.55                         | 172.64.151.55   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 498  | wilson.ns.cloudflare.com              | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 316  | otto.ns.cloudflare.com                | 172.64.35.135   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 267  | dnschecker.org                        | 104.26.6.89     | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 481  | cu.877774.xyz                         | 104.26.4.115    | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 295  | cf.877774.xyz                         | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 130  | braden.ns.cloudflare.com              | 172.64.35.169   | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 199  | www.ipchicken.com                     | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 287  | tasteatlas.com                        | 104.17.37.105   | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 408  | 104.19.175.123                        | 104.19.175.123  | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 452  | 104.19.220.22                         | 104.19.220.22   | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 95   | 104.16.223.179                        | 104.16.223.179  | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 409  | 108.162.198.54                        | 108.162.198.54  | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 332  | cfip.1323123.xyz                      | 104.16.133.220  | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 374  | 104.18.78.214                         | 104.18.78.214   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 167  | fbi.gov                               | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 427  | 104.26.5.134                          | 104.26.5.134    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 202  | rustam.ns.cloudflare.com              | 172.64.35.148   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 69   | cloudflare.182682.xyz                 | 104.21.224.5    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 209  | ip.sb                                 | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 185  | time.is                               | 104.26.12.54    | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 223  | palera.in                             | 104.21.58.72    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 247  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 250  | 104.18.42.26                          | 104.18.42.26    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 83   | yx-auto.pages.dev                     | 172.66.44.144   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 251  | 172.67.106.26                         | 172.67.106.26   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 278  | uriah.ns.cloudflare.com               | 172.64.35.194   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 430  | abdullah.ns.cloudflare.com            | 172.64.35.203   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 104  | www.whatismyip.com                    | 172.67.69.129   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 224  | palera.in                             | 172.67.157.122  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 310  | cfip.xxxxxxxx.tk                      | 104.27.21.118   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 454  | 104.18.255.167                        | 104.18.255.167  | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 487  | www.hugedomains.com                   | 104.26.7.37     | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 142  | cmcc.877774.xyz                       | 104.16.148.12   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 183  | saas.sin.fan                          | 162.159.36.20   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 138  | cmcc.877774.xyz                       | 104.16.148.8    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 233  | ip.gs                                 | 104.21.14.176   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 436  | 104.18.254.88                         | 104.18.254.88   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 451  | 104.18.166.129                        | 104.18.166.129  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 20   | craig.ns.cloudflare.com               | 172.64.35.192   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 117  | cf.zhetengsha.eu.org                  | 104.18.35.15    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 153  | cmcc.877774.xyz                       | 104.16.149.10   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 476  | cu.877774.xyz                         | 104.26.4.119    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 455  | 104.19.212.207                        | 104.19.212.207  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 482  | cu.877774.xyz                         | 104.26.4.116    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 497  | wilson.ns.cloudflare.com              | 172.64.35.110   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 46   | cris.ns.cloudflare.com                | 172.64.35.202   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 277  | uriah.ns.cloudflare.com               | 108.162.195.194 | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 411  | 198.41.194.162                        | 198.41.194.162  | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 96   | 104.17.79.11                          | 104.17.79.11    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 313  | cfip.xxxxxxxx.tk                      | 188.114.96.125  | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 401  | eur.877774.xyz                        | 104.21.29.164   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 426  | 162.159.58.65                         | 162.159.58.65   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 447  | www.4chan.org                         | 104.16.229.229  | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 12   | ct.877774.xyz                         | 172.64.229.173  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 28   | www.okcupid.com                       | 104.16.144.63   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 140  | cmcc.877774.xyz                       | 104.16.148.10   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 305  | cfip.xxxxxxxx.tk                      | 198.41.214.141  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 364  | 104.18.37.40                          | 104.18.37.40    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 365  | 172.64.35.24                          | 172.64.35.24    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 381  | shopify.com                           | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 423  | 173.245.49.194                        | 173.245.49.194  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 14   | ct.877774.xyz                         | 172.64.229.185  | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 37   | cf.877771.xyz                         | 104.21.80.180   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 237  | whatismyipaddress.com                 | 104.19.223.79   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 386  | ifconfig.co                           | 172.67.168.106  | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 448  | www.4chan.org                         | 104.16.228.229  | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 31   | comicabc.com                          | 172.67.174.21   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 367  | 104.26.13.31                          | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 380  | 104.19.223.58                         | 104.19.223.58   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 405  | www.wto.org                           | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 449  | 104.18.81.19                          | 104.18.81.19    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 26   | www.okcupid.com                       | 104.16.223.254  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 61   | decker.ns.cloudflare.com              | 108.162.195.155 | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 79   | www.ipget.net                         | 104.21.15.212   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 200  | rustam.ns.cloudflare.com              | 108.162.195.148 | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 266  | 162.159.133.85                        | 162.159.133.85  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 268  | dnschecker.org                        | 104.26.7.89     | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 274  | www.visa.com.hk                       | 104.18.21.69    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 424  | 162.159.136.89                        | 162.159.136.89  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 18   | ct.877774.xyz                         | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 27   | www.okcupid.com                       | 104.18.160.63   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 68   | cloudflare.182682.xyz                 | 104.17.25.173   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 162  | cloudflare-ip.mofashi.ltd             | 104.21.72.233   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 253  | ashton.ns.cloudflare.com              | 172.64.35.173   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 311  | cfip.xxxxxxxx.tk                      | 104.16.241.229  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 328  | damien.ns.cloudflare.com              | 172.64.35.168   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 493  | trevor.ns.cloudflare.com              | 108.162.195.154 | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 124  | xn--b6gac.eu.org                      | 104.21.90.78    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 148  | cmcc.877774.xyz                       | 104.16.149.5    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 361  | www.gov.ua                            | 172.67.209.127  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 453  | 104.19.154.200                        | 104.19.154.200  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 11   | ct.877774.xyz                         | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 339  | 104.18.39.196                         | 104.18.39.196   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 358  | ipv4.ip.sb                            | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 410  | 172.64.82.114                         | 172.64.82.114   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 29   | www.okcupid.com                       | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 40   | pranab.ns.cloudflare.com              | 172.64.35.199   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 52   | na.877774.xyz                         | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 147  | cmcc.877774.xyz                       | 104.16.149.4    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 181  | asia.877774.xyz                       | 104.16.211.153  | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 182  | asia.877774.xyz                       | 104.17.139.62   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 243  | singapore.com                         | 104.26.12.140   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 464  | 104.17.142.212                        | 104.17.142.212  | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 25   | 172.67.120.0                          | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 166  | fbi.gov                               | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 177  | cf.090227.xyz                         | 104.18.43.174   | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 293  | 172.67.79.211                         | 172.67.79.211   | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 341  | www.pcmag.com                         | 104.16.20.118   | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 54   | na.877774.xyz                         | 104.18.187.25   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 137  | cmcc.877774.xyz                       | 104.16.148.7    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 463  | 104.16.105.166                        | 104.16.105.166  | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 479  | cu.877774.xyz                         | 104.26.4.113    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 63   | decker.ns.cloudflare.com              | 162.159.44.155  | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 67   | cloudflare.182682.xyz                 | 104.18.185.26   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 112  | zread.ai                              | 172.67.202.78   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 145  | cmcc.877774.xyz                       | 104.16.149.2    | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 206  | 104.16.61.163                         | 104.16.61.163   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 282  | www.udemy.com                         | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 283  | www.udemy.com                         | 104.16.143.237  | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 296  | cf.877774.xyz                         | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 340  | 104.16.45.84                          | 104.16.45.84    | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 403  | eur.877774.xyz                        | 104.21.26.150   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 439  | www.csgo.com                          | 195.85.59.95    | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 450  | 104.18.89.52                          | 104.18.89.52    | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 62   | decker.ns.cloudflare.com              | 172.64.35.155   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 143  | cmcc.877774.xyz                       | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 156  | cmcc.877774.xyz                       | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 158  | cmcc.877774.xyz                       | 104.16.148.2    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 197  | www.ipchicken.com                     | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 273  | www.visa.com.hk                       | 104.18.20.69    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 346  | 103.160.204.59                        | 103.160.204.59  | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 356  | 162.159.128.253                       | 162.159.128.253 | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 376  | cf.0sm.com                            | 104.21.7.133    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 417  | stock.hostmonit.com                   | 104.21.7.193    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 437  | 162.159.137.204                       | 162.159.137.204 | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 484  | cu.877774.xyz                         | 104.26.4.118    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 87   | kyree.ns.cloudflare.com               | 108.162.195.207 | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 120  | bestcf.030101.xyz                     | 104.19.156.139  | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 163  | cloudflare-ip.mofashi.ltd             | 172.67.155.172  | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 335  | huxley.ns.cloudflare.com              | 172.64.35.188   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 383  | icook.hk                              | 172.67.161.104  | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 397  | japan.com                             | 104.26.4.60     | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 446  | 162.159.140.116                       | 162.159.140.116 | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 457  | 104.31.16.158                         | 104.31.16.158   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 486  | www.hugedomains.com                   | 104.26.6.37     | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 47   | cris.ns.cloudflare.com                | 108.162.195.202 | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 55   | toy-people.com                        | 172.67.72.18    | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 77   | www.visa.com.sg                       | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 229  | 456.cloudflare.182682.xyz             | 104.26.8.160    | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 395  | japan.com                             | 172.67.70.92    | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 443  | 104.18.223.253                        | 104.18.223.253  | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 53   | na.877774.xyz                         | 104.18.38.235   | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 139  | cmcc.877774.xyz                       | 104.16.148.9    | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 170  | bowen.ns.cloudflare.com               | 108.162.195.83  | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 322  | www.digitalocean.com                  | 104.19.174.68   | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 370  | iplocation.io                         | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 414  | 172.64.52.127                         | 172.64.52.127   | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 131  | braden.ns.cloudflare.com              | 108.162.195.169 | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 215  | www.glassdoor.com                     | 104.17.64.70    | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 306  | cfip.xxxxxxxx.tk                      | 104.17.127.110  | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 416  | stock.hostmonit.com                   | 172.67.187.251  | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 228  | 456.cloudflare.182682.xyz             | 172.67.75.208   | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 425  | 104.26.4.90                           | 104.26.4.90     | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 485  | www.hugedomains.com                   | 172.67.70.191   | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 105  | www.whatismyip.com                    | 104.26.13.23    | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 210  | ip.sb                                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 219  | benedict.ns.cloudflare.com            | 172.64.35.205   | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 440  | 104.17.162.3                          | 104.17.162.3    | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 456  | 104.17.69.244                         | 104.17.69.244   | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 459  | 104.19.148.121                        | 104.19.148.121  | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 460  | 198.41.208.224                        | 198.41.208.224  | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 36   | cf.877771.xyz                         | 172.67.152.183  | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 71   | cloudflare.182682.xyz                 | 104.16.250.22   | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 136  | cmcc.877774.xyz                       | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 172  | bowen.ns.cloudflare.com               | 172.64.35.83    | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 184  | saas.sin.fan                          | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 260  | julio.ns.cloudflare.com               | 172.64.35.209   | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 394  | 172.67.181.209                        | 172.67.181.209  | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 466  | 104.18.189.153                        | 104.18.189.153  | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 478  | cu.877774.xyz                         | 104.26.4.112    | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 499  | wilson.ns.cloudflare.com              | 162.159.44.110  | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 180  | asia.877774.xyz                       | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 309  | cfip.xxxxxxxx.tk                      | 190.93.244.201  | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 415  | 162.159.61.183                        | 162.159.61.183  | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 441  | 104.26.3.162                          | 104.26.3.162    | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 110  | www.visa.cn                           | 162.159.153.2   | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 157  | cmcc.877774.xyz                       | 104.16.148.1    | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 161  | cmcc.877774.xyz                       | 104.16.148.5    | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 360  | www.gov.ua                            | 104.21.23.72    | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 377  | cf.0sm.com                            | 172.67.187.145  | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 413  | 162.159.140.85                        | 162.159.140.85  | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 15   | ct.877774.xyz                         | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 80   | www.ipget.net                         | 172.67.207.26   | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 84   | yx-auto.pages.dev                     | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 176  | cf.090227.xyz                         | 172.64.144.82   | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 191  | moura.ns.cloudflare.com               | 108.162.195.217 | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 207  | 162.159.36.104                        | 162.159.36.104  | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 353  | 104.26.6.112                          | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 56   | toy-people.com                        | 104.26.2.36     | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 113  | zread.ai                              | 104.21.76.240   | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 159  | cmcc.877774.xyz                       | 104.16.148.3    | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 208  | ip.sb                                 | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 216  | 104.17.68.85                          | 104.17.68.85    | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 299  | icook.tw                              | 172.66.158.115  | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 357  | ipv4.ip.sb                            | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 429  | abdullah.ns.cloudflare.com            | 162.159.44.203  | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 242  | singapore.com                         | 172.67.75.194   | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 269  | dnschecker.org                        | 172.67.73.216   | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 428  | abdullah.ns.cloudflare.com            | 108.162.195.203 | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 483  | cu.877774.xyz                         | 104.26.4.117    | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 70   | cloudflare.182682.xyz                 | 104.21.227.134  | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 368  | iplocation.io                         | 104.26.10.222   | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 420  | 172.64.91.69                          | 172.64.91.69    | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 477  | cu.877774.xyz                         | 104.26.4.111    | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 30   | www.okcupid.com                       | 104.16.239.254  | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 255  | ashton.ns.cloudflare.com              | 162.159.44.173  | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 369  | iplocation.io                         | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 444  | 104.18.151.172                        | 104.18.151.172  | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 146  | cmcc.877774.xyz                       | 104.16.149.3    | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 186  | time.is                               | 104.26.13.54    | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 254  | ashton.ns.cloudflare.com              | 108.162.195.173 | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 359  | ipv4.ip.sb                            | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 458  | 104.17.167.134                        | 104.17.167.134  | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 492  | trevor.ns.cloudflare.com              | 172.64.35.154   | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 57   | toy-people.com                        | 104.26.3.36     | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 125  | xn--b6gac.eu.org                      | 172.67.153.253  | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 151  | cmcc.877774.xyz                       | 104.16.149.8    | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 214  | www.glassdoor.com                     | 104.16.25.46    | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 227  | 456.cloudflare.182682.xyz             | 104.26.9.160    | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 307  | cfip.xxxxxxxx.tk                      | 190.93.246.67   | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 421  | 162.159.24.131                        | 162.159.24.131  | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 461  | www.7749tv.com                        | 104.16.115.36   | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 252  | 104.18.14.76                          | 104.18.14.76    | IPv4   | h3   | ✅ 成功 | 124      | cloudflare |
| 259  | julio.ns.cloudflare.com               | 108.162.195.209 | IPv4   | h3   | ✅ 成功 | 124      | cloudflare |
| 294  | 104.17.142.12                         | 104.17.142.12   | IPv4   | h3   | ✅ 成功 | 124      | cloudflare |
| 326  | damien.ns.cloudflare.com              | 108.162.195.168 | IPv4   | h3   | ✅ 成功 | 124      | cloudflare |
| 480  | cu.877774.xyz                         | 104.26.4.114    | IPv4   | h3   | ✅ 成功 | 124      | cloudflare |
| 5    | sullivan.ns.cloudflare.com            | 108.162.195.161 | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 97   | dylan.ns.cloudflare.com               | 108.162.195.187 | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 300  | icook.tw                              | 104.20.28.74    | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 315  | cfip.xxxxxxxx.tk                      | 104.16.232.223  | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 442  | 104.26.8.117                          | 104.26.8.117    | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 17   | ct.877774.xyz                         | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 291  | gamer.com.tw                          | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 438  | www.csgo.com                          | 195.85.59.161   | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 3    | 198.62.62.4                           | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 19   | craig.ns.cloudflare.com               | 108.162.195.192 | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 88   | kyree.ns.cloudflare.com               | 172.64.35.207   | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 152  | cmcc.877774.xyz                       | 104.16.149.9    | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 308  | cfip.xxxxxxxx.tk                      | 104.25.105.1    | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 312  | cfip.xxxxxxxx.tk                      | 188.114.97.144  | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 323  | www.digitalocean.com                  | 104.19.173.68   | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 350  | ipinfo.in                             | 172.67.198.203  | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 155  | cmcc.877774.xyz                       | 104.16.149.12   | IPv4   | h3   | ✅ 成功 | 128      | cloudflare |
| 342  | www.pcmag.com                         | 104.16.21.118   | IPv4   | h3   | ✅ 成功 | 128      | cloudflare |
| 98   | dylan.ns.cloudflare.com               | 172.64.35.187   | IPv4   | h3   | ✅ 成功 | 129      | cloudflare |
| 121  | bestcf.030101.xyz                     | 104.19.157.251  | IPv4   | h3   | ✅ 成功 | 129      | cloudflare |
| 193  | moura.ns.cloudflare.com               | 172.64.35.217   | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 241  | singapore.com                         | 104.26.13.140   | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 304  | cfip.xxxxxxxx.tk                      | 190.93.247.169  | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 412  | 198.41.208.15                         | 198.41.208.15   | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 32   | comicabc.com                          | 104.21.64.10    | IPv4   | h3   | ✅ 成功 | 131      | cloudflare |
| 42   | pranab.ns.cloudflare.com              | 162.159.44.199  | IPv4   | h3   | ✅ 成功 | 131      | cloudflare |
| 93   | freeyx.cloudflare88.eu.org            | 141.101.120.159 | IPv4   | h3   | ✅ 成功 | 131      | cloudflare |
| 292  | gamer.com.tw                          | 104.18.2.197    | IPv4   | h3   | ✅ 成功 | 131      | cloudflare |
| 355  | 172.67.110.232                        | 172.67.110.232  | IPv4   | h3   | ✅ 成功 | 131      | cloudflare |
| 389  | lewis.ns.cloudflare.com               | 108.162.195.159 | IPv4   | h3   | ✅ 成功 | 132      | cloudflare |
| 404  | www.wto.org                           | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 132      | cloudflare |
| 422  | 172.64.48.226                         | 172.64.48.226   | IPv4   | h3   | ✅ 成功 | 132      | cloudflare |
| 89   | kyree.ns.cloudflare.com               | 162.159.44.207  | IPv4   | h3   | ✅ 成功 | 133      | cloudflare |
| 132  | braden.ns.cloudflare.com              | 162.159.44.169  | IPv4   | h3   | ✅ 成功 | 133      | cloudflare |
| 41   | pranab.ns.cloudflare.com              | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 134      | cloudflare |
| 201  | rustam.ns.cloudflare.com              | 162.159.44.148  | IPv4   | h3   | ✅ 成功 | 134      | cloudflare |
| 334  | huxley.ns.cloudflare.com              | 162.159.44.188  | IPv4   | h3   | ✅ 成功 | 135      | cloudflare |
| 144  | cmcc.877774.xyz                       | 104.16.149.1    | IPv4   | h3   | ✅ 成功 | 136      | cloudflare |
| 149  | cmcc.877774.xyz                       | 104.16.149.6    | IPv4   | h3   | ✅ 成功 | 136      | cloudflare |
| 171  | bowen.ns.cloudflare.com               | 162.159.44.83   | IPv4   | h3   | ✅ 成功 | 136      | cloudflare |
| 382  | icook.hk                              | 104.21.90.210   | IPv4   | h3   | ✅ 成功 | 136      | cloudflare |
| 4    | 172.64.154.18                         | 172.64.154.18   | IPv4   | h3   | ✅ 成功 | 137      | cloudflare |
| 13   | ct.877774.xyz                         | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 137      | cloudflare |
| 48   | cris.ns.cloudflare.com                | 162.159.44.202  | IPv4   | h3   | ✅ 成功 | 137      | cloudflare |
| 116  | cf.zhetengsha.eu.org                  | 172.64.152.241  | IPv4   | h3   | ✅ 成功 | 137      | cloudflare |
| 198  | www.ipchicken.com                     | 104.26.7.112    | IPv4   | h3   | ✅ 成功 | 138      | cloudflare |
| 218  | benedict.ns.cloudflare.com            | 162.159.44.205  | IPv4   | h3   | ✅ 成功 | 138      | cloudflare |
| 317  | otto.ns.cloudflare.com                | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 138      | cloudflare |
| 150  | cmcc.877774.xyz                       | 104.16.149.7    | IPv4   | h3   | ✅ 成功 | 139      | cloudflare |
| 434  | 172.64.229.7                          | 172.64.229.7    | IPv4   | h3   | ✅ 成功 | 139      | cloudflare |
| 16   | ct.877774.xyz                         | 172.64.229.217  | IPv4   | h3   | ✅ 成功 | 141      | cloudflare |
| 238  | whatismyipaddress.com                 | 104.19.222.79   | IPv4   | h3   | ✅ 成功 | 141      | cloudflare |
| 396  | japan.com                             | 104.26.5.60     | IPv4   | h3   | ✅ 成功 | 142      | cloudflare |
| 318  | otto.ns.cloudflare.com                | 162.159.44.135  | IPv4   | h3   | ✅ 成功 | 143      | cloudflare |
| 349  | ipinfo.in                             | 104.21.21.129   | IPv4   | h3   | ✅ 成功 | 144      | cloudflare |
| 21   | craig.ns.cloudflare.com               | 162.159.44.192  | IPv4   | h3   | ✅ 成功 | 145      | cloudflare |
| 160  | cmcc.877774.xyz                       | 104.16.148.4    | IPv4   | h3   | ✅ 成功 | 145      | cloudflare |
| 475  | 104.16.65.1                           | 104.16.65.1     | IPv4   | h3   | ✅ 成功 | 147      | cloudflare |
| 402  | eur.877774.xyz                        | 104.21.47.209   | IPv4   | h3   | ✅ 成功 | 149      | cloudflare |
| 491  | trevor.ns.cloudflare.com              | 162.159.44.154  | IPv4   | h3   | ✅ 成功 | 151      | cloudflare |
| 435  | 104.18.37.13                          | 104.18.37.13    | IPv4   | h3   | ✅ 成功 | 152      | cloudflare |
| 154  | cmcc.877774.xyz                       | 104.16.149.11   | IPv4   | h3   | ✅ 成功 | 154      | cloudflare |
| 445  | 104.17.139.37                         | 104.17.139.37   | IPv4   | h3   | ✅ 成功 | 154      | cloudflare |
| 7    | sullivan.ns.cloudflare.com            | 172.64.35.161   | IPv4   | h3   | ✅ 成功 | 155      | cloudflare |
| 261  | julio.ns.cloudflare.com               | 162.159.44.209  | IPv4   | h3   | ✅ 成功 | 155      | cloudflare |
| 234  | ip.gs                                 | 172.67.160.28   | IPv4   | h3   | ✅ 成功 | 156      | cloudflare |
| 99   | dylan.ns.cloudflare.com               | 162.159.44.187  | IPv4   | h3   | ✅ 成功 | 158      | cloudflare |
| 187  | time.is                               | 172.67.68.157   | IPv4   | h3   | ✅ 成功 | 160      | cloudflare |
| 35   | 172.67.243.218                        | 172.67.243.218  | IPv4   | h3   | ✅ 成功 | 161      | cloudflare |
| 333  | huxley.ns.cloudflare.com              | 108.162.195.188 | IPv4   | h3   | ✅ 成功 | 162      | cloudflare |
| 388  | lewis.ns.cloudflare.com               | 172.64.35.159   | IPv4   | h3   | ✅ 成功 | 162      | cloudflare |
| 345  | 172.67.75.172                         | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 164      | cloudflare |
| 141  | cmcc.877774.xyz                       | 104.16.148.11   | IPv4   | h3   | ✅ 成功 | 167      | cloudflare |
| 192  | moura.ns.cloudflare.com               | 162.159.44.217  | IPv4   | h3   | ✅ 成功 | 168      | cloudflare |
| 103  | www.whatismyip.com                    | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 171      | cloudflare |
| 109  | www.visa.cn                           | 162.159.152.2   | IPv4   | h3   | ✅ 成功 | 176      | cloudflare |
| 390  | lewis.ns.cloudflare.com               | 162.159.44.159  | IPv4   | h3   | ✅ 成功 | 186      | cloudflare |
| 276  | uriah.ns.cloudflare.com               | 162.159.44.194  | IPv4   | h3   | ✅ 成功 | 197      | cloudflare |
| 288  | tasteatlas.com                        | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 201      | cloudflare |
| 6    | sullivan.ns.cloudflare.com            | 162.159.44.161  | IPv4   | h3   | ✅ 成功 | 254      | cloudflare |
| 327  | damien.ns.cloudflare.com              | 162.159.44.168  | IPv4   | h3   | ✅ 成功 | 332      | cloudflare |
| 303  | 141.147.185.63                        | 141.147.185.63  | IPv4   | h2   | ✅ 成功 | 339      | cloudflare |
| 217  | benedict.ns.cloudflare.com            | 108.162.195.205 | IPv4   | h3   | ✅ 成功 | 492      | cloudflare |
| 465  | 34.143.159.175                        | 34.143.159.175  | IPv4   | h2   | ✅ 成功 | 502      | cloudflare |
| 503  | 3.0.50.69                             | 3.0.50.69       | IPv4   | h2   | ✅ 成功 | 698      | cloudflare |
| 504  | 168.138.165.174                       | 168.138.165.174 | IPv4   | h2   | ✅ 成功 | 1500     | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 81 条记录
- **正常 (100-200ms)**: 236 条记录
- **慢 (200-500ms)**: 5 条记录
- **很慢 (>500ms)**: 3 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 3 次
- **IPv6 失败**: 178 次

### 按协议统计

- **none**: 181 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
