# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/15 09:58:02
- **数据来源**: connectivity_results-20251215-095650.json
- **总测试数**: 450
- **失败测试数**: 7
- **成功测试数**: 443
- **失败率**: 1.56%
- **平均延迟**: 465.40ms
- **最小延迟**: 133ms
- **最大延迟**: 3571ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/15 09:58:02
- **IP地址**: 240e:b8e:35cd:a601:6d84:88c0:8d09:973f
- **国家/地区**: China (CN)
- **ASN**: AS4134
- **网络组织**: Chinanet
- **网络域名**: chinatelecom.com.cn
- **大洲**: Asia (AS)
- **地理坐标**: 34.7732, 113.722
- **时区**: Asia/Shanghai
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: I/O超时**: 7 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (7 次测试)

| 序号 | 主机/域名                | 目标IP                  | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                            |
| ---- | ------------------------ | ----------------------- | ------ | ---- | ------ | -------- | ------ | --------------------------------------------------- |
| 94   | cfip.xxxxxxxx.tk         | 198.41.212.130          | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout            |
| 175  | cloudflare.182682.xyz    | 2606:4700:e7::3151:47a9 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:e7::3151:47a9]:443: i/o timeout |
| 180  | trevor.ns.cloudflare.com | 108.162.195.154         | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout           |
| 203  | cf.877774.xyz            | cf.877774.xyz           | IPv4   | none | N/A    | 0        | N/A    | dial tcp cf.877774.xyz:443: i/o timeout             |
| 361  | 141.147.185.63           | 141.147.185.63          | IPv4   | none | N/A    | 0        | N/A    | dial tcp 141.147.185.63:443: i/o timeout            |
| 443  | ct.877774.xyz            | ct.877774.xyz           | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout             |
| 444  | 3.0.50.69                | 3.0.50.69               | IPv4   | none | N/A    | 0        | N/A    | dial tcp 3.0.50.69:443: i/o timeout                 |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 7 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 7 次超时，主要集中在IP段 198.41（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 7 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 6 次，IPv6失败 1 次，两种协议都存在问题

