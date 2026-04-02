# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/30 01:18:55
- **数据来源**: connectivity_results-20251229-171842.json
- **总测试数**: 414
- **失败测试数**: 15
- **成功测试数**: 399
- **失败率**: 3.62%
- **平均延迟**: 1030.38ms
- **最小延迟**: 372ms
- **最大延迟**: 9779ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/30 01:18:55
- **IP地址**: 2408:840d:8830:293b:cc53:eeff:fe44:c8ad
- **国家/地区**: China (CN)
- **ASN**: AS17621
- **网络组织**: China Unicom Shanghai network
- **网络域名**: chinaunicom.cn
- **大洲**: Asia (AS)
- **地理坐标**: 34.7732, 113.722
- **时区**: Asia/Shanghai
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: 上下文超时**: 9 次 (60.0%)
- **连接超时: I/O超时**: 6 次 (40.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: 上下文超时 (9 次测试)

| 序号 | 主机/域名                  | 目标IP        | IP版本  | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                                              |
| ---- | -------------------------- | ------------- | ------- | ---- | ------ | -------- | ------ | ----------------------------------------------------------------------------------------------------- |
| 105  | www.hugedomains.com        | Unknown       | Unknown | none | N/A    | 0        | N/A    | DNS解析失败: Post "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": context deadline exceeded |
| 106  | www.ipget.net              | 172.67.207.26 | IPv4    | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded                   |
| 156  | decker.ns.cloudflare.com   | Unknown       | Unknown | none | N/A    | 0        | N/A    | DNS解析失败: Post "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": context deadline exceeded |
| 217  | shopify.com                | Unknown       | Unknown | none | N/A    | 0        | N/A    | DNS解析失败: Post "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": context deadline exceeded |
| 281  | stock.hostmonit.com        | Unknown       | Unknown | none | N/A    | 0        | N/A    | DNS解析失败: Post "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": context deadline exceeded |
| 286  | abdullah.ns.cloudflare.com | Unknown       | Unknown | none | N/A    | 0        | N/A    | DNS解析失败: Post "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": context deadline exceeded |
| 358  | ip.sb                      | Unknown       | Unknown | none | N/A    | 0        | N/A    | DNS解析失败: Post "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": context deadline exceeded |
| 409  | www.visa.com.hk            | Unknown       | Unknown | none | N/A    | 0        | N/A    | DNS解析失败: Post "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": context deadline exceeded |
| 412  | www.udemy.com              | Unknown       | Unknown | none | N/A    | 0        | N/A    | DNS解析失败: Post "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": context deadline exceeded |

#### 连接超时: I/O超时 (6 次测试)

| 序号 | 主机/域名             | 目标IP                  | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                            |
| ---- | --------------------- | ----------------------- | ------ | ---- | ------ | -------- | ------ | --------------------------------------------------- |
| 69   | cfip.xxxxxxxx.tk      | 198.41.212.130          | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout            |
| 95   | cf.877774.xyz         | cf.877774.xyz           | IPv4   | none | N/A    | 0        | N/A    | dial tcp cf.877774.xyz:443: i/o timeout             |
| 122  | cloudflare.182682.xyz | 6666.cloudflare.182682.xyz | IPv6   | none | N/A    | 0        | N/A    | dial tcp 6666.cloudflare.182682.xyz:443: i/o timeout |
| 280  | 3.0.50.69             | 3.0.50.69               | IPv4   | none | N/A    | 0        | N/A    | dial tcp 3.0.50.69:443: i/o timeout                 |
| 320  | 141.147.185.63        | 141.147.185.63          | IPv4   | none | N/A    | 0        | N/A    | dial tcp 141.147.185.63:443: i/o timeout            |
| 408  | ct.877774.xyz         | ct.877774.xyz           | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout             |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 15 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 6 次超时，主要集中在IP段 198.41（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 14 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 6 次，IPv6失败 1 次，两种协议都存在问题

---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                               | 目标IP                                  | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | --------------------------------------- | --------------------------------------- | ------ | ---- | ------- | -------- | ---------- |
| 276  | 104.18.151.172                          | 104.18.151.172                          | IPv4   | h3   | ✅ 成功 | 372      | cloudflare |
| 272  | 104.18.89.52                            | 104.18.89.52                            | IPv4   | h3   | ✅ 成功 | 375      | cloudflare |
| 293  | whatismyipaddress.com                   | 104.19.223.79                           | IPv4   | h3   | ✅ 成功 | 379      | cloudflare |
| 74   | 168.138.165.174                         | 168.138.165.174                         | IPv4   | h2   | ✅ 成功 | 389      | cloudflare |
| 271  | 104.18.81.19                            | 104.18.81.19                            | IPv4   | h3   | ✅ 成功 | 393      | cloudflare |
| 67   | cmcc.877774.xyz                         | 104.16.149.10                           | IPv4   | h3   | ✅ 成功 | 403      | cloudflare |
| 279  | 104.19.220.22                           | 104.19.220.22                           | IPv4   | h3   | ✅ 成功 | 408      | cloudflare |
| 292  | whatismyipaddress.com                   | 104.19.222.79                           | IPv4   | h3   | ✅ 成功 | 412      | cloudflare |
| 341  | 104.18.189.153                          | 104.18.189.153                          | IPv4   | h3   | ✅ 成功 | 420      | cloudflare |
| 216  | 104.19.223.58                           | 104.19.223.58                           | IPv4   | h3   | ✅ 成功 | 422      | cloudflare |
| 335  | 104.17.69.244                           | 104.17.69.244                           | IPv4   | h3   | ✅ 成功 | 423      | cloudflare |
| 392  | tasteatlas.com                          | 104.17.36.105                           | IPv4   | h3   | ✅ 成功 | 423      | cloudflare |
| 330  | 104.16.61.163                           | 104.16.61.163                           | IPv4   | h3   | ✅ 成功 | 424      | cloudflare |
| 356  | www.glassdoor.com                       | 104.17.64.70                            | IPv4   | h3   | ✅ 成功 | 424      | cloudflare |
| 68   | 104.18.254.88                           | 104.18.254.88                           | IPv4   | h3   | ✅ 成功 | 425      | cloudflare |
| 90   | www.okcupid.com                         | 104.16.239.254                          | IPv4   | h3   | ✅ 成功 | 429      | cloudflare |
| 56   | cmcc.877774.xyz                         | 104.16.148.12                           | IPv4   | h3   | ✅ 成功 | 431      | cloudflare |
| 149  | 104.17.79.11                            | 104.17.79.11                            | IPv4   | h3   | ✅ 成功 | 431      | cloudflare |
| 274  | 198.41.208.224                          | 198.41.208.224                          | IPv4   | h3   | ✅ 成功 | 431      | cloudflare |
| 71   | cfip.xxxxxxxx.tk                        | 190.93.244.201                          | IPv4   | h3   | ✅ 成功 | 436      | cloudflare |
| 249  | 198.41.208.15                           | 198.41.208.15                           | IPv4   | h3   | ✅ 成功 | 437      | cloudflare |
| 337  | 104.17.167.134                          | 104.17.167.134                          | IPv4   | h3   | ✅ 成功 | 438      | cloudflare |
| 66   | cmcc.877774.xyz                         | 104.16.149.9                            | IPv4   | h3   | ✅ 成功 | 439      | cloudflare |
| 91   | www.okcupid.com                         | 104.16.223.254                          | IPv4   | h3   | ✅ 成功 | 440      | cloudflare |
| 278  | 104.17.139.37                           | 104.17.139.37                           | IPv4   | h3   | ✅ 成功 | 443      | cloudflare |
| 89   | www.okcupid.com                         | 104.17.48.63                            | IPv4   | h3   | ✅ 成功 | 444      | cloudflare |
| 81   | www.4chan.org                           | 104.16.229.229                          | IPv4   | h3   | ✅ 成功 | 445      | cloudflare |
| 353  | 104.17.68.85                            | 104.17.68.85                            | IPv4   | h3   | ✅ 成功 | 447      | cloudflare |
| 170  | asia.877774.xyz                         | 104.17.139.62                           | IPv4   | h3   | ✅ 成功 | 450      | cloudflare |
| 273  | 104.18.166.129                          | 104.18.166.129                          | IPv4   | h3   | ✅ 成功 | 451      | cloudflare |
| 47   | cmcc.877774.xyz                         | 104.16.148.3                            | IPv4   | h3   | ✅ 成功 | 452      | cloudflare |
| 65   | cmcc.877774.xyz                         | 104.16.149.8                            | IPv4   | h3   | ✅ 成功 | 455      | cloudflare |
| 138  | na.877774.xyz                           | 104.18.187.25                           | IPv4   | h3   | ✅ 成功 | 455      | cloudflare |
| 212  | www.digitalocean.com                    | 104.19.173.68                           | IPv4   | h3   | ✅ 成功 | 457      | cloudflare |
| 43   | cmcc.877774.xyz                         | 104.16.149.12                           | IPv4   | h3   | ✅ 成功 | 463      | cloudflare |
| 338  | 104.18.223.253                          | 104.18.223.253                          | IPv4   | h3   | ✅ 成功 | 464      | cloudflare |
| 355  | www.glassdoor.com                       | 104.16.25.46                            | IPv4   | h3   | ✅ 成功 | 468      | cloudflare |
| 235  | 104.19.175.123                          | 104.19.175.123                          | IPv4   | h3   | ✅ 成功 | 480      | cloudflare |
| 213  | www.digitalocean.com                    | 104.19.174.68                           | IPv4   | h3   | ✅ 成功 | 482      | cloudflare |
| 82   | www.4chan.org                           | 104.16.228.229                          | IPv4   | h3   | ✅ 成功 | 493      | cloudflare |
| 254  | 172.64.48.226                           | 172.64.48.226                           | IPv4   | h3   | ✅ 成功 | 494      | cloudflare |
| 277  | 104.17.162.3                            | 104.17.162.3                            | IPv4   | h3   | ✅ 成功 | 494      | cloudflare |
| 42   | cmcc.877774.xyz                         | 104.16.149.11                           | IPv4   | h3   | ✅ 成功 | 495      | cloudflare |
| 339  | 104.17.142.212                          | 104.17.142.212                          | IPv4   | h3   | ✅ 成功 | 495      | cloudflare |
| 44   | cmcc.877774.xyz                         | 104.16.149.244                          | IPv4   | h3   | ✅ 成功 | 498      | cloudflare |
| 287  | cf.090227.xyz                           | 172.64.144.82                           | IPv4   | h3   | ✅ 成功 | 498      | cloudflare |
| 148  | 104.16.223.179                          | 104.16.223.179                          | IPv4   | h3   | ✅ 成功 | 499      | cloudflare |
| 187  | 104.17.142.12                           | 104.17.142.12                           | IPv4   | h3   | ✅ 成功 | 500      | cloudflare |
| 224  | cfip.1323123.xyz                        | 104.16.133.220                          | IPv4   | h3   | ✅ 成功 | 500      | cloudflare |
| 333  | 104.18.255.167                          | 104.18.255.167                          | IPv4   | h3   | ✅ 成功 | 500      | cloudflare |
| 270  | 162.159.140.116                         | 162.159.140.116                         | IPv4   | h3   | ✅ 成功 | 503      | cloudflare |
| 96   | craig.ns.cloudflare.com                 | 108.162.195.192                         | IPv4   | h3   | ✅ 成功 | 505      | cloudflare |
| 98   | craig.ns.cloudflare.com                 | 172.64.35.192                           | IPv4   | h3   | ✅ 成功 | 505      | cloudflare |
| 131  | pranab.ns.cloudflare.com                | 108.162.195.199                         | IPv4   | h3   | ✅ 成功 | 505      | cloudflare |
| 275  | 104.19.148.121                          | 104.19.148.121                          | IPv4   | h3   | ✅ 成功 | 507      | cloudflare |
| 283  | [2606:4700:440f::53aa:4126]             | 2606:4700:440f::53aa:4126               | IPv6   | h3   | ✅ 成功 | 509      | cloudflare |
| 111  | sullivan.ns.cloudflare.com              | 162.159.44.161                          | IPv4   | h3   | ✅ 成功 | 510      | cloudflare |
| 237  | lewis.ns.cloudflare.com                 | 172.64.35.159                           | IPv4   | h3   | ✅ 成功 | 512      | cloudflare |
| 88   | www.okcupid.com                         | 104.16.144.63                           | IPv4   | h3   | ✅ 成功 | 513      | cloudflare |
| 260  | 162.159.136.89                          | 162.159.136.89                          | IPv4   | h3   | ✅ 成功 | 514      | cloudflare |
| 115  | sullivan.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a1                 | IPv6   | h3   | ✅ 成功 | 516      | cloudflare |
| 188  | braden.ns.cloudflare.com                | 162.159.44.169                          | IPv4   | h3   | ✅ 成功 | 517      | cloudflare |
| 266  | 162.159.137.204                         | 162.159.137.204                         | IPv4   | h3   | ✅ 成功 | 518      | cloudflare |
| 316  | moura.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23d9                 | IPv6   | h3   | ✅ 成功 | 519      | cloudflare |
| 284  | www.csgo.com                            | 195.85.59.161                           | IPv4   | h3   | ✅ 成功 | 520      | cloudflare |
| 289  | cf.090227.xyz                           | 2606:4700:4407::ac40:9052               | IPv6   | h3   | ✅ 成功 | 520      | cloudflare |
| 259  | 104.19.154.200                          | 104.19.154.200                          | IPv4   | h3   | ✅ 成功 | 521      | cloudflare |
| 285  | www.csgo.com                            | 195.85.59.95                            | IPv4   | h3   | ✅ 成功 | 522      | cloudflare |
| 136  | pranab.ns.cloudflare.com                | 2803:f800:50::6ca2:c3c7                 | IPv6   | h3   | ✅ 成功 | 523      | cloudflare |
| 33   | ct.877774.xyz                           | 172.64.229.161                          | IPv4   | h3   | ✅ 成功 | 525      | cloudflare |
| 256  | 162.159.24.131                          | 162.159.24.131                          | IPv4   | h3   | ✅ 成功 | 525      | cloudflare |
| 113  | sullivan.ns.cloudflare.com              | 2606:4700:58::a29f:2ca1                 | IPv6   | h3   | ✅ 成功 | 526      | cloudflare |
| 200  | huxley.ns.cloudflare.com                | 172.64.35.188                           | IPv4   | h3   | ✅ 成功 | 527      | cloudflare |
| 267  | 162.159.128.253                         | 162.159.128.253                         | IPv4   | h3   | ✅ 成功 | 529      | cloudflare |
| 130  | www.visa.com.sg                         | 104.18.13.229                           | IPv4   | h3   | ✅ 成功 | 530      | cloudflare |
| 134  | pranab.ns.cloudflare.com                | 2a06:98c1:50::ac40:23c7                 | IPv6   | h3   | ✅ 成功 | 531      | cloudflare |
| 207  | fbi.gov                                 | 2606:4700::6810:95f4                    | IPv6   | h3   | ✅ 成功 | 531      | cloudflare |
| 239  | lewis.ns.cloudflare.com                 | 162.159.44.159                          | IPv4   | h3   | ✅ 成功 | 531      | cloudflare |
| 380  | uriah.ns.cloudflare.com                 | 162.159.44.194                          | IPv4   | h3   | ✅ 成功 | 531      | cloudflare |
| 59   | cmcc.877774.xyz                         | 104.16.149.2                            | IPv4   | h3   | ✅ 成功 | 533      | cloudflare |
| 85   | trevor.ns.cloudflare.com                | 2803:f800:50::6ca2:c39a                 | IPv6   | h3   | ✅ 成功 | 533      | cloudflare |
| 288  | cf.090227.xyz                           | 104.18.43.174                           | IPv4   | h3   | ✅ 成功 | 533      | cloudflare |
| 328  | benedict.ns.cloudflare.com              | 2803:f800:50::6ca2:c3cd                 | IPv6   | h3   | ✅ 成功 | 533      | cloudflare |
| 359  | singapore.com                           | 104.26.12.140                           | IPv4   | h3   | ✅ 成功 | 533      | cloudflare |
| 180  | bestcf.030101.xyz                       | 2606:4700:0:c5:4803:8845:8bde:1897      | IPv6   | h3   | ✅ 成功 | 534      | cloudflare |
| 332  | 104.16.65.1                             | 104.16.65.1                             | IPv4   | h3   | ✅ 成功 | 534      | cloudflare |
| 94   | www.4444.cloudflare.182682.xyz                             | 162.159.153.2                           | IPv4   | h3   | ✅ 成功 | 535      | cloudflare |
| 214  | www.digitalocean.com                    | 2606:4700::6813:ad44                    | IPv6   | h3   | ✅ 成功 | 536      | cloudflare |
| 78   | cu.877774.xyz                           | 172.64.145.202                          | IPv4   | h3   | ✅ 成功 | 538      | cloudflare |
| 221  | otto.ns.cloudflare.com                  | 2606:4700:58::a29f:2c87                 | IPv6   | h3   | ✅ 成功 | 538      | cloudflare |
| 238  | lewis.ns.cloudflare.com                 | 108.162.195.159                         | IPv4   | h3   | ✅ 成功 | 538      | cloudflare |
| 251  | 162.159.140.85                          | 162.159.140.85                          | IPv4   | h3   | ✅ 成功 | 538      | cloudflare |
| 246  | www.wto.org                             | 172.64.146.66                           | IPv4   | h3   | ✅ 成功 | 539      | cloudflare |
| 253  | 162.159.61.183                          | 162.159.61.183                          | IPv4   | h3   | ✅ 成功 | 540      | cloudflare |
| 297  | wilson.ns.cloudflare.com                | 172.64.35.110                           | IPv4   | h3   | ✅ 成功 | 540      | cloudflare |
| 331  | 162.159.36.104                          | 162.159.36.104                          | IPv4   | h3   | ✅ 成功 | 540      | cloudflare |
| 261  | 162.159.58.65                           | 162.159.58.65                           | IPv4   | h3   | ✅ 成功 | 541      | cloudflare |
| 186  | cf.zhetengsha.eu.org                    | 2a06:98c1:3101::ac40:919e               | IPv6   | h3   | ✅ 成功 | 542      | cloudflare |
| 252  | 172.64.52.127                           | 172.64.52.127                           | IPv4   | h3   | ✅ 成功 | 542      | cloudflare |
| 125  | yx-auto.pages.dev                       | 172.66.44.144                           | IPv4   | h3   | ✅ 成功 | 543      | cloudflare |
| 374  | ashton.ns.cloudflare.com                | 172.64.35.173                           | IPv4   | h3   | ✅ 成功 | 543      | cloudflare |
| 73   | cfip.xxxxxxxx.tk                        | 104.16.241.229                          | IPv4   | h3   | ✅ 成功 | 544      | cloudflare |
| 126  | yx-auto.pages.dev                       | 172.66.47.112                           | IPv4   | h3   | ✅ 成功 | 544      | cloudflare |
| 334  | 104.19.212.207                          | 104.19.212.207                          | IPv4   | h3   | ✅ 成功 | 544      | cloudflare |
| 220  | otto.ns.cloudflare.com                  | 108.162.195.135                         | IPv4   | h3   | ✅ 成功 | 545      | cloudflare |
| 8    | www.pcmag.com                           | 104.16.21.118                           | IPv4   | h3   | ✅ 成功 | 546      | cloudflare |
| 17   | ipv4.ip.sb                              | 172.67.75.172                           | IPv4   | h3   | ✅ 成功 | 546      | cloudflare |
| 199  | 172.64.35.24                            | 172.64.35.24                            | IPv4   | h3   | ✅ 成功 | 547      | cloudflare |
| 219  | otto.ns.cloudflare.com                  | 162.159.44.135                          | IPv4   | h3   | ✅ 成功 | 547      | cloudflare |
| 247  | www.wto.org                             | 2a06:98c1:3102::6812:29be               | IPv6   | h3   | ✅ 成功 | 548      | cloudflare |
| 360  | singapore.com                           | 104.26.13.140                           | IPv4   | h3   | ✅ 成功 | 550      | cloudflare |
| 400  | cf.877774.xyz                           | 172.64.146.66                           | IPv4   | h3   | ✅ 成功 | 550      | cloudflare |
| 182  | [2606:4700:4408::18c5:3304]             | 2606:4700:4408::18c5:3304               | IPv6   | h3   | ✅ 成功 | 551      | cloudflare |
| 268  | 104.26.3.162                            | 104.26.3.162                            | IPv4   | h3   | ✅ 成功 | 551      | cloudflare |
| 127  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2c90               | IPv6   | h3   | ✅ 成功 | 552      | cloudflare |
| 143  | dylan.ns.cloudflare.com                 | 2606:4700:58::a29f:2cbb                 | IPv6   | h3   | ✅ 成功 | 552      | cloudflare |
| 324  | benedict.ns.cloudflare.com              | 108.162.195.205                         | IPv4   | h3   | ✅ 成功 | 552      | cloudflare |
| 41   | 103.160.204.59                          | 103.160.204.59                          | IPv4   | h3   | ✅ 成功 | 554      | cloudflare |
| 191  | braden.ns.cloudflare.com                | 2803:f800:50::6ca2:c3a9                 | IPv6   | h3   | ✅ 成功 | 554      | cloudflare |
| 93   | www.4444.cloudflare.182682.xyz                             | 162.159.152.2                           | IPv4   | h3   | ✅ 成功 | 555      | cloudflare |
| 100  | craig.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23c0                 | IPv6   | h3   | ✅ 成功 | 555      | cloudflare |
| 329  | benedict.ns.cloudflare.com              | 2a06:98c1:50::ac40:23cd                 | IPv6   | h3   | ✅ 成功 | 556      | cloudflare |
| 192  | braden.ns.cloudflare.com                | 2606:4700:58::a29f:2ca9                 | IPv6   | h3   | ✅ 成功 | 557      | cloudflare |
| 349  | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6   | h3   | ✅ 成功 | 557      | cloudflare |
| 390  | julio.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3d1                 | IPv6   | h3   | ✅ 成功 | 557      | cloudflare |
| 101  | craig.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3c0                 | IPv6   | h3   | ✅ 成功 | 558      | cloudflare |
| 92   | 172.67.120.0                            | 172.67.120.0                            | IPv4   | h3   | ✅ 成功 | 559      | cloudflare |
| 240  | lewis.ns.cloudflare.com                 | 2606:4700:58::a29f:2c9f                 | IPv6   | h3   | ✅ 成功 | 559      | cloudflare |
| 310  | www.ipchicken.com                       | 104.26.6.112                            | IPv4   | h3   | ✅ 成功 | 559      | cloudflare |
| 340  | 104.16.105.166                          | 104.16.105.166                          | IPv4   | h3   | ✅ 成功 | 559      | cloudflare |
| 70   | cfip.xxxxxxxx.tk                        | 104.16.232.223                          | IPv4   | h3   | ✅ 成功 | 560      | cloudflare |
| 151  | cris.ns.cloudflare.com                  | 162.159.44.202                          | IPv4   | h3   | ✅ 成功 | 560      | cloudflare |
| 296  | wilson.ns.cloudflare.com                | 162.159.44.110                          | IPv4   | h3   | ✅ 成功 | 560      | cloudflare |
| 370  | 456.cloudflare.182682.xyz               | 104.26.9.160                            | IPv4   | h3   | ✅ 成功 | 560      | cloudflare |
| 379  | ashton.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ad                 | IPv6   | h3   | ✅ 成功 | 561      | cloudflare |
| 319  | time.is                                 | 172.67.68.157                           | IPv4   | h3   | ✅ 成功 | 562      | cloudflare |
| 398  | dnschecker.org                          | 104.26.7.89                             | IPv4   | h3   | ✅ 成功 | 562      | cloudflare |
| 262  | 104.26.5.134                            | 104.26.5.134                            | IPv4   | h3   | ✅ 成功 | 563      | cloudflare |
| 184  | cf.zhetengsha.eu.org                    | 172.64.152.241                          | IPv4   | h3   | ✅ 成功 | 564      | cloudflare |
| 140  | dylan.ns.cloudflare.com                 | 162.159.44.187                          | IPv4   | h3   | ✅ 成功 | 566      | cloudflare |
| 147  | freeyx.cloudflare88.eu.org              | 141.101.121.109                         | IPv4   | h3   | ✅ 成功 | 566      | cloudflare |
| 314  | moura.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3d9                 | IPv6   | h3   | ✅ 成功 | 567      | cloudflare |
| 53   | cmcc.877774.xyz                         | 104.16.148.9                            | IPv4   | h3   | ✅ 成功 | 568      | cloudflare |
| 194  | 172.67.79.211                           | 172.67.79.211                           | IPv4   | h3   | ✅ 成功 | 568      | cloudflare |
| 226  | damien.ns.cloudflare.com                | 2606:4700:58::a29f:2ca8                 | IPv6   | h3   | ✅ 成功 | 568      | cloudflare |
| 301  | wilson.ns.cloudflare.com                | 2803:f800:50::6ca2:c36e                 | IPv6   | h3   | ✅ 成功 | 568      | cloudflare |
| 322  | rustam.ns.cloudflare.com                | 2803:f800:50::6ca2:c394                 | IPv6   | h3   | ✅ 成功 | 568      | cloudflare |
| 137  | na.877774.xyz                           | 104.18.38.235                           | IPv4   | h3   | ✅ 成功 | 569      | cloudflare |
| 325  | benedict.ns.cloudflare.com              | 162.159.44.205                          | IPv4   | h3   | ✅ 成功 | 569      | cloudflare |
| 376  | ashton.ns.cloudflare.com                | 162.159.44.173                          | IPv4   | h3   | ✅ 成功 | 570      | cloudflare |
| 201  | huxley.ns.cloudflare.com                | 108.162.195.188                         | IPv4   | h3   | ✅ 成功 | 571      | cloudflare |
| 383  | uriah.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3c2                 | IPv6   | h3   | ✅ 成功 | 571      | cloudflare |
| 86   | trevor.ns.cloudflare.com                | 2a06:98c1:50::ac40:239a                 | IPv6   | h3   | ✅ 成功 | 572      | cloudflare |
| 84   | trevor.ns.cloudflare.com                | 2606:4700:58::a29f:2c9a                 | IPv6   | h3   | ✅ 成功 | 573      | cloudflare |
| 318  | time.is                                 | 104.26.13.54                            | IPv4   | h3   | ✅ 成功 | 573      | cloudflare |
| 393  | tasteatlas.com                          | 104.17.37.105                           | IPv4   | h3   | ✅ 成功 | 573      | cloudflare |
| 173  | 104.26.6.112                            | 104.26.6.112                            | IPv4   | h3   | ✅ 成功 | 574      | cloudflare |
| 348  | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6   | h3   | ✅ 成功 | 574      | cloudflare |
| 336  | 104.31.16.158                           | 104.31.16.158                           | IPv4   | h3   | ✅ 成功 | 575      | cloudflare |
| 394  | tasteatlas.com                          | 2606:4700::6811:2469                    | IPv6   | h3   | ✅ 成功 | 575      | cloudflare |
| 222  | otto.ns.cloudflare.com                  | 2803:f800:50::6ca2:c387                 | IPv6   | h3   | ✅ 成功 | 576      | cloudflare |
| 223  | otto.ns.cloudflare.com                  | 2a06:98c1:50::ac40:2387                 | IPv6   | h3   | ✅ 成功 | 576      | cloudflare |
| 236  | 108.162.198.54                          | 108.162.198.54                          | IPv4   | h3   | ✅ 成功 | 577      | cloudflare |
| 304  | bowen.ns.cloudflare.com                 | 108.162.195.83                          | IPv4   | h3   | ✅ 成功 | 577      | cloudflare |
| 99   | craig.ns.cloudflare.com                 | 2606:4700:58::a29f:2cc0                 | IPv6   | h3   | ✅ 成功 | 578      | cloudflare |
| 114  | sullivan.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a1                 | IPv6   | h3   | ✅ 成功 | 578      | cloudflare |
| 391  | julio.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23d1                 | IPv6   | h3   | ✅ 成功 | 578      | cloudflare |
| 150  | cris.ns.cloudflare.com                  | 108.162.195.202                         | IPv4   | h3   | ✅ 成功 | 580      | cloudflare |
| 205  | huxley.ns.cloudflare.com                | 2a06:98c1:50::ac40:23bc                 | IPv6   | h3   | ✅ 成功 | 580      | cloudflare |
| 269  | 104.26.8.117                            | 104.26.8.117                            | IPv4   | h3   | ✅ 成功 | 581      | cloudflare |
| 64   | cmcc.877774.xyz                         | 104.16.149.7                            | IPv4   | h3   | ✅ 成功 | 583      | cloudflare |
| 46   | cmcc.877774.xyz                         | 104.16.148.2                            | IPv4   | h3   | ✅ 成功 | 584      | cloudflare |
| 110  | sullivan.ns.cloudflare.com              | 108.162.195.161                         | IPv4   | h3   | ✅ 成功 | 584      | cloudflare |
| 375  | ashton.ns.cloudflare.com                | 108.162.195.173                         | IPv4   | h3   | ✅ 成功 | 584      | cloudflare |
| 395  | tasteatlas.com                          | 2606:4700::6811:2569                    | IPv6   | h3   | ✅ 成功 | 584      | cloudflare |
| 152  | cris.ns.cloudflare.com                  | 172.64.35.202                           | IPv4   | h3   | ✅ 成功 | 585      | cloudflare |
| 215  | www.digitalocean.com                    | 2606:4700::6813:ae44                    | IPv6   | h3   | ✅ 成功 | 585      | cloudflare |
| 350  | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | IPv6   | h3   | ✅ 成功 | 585      | cloudflare |
| 181  | bestcf.030101.xyz                       | 2606:4700:0:e7ac:854f:c15c:d3b1:fc6a    | IPv6   | h3   | ✅ 成功 | 586      | cloudflare |
| 248  | www.wto.org                             | 2606:4700:4406::ac40:9242               | IPv6   | h3   | ✅ 成功 | 587      | cloudflare |
| 141  | dylan.ns.cloudflare.com                 | 172.64.35.187                           | IPv4   | h3   | ✅ 成功 | 588      | cloudflare |
| 202  | huxley.ns.cloudflare.com                | 162.159.44.188                          | IPv4   | h3   | ✅ 成功 | 589      | cloudflare |
| 145  | dylan.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23bb                 | IPv6   | h3   | ✅ 成功 | 590      | cloudflare |
| 323  | rustam.ns.cloudflare.com                | 2a06:98c1:50::ac40:2394                 | IPv6   | h3   | ✅ 成功 | 591      | cloudflare |
| 129  | www.visa.com.sg                         | 104.18.12.229                           | IPv4   | h3   | ✅ 成功 | 592      | cloudflare |
| 61   | cmcc.877774.xyz                         | 104.16.149.4                            | IPv4   | h3   | ✅ 成功 | 593      | cloudflare |
| 397  | dnschecker.org                          | 104.26.6.89                             | IPv4   | h3   | ✅ 成功 | 593      | cloudflare |
| 36   | ct.877774.xyz                           | 172.64.229.185                          | IPv4   | h3   | ✅ 成功 | 595      | cloudflare |
| 133  | pranab.ns.cloudflare.com                | 162.159.44.199                          | IPv4   | h3   | ✅ 成功 | 595      | cloudflare |
| 382  | uriah.ns.cloudflare.com                 | 108.162.195.194                         | IPv4   | h3   | ✅ 成功 | 595      | cloudflare |
| 231  | japan.com                               | 104.26.4.60                             | IPv4   | h3   | ✅ 成功 | 598      | cloudflare |
| 311  | moura.ns.cloudflare.com                 | 108.162.195.217                         | IPv4   | h3   | ✅ 成功 | 599      | cloudflare |
| 142  | dylan.ns.cloudflare.com                 | 108.162.195.187                         | IPv4   | h3   | ✅ 成功 | 600      | cloudflare |
| 204  | huxley.ns.cloudflare.com                | 2803:f800:50::6ca2:c3bc                 | IPv6   | h3   | ✅ 成功 | 600      | cloudflare |
| 399  | dnschecker.org                          | 172.67.73.216                           | IPv4   | h3   | ✅ 成功 | 600      | cloudflare |
| 146  | freeyx.cloudflare88.eu.org              | 141.101.121.38                          | IPv4   | h3   | ✅ 成功 | 601      | cloudflare |
| 227  | damien.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a8                 | IPv6   | h3   | ✅ 成功 | 601      | cloudflare |
| 250  | 198.41.194.162                          | 198.41.194.162                          | IPv4   | h3   | ✅ 成功 | 601      | cloudflare |
| 363  | singapore.com                           | 2606:4700:20::681a:d8c                  | IPv6   | h3   | ✅ 成功 | 601      | cloudflare |
| 144  | dylan.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3bb                 | IPv6   | h3   | ✅ 成功 | 602      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 0 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 47 条记录
- **很慢 (>500ms)**: 153 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 6 次
- **IPv6 失败**: 1 次

### 按协议统计

- **none**: 14 次失败
- **h2**: 1 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
