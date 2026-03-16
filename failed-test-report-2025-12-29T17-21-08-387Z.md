# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/29 17:21:08
- **数据来源**: connectivity_results-20251229-172107.json
- **总测试数**: 498
- **失败测试数**: 181
- **成功测试数**: 317
- **失败率**: 36.35%
- **平均延迟**: 114.42ms
- **最小延迟**: 63ms
- **最大延迟**: 1490ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/29 17:21:08
- **IP地址**: 52.173.182.162
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
| 489  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 497  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |
| 498  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |

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
trevor.ns.cloudflare.com (3次), wilson.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 279  | www.glassdoor.com                     | 104.16.25.46    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 280  | ip.sb                                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 377  | lewis.ns.cloudflare.com               | 172.64.35.159   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 426  | 162.159.136.89                        | 162.159.136.89  | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 259  | time.is                               | 172.67.68.157   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 240  | fbi.gov                               | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 414  | eur.877774.xyz                        | 104.21.47.209   | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 387  | 172.67.181.209                        | 172.67.181.209  | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 406  | stock.hostmonit.com                   | 172.67.187.251  | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 457  | uriah.ns.cloudflare.com               | 172.64.35.194   | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 72   | ipv4.ip.sb                            | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 110  | cf.877771.xyz                         | 172.67.152.183  | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 133  | 172.67.243.218                        | 172.67.243.218  | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 214  | benedict.ns.cloudflare.com            | 108.162.195.205 | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 319  | 456.cloudflare.182682.xyz             | 172.67.75.208   | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 334  | www.udemy.com                         | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 65   | iplocation.io                         | 104.26.10.222   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 103  | www.hugedomains.com                   | 172.67.70.191   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 258  | time.is                               | 104.26.13.54    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 313  | ip.gs                                 | 172.67.160.28   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 463  | 104.17.139.37                         | 104.17.139.37   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 466  | 104.19.212.207                        | 104.19.212.207  | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 30   | www.whatismyip.com                    | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 61   | www.gov.ua                            | 172.67.209.127  | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 316  | 162.159.133.85                        | 162.159.133.85  | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 397  | 198.41.208.15                         | 198.41.208.15   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 422  | 104.18.42.26                          | 104.18.42.26    | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 490  | cfip.xxxxxxxx.tk                      | 188.114.96.125  | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 70   | ipv4.ip.sb                            | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 94   | huxley.ns.cloudflare.com              | 172.64.35.188   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 432  | 172.64.229.7                          | 172.64.229.7    | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 6    | www.ipget.net                         | 172.67.207.26   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 20   | www.pcmag.com                         | 104.16.21.118   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 124  | www.okcupid.com                       | 104.16.144.63   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 257  | time.is                               | 104.26.12.54    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 363  | 104.18.78.214                         | 104.18.78.214   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 473  | 104.16.105.166                        | 104.16.105.166  | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 173  | cmcc.877774.xyz                       | 104.16.148.9    | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 201  | kyree.ns.cloudflare.com               | 108.162.195.207 | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 347  | 198.62.62.4                           | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 373  | ifconfig.co                           | 172.67.168.106  | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 411  | 162.159.24.131                        | 162.159.24.131  | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 54   | steamdb.info                          | 172.66.175.250  | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 64   | iplocation.io                         | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 340  | gamer.com.tw                          | 104.18.2.197    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 353  | 172.64.35.24                          | 172.64.35.24    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 371  | cfip.1323123.xyz                      | 104.16.133.220  | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 392  | www.wto.org                           | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 49   | ipinfo.in                             | 104.21.21.129   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 187  | cmcc.877774.xyz                       | 104.16.149.10   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 189  | dylan.ns.cloudflare.com               | 108.162.195.187 | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 207  | 104.16.223.179                        | 104.16.223.179  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 278  | www.glassdoor.com                     | 104.17.64.70    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 300  | palera.in                             | 104.21.58.72    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 324  | whatismyipaddress.com                 | 104.19.222.79   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 434  | dnschecker.org                        | 104.26.7.89     | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 441  | julio.ns.cloudflare.com               | 108.162.195.209 | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 57   | 104.18.39.196                         | 104.18.39.196   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 127  | craig.ns.cloudflare.com               | 172.64.35.192   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 232  | cf.zhetengsha.eu.org                  | 104.18.35.15    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 235  | xn--b6gac.eu.org                      | 188.114.97.3    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 58   | 104.16.45.84                          | 104.16.45.84    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 417  | www.csgo.com                          | 195.85.59.95    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 431  | 162.159.58.65                         | 162.159.58.65   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 142  | na.877774.xyz                         | 104.18.187.25   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 348  | www.digitalocean.com                  | 104.19.173.68   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 383  | japan.com                             | 104.26.4.60     | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 409  | 162.159.61.183                        | 162.159.61.183  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 15   | trevor.ns.cloudflare.com              | 172.64.35.154   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 75   | 172.67.110.232                        | 172.67.110.232  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 84   | ct.877774.xyz                         | 172.64.229.217  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 122  | www.okcupid.com                       | 104.16.223.254  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 181  | cmcc.877774.xyz                       | 104.16.149.4    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 269  | moura.ns.cloudflare.com               | 108.162.195.217 | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 339  | gamer.com.tw                          | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 454  | 104.19.148.121                        | 104.19.148.121  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 226  | asia.877774.xyz                       | 104.17.139.62   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 448  | 104.26.3.162                          | 104.26.3.162    | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 472  | 104.31.16.158                         | 104.31.16.158   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 483  | 104.17.142.212                        | 104.17.142.212  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 53   | steamdb.info                          | 104.20.34.212   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 276  | www.ipchicken.com                     | 104.26.7.112    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 428  | cf.877774.xyz                         | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 488  | cfip.xxxxxxxx.tk                      | 198.41.214.141  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 493  | cfip.xxxxxxxx.tk                      | 190.93.246.67   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 304  | 104.17.68.85                          | 104.17.68.85    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 307  | singapore.com                         | 104.26.12.140   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 362  | 104.26.13.31                          | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 166  | cmcc.877774.xyz                       | 104.16.148.2    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 218  | 104.17.79.11                          | 104.17.79.11    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 413  | eur.877774.xyz                        | 104.21.29.164   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 420  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 447  | 162.159.128.253                       | 162.159.128.253 | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 5    | www.ipget.net                         | 104.21.15.212   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 131  | www.visa.cn                           | 162.159.152.2   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 140  | na.877774.xyz                         | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 281  | ip.sb                                 | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 381  | japan.com                             | 104.26.5.60     | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 59   | 172.67.75.172                         | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 271  | moura.ns.cloudflare.com               | 172.64.35.217   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 277  | www.ipchicken.com                     | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 13   | trevor.ns.cloudflare.com              | 108.162.195.154 | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 123  | www.okcupid.com                       | 104.16.239.254  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 372  | 104.19.223.58                         | 104.19.223.58   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 455  | uriah.ns.cloudflare.com               | 108.162.195.194 | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 470  | www.visa.com.hk                       | 104.18.20.69    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 494  | cfip.xxxxxxxx.tk                      | 104.16.241.229  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 10   | comicabc.com                          | 104.21.64.10    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 87   | 104.18.254.88                         | 104.18.254.88   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 115  | sullivan.ns.cloudflare.com            | 108.162.195.161 | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 82   | ct.877774.xyz                         | 172.64.229.185  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 180  | cmcc.877774.xyz                       | 104.16.149.3    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 335  | www.udemy.com                         | 104.16.143.237  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 343  | icook.tw                              | 172.66.158.115  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 104  | www.hugedomains.com                   | 104.26.6.37     | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 288  | rustam.ns.cloudflare.com              | 172.64.35.148   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 135  | yx-auto.pages.dev                     | 172.66.44.144   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 39   | cloudflare.182682.xyz                 | 104.21.227.134  | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 165  | cmcc.877774.xyz                       | 104.16.148.1    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 174  | cmcc.877774.xyz                       | 104.16.148.10   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 251  | saas.sin.fan                          | 162.159.36.20   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 331  | tasteatlas.com                        | 104.17.37.105   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 354  | otto.ns.cloudflare.com                | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 451  | 104.18.89.52                          | 104.18.89.52    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 485  | 104.16.65.1                           | 104.16.65.1     | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 2    | 104.18.14.76                          | 104.18.14.76    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 151  | cris.ns.cloudflare.com                | 172.64.35.202   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 179  | cmcc.877774.xyz                       | 104.16.149.2    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 231  | cf.zhetengsha.eu.org                  | 172.64.152.241  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 254  | cf.090227.xyz                         | 172.64.152.241  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 423  | 104.17.69.244                         | 104.17.69.244   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 81   | ct.877774.xyz                         | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 170  | cmcc.877774.xyz                       | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 171  | cmcc.877774.xyz                       | 104.16.148.7    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 188  | cmcc.877774.xyz                       | 104.16.149.11   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 282  | ip.sb                                 | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 89   | icook.hk                              | 188.114.96.3    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 191  | dylan.ns.cloudflare.com               | 172.64.35.187   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 301  | palera.in                             | 172.67.157.122  | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 433  | 104.26.5.134                          | 104.26.5.134    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 36   | cloudflare.182682.xyz                 | 104.21.224.5    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 38   | cloudflare.182682.xyz                 | 104.17.25.173   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 330  | tasteatlas.com                        | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 427  | cf.877774.xyz                         | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 467  | 104.18.255.167                        | 104.18.255.167  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 375  | lewis.ns.cloudflare.com               | 108.162.195.159 | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 440  | 162.159.137.204                       | 162.159.137.204 | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 482  | 104.17.167.134                        | 104.17.167.134  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 83   | ct.877774.xyz                         | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 172  | cmcc.877774.xyz                       | 104.16.148.8    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 419  | www.7749tv.com                        | 104.19.133.4    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 453  | 104.18.166.129                        | 104.18.166.129  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 462  | 104.18.151.172                        | 104.18.151.172  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 31   | www.whatismyip.com                    | 172.67.69.129   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 105  | www.hugedomains.com                   | 104.26.7.37     | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 183  | cmcc.877774.xyz                       | 104.16.149.6    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 308  | singapore.com                         | 172.67.75.194   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 197  | decker.ns.cloudflare.com              | 172.64.35.155   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 312  | ip.gs                                 | 104.21.14.176   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 320  | 456.cloudflare.182682.xyz             | 104.26.8.160    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 449  | 104.26.8.117                          | 104.26.8.117    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 92   | huxley.ns.cloudflare.com              | 108.162.195.188 | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 159  | toy-people.com                        | 104.26.3.36     | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 184  | cmcc.877774.xyz                       | 104.16.149.7    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 236  | xn--b6gac.eu.org                      | 188.114.96.3    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 85   | ct.877774.xyz                         | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 164  | cmcc.877774.xyz                       | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 404  | 162.159.140.85                        | 162.159.140.85  | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 24   | wilson.ns.cloudflare.com              | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 185  | cmcc.877774.xyz                       | 104.16.149.8    | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 209  | zread.ai                              | 104.21.76.240   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 469  | 104.19.154.200                        | 104.19.154.200  | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 102  | www.4chan.org                         | 104.16.228.229  | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 225  | asia.877774.xyz                       | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 252  | saas.sin.fan                          | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 344  | icook.tw                              | 104.20.28.74    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 382  | japan.com                             | 172.67.70.92    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 3    | 172.64.154.18                         | 172.64.154.18   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 88   | icook.hk                              | 188.114.97.3    | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 306  | singapore.com                         | 104.26.13.140   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 23   | wilson.ns.cloudflare.com              | 172.64.35.110   | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 101  | www.4chan.org                         | 104.16.229.229  | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 195  | decker.ns.cloudflare.com              | 108.162.195.155 | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 71   | ipv4.ip.sb                            | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 77   | shopify.com                           | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 79   | ct.877774.xyz                         | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 342  | 172.67.79.211                         | 172.67.79.211   | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 468  | 104.19.220.22                         | 104.19.220.22   | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 76   | 104.26.6.112                          | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 286  | rustam.ns.cloudflare.com              | 108.162.195.148 | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 398  | abdullah.ns.cloudflare.com            | 108.162.195.203 | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 412  | 172.64.48.226                         | 172.64.48.226   | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 325  | whatismyipaddress.com                 | 104.19.223.79   | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 395  | 172.64.82.114                         | 172.64.82.114   | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 66   | iplocation.io                         | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 143  | freeyx.cloudflare88.eu.org            | 141.101.120.176 | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 169  | cmcc.877774.xyz                       | 104.16.148.5    | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 178  | cmcc.877774.xyz                       | 104.16.149.1    | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 244  | 172.64.151.55                         | 172.64.151.55   | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 349  | www.digitalocean.com                  | 104.19.174.68   | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 145 条记录
- **正常 (100-200ms)**: 55 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

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