---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                   | 目标IP                              | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | --------------------------- | ----------------------------------- | ------ | ---- | ------- | -------- | ---------- |
| 330  | ashton.ns.cloudflare.com    | 162.159.44.173                      | IPv4   | h3   | ✅ 成功 | 133      | cloudflare |
| 3    | ct.877774.xyz               | 172.64.229.161                      | IPv4   | h3   | ✅ 成功 | 137      | cloudflare |
| 4    | ct.877774.xyz               | 172.64.229.44                       | IPv4   | h3   | ✅ 成功 | 140      | cloudflare |
| 5    | ct.877774.xyz               | 172.64.229.236                      | IPv4   | h3   | ✅ 成功 | 147      | cloudflare |
| 252  | bowen.ns.cloudflare.com     | 162.159.44.83                       | IPv4   | h3   | ✅ 成功 | 150      | cloudflare |
| 395  | otto.ns.cloudflare.com      | 162.159.44.135                      | IPv4   | h3   | ✅ 成功 | 150      | cloudflare |
| 2    | ct.877774.xyz               | 172.64.229.173                      | IPv4   | h3   | ✅ 成功 | 151      | cloudflare |
| 8    | ct.877774.xyz               | 172.64.229.185                      | IPv4   | h3   | ✅ 成功 | 158      | cloudflare |
| 95   | sullivan.ns.cloudflare.com  | 162.159.44.161                      | IPv4   | h3   | ✅ 成功 | 158      | cloudflare |
| 181  | trevor.ns.cloudflare.com    | 162.159.44.154                      | IPv4   | h3   | ✅ 成功 | 158      | cloudflare |
| 14   | wilson.ns.cloudflare.com    | 162.159.44.110                      | IPv4   | h3   | ✅ 成功 | 159      | cloudflare |
| 408  | lewis.ns.cloudflare.com     | 162.159.44.159                      | IPv4   | h3   | ✅ 成功 | 160      | cloudflare |
| 7    | ct.877774.xyz               | 172.64.229.195                      | IPv4   | h3   | ✅ 成功 | 162      | cloudflare |
| 269  | moura.ns.cloudflare.com     | 162.159.44.217                      | IPv4   | h3   | ✅ 成功 | 162      | cloudflare |
| 389  | damien.ns.cloudflare.com    | 162.159.44.168                      | IPv4   | h3   | ✅ 成功 | 163      | cloudflare |
| 199  | kyree.ns.cloudflare.com     | 162.159.44.207                      | IPv4   | h3   | ✅ 成功 | 170      | cloudflare |
| 233  | braden.ns.cloudflare.com    | 162.159.44.169                      | IPv4   | h3   | ✅ 成功 | 174      | cloudflare |
| 1    | ct.877774.xyz               | 172.64.229.174                      | IPv4   | h3   | ✅ 成功 | 177      | cloudflare |
| 160  | cris.ns.cloudflare.com      | 162.159.44.202                      | IPv4   | h3   | ✅ 成功 | 186      | cloudflare |
| 6    | ct.877774.xyz               | 172.64.229.217                      | IPv4   | h3   | ✅ 成功 | 188      | cloudflare |
| 108  | craig.ns.cloudflare.com     | 162.159.44.192                      | IPv4   | h3   | ✅ 成功 | 193      | cloudflare |
| 413  | 108.162.198.54              | 108.162.198.54                      | IPv4   | h3   | ✅ 成功 | 195      | cloudflare |
| 264  | rustam.ns.cloudflare.com    | 162.159.44.148                      | IPv4   | h3   | ✅ 成功 | 198      | cloudflare |
| 429  | abdullah.ns.cloudflare.com  | 162.159.44.203                      | IPv4   | h3   | ✅ 成功 | 199      | cloudflare |
| 208  | dylan.ns.cloudflare.com     | 162.159.44.187                      | IPv4   | h3   | ✅ 成功 | 201      | cloudflare |
| 165  | decker.ns.cloudflare.com    | 162.159.44.155                      | IPv4   | h3   | ✅ 成功 | 202      | cloudflare |
| 266  | rustam.ns.cloudflare.com    | 2a06:98c1:50::ac40:2394             | IPv6   | h3   | ✅ 成功 | 203      | cloudflare |
| 318  | benedict.ns.cloudflare.com  | 162.159.44.205                      | IPv4   | h3   | ✅ 成功 | 203      | cloudflare |
| 154  | pranab.ns.cloudflare.com    | 162.159.44.199                      | IPv4   | h3   | ✅ 成功 | 205      | cloudflare |
| 399  | otto.ns.cloudflare.com      | 2803:f800:50::6ca2:c387             | IPv6   | h3   | ✅ 成功 | 206      | cloudflare |
| 257  | cf.zhetengsha.eu.org        | 172.64.144.82                       | IPv4   | h3   | ✅ 成功 | 209      | cloudflare |
| 390  | damien.ns.cloudflare.com    | 2606:4700:58::a29f:2ca8             | IPv6   | h3   | ✅ 成功 | 210      | cloudflare |
| 278  | cf.090227.xyz               | 172.64.152.241                      | IPv4   | h3   | ✅ 成功 | 213      | cloudflare |
| 250  | bowen.ns.cloudflare.com     | 172.64.35.83                        | IPv4   | h3   | ✅ 成功 | 215      | cloudflare |
| 377  | 172.64.35.24                | 172.64.35.24                        | IPv4   | h3   | ✅ 成功 | 215      | cloudflare |
| 271  | moura.ns.cloudflare.com     | 2606:4700:58::a29f:2cd9             | IPv6   | h3   | ✅ 成功 | 219      | cloudflare |
| 202  | kyree.ns.cloudflare.com     | 2a06:98c1:50::ac40:23cf             | IPv6   | h3   | ✅ 成功 | 220      | cloudflare |
| 236  | braden.ns.cloudflare.com    | 2803:f800:50::6ca2:c3a9             | IPv6   | h3   | ✅ 成功 | 220      | cloudflare |
| 409  | lewis.ns.cloudflare.com     | 172.64.35.159                       | IPv4   | h3   | ✅ 成功 | 220      | cloudflare |
| 348  | julio.ns.cloudflare.com     | 2803:f800:50::6ca2:c3d1             | IPv6   | h3   | ✅ 成功 | 221      | cloudflare |
| 36   | huxley.ns.cloudflare.com    | 2803:f800:50::6ca2:c3bc             | IPv6   | h3   | ✅ 成功 | 222      | cloudflare |
| 212  | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758           | IPv6   | h3   | ✅ 成功 | 222      | cloudflare |
| 280  | cf.090227.xyz               | 2a06:98c1:310d::6812:2bae           | IPv6   | h3   | ✅ 成功 | 222      | cloudflare |
| 162  | cris.ns.cloudflare.com      | 2606:4700:58::a29f:2cca             | IPv6   | h3   | ✅ 成功 | 224      | cloudflare |
| 260  | saas.sin.fan                | 162.159.36.5                        | IPv4   | h3   | ✅ 成功 | 224      | cloudflare |
| 407  | lewis.ns.cloudflare.com     | 108.162.195.159                     | IPv4   | h3   | ✅ 成功 | 224      | cloudflare |
| 270  | moura.ns.cloudflare.com     | 172.64.35.217                       | IPv4   | h3   | ✅ 成功 | 225      | cloudflare |
| 185  | trevor.ns.cloudflare.com    | 2803:f800:50::6ca2:c39a             | IPv6   | h3   | ✅ 成功 | 226      | cloudflare |
| 254  | bowen.ns.cloudflare.com     | 2803:f800:50::6ca2:c353             | IPv6   | h3   | ✅ 成功 | 226      | cloudflare |
| 259  | cf.zhetengsha.eu.org        | 2606:4700:4407::ac40:9052           | IPv6   | h3   | ✅ 成功 | 226      | cloudflare |
| 40   | 104.18.39.196               | 104.18.39.196                       | IPv4   | h3   | ✅ 成功 | 227      | cloudflare |
| 158  | cris.ns.cloudflare.com      | 172.64.35.202                       | IPv4   | h3   | ✅ 成功 | 227      | cloudflare |
| 237  | braden.ns.cloudflare.com    | 2a06:98c1:50::ac40:23a9             | IPv6   | h3   | ✅ 成功 | 227      | cloudflare |
| 418  | www.wto.org                 | 104.18.41.190                       | IPv4   | h3   | ✅ 成功 | 227      | cloudflare |
| 110  | craig.ns.cloudflare.com     | 2803:f800:50::6ca2:c3c0             | IPv6   | h3   | ✅ 成功 | 229      | cloudflare |
| 209  | dylan.ns.cloudflare.com     | 2606:4700:58::a29f:2cbb             | IPv6   | h3   | ✅ 成功 | 229      | cloudflare |
| 272  | moura.ns.cloudflare.com     | 2803:f800:50::6ca2:c3d9             | IPv6   | h3   | ✅ 成功 | 229      | cloudflare |
| 255  | bowen.ns.cloudflare.com     | 2a06:98c1:50::ac40:2353             | IPv6   | h3   | ✅ 成功 | 230      | cloudflare |
| 166  | decker.ns.cloudflare.com    | 108.162.195.155                     | IPv4   | h3   | ✅ 成功 | 231      | cloudflare |
| 235  | braden.ns.cloudflare.com    | 2606:4700:58::a29f:2ca9             | IPv6   | h3   | ✅ 成功 | 231      | cloudflare |
| 15   | wilson.ns.cloudflare.com    | 172.64.35.110                       | IPv4   | h3   | ✅ 成功 | 232      | cloudflare |
| 33   | huxley.ns.cloudflare.com    | 172.64.35.188                       | IPv4   | h3   | ✅ 成功 | 232      | cloudflare |
| 51   | 104.18.37.13                | 104.18.37.13                        | IPv4   | h3   | ✅ 成功 | 233      | cloudflare |
| 267  | rustam.ns.cloudflare.com    | 2803:f800:50::6ca2:c394             | IPv6   | h3   | ✅ 成功 | 233      | cloudflare |
| 53   | 172.64.154.18               | 172.64.154.18                       | IPv4   | h3   | ✅ 成功 | 234      | cloudflare |
| 163  | cris.ns.cloudflare.com      | 2803:f800:50::6ca2:c3ca             | IPv6   | h3   | ✅ 成功 | 234      | cloudflare |
| 159  | cris.ns.cloudflare.com      | 108.162.195.202                     | IPv4   | h3   | ✅ 成功 | 235      | cloudflare |
| 32   | huxley.ns.cloudflare.com    | 162.159.44.188                      | IPv4   | h3   | ✅ 成功 | 236      | cloudflare |
| 96   | sullivan.ns.cloudflare.com  | 108.162.195.161                     | IPv4   | h3   | ✅ 成功 | 236      | cloudflare |
| 155  | pranab.ns.cloudflare.com    | 2803:f800:50::6ca2:c3c7             | IPv6   | h3   | ✅ 成功 | 236      | cloudflare |
| 201  | kyree.ns.cloudflare.com     | 2803:f800:50::6ca2:c3cf             | IPv6   | h3   | ✅ 成功 | 236      | cloudflare |
| 358  | uriah.ns.cloudflare.com     | 2606:4700:58::a29f:2cc2             | IPv6   | h3   | ✅ 成功 | 236      | cloudflare |
| 16   | wilson.ns.cloudflare.com    | 2606:4700:58::a29f:2c6e             | IPv6   | h3   | ✅ 成功 | 237      | cloudflare |
| 164  | decker.ns.cloudflare.com    | 172.64.35.155                       | IPv4   | h3   | ✅ 成功 | 237      | cloudflare |
| 230  | 172.64.151.55               | 172.64.151.55                       | IPv4   | h3   | ✅ 成功 | 237      | cloudflare |
| 419  | www.wto.org                 | 2a06:98c1:3102::6812:29be           | IPv6   | h3   | ✅ 成功 | 237      | cloudflare |
| 190  | na.877774.xyz               | 104.18.38.235                       | IPv4   | h3   | ✅ 成功 | 238      | cloudflare |
| 232  | braden.ns.cloudflare.com    | 108.162.195.169                     | IPv4   | h3   | ✅ 成功 | 238      | cloudflare |
| 251  | bowen.ns.cloudflare.com     | 108.162.195.83                      | IPv4   | h3   | ✅ 成功 | 238      | cloudflare |
| 317  | benedict.ns.cloudflare.com  | 108.162.195.205                     | IPv4   | h3   | ✅ 成功 | 238      | cloudflare |
| 319  | benedict.ns.cloudflare.com  | 172.64.35.205                       | IPv4   | h3   | ✅ 成功 | 238      | cloudflare |
| 412  | lewis.ns.cloudflare.com     | 2a06:98c1:50::ac40:239f             | IPv6   | h3   | ✅ 成功 | 238      | cloudflare |
| 428  | abdullah.ns.cloudflare.com  | 108.162.195.203                     | IPv4   | h3   | ✅ 成功 | 238      | cloudflare |
| 18   | wilson.ns.cloudflare.com    | 2a06:98c1:50::ac40:236e             | IPv6   | h3   | ✅ 成功 | 240      | cloudflare |
| 200  | kyree.ns.cloudflare.com     | 2606:4700:58::a29f:2ccf             | IPv6   | h3   | ✅ 成功 | 240      | cloudflare |
| 231  | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126           | IPv6   | h3   | ✅ 成功 | 240      | cloudflare |
| 168  | decker.ns.cloudflare.com    | 2803:f800:50::6ca2:c39b             | IPv6   | h3   | ✅ 成功 | 241      | cloudflare |
| 410  | lewis.ns.cloudflare.com     | 2606:4700:58::a29f:2c9f             | IPv6   | h3   | ✅ 成功 | 241      | cloudflare |
| 111  | craig.ns.cloudflare.com     | 2a06:98c1:50::ac40:23c0             | IPv6   | h3   | ✅ 成功 | 242      | cloudflare |
| 387  | damien.ns.cloudflare.com    | 172.64.35.168                       | IPv4   | h3   | ✅ 成功 | 242      | cloudflare |
| 256  | cf.zhetengsha.eu.org        | 104.18.43.174                       | IPv4   | h3   | ✅ 成功 | 243      | cloudflare |
| 283  | 162.159.36.104              | 162.159.36.104                      | IPv4   | h3   | ✅ 成功 | 243      | cloudflare |
| 352  | cf.877774.xyz               | 2a06:98c1:3102::6812:29be           | IPv6   | h3   | ✅ 成功 | 243      | cloudflare |
| 357  | uriah.ns.cloudflare.com     | 2803:f800:50::6ca2:c3c2             | IPv6   | h3   | ✅ 成功 | 243      | cloudflare |
| 391  | damien.ns.cloudflare.com    | 2803:f800:50::6ca2:c3a8             | IPv6   | h3   | ✅ 成功 | 243      | cloudflare |
| 397  | otto.ns.cloudflare.com      | 172.64.35.135                       | IPv4   | h3   | ✅ 成功 | 243      | cloudflare |
| 417  | www.wto.org                 | 172.64.146.66                       | IPv4   | h3   | ✅ 成功 | 243      | cloudflare |
| 156  | pranab.ns.cloudflare.com    | 2606:4700:58::a29f:2cc7             | IPv6   | h3   | ✅ 成功 | 244      | cloudflare |
| 167  | decker.ns.cloudflare.com    | 2606:4700:58::a29f:2c9b             | IPv6   | h3   | ✅ 成功 | 244      | cloudflare |
| 182  | trevor.ns.cloudflare.com    | 172.64.35.154                       | IPv4   | h3   | ✅ 成功 | 244      | cloudflare |
| 206  | dylan.ns.cloudflare.com     | 172.64.35.187                       | IPv4   | h3   | ✅ 成功 | 244      | cloudflare |
| 210  | dylan.ns.cloudflare.com     | 2803:f800:50::6ca2:c3bb             | IPv6   | h3   | ✅ 成功 | 244      | cloudflare |
| 262  | rustam.ns.cloudflare.com    | 172.64.35.148                       | IPv4   | h3   | ✅ 成功 | 244      | cloudflare |
| 343  | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06           | IPv6   | h3   | ✅ 成功 | 244      | cloudflare |
| 98   | sullivan.ns.cloudflare.com  | 2606:4700:58::a29f:2ca1             | IPv6   | h3   | ✅ 成功 | 245      | cloudflare |
| 349  | julio.ns.cloudflare.com     | 2606:4700:58::a29f:2cd1             | IPv6   | h3   | ✅ 成功 | 245      | cloudflare |
| 351  | cf.877774.xyz               | 104.18.41.190                       | IPv4   | h3   | ✅ 成功 | 245      | cloudflare |
| 396  | otto.ns.cloudflare.com      | 108.162.195.135                     | IPv4   | h3   | ✅ 成功 | 245      | cloudflare |
| 97   | sullivan.ns.cloudflare.com  | 172.64.35.161                       | IPv4   | h3   | ✅ 成功 | 246      | cloudflare |
| 432  | abdullah.ns.cloudflare.com  | 2606:4700:58::a29f:2ccb             | IPv6   | h3   | ✅ 成功 | 246      | cloudflare |
| 107  | craig.ns.cloudflare.com     | 108.162.195.192                     | IPv4   | h3   | ✅ 成功 | 247      | cloudflare |
| 392  | damien.ns.cloudflare.com    | 2a06:98c1:50::ac40:23a8             | IPv6   | h3   | ✅ 成功 | 247      | cloudflare |
| 211  | dylan.ns.cloudflare.com     | 2a06:98c1:50::ac40:23bb             | IPv6   | h3   | ✅ 成功 | 248      | cloudflare |
| 261  | saas.sin.fan                | 162.159.36.20                       | IPv4   | h3   | ✅ 成功 | 248      | cloudflare |
| 112  | craig.ns.cloudflare.com     | 2606:4700:58::a29f:2cc0             | IPv6   | h3   | ✅ 成功 | 249      | cloudflare |
| 184  | trevor.ns.cloudflare.com    | 2606:4700:58::a29f:2c9a             | IPv6   | h3   | ✅ 成功 | 249      | cloudflare |
| 198  | kyree.ns.cloudflare.com     | 108.162.195.207                     | IPv4   | h3   | ✅ 成功 | 249      | cloudflare |
| 279  | cf.090227.xyz               | 2606:4700:4407::ac40:9052           | IPv6   | h3   | ✅ 成功 | 249      | cloudflare |
| 398  | otto.ns.cloudflare.com      | 2606:4700:58::a29f:2c87             | IPv6   | h3   | ✅ 成功 | 249      | cloudflare |
| 431  | abdullah.ns.cloudflare.com  | 2a06:98c1:50::ac40:23cb             | IPv6   | h3   | ✅ 成功 | 249      | cloudflare |
| 109  | craig.ns.cloudflare.com     | 172.64.35.192                       | IPv4   | h3   | ✅ 成功 | 250      | cloudflare |
| 183  | trevor.ns.cloudflare.com    | 2a06:98c1:50::ac40:239a             | IPv6   | h3   | ✅ 成功 | 250      | cloudflare |
| 234  | braden.ns.cloudflare.com    | 172.64.35.169                       | IPv4   | h3   | ✅ 成功 | 251      | cloudflare |
| 370  | 104.18.37.40                | 104.18.37.40                        | IPv4   | h3   | ✅ 成功 | 251      | cloudflare |
| 153  | pranab.ns.cloudflare.com    | 108.162.195.199                     | IPv4   | h3   | ✅ 成功 | 252      | cloudflare |
| 277  | cf.090227.xyz               | 104.18.35.15                        | IPv4   | h3   | ✅ 成功 | 252      | cloudflare |
| 65   | 34.143.159.175              | 34.143.159.175                      | IPv4   | h2   | ✅ 成功 | 254      | cloudflare |
| 161  | cris.ns.cloudflare.com      | 2a06:98c1:50::ac40:23ca             | IPv6   | h3   | ✅ 成功 | 255      | cloudflare |
| 265  | rustam.ns.cloudflare.com    | 2606:4700:58::a29f:2c94             | IPv6   | h3   | ✅ 成功 | 255      | cloudflare |
| 360  | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f           | IPv6   | h3   | ✅ 成功 | 255      | cloudflare |
| 268  | moura.ns.cloudflare.com     | 108.162.195.217                     | IPv4   | h3   | ✅ 成功 | 256      | cloudflare |
| 152  | pranab.ns.cloudflare.com    | 172.64.35.199                       | IPv4   | h3   | ✅ 成功 | 258      | cloudflare |
| 273  | moura.ns.cloudflare.com     | 2a06:98c1:50::ac40:23d9             | IPv6   | h3   | ✅ 成功 | 259      | cloudflare |
| 99   | sullivan.ns.cloudflare.com  | 2803:f800:50::6ca2:c3a1             | IPv6   | h3   | ✅ 成功 | 260      | cloudflare |
| 253  | bowen.ns.cloudflare.com     | 2606:4700:58::a29f:2c53             | IPv6   | h3   | ✅ 成功 | 260      | cloudflare |
| 258  | cf.zhetengsha.eu.org        | 2a06:98c1:310d::6812:2bae           | IPv6   | h3   | ✅ 成功 | 260      | cloudflare |
| 433  | abdullah.ns.cloudflare.com  | 2803:f800:50::6ca2:c3cb             | IPv6   | h3   | ✅ 成功 | 260      | cloudflare |
| 31   | huxley.ns.cloudflare.com    | 108.162.195.188                     | IPv4   | h3   | ✅ 成功 | 261      | cloudflare |
| 362  | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304           | IPv6   | h3   | ✅ 成功 | 261      | cloudflare |
| 347  | julio.ns.cloudflare.com     | 2a06:98c1:50::ac40:23d1             | IPv6   | h3   | ✅ 成功 | 262      | cloudflare |
| 420  | www.wto.org                 | 2606:4700:4406::ac40:9242           | IPv6   | h3   | ✅ 成功 | 262      | cloudflare |
| 400  | otto.ns.cloudflare.com      | 2a06:98c1:50::ac40:2387             | IPv6   | h3   | ✅ 成功 | 266      | cloudflare |
| 388  | damien.ns.cloudflare.com    | 108.162.195.168                     | IPv4   | h3   | ✅ 成功 | 267      | cloudflare |
| 100  | sullivan.ns.cloudflare.com  | 2a06:98c1:50::ac40:23a1             | IPv6   | h3   | ✅ 成功 | 269      | cloudflare |
| 35   | huxley.ns.cloudflare.com    | 2606:4700:58::a29f:2cbc             | IPv6   | h3   | ✅ 成功 | 275      | cloudflare |
| 207  | dylan.ns.cloudflare.com     | 108.162.195.187                     | IPv4   | h3   | ✅ 成功 | 275      | cloudflare |
| 263  | rustam.ns.cloudflare.com    | 108.162.195.148                     | IPv4   | h3   | ✅ 成功 | 275      | cloudflare |
| 189  | na.877774.xyz               | 104.19.74.233                       | IPv4   | h3   | ✅ 成功 | 312      | cloudflare |
| 127  | cmcc.877774.xyz             | 104.16.149.6                        | IPv4   | h3   | ✅ 成功 | 334      | cloudflare |
| 132  | cmcc.877774.xyz             | 104.16.148.5                        | IPv4   | h3   | ✅ 成功 | 341      | cloudflare |
| 241  | bestcf.030101.xyz           | 2606:4700:0:6de1:d9f:4f26:52e9:5692 | IPv6   | h3   | ✅ 成功 | 348      | cloudflare |
| 157  | pranab.ns.cloudflare.com    | 2a06:98c1:50::ac40:23c7             | IPv6   | h3   | ✅ 成功 | 352      | cloudflare |
| 22   | www.pcmag.com               | 2606:4700::6810:1476                | IPv6   | h3   | ✅ 成功 | 362      | cloudflare |
| 415  | 104.18.78.214               | 104.18.78.214                       | IPv4   | h3   | ✅ 成功 | 362      | cloudflare |
| 59   | iplocation.io               | 2606:4700:20::ac43:4664             | IPv6   | h3   | ✅ 成功 | 367      | cloudflare |
| 118  | cf.877771.xyz               | 172.67.152.183                      | IPv4   | h3   | ✅ 成功 | 368      | cloudflare |
| 37   | ipv4.ip.sb                  | 104.26.13.31                        | IPv4   | h3   | ✅ 成功 | 369      | cloudflare |
| 19   | www.pcmag.com               | 104.16.20.118                       | IPv4   | h3   | ✅ 成功 | 370      | cloudflare |
| 450  | japan.com                   | 2606:4700:20::ac43:465c             | IPv6   | h3   | ✅ 成功 | 370      | cloudflare |
| 93   | cfip.xxxxxxxx.tk            | 190.93.244.201                      | IPv4   | h3   | ✅ 成功 | 372      | cloudflare |
| 115  | www.visa.com.sg             | 104.18.13.229                       | IPv4   | h3   | ✅ 成功 | 372      | cloudflare |
| 216  | www.whatismyip.com          | 2606:4700:20::681a:d17              | IPv6   | h3   | ✅ 成功 | 375      | cloudflare |
| 376  | www.digitalocean.com        | 2606:4700::6813:ad44                | IPv6   | h3   | ✅ 成功 | 375      | cloudflare |
| 12   | steamdb.info                | 2606:4700:10::6814:22d4             | IPv6   | h3   | ✅ 成功 | 377      | cloudflare |
| 305  | ip.gs                       | 2606:4700:3036::6815:eb0            | IPv6   | h3   | ✅ 成功 | 377      | cloudflare |
| 315  | whatismyipaddress.com       | 2606:4700::6813:de4f                | IPv6   | h3   | ✅ 成功 | 377      | cloudflare |
| 29   | www.ipget.net               | 2606:4700:3031::ac43:cf1a           | IPv6   | h3   | ✅ 成功 | 378      | cloudflare |
| 223  | asia.877774.xyz             | 104.17.142.146                      | IPv4   | h3   | ✅ 成功 | 378      | cloudflare |
| 448  | japan.com                   | 2606:4700:20::681a:43c              | IPv6   | h3   | ✅ 成功 | 378      | cloudflare |
| 133  | cmcc.877774.xyz             | 104.16.149.11                       | IPv4   | h3   | ✅ 成功 | 380      | cloudflare |
| 285  | time.is                     | 172.67.68.157                       | IPv4   | h3   | ✅ 成功 | 380      | cloudflare |
| 71   | ipinfo.in                   | 172.67.198.203                      | IPv4   | h3   | ✅ 成功 | 381      | cloudflare |
| 126  | cmcc.877774.xyz             | 104.16.149.3                        | IPv4   | h3   | ✅ 成功 | 382      | cloudflare |
| 117  | www.visa.cn                 | 162.159.153.2                       | IPv4   | h3   | ✅ 成功 | 384      | cloudflare |
| 193  | toy-people.com              | 104.26.3.36                         | IPv4   | h3   | ✅ 成功 | 384      | cloudflare |
| 34   | huxley.ns.cloudflare.com    | 2a06:98c1:50::ac40:23bc             | IPv6   | h3   | ✅ 成功 | 385      | cloudflare |
| 84   | cu.877774.xyz               | 104.26.4.112                        | IPv4   | h3   | ✅ 成功 | 385      | cloudflare |
| 179  | cloudflare.182682.xyz       | 2a06:98c1:3120::5692:61a4           | IPv6   | h3   | ✅ 成功 | 385      | cloudflare |
| 289  | time.is                     | 2606:4700:20::681a:d36              | IPv6   | h3   | ✅ 成功 | 385      | cloudflare |
| 104  | www.okcupid.com             | 104.16.144.63                       | IPv4   | h3   | ✅ 成功 | 386      | cloudflare |
| 177  | cloudflare.182682.xyz       | 2606:4700:3035::1a4f:5642           | IPv6   | h3   | ✅ 成功 | 386      | cloudflare |
| 187  | freeyx.cloudflare88.eu.org  | 141.101.120.65                      | IPv4   | h3   | ✅ 成功 | 387      | cloudflare |
| 20   | www.pcmag.com               | 104.16.21.118                       | IPv4   | h3   | ✅ 成功 | 388      | cloudflare |
| 172  | cloudflare.182682.xyz       | 104.18.185.26                       | IPv4   | h3   | ✅ 成功 | 388      | cloudflare |
| 188  | na.877774.xyz               | 104.18.187.25                       | IPv4   | h3   | ✅ 成功 | 388      | cloudflare |
| 326  | 456.cloudflare.182682.xyz   | 2606:4700:20::681a:8a0              | IPv6   | h3   | ✅ 成功 | 389      | cloudflare |
| 369  | dnschecker.org              | 2606:4700:20::681a:659              | IPv6   | h3   | ✅ 成功 | 389      | cloudflare |
| 46   | www.gov.ua                  | 104.21.23.72                        | IPv4   | h3   | ✅ 成功 | 390      | cloudflare |
| 186  | freeyx.cloudflare88.eu.org  | 141.101.121.127                     | IPv4   | h3   | ✅ 成功 | 390      | cloudflare |
| 275  | www.ipchicken.com           | 104.26.6.112                        | IPv4   | h3   | ✅ 成功 | 390      | cloudflare |
| 411  | lewis.ns.cloudflare.com     | 2803:f800:50::6ca2:c39f             | IPv6   | h3   | ✅ 成功 | 390      | cloudflare |
| 52   | 172.67.110.232              | 172.67.110.232                      | IPv4   | h3   | ✅ 成功 | 391      | cloudflare |
| 427  | www.csgo.com                | 195.85.59.161                       | IPv4   | h3   | ✅ 成功 | 391      | cloudflare |
| 446  | japan.com                   | 104.26.5.60                         | IPv4   | h3   | ✅ 成功 | 391      | cloudflare |
| 213  | www.whatismyip.com          | 104.26.13.23                        | IPv4   | h3   | ✅ 成功 | 392      | cloudflare |
| 302  | palera.in                   | 2606:4700:3032::ac43:9d7a           | IPv6   | h3   | ✅ 成功 | 392      | cloudflare |
| 41   | comicabc.com                | 104.21.64.10                        | IPv4   | h3   | ✅ 成功 | 393      | cloudflare |
| 48   | www.gov.ua                  | 2606:4700:3031::6815:1748           | IPv6   | h3   | ✅ 成功 | 393      | cloudflare |
| 76   | www.4chan.org               | 104.16.229.229                      | IPv4   | h3   | ✅ 成功 | 393      | cloudflare |
| 430  | abdullah.ns.cloudflare.com  | 172.64.35.203                       | IPv4   | h3   | ✅ 成功 | 393      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 0 条记录
- **正常 (100-200ms)**: 24 条记录
- **慢 (200-500ms)**: 176 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 6 次
- **IPv6 失败**: 1 次

### 按协议统计

- **none**: 7 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
