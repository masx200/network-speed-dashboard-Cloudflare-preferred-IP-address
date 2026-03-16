# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/30 01:10:28
- **数据来源**: connectivity_results-20251230-011027.json
- **总测试数**: 507
- **失败测试数**: 183
- **成功测试数**: 324
- **失败率**: 36.09%
- **平均延迟**: 121.52ms
- **最小延迟**: 50ms
- **最大延迟**: 748ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/30 01:10:28
- **IP地址**: 48.211.212.212
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

- **网络不可达: 网络不可达**: 179 次 (97.8%)
- **连接超时: I/O超时**: 4 次 (2.2%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (179 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
| ---- | --------- | ------ | ------ | ---- | ------ | -------- | ------ | -------- |

#### 连接超时: I/O超时 (4 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 362  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |
| 500  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 505  | cfip.xxxxxxxx.tk | 104.20.255.53  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.20.255.53:443: i/o timeout  |
| 507  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 179 次 (97.8%)
- **连接超时**: 4 次 (2.2%)

#### 错误模式分析

**超时集中度分析**: 共有 4 次超时，主要集中在IP段 172.67（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 183 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 4 次，IPv6失败 179 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次),
trevor.ns.cloudflare.com (3次), wilson.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                  | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | -------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 71   | 103.160.204.59             | 103.160.204.59  | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 127  | www.okcupid.com            | 104.16.144.63   | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 420  | abdullah.ns.cloudflare.com | 172.64.35.203   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 488  | eur.877774.xyz             | 104.21.29.164   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 278  | cf.zhetengsha.eu.org       | 172.64.152.241  | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 399  | www.wto.org                | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 489  | 104.16.65.1                | 104.16.65.1     | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 439  | 173.245.49.194             | 173.245.49.194  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 411  | 172.67.181.209             | 172.67.181.209  | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 470  | 104.17.69.244              | 104.17.69.244   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 331  | ashton.ns.cloudflare.com   | 172.64.35.173   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 279  | cf.zhetengsha.eu.org       | 104.18.35.15    | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 370  | 104.17.142.12              | 104.17.142.12   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 438  | 104.18.255.167             | 104.18.255.167  | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 471  | 104.31.16.158              | 104.31.16.158   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 403  | stock.hostmonit.com        | 104.21.7.193    | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 390  | ifconfig.co                | 172.67.168.106  | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 451  | 104.18.81.19               | 104.18.81.19    | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 255  | cf.090227.xyz              | 104.18.43.174   | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 365  | gamer.com.tw               | 104.18.2.197    | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 292  | 162.159.36.104             | 162.159.36.104  | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 55   | 104.18.37.13               | 104.18.37.13    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 28   | cloudflare.182682.xyz      | 104.17.25.173   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 197  | 104.16.223.179             | 104.16.223.179  | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 445  | 104.26.5.134               | 104.26.5.134    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 57   | ct.877774.xyz              | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 371  | 172.67.79.211              | 172.67.79.211   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 117  | www.4chan.org              | 104.16.229.229  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 395  | japan.com                  | 104.26.4.60     | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 63   | ct.877774.xyz              | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 340  | dnschecker.org             | 104.26.7.89     | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 498  | cfip.xxxxxxxx.tk           | 104.16.232.223  | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 100  | cmcc.877774.xyz            | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 139  | iplocation.io              | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 302  | palera.in                  | 172.67.157.122  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 355  | uriah.ns.cloudflare.com    | 172.64.35.194   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 298  | ip.sb                      | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 376  | 198.62.62.4                | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 487  | eur.877774.xyz             | 104.21.26.150   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 49   | 104.18.39.196              | 104.18.39.196   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 440  | www.csgo.com               | 195.85.59.95    | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 81   | icook.hk                   | 172.67.161.104  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 393  | japan.com                  | 172.67.70.92    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 26   | cloudflare.182682.xyz      | 104.21.227.134  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 113  | cmcc.877774.xyz            | 104.16.149.10   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 433  | 162.159.140.85             | 162.159.140.85  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 474  | 104.17.142.212             | 104.17.142.212  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 493  | cfip.xxxxxxxx.tk           | 188.114.96.125  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 497  | cfip.xxxxxxxx.tk           | 104.27.21.118   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 499  | cfip.xxxxxxxx.tk           | 190.93.244.201  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 132  | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 228  | zread.ai                   | 172.67.202.78   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 37   | wilson.ns.cloudflare.com   | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 60   | ct.877774.xyz              | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 108  | cmcc.877774.xyz            | 104.16.149.12   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 145  | craig.ns.cloudflare.com    | 172.64.35.192   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 434  | 172.64.52.127              | 172.64.52.127   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 101  | cmcc.877774.xyz            | 104.16.148.7    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 130  | 172.67.120.0               | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 295  | 104.17.68.85               | 104.17.68.85    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 372  | cf.877774.xyz              | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 25   | cloudflare.182682.xyz      | 104.21.224.5    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 56   | ct.877774.xyz              | 172.64.229.185  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 249  | bowen.ns.cloudflare.com    | 172.64.35.83    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 268  | time.is                    | 104.26.13.54    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 436  | 172.64.48.226              | 172.64.48.226   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 485  | 104.18.189.153             | 104.18.189.153  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 503  | cfip.xxxxxxxx.tk           | 190.93.246.67   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 42   | ipinfo.in                  | 172.67.198.203  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 72   | shopify.com                | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 96   | cmcc.877774.xyz            | 104.16.149.2    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 109  | cmcc.877774.xyz            | 104.16.148.4    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 467  | 104.19.154.200             | 104.19.154.200  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 88   | cmcc.877774.xyz            | 104.16.148.5    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 180  | cris.ns.cloudflare.com     | 108.162.195.202 | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 204  | 104.17.79.11               | 104.17.79.11    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 443  | 162.159.136.89             | 162.159.136.89  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 496  | cfip.xxxxxxxx.tk           | 104.16.241.229  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 4    | www.pcmag.com              | 104.16.20.118   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 52   | ipv4.ip.sb                 | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 168  | www.visa.cn                | 162.159.153.2   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 192  | toy-people.com             | 104.26.2.36     | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 449  | 162.159.128.253            | 162.159.128.253 | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 469  | 104.19.212.207             | 104.19.212.207  | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 7    | comicabc.com               | 172.67.174.21   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 8    | comicabc.com               | 104.21.64.10    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 54   | ipv4.ip.sb                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 91   | cmcc.877774.xyz            | 104.16.148.2    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 137  | iplocation.io              | 104.26.10.222   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 286  | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 373  | cf.877774.xyz              | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 45   | www.udemy.com              | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 62   | ct.877774.xyz              | 172.64.229.173  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 90   | cmcc.877774.xyz            | 104.16.149.1    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 98   | cmcc.877774.xyz            | 104.16.148.9    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 236  | braden.ns.cloudflare.com   | 108.162.195.169 | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 450  | 104.26.3.162               | 104.26.3.162    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 46   | www.udemy.com              | 104.16.143.237  | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 313  | singapore.com              | 104.26.12.140   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 383  | damien.ns.cloudflare.com   | 108.162.195.168 | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 492  | cfip.xxxxxxxx.tk           | 190.93.247.169  | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 58   | ct.877774.xyz              | 172.64.229.217  | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 110  | cmcc.877774.xyz            | 104.16.148.11   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 388  | 104.18.78.214              | 104.18.78.214   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 392  | 104.19.223.58              | 104.19.223.58   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 441  | www.csgo.com               | 195.85.59.161   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 464  | 104.17.162.3               | 104.17.162.3    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 490  | www.7749tv.com             | 104.19.133.4    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 191  | toy-people.com             | 104.26.3.36     | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 407  | icook.tw                   | 104.20.28.74    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 495  | cfip.xxxxxxxx.tk           | 104.18.228.35   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 154  | pranab.ns.cloudflare.com   | 172.64.35.199   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 283  | www.ipchicken.com          | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 129  | www.okcupid.com            | 104.16.239.254  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 171  | www.whatismyip.com         | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 404  | stock.hostmonit.com        | 172.67.187.251  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 413  | otto.ns.cloudflare.com     | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 446  | 172.64.229.7               | 172.64.229.7    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 455  | 104.18.166.129             | 104.18.166.129  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 486  | eur.877774.xyz             | 104.21.47.209   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 73   | 104.18.254.88              | 104.18.254.88   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 95   | cmcc.877774.xyz            | 104.16.149.7    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 102  | cmcc.877774.xyz            | 104.16.149.3    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 126  | www.okcupid.com            | 104.18.160.63   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 128  | www.okcupid.com            | 104.16.223.254  | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 121  | www.hugedomains.com        | 104.26.7.37     | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 133  | sullivan.ns.cloudflare.com | 172.64.35.161   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 408  | icook.tw                   | 172.66.158.115  | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 456  | 198.41.208.224             | 198.41.208.224  | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 466  | 104.18.151.172             | 104.18.151.172  | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 61   | ct.877774.xyz              | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 92   | cmcc.877774.xyz            | 104.16.149.9    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 273  | rustam.ns.cloudflare.com   | 172.64.35.148   | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 297  | ip.sb                      | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 425  | www.digitalocean.com       | 104.19.174.68   | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 506  | cfip.xxxxxxxx.tk           | 104.17.127.110  | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 17   | trevor.ns.cloudflare.com   | 172.64.35.154   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 50   | 104.16.45.84               | 104.16.45.84    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 89   | cmcc.877774.xyz            | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 112  | cmcc.877774.xyz            | 104.16.149.4    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 115  | cu.877774.xyz              | 172.64.145.202  | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 119  | www.hugedomains.com        | 172.67.70.191   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 323  | whatismyipaddress.com      | 104.19.223.79   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 143  | craig.ns.cloudflare.com    | 108.162.195.192 | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 458  | lewis.ns.cloudflare.com    | 172.64.35.159   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 504  | cfip.xxxxxxxx.tk           | 104.21.91.19    | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 36   | wilson.ns.cloudflare.com   | 172.64.35.110   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 51   | 172.67.75.172              | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 307  | 456.cloudflare.182682.xyz  | 104.26.9.160    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 308  | 456.cloudflare.182682.xyz  | 172.67.75.208   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 384  | damien.ns.cloudflare.com   | 162.159.44.168  | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 460  | lewis.ns.cloudflare.com    | 162.159.44.159  | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 468  | 104.19.220.22              | 104.19.220.22   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 3    | www.pcmag.com              | 104.16.21.118   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 107  | cmcc.877774.xyz            | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 224  | xn--b6gac.eu.org           | 172.67.153.253  | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 332  | ashton.ns.cloudflare.com   | 108.162.195.173 | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 457  | 104.19.148.121             | 104.19.148.121  | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 12   | www.ipget.net              | 104.21.15.212   | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 200  | decker.ns.cloudflare.com   | 172.64.35.155   | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 225  | xn--b6gac.eu.org           | 104.21.90.78    | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 338  | dnschecker.org             | 104.26.6.89     | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 125  | www.okcupid.com            | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 360  | www.visa.com.hk            | 104.18.21.69    | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 378  | 172.64.35.24               | 172.64.35.24    | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 381  | 104.26.13.31               | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 412  | otto.ns.cloudflare.com     | 172.64.35.135   | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 59   | ct.877774.xyz              | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 84   | www.gov.ua                 | 104.21.23.72    | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 94   | cmcc.877774.xyz            | 104.16.148.12   | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 104  | cmcc.877774.xyz            | 104.16.149.6    | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 114  | cu.877774.xyz              | 104.18.42.54    | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 138  | iplocation.io              | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 502  | cfip.xxxxxxxx.tk           | 188.114.97.144  | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 53   | ipv4.ip.sb                 | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 240  | fbi.gov                    | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 432  | 198.41.208.15              | 198.41.208.15   | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 22   | cf.0sm.com                 | 172.67.187.145  | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 156  | pranab.ns.cloudflare.com   | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 211  | freeyx.cloudflare88.eu.org | 141.101.120.156 | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 232  | 172.64.151.55              | 172.64.151.55   | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 64   | steamdb.info               | 104.20.34.212   | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 118  | www.4chan.org              | 104.16.228.229  | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 162  | www.visa.com.sg            | 104.18.12.229   | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 193  | toy-people.com             | 172.67.72.18    | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 400  | www.wto.org                | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 428  | 104.19.175.123             | 104.19.175.123  | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 21   | cf.0sm.com                 | 104.21.7.133    | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 172  | www.whatismyip.com         | 104.26.13.23    | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 177  | na.877774.xyz              | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 293  | www.glassdoor.com          | 104.17.64.70    | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 452  | 104.26.8.117               | 104.26.8.117    | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 99   | cmcc.877774.xyz            | 104.16.148.8    | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 223  | asia.877774.xyz            | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 254  | cf.090227.xyz              | 172.64.144.82   | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 435  | 162.159.61.183             | 162.159.61.183  | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 178  | na.877774.xyz              | 104.18.38.235   | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 418  | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 41   | ipinfo.in                  | 104.21.21.129   | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 105  | cmcc.877774.xyz            | 104.16.148.1    | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 111 条记录
- **正常 (100-200ms)**: 89 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 4 次
- **IPv6 失败**: 179 次

### 按协议统计

- **none**: 183 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
